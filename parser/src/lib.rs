//
//  0 1 0  Mnemosyne: a functional systems programming language.
//  0 0 1  (c) 2015 Hawk Weisman
//  1 1 1  hi@hawkweisman.me
//
//  Mnemosyne is released under the MIT License. Please refer to
//  the LICENSE file at the top-level directory of this distribution
//  or at https://github.com/hawkw/mnemosyne/.
//
//! The Mnemosyne parser

extern crate combine;
extern crate combine_language;
extern crate mnemosyne as core;

use combine::*;
use combine_language::{ LanguageEnv
                      , LanguageDef
                      , Identifier
                      };
use combine::primitives::{ Stream
                         , Positioner
                         , SourcePosition
                         };
use core::chars;
use core::semantic::*;
use core::semantic::annotations::{ Annotated
                                 , UnscopedState
                                 , Unscoped
                                 };
use core::semantic::types::*;
use core::semantic::ast::*;
use core::position::*;

use std::rc::Rc;
use std::collections::HashMap;
use std::hash::Hash;

type ParseFn<'a, I, T> = fn (&MnEnv<'a, I>, State<I>) -> ParseResult<T, I>;

type U = UnscopedState;

mod tests;

/// Wraps a parsing function with a language definition environment.
///
/// TODO: this could probably push identifiers to the symbol table here?
#[derive(Copy)]
struct MnParser<'a: 'b, 'b, I, T>
where I: Stream<Item=char>
    , I::Range: 'b
    , I: 'b
    , I: 'a
    , T: 'a {
        env: &'b MnEnv<'a, I>
      , parser: ParseFn<'a, I, T>
}

impl<'a, 'b, I, T> Clone for MnParser<'a, 'b, I, T>
where I: Stream<Item=char>
    , I::Range: 'b
    , I: 'b
    , T: 'a
    , 'a: 'b {

    fn clone(&self) -> Self {
        MnParser { env: self.env , parser: self.parser }
    }
}

impl<'a, 'b, I, T> Parser for MnParser<'a, 'b, I, T>
where I: Stream<Item=char>
    , I::Range: 'b
    , I: 'b
    , T: 'a
    , 'a: 'b {

    type Input = I;
    type Output = T;

    fn parse_state(&mut self, input: State<I>) -> ParseResult<T, I> {
        (self.parser)(self.env, input)
    }

}

struct MnEnv<'a, I>
where I: Stream<Item = char>
    , I::Item: Positioner<Position = SourcePosition>
    , I: 'a {
    env: LanguageEnv<'a, I>
}

impl <'a, I> std::ops::Deref for MnEnv<'a, I>
where I: Stream<Item=char>
    , I: 'a {
    type Target = LanguageEnv<'a, I>;
    fn deref(&self) -> &LanguageEnv<'a, I> { &self.env }
}

impl<'a, 'b, I> MnEnv<'a, I>
where I: Stream<Item=char>
    , I::Item: Positioner<Position = SourcePosition>
    , I::Range: 'b {

    /// Wrap a function into a MnParser with this environment
    fn parser<T>(&'b self, parser: ParseFn<'a, I, T>)
                -> MnParser<'a, 'b, I, T> {
        MnParser { env: self, parser: parser }
    }

    #[allow(dead_code)]
    fn parse_def(&self, input: State<I>) -> ParseResult<Form<'a, U>, I> {
        let function_form
            = self.name()
                  .and(self.annotated_fn())
                  .map(|(name, fun)| DefForm::Function { name: name
                                                       , fun: fun
                                                       });

        let top_level
            = self.name()
                  .and(self.type_name())
                  .and(self.expr())
                  .map(|((name, ty), body)|
                    DefForm::TopLevel { name: name
                                      , annot: ty
                                      , value: Rc::new(body) });

        self.reserved("def").or(self.reserved("define"))
            .with(function_form.or(top_level))
            .map(Form::Define)
            .parse_state(input)
    }

    pub fn data(&'b self) -> MnParser<'a, 'b, I, Data<'a, U>> {
        self.parser(MnEnv::parse_data)
    }

    #[allow(dead_code)]
    fn parse_data(&self, input: State<I>) -> ParseResult<Data<'a, U>, I> {
        let sum = self.parser(MnEnv::parse_sum);
        let record = self.parser(MnEnv::parse_record);

        self.reserved("data")
            .with(sum.or(record))
            .parse_state(input);

        unimplemented!()
    }

    fn parse_sum(&self, input: State<I>)
                -> ParseResult<HashMap<Ident, Variant<'a, U>>, I>
    {
        self.reserved_op("|")
            .parse_state(input);

        unimplemented!()
    }

    fn parse_record(&self, input: State<I>)
                   -> ParseResult<HashMap<Ident, Variant<'a, U>>, I>
    {
        unimplemented!()
    }

    #[allow(dead_code)]
    fn parse_if(&self, input: State<I>) -> ParseResult<Form<'a, U>, I> {
        self.reserved("if")
            .with(self.expr())
            .and(self.expr())
            .and(optional(self.expr()))
            .map(|((cond, if_clause), else_clause)|
                Form::If { condition: Rc::new(cond)
                         , if_clause: Rc::new(if_clause)
                         , else_clause: else_clause.map(Rc::new)
                         })
            .parse_state(input)
    }

    #[allow(dead_code)]
    fn parse_lambda(&self, input: State<I>) -> ParseResult<Form<'a, U>, I> {
        self.reserved("lambda")
            .or(self.reserved(chars::LAMBDA))
            .with(self.function())
            .map(Form::Lambda)
            .parse_state(input)
    }

    #[allow(dead_code)]
    fn parse_function(&self, input: State<I>)
                    -> ParseResult<Function<'a, U>, I> {
        let fn_kwd = choice([ self.reserved("fn")
                            , self.reserved("lambda")
                            , self.reserved(chars::LAMBDA)
                            ]);

        self.parens(fn_kwd.with(
            self.signature()
                .and(many1(self.equation()))
                .map(|(sig, eqs)| Function { sig: sig
                                           , equations: eqs
                                           })
                ))
            .parse_state(input)
    }

    /// Parses a function annotated with a position.
    ///
    /// Needed for def functions but not lambdas
    fn parse_fn_pos(&self, input: State<I>)
                    -> ParseResult<Annotated<'a, Function<'a, U>, U>, I> {
        let pos = input.position.clone();
        self.function()
            .map(|fun| Annotated::new(fun, Position::from(pos)))
            .parse_state(input)
    }

    fn annotated_fn(&'b self) -> MnParser<'a, 'b, I
                                         , Annotated<'a, Function<'a, U>, U>>
    {
        self.parser(MnEnv::parse_fn_pos)
    }

    #[allow(dead_code)]
    fn parse_primitive_ty(&self, input: State<I>) -> ParseResult<Type, I> {
        choice([ self.reserved("int")
                     .with(value(Primitive::IntSize))
               , self.reserved("uint")
                     .with(value(Primitive::IntSize))
               , self.reserved("float")
                     .with(value(Primitive::Float))
               , self.reserved("double")
                     .with(value(Primitive::Double))
               , self.reserved("bool")
                     .with(value(Primitive::Bool))
               , self.reserved("i8")
                     .with(value(Primitive::Int(Int::Int8)))
               , self.reserved("i16")
                     .with(value(Primitive::Int(Int::Int16)))
               , self.reserved("i32")
                    .with(value(Primitive::Int(Int::Int32)))
               , self.reserved("i64")
                     .with(value(Primitive::Int(Int::Int64)))
              , self.reserved("u8")
                    .with(value(Primitive::Uint(Int::Int8)))
              , self.reserved("u16")
                    .with(value(Primitive::Uint(Int::Int16)))
              , self.reserved("u32")
                    .with(value(Primitive::Uint(Int::Int32)))
              , self.reserved("u64")
                    .with(value(Primitive::Uint(Int::Int64)))
               ])
               .map(|primitive| Type::Prim(primitive))
               .parse_state(input)
    }

    pub fn raw_ptr_ty(&self, input: State<I>) -> ParseResult<Type, I> {
        char('*').with(self.type_name())
                 .map(|t| Type::Ref(Reference::Raw(Rc::new(t))))
                 .parse_state(input)
    }

    pub fn unique_ptr_ty(&self, input: State<I>) -> ParseResult<Type, I> {
        char('@').with(self.type_name())
                 .map(|t| Type::Ref(Reference::Unique(Rc::new(t))))
                 .parse_state(input)
    }

    pub fn borrow_ptr_ty(&self, input: State<I>) -> ParseResult<Type, I> {
        char('&').with(self.type_name())
                 .map(|t| Type::Ref(Reference::Borrowed(Rc::new(t))))
                 .parse_state(input)
    }
    fn parse_type(&self, input: State<I>) -> ParseResult<Type, I> {
        choice([ self.parser(MnEnv::parse_primitive_ty)
               , self.parser(MnEnv::raw_ptr_ty)
               , self.parser(MnEnv::unique_ptr_ty)
               , self.parser(MnEnv::borrow_ptr_ty)
               ])
            .parse_state(input)
    }

    fn parse_name_deref(&self, input: State<I>) -> ParseResult<NameRef, I> {
        char('$').with(self.name())
                 .map(NameRef::Deref)
                 .parse_state(input)
    }

    fn parse_name_unique(&self, input: State<I>) -> ParseResult<NameRef, I> {
        char('@').with(self.name())
                 .map(NameRef::Unique)
                 .parse_state(input)
    }

    fn parse_name_borrow(&self, input: State<I>)-> ParseResult<NameRef, I> {
        char('&').with(self.name())
                 .map(NameRef::Borrowed)
                 .parse_state(input)
    }

    fn parse_owned_name(&self, input: State<I>) -> ParseResult<NameRef, I> {
        self.name()
            .map(NameRef::Owned)
            .parse_state(input)
    }

    fn parse_name_ref(&self, input: State<I>)
                     -> ParseResult<Form<'a, U>, I> {
        choice([ self.parser(MnEnv::parse_name_deref)
               , self.parser(MnEnv::parse_name_unique)
               , self.parser(MnEnv::parse_name_borrow)
               , self.parser(MnEnv::parse_owned_name)
               ])
            .map(Form::NameRef)
            .parse_state(input)
    }

    // fn parse_typeclass_arrow(&self, input: State<I>) -> ParseResult<&str, I> {
    //     self.reserved_op("=>")
    //         .or(self.reserved_op(FAT_ARROW))
    //         .parse_state(input)
    // }

    // fn parse_arrow(&self, input: State<I>) -> ParseResult<&str, I> {
    //     self.reserved_op("->")
    //         .or(self.reserved_op(ARROW))
    //         .parse_state(input)
    // }

    fn parse_prefix_constraint(&self, input: State<I>)
                              -> ParseResult<Constraint, I> {
        self.parens(self.reserved_op("=>")
                        .or(self.reserved_op(chars::FAT_ARROW))
                        .with(self.name())
                        .and(many1(self.name())) )
            .map(|(c, gs)| Constraint { typeclass: c
                                      , generics: gs })
            .parse_state(input)
    }

    fn parse_infix_constraint(&self, input: State<I>)
                              -> ParseResult<Constraint, I> {
        self.braces(self.name()
                        .skip(self.reserved_op("=>")
                                  .or(self.reserved_op(chars::FAT_ARROW)))
                        .and(many1(self.name())) )
            .map(|(c, gs)| Constraint { typeclass: c
                                      , generics: gs })
            .parse_state(input)
    }

    fn parse_constraint(&self, input: State<I>)
                              -> ParseResult<Constraint, I> {
        self.parser(MnEnv::parse_prefix_constraint)
            .or(self.parser(MnEnv::parse_infix_constraint))
            .parse_state(input)
    }

    pub fn constraint(&'b self) -> MnParser<'a, 'b, I, Constraint> {
        self.parser(MnEnv::parse_constraint)
    }

    fn parse_prefix_sig(&self, input: State<I>) -> ParseResult<Signature, I> {
        self.parens(self.reserved_op("->")
                        .or(self.reserved_op(chars::ARROW))
                        .with(optional(many1(self.constraint())))
                        .and(many1(self.type_name())) )
            .map(|(cs, glob)| Signature { constraints: cs
                                        , typechain: glob })
            .parse_state(input)
    }

    fn parse_infix_sig(&self, input: State<I>) -> ParseResult<Signature, I> {
        self.braces(optional(
            many1(self.constraint()))
                .and(sep_by1::< Vec<Type>, _, _>(
                    self.lex(self.type_name())
                  , self.reserved_op("->")
                        .or(self.reserved_op(chars::ARROW))
                )))
            .map(|(cs, glob)| Signature { constraints: cs
                                        , typechain: glob })
            .parse_state(input)
    }

    fn parse_signature(&self, input: State<I>) -> ParseResult<Signature, I> {
        // let prefix =
        //     self.parens(self.reserved_op("->")
        //                     .or(self.reserved_op(ARROW))
        //                     .with(optional(many1(self.constraint())))
        //                     .and(many1(self.type_name())) )
        //         .map(|(cs, glob)| Signature { constraints: cs
        //                                     , typechain: glob });
        //
        // let infix =
        //     self.braces(optional(many1(self.constraint()))
        //                     .and(sep_by1::< Vec<Type>
        //                                   , _, _>( self.lex(self.type_name())
        //                                          , self.reserved_op("->")
        //                                                .or(self.reserved_op(ARROW))
        //                                          )))
        //         .map(|(cs, glob)| Signature { constraints: cs
        //                                     , typechain: glob });
        // prefix.or(infix)
        //       .parse_state(input)
        self.parser(MnEnv::parse_prefix_sig)
            .or(self.parser(MnEnv::parse_infix_sig))
            .parse_state(input)
    }

    pub fn signature(&'b self) -> MnParser<'a, 'b, I, Signature> {
        self.parser(MnEnv::parse_signature)
    }

    fn parse_binding(&self, input: State<I>)
                    -> ParseResult<Unscoped<'a, Binding<'a, U>>, I> {
        let pos = input.position.clone();
        self.parser(MnEnv::parse_name)
            .and(self.type_name())
            .and(self.expr())
            .map(|((name, typ), value)|
                Annotated::new( Binding { name: name
                                        , typ: typ
                                        , value: Rc::new(value)
                                        }
                               , Position::from(pos)
                           ))
            .parse_state(input)
    }

    #[allow(dead_code)]
    fn parse_logical(&self, input: State<I>)
                    -> ParseResult<Logical<'a, U>, I> {
        let and = self.reserved("and")
                      .with(self.expr())
                      .and(self.expr())
                      .map(|(a, b)| Logical::And { a: Rc::new(a)
                                                 , b: Rc::new(b)
                                                 });

         let or = self.reserved("or")
                      .with(self.expr())
                      .and(self.expr())
                      .map(|(a, b)| Logical::And { a: Rc::new(a)
                                                 , b: Rc::new(b)
                                                 });

        and.or(or)
           .parse_state(input)
    }

    pub fn int_const(&'b self) -> MnParser<'a, 'b, I, Literal> {
        self.parser(MnEnv::parse_int_const)
    }

    #[allow(dead_code)]
    fn parse_int_const(&self, input: State<I>) -> ParseResult<Literal, I> {
        self.integer()
            .map(Literal::IntConst)
            .parse_state(input)
    }

    fn parse_let(&self, input: State<I>) -> ParseResult<Form<'a, U>, I> {

        let binding_form =
            self.reserved("let")
                .with(self.parens(many(self.parens(self.binding()))))
                .and(many(self.expr()))
                .map(|(bindings, body)| LetForm::Let { bindings: bindings
                                                     , body: body });

        choice([ binding_form ])
            .map(Form::Let)
            .parse_state(input)
    }

    fn parse_name (&self, input: State<I>) -> ParseResult<Ident, I> {
        let position = input.position.clone();
        self.env.identifier::<'b>()
            .map(|name| Positional { pos: Position::from(position)
                                   , value: name })
            .parse_state(input)
    }

    fn parse_call(&self, input: State<I>) -> ParseResult<Form<'a, U>, I> {
        self.name()
            .and(many(self.expr()))
            .map(|(name, args)| Form::App(AppForm{ fun: name, params: args }))
            .parse_state(input)
    }

    fn parse_expr(&self, input: State<I>) -> ParseResult<Expr<'a, U>, I> {
        let pos = Position::from(input.position.clone());
        self.env.parens(choice([ try(self.call())
                               , try(self.def())
                               , try(self.if_form())
                               , try(self.lambda())
                               , try(self.let_form())
                               ]))
            .or(try(self.int_const()
                        .map(Form::Lit)))
            .or(try(self.name_ref()))
            .map(|f| Annotated::new(f, pos) )
            .parse_state(input)
    }

    fn parse_pattern(&self, input: State<I>) -> ParseResult<Pattern, I> {
        let pat_elem =
            self.name().map(PatElement::Name)
                .or(self.int_const().map(PatElement::Lit));

        self.parens(many(pat_elem))
            .parse_state(input)
    }

    pub fn pattern(&'b self) -> MnParser<'a, 'b, I, Pattern> {
        self.parser(MnEnv::parse_pattern)
    }

    fn parse_equation(&self, input: State<I>)
                     -> ParseResult< Annotated< 'a
                                              , Equation< 'a, U>
                                              , U>
                                    , I> {
        let pos = Position::from(input.position.clone());
        self.parens(self.pattern()
                        .and(many(self.expr())))
            .map(|(pat, body)| Annotated::new( Equation { pattern: pat
                                                        , body: body }
                                             , pos ))
            .parse_state(input)
    }

    pub fn equation(&'b self) -> MnParser< 'a, 'b, I
                                     , Annotated< 'a
                                                , Equation<'a, U>
                                                , U>
                                     > {
        self.parser(MnEnv::parse_equation)
    }

    pub fn expr(&'b self) -> MnParser<'a, 'b, I, Expr<'a, U>> {
        self.parser(MnEnv::parse_expr)
    }

    pub fn def(&'b self) -> MnParser<'a, 'b, I, Form<'a, U>> {
        self.parser(MnEnv::parse_def)
    }

    pub fn if_form(&'b self) -> MnParser<'a, 'b, I, Form<'a, U>> {
        self.parser(MnEnv::parse_if)
    }

    pub fn let_form(&'b self) -> MnParser<'a, 'b, I, Form<'a, U>> {
        self.parser(MnEnv::parse_let)
    }

    pub fn lambda(&'b self)-> MnParser<'a, 'b, I, Form<'a, U>> {
        self.parser(MnEnv::parse_lambda)
    }

    pub fn call(&'b self) -> MnParser<'a, 'b, I, Form<'a, U>> {
        self.parser(MnEnv::parse_call)
    }

    pub fn name(&'b self) -> MnParser<'a, 'b, I, Ident> {
        self.parser(MnEnv::parse_name)
    }

    pub fn name_ref(&'b self) -> MnParser<'a, 'b, I, Form<'a, U>> {
        self.parser(MnEnv::parse_name_ref)
    }


    pub fn binding(&'b self)
              -> MnParser< 'a, 'b, I, Unscoped<'a, Binding<'a, U>>> {
        self.parser(MnEnv::parse_binding)
    }

    pub fn type_name(&'b self) -> MnParser<'a, 'b, I, types::Type> {
        self.parser(MnEnv::parse_type)
    }

    pub fn function(&'b self) -> MnParser<'a, 'b, I, Function<'a, U>> {
        self.parser(MnEnv::parse_function)
    }

}
pub fn parse_module<'a>(code: &'a str)
                        -> Result< Vec<Expr<'a, UnscopedState>>
                                 , ParseError<&'a str>>
 {
    let env = LanguageEnv::new(LanguageDef {
        ident: Identifier {
            start: letter().or(satisfy(move |c| chars::ALPHA_EXT.contains(c)))
          , rest: alpha_num().or(satisfy(move |c| chars::ALPHA_EXT_SUBSEQUENT.contains(c)))
          , reserved: [ // a number of these reserved words have no meaning yet
                        "and"               , "begin"
                      , "case"              , "cond"        , "class"
                      , "data"
                      , "define"            , "defn"        , "def"
                      , "delay"             , "fn"
                      , "do"                , "else"
                      , "if"                , "lambda"      , chars::LAMBDA
                      , "let"               , "let*"        , "letrec"
                      , "or"
                      , "quasiquote"        , "quote"       , "unquote"
                      , "set!"              , "unquote-splicing"
                      , "struct"            , "union"
                      , "i8"                , "u8"
                      , "i16"               , "u16"
                      , "i32"               , "u32"         , "f32"
                      , "i64"               , "u64"         , "f64"
                      , "int"               , "uint"        , "float"
                      , "bool"              , "string"      , "double"
                      , "ref"               , "move"        , "borrow"
                      , "trait"             , "typeclass"
                      , "instance"          , "impl"
                      ].iter().map(|x| (*x).into())
                       .collect()
        }
      , op: Identifier {
            start: satisfy(move |c| chars::OPS.contains(c))
          , rest:  satisfy(move |c| chars::OPS.contains(c))
          , reserved: [ "=>" , "->" , "\\" , "|" , "_" , "$"
                      , chars::ARROW , chars::FAT_ARROW
                      ].iter().map(|x| (*x).into())
                       .collect()
        }
      , comment_line: string("#").map(|_| ())
      , comment_start: string("#|").map(|_| ())
      , comment_end: string("|#").map(|_| ())
    });
    let env = MnEnv { env: env };

    env.white_space()
       .with(many1::<Vec<Expr<'a, U>>, _>(env.expr()))
       .parse(code)
       .map(|(e, _)| e)
}

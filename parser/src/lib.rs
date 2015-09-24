//
// Mnemosyne: a functional systems programming language.
// (c) 2015 Hawk Weisman
//
// Mnemosyne is released under the MIT License. Please refer to
// the LICENSE file at the top-level directory of this distribution
// or at https://github.com/hawkw/mnemosyne/.
//

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

use core::semantic::*;
use core::semantic::annotations::{ Annotated
                                 , UnscopedState
                                 , Unscoped
                                 };
use core::semantic::types::*;
use core::semantic::ast::*;
use core::position::*;

use std::rc::Rc;

type ParseFn<'a, I, T> = fn (&MnEnv<'a, I>, State<I>) -> ParseResult<T, I>;

type U = UnscopedState;

/// Unicode code point for the lambda character
const LAMBDA: &'static str      = "\u{03bb}";
const ARROW: &'static str       = "\u{8594}";
const FAT_ARROW: &'static str   = "\u{8685}";

mod tests;

/// Wraps a parsing function with a language definition environment.
///
/// TODO: this could probably push identifiers to the symbol table here?
#[derive(Copy)]
struct MnParser<'a: 'b, 'b, I, T>
where I: Stream<Item=char>
    , I::Range: 'b
    , I: 'b {
        env: &'b MnEnv<'a, I>
      , parser: ParseFn<'a, I, T>
}

impl<'a, 'b, I, T> Clone for MnParser<'a, 'b, I, T>
where I: Stream<Item=char>
    , I::Range: 'b
    , I: 'b
    , 'a: 'b {

    fn clone(&self) -> Self {
        MnParser { env: self.env , parser: self.parser }
    }
}

impl<'a, 'b, I, T> Parser for MnParser<'a, 'b, I, T>
where I: Stream<Item=char>
    , I::Range: 'b
    , I: 'b
    , 'a: 'b {

    type Input = I;
    type Output = T;

    fn parse_state(&mut self, input: State<I>) -> ParseResult<T, I> {
        (self.parser)(self.env, input)
    }

}

struct MnEnv<'a, I>
where I: Stream<Item = char>
    , I::Item: Positioner<Position = SourcePosition> {
    env: LanguageEnv<'a, I>
}

impl <'a, I> std::ops::Deref for MnEnv<'a, I>
where I: Stream<Item=char> {
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
                  .and(self.function())
                  .map(|(name, fun)| DefForm::Function { name: name
                                                       , fun: fun });
        let top_level
            = self.name()
                  .and(self.type_name())
                  .and(self.expr())
                  .map(|((name, ty), body)|
                    DefForm::TopLevel { name: name
                                      , annot: ty
                                      , value: Rc::new(body) });

        self.reserved("defn").or(self.reserved("define"))
            .with(function_form.or(top_level))
            .map(Form::Define)
            .parse_state(input)
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
            .or(self.reserved(LAMBDA))
            .with(self.function())
            .map(Form::Lambda)
            .parse_state(input)
    }

    #[allow(dead_code)]
    fn parse_function(&self, input: State<I>) -> ParseResult<Function<'a, U>, I> {
        unimplemented!()
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

    fn raw_ptr_ty(&self, input: State<I>) -> ParseResult<Type, I> {
        char('*').with(self.type_name())
                 .map(|t| Type::Ref(Reference::Raw(Rc::new(t))))
                 .parse_state(input)
    }

    fn unique_ptr_ty(&self, input: State<I>) -> ParseResult<Type, I> {
        char('@').with(self.type_name())
                 .map(|t| Type::Ref(Reference::Unique(Rc::new(t))))
                 .parse_state(input)
    }

    fn borrow_ptr_ty(&self, input: State<I>) -> ParseResult<Type, I> {
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
        char('*').with(self.name())
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
                        .or(self.reserved_op(FAT_ARROW))
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
                                  .or(self.reserved_op(FAT_ARROW)))
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

    fn constraint(&'b self) -> MnParser<'a, 'b, I, Constraint> {
        self.parser(MnEnv::parse_constraint)
    }

    fn parse_prefix_sig(&self, input: State<I>) -> ParseResult<Signature, I> {
        self.parens(self.reserved_op("->")
                        .or(self.reserved_op(ARROW))
                        .with(optional(many1(self.constraint())))
                        .and(many1(self.type_name())) )
            .map(|(cs, glob)| Signature { constraints: cs
                                        , typechain: glob })
            .parse_state(input)
    }

    fn parse_infix_sig(&self, input: State<I>) -> ParseResult<Signature, I> {
        self.braces(optional(many1(self.constraint()))
                        .and(sep_by1::< Vec<Type>
                                      , _, _>( self.lex(self.type_name())
                                             , self.reserved_op("->")
                                                   .or(self.reserved_op(ARROW))
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

    fn signature(&'b self) -> MnParser<'a, 'b, I, Signature> {
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

    fn int_const(&'b self) -> MnParser<'a, 'b, I, Const> {
        self.parser(MnEnv::parse_int_const)
    }

    #[allow(dead_code)]
    fn parse_int_const(&self, input: State<I>) -> ParseResult<Const, I> {
        self.integer()
            .map(Const::IntConst)
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
            .map(|(name, args)| Form::Call { fun: name, body: args })
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
                        .map(Form::Constant)))
            .or(try(self.name_ref()))
            .map(|f| Annotated::new(f, pos) )
            .parse_state(input)
    }

    fn expr(&'b self) -> MnParser<'a, 'b, I, Expr<'a, U>> {
        self.parser(MnEnv::parse_expr)
    }

    fn def(&'b self) -> MnParser<'a, 'b, I, Form<'a, U>> {
        self.parser(MnEnv::parse_def)
    }

    fn if_form(&'b self) -> MnParser<'a, 'b, I, Form<'a, U>> {
        self.parser(MnEnv::parse_if)
    }

    fn let_form(&'b self) -> MnParser<'a, 'b, I, Form<'a, U>> {
        self.parser(MnEnv::parse_let)
    }

    fn lambda(&'b self)-> MnParser<'a, 'b, I, Form<'a, U>> {
        self.parser(MnEnv::parse_lambda)
    }

    fn call(&'b self) -> MnParser<'a, 'b, I, Form<'a, U>> {
        self.parser(MnEnv::parse_call)
    }

    fn name(&'b self) -> MnParser<'a, 'b, I, Ident> {
        self.parser(MnEnv::parse_name)
    }

    fn name_ref(&'b self) -> MnParser<'a, 'b, I, Form<'a, U>> {
        self.parser(MnEnv::parse_name_ref)
    }


    fn binding(&'b self)
              -> MnParser< 'a, 'b, I, Unscoped<'a, Binding<'a, U>>> {
        self.parser(MnEnv::parse_binding)
    }

    fn type_name(&'b self) -> MnParser<'a, 'b, I, types::Type> {
        self.parser(MnEnv::parse_type)
    }

    fn function(&'b self) -> MnParser<'a, 'b, I, Function<'a, U>> {
        self.parser(MnEnv::parse_function)
    }

}
pub fn parse_module<'a>(code: &'a str)
                        -> Result< Vec<Expr<'a, UnscopedState>>
                                 , ParseError<&'a str>>
 {
    let alpha_ext = "+-*/<=>!:$%_^";
    let ops = "+-*/|=<>";
    let env = LanguageEnv::new(LanguageDef {
        ident: Identifier {
            start: letter().or(satisfy(move |c| alpha_ext.contains(c)))
          , rest: alpha_num().or(satisfy(move |c| alpha_ext.contains(c)))
          , reserved: [ // a number of these reserved words have no meaning yet
                        "and"               , "begin"
                      , "case"              , "cond"        , "class"
                      , "data"
                      , "define"            , "defn"        , "def"
                      , "delay"             , "fn"
                      , "do"                , "else"
                      , "if"                , "lambda"      , LAMBDA
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
                      , "bool"                              , "double"
                      , "ref"               , "move"        , "borrow"
                      , "trait"             , "typeclass"
                      , "instance"          , "impl"
                      ].iter().map(|x| (*x).into())
                       .collect()
        }
      , op: Identifier {
            start: satisfy(move |c| ops.contains(c))
          , rest:  satisfy(move |c| ops.contains(c))
          , reserved: [ "=>", "->", "\\", "|", ARROW, FAT_ARROW]
                .iter().map(|x| (*x).into()).collect()
        }
      , comment_line: string(";").map(|_| ())
      , comment_start: string("#|").map(|_| ())
      , comment_end: string("|#").map(|_| ())
    });
    let env = MnEnv { env: env };

    env.white_space()
       .with(many1::<Vec<Expr<'a, U>>, _>(env.expr()))
       .parse(code)
       .map(|(e, _)| e)
}

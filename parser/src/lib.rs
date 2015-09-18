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
const LAMBDA: &'static str = "\u{03bb}";

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
                  .and(self.type_annotation())
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
        char('*').with(self.type_annotation())
                 .map(|t| Type::Ref(Reference::Raw(Rc::new(t))))
                 .parse_state(input)
    }

    fn unique_ptr_ty(&self, input: State<I>) -> ParseResult<Type, I> {
        char('@').with(self.type_annotation())
                 .map(|t| Type::Ref(Reference::Unique(Rc::new(t))))
                 .parse_state(input)
    }

    fn borrow_ptr_ty(&self, input: State<I>) -> ParseResult<Type, I> {
        char('&').with(self.type_annotation())
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

    fn parse_binding(&self, input: State<I>)
                    -> ParseResult<Unscoped<'a, Binding<'a, U>>, I> {
        let pos = input.position.clone();
        self.parser(MnEnv::parse_name)
            .and(self.type_annotation())
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
                .with(many(self.parens(self.binding())))
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
        self.env
            .parens(choice([ self.def()
                           , self.if_form()
                           , self.lambda()
                           , self.let_form()
                           , self.call()
                           ]))
            .or(self.int_const()
                    .map(Form::Constant))
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

    fn binding(&'b self)
              -> MnParser< 'a, 'b, I, Unscoped<'a, Binding<'a, U>>> {
        self.parser(MnEnv::parse_binding)
    }

    fn type_annotation(&'b self) -> MnParser<'a, 'b, I, types::Type> {
        self.parser(MnEnv::parse_type)
    }

    fn function(&'b self) -> MnParser<'a, 'b, I, Function<'a, U>> {
        self.parser(MnEnv::parse_function)
    }

}
pub fn parse_module<'a>(code: &'a str)
                        -> Result< Vec<Expr<'a, UnscopedState>>
                                 , ParseError<&str>>
 {
    let alpha_ext = "+-*/<=>!?:$%_~^";
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
          , reserved: [ "=>", "->", "\\", "|"].iter().map(|x| (*x).into())
                                                     .collect()
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

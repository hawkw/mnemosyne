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
                                 };
use core::semantic::ast::*;
use core::position::*;

type ParseFn<'a, I, T> = fn (&MnEnv<'a, I>, State<I>) -> ParseResult<T, I>;

type U = UnscopedState;

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

        fn parse_def(&self, input: State<I>) -> ParseResult<Form<'a, U>, I> {
            unimplemented!()
        }

        fn parse_expr(&self, input: State<I>) -> ParseResult<Expr<'a, U>, I> {
            let pos = Position::from(input.position.clone());
            self.env
                .parens(choice([
                        self.parser(MnEnv::parse_def)
                    ]))
                .map(|f| Annotated::new(f, pos) )
                .parse_state(input)
        }

        fn expr(&'b self) -> MnParser<'a, 'b, I, Expr<'a, U>> {
            self.parser(MnEnv::parse_expr)
        }

        fn def(&'b self) -> MnParser<'a, 'b, I, Form<'a, U>> {
            unimplemented!()
        }

}
pub fn parse_module<N: ?Sized>(code: &str) -> Result<Vec<N>, ParseError<&str>>
where N: ASTNode + Sized {
    let alpha_ext = "+-*/<=>!?:$%_~^";
    let ops = "+-*/|=<>";
    let env = LanguageEnv::new(LanguageDef {
        ident: Identifier {
            start: letter().or(satisfy(|c| alpha_ext.contains(c)))
          , rest: alpha_num().or(satisfy(|c| alpha_ext.contains(c)))
          , reserved: [ "and"               , "begin"
                      , "case"              , "cond"
                      , "data"              , "define"
                      , "defn"              , "delay"
                      , "do"                , "else"
                      , "if"                , "lambda"
                      , "let"               , "let*"
                      , "letrec"            , "or"
                      , "quasiquote"        , "quote"
                      , "set!"              , "unquote"
                      , "unquote-splicing"
                      ].iter().map(|x| (*x).into()).collect()
        }
      , op: Identifier {
            start: satisfy(|c| ops.contains(c))
          , rest:  satisfy(|c| ops.contains(c))
          , reserved: [ "=>", "->", "\\", "|"].iter().map(|x| (*x).into())
                                                     .collect()
        }
      , comment_line: string(";").map(|_| ())
      , comment_start: string("#|").map(|_| ())
      , comment_end: string("|#").map(|_| ())
    });
    let env = MnEnv { env: env };

    env.white_space()
       .parse(code);

    unimplemented!()
}

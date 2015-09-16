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
use core::semantic::annotations::UnscopedState;
use core::position::*;

type ParseFn<'a, I, T> = fn (&LanguageEnv<'a, I>, State<I>)
                            -> ParseResult<T, I>;

/// Wraps a parsing function with a language definition environment.
///
/// TODO: this could probably push identifiers to the symbol table here?
#[derive(Copy)]
struct MnParser<'a: 'b, 'b, I, T>
where I: Stream<Item=char>
    , I::Range: 'b
    , I: 'b {
        env: &'b LanguageEnv<'a, I>
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

impl<'a, I> MnEnv<'a, I>
where I: Stream<Item=char>
    , I::Item: Positioner<Position = SourcePosition> {

}


fn def<'a, I>(env: &LanguageEnv<'a, I>,
              input: State<I>) -> ParseResult<ast::Form<'a, UnscopedState>, I>
where I: Stream<Item=char> {

    unimplemented!()
}

fn expr<'a, I>(env: &LanguageEnv<'a, I>, input: State<I>)
              -> ParseResult<ast::Expr<'a, UnscopedState>, I>
where I: Stream<Item=char> {
    // env.parens( choice([ form(
    //                     ])
    //           )
    //    .parse_state(input)
    unimplemented!()

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

    env.white_space()
       .parse(code);

    unimplemented!()
}

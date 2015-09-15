extern crate combine;
extern crate combine_language;
extern crate mnemosyne as core;

use combine::*;
use combine_language::{ LanguageEnv
                      , LanguageDef
                      , Identifier
                      };
use combine::primitives::{ Stream };

use core::semantic::*;
use core::position::*;

// type ParseFn<'a, I, T> = fn (&LanguageEnv<'a, I>, State<I>)
//                             -> ParseResult<T, I>;

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

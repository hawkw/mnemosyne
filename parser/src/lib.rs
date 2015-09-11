extern crate combine;
extern crate combine_language;
use combine::*;
use combine::primitives::Stream;
use combine_language::{LanguageEnv, LanguageDef, Identifier};

type ParseFn<'a, I, T> = fn (&LanguageEnv<'a, I>, State<I>)
                            -> ParseResult<T, I>;

struct LangParser<'a: 'b, 'b, I, T>
where I: Stream<Item=char>
    , I: 'b
{
    env: &'b LanguageEnv<'a, I>
  , parser: ParseFn<'a, I, T>
}

#[test]
fn it_works() {
}

use super::parse_module;

use core::semantic::ast::Node;

#[test]
fn test_basic_parse() {
    assert_eq!( parse_module("(+ 1 2)")
                    .unwrap()[0]
                    .to_sexpr(0)
              , "(+ 1 2)" )
}

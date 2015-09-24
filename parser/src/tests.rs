use super::parse_module;

use core::semantic::ast::Node;

macro_rules! expr_test {
    ($name:ident, $code:expr) => {
        #[test]
        fn $name() {
            assert_eq!( parse_module($code)
                            .unwrap()[0]
                            .to_sexpr(0)
                      , $code)
        }
    }
}

expr_test!(test_basic_add, "(+ 1 2)");

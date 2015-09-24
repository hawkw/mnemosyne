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
expr_test!(test_basic_sub, "(- 3 4)");
expr_test!(test_basic_div, "(/ 5 6)");
expr_test!(test_basic_mul, "(* 1 2)");
expr_test!(test_nested_arith_1, "(+ 1 (- 2 3))");
expr_test!(test_nested_arith_2, "(* (+ 1 2) 3 4)");
expr_test!(test_nested_arith_3, "(+ (/ 1 2) (* 3 4))");

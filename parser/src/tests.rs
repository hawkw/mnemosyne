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

expr_test!(test_call_1, "(my_fn 1 2)");
expr_test!(test_call_2, "(my_fn (my_other_fn a_var a_different_var))");
expr_test!(test_call_3
  , "(my_fn (my_other_fn a_var a_different_var) VarWithUppercase Othervar)");
expr_test!(test_call_4
  , "(my_fn (my_other_fn a_var a_different_var) (another_fn a_var))");

expr_test!(test_call_ptr_1, "(my_fn a $b)");
expr_test!(test_call_ptr_2, "(my_fn $a $b)");
expr_test!(test_call_ptr_3, "(my_fn &a)");
expr_test!(test_call_ptr_4, "(my_fn a &b)");
expr_test!(test_call_ptr_5, "(my_fn &a &b)");
expr_test!(test_call_ptr_6, "(my_fn @a)");
expr_test!(test_call_ptr_7, "(my_fn a @b)");
expr_test!(test_call_ptr_8, "(my_fn @a @b)");

expr_test!(test_defsyntax_1,
"(define fac (\u{3bb} (\u{2192} int int)
\t((0) 1)
\t((n) (fac (- n 1))))\n)");

#[test]
fn test_defsyntax_sugar() {
    let string =
r#"(def fac (fn {int -> int}
    ((0) 1)
    ((n) (fac (- n 1)))))"#;
    assert_eq!( parse_module(string).unwrap()[0]
                                   .to_sexpr(0)
              , "(define fac (\u{3bb} (\u{2192} int int)
\t((0) 1)
\t((n) (fac (- n 1))))\n)" )
}

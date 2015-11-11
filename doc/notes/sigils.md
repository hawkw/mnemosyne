# Sigils

In types:
  + `?` indicates an Option (as in `?i64` => `(Option i64)`)
  + `&` borrowed pointer (as in `&i64`)
  + `@` boxed pointer (as in `@i64` => `(Box i64)`, as Box is a type?)
  + `*` raw (unsafe) pointer? (as in `*i64`)

In expressions:
  + `&` construct-a-borrowed-pointer-to operator
    - As in `(& 3256)` which constructs a borrow to 3256
    - Probably also the syntax `&3256` is acceptable since it's a unary operator
    - Possibly this should be done with the word `borrow` instead, IDK (as in `(borrow 3256)`)
  + `@` box operator
    - As in `(@ 3256)` or `@3256` which constructs a borrow to 3256
    - Possibly this should be done with the word `box` instead, IDK
  + `*` raw pointer creation operator
    - as in `(* 3546)` or `*3546`
  + `$` is the pointer-dereference operator?
    - as in `($ a_ptr)` or `(+ 2 $ptr_to_int)`
    - it's pronounced "value of"
    - you can include this in your patterns to dereference a pointer argument?
    - for example:
 ```clojure
    (def add_to_ptr (fn {&i64 -> i64 -> i64}
        (add_to_ptr $a b) (+ a b) )) ; the pattern `$a b` captures the first argument by dereferencing it and the second argument as a move
 ```
  + possibly either `?` or `!` can be used to do the same thing to Options? (unwrap)
    - maybe the `?` operator when passed with arity 1 unwraps or panic/exceptions and with arity 2 unwraps or the second argument
    - as in the following:
```clojure
    (let (defined_option (Some 123))
        (? defined_option)     ; => 123
        (? defined_option 456) ; => 123
    )
    (let (undefined_option (None))
        (? undefined_option 456)      ; => 456
        (? undefined_option (+ 1 2))  ; => 3 (the expression is lazy eval-ed?)
        (? undefined_option)          ; => exception/panic
    )
```
    - single-arity `?` can also be used as a prefix?  as in:
```clojure
    (let (defined_option (Some 5))
       (+ 10 ?defined_option) ; => 15
    )
```
  

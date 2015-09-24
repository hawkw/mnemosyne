Mnemosyne Style Guide
=====================

This is a preliminary style guide for writing idiomatic code in Mnemosyne. Note that as the language grows and evolves, what is considered good style may shift. Also note that none of these guidelines are enforced by the compiler.

Naming
------

The names of types (including function types), type classes, and type aliases should be written in `CamelCase`, with the first letter upper case.

The names of value-level constructs, such as functions, fields in records, and variables, should be written in `snake_case`.

Indentation & Delimiters
------------------------

When using S-expression syntax...

1. **Do not place opening or closing perentheses on their own line.**

   For example, this is considered good stye:
   ```clojure
   (defn fac (I64 -> i64)
        ([0] 1)
        ([n] (fac (- n 1))))
   ```
   While this is not:
   ```clojure
   (defn fac (I64 -> i64)
        ([0] 1)
        ([n]
            (fac (- n 1))
        )
    )
   ```

2. **Indent all subexpressions to the same level.**
   This is of particular
3.

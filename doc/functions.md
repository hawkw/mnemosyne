Def syntax
----------

#### Haskell/Lux style signatures

Matching function syntax with general def (where `(fn <signature> <body>)` means the same thing as `(lambda <signature> <body>)`, and function bodies are defined in the Haskellular pattern-matchy style).

Racket has a "`case-lambda`" construct that is basically the Haskelloid pattern-matchy function binding...

"General def" means the `def` keyword can be reused for defining record/algebraic data types, constants, typeclasses, et cetera.

```clojure
(def quicksort (fn (-> List List)
    (quicksort [Nil] Nil)
    (quicksort [x:xs]
        (++ (filter (< x) xs) (x) (filter (> x) xs))
    )))
```

The same, with infix sugar
```clojure
(def quicksort (fn { List -> List }
    (quicksort [Nil] Nil)
    (quicksort [x:xs] {
            (filter (< x) xs) ++ (x) ++ (filter (> x) xs)
        }
    )))
```

With infix sugar & tab sugar (this is starting to look a lot less like Lisp, to the extent that I had to use Python syntax highlighting.)

```python
def quicksort fn { List -> List }
    quicksort [Nil] Nil
    quicksort [x:xs] {
        (filter (< x) xs) ++ (x) ++ (filter (> x) xs)
    }
```

Without "general def" (which I'm still on the fence about):

```clojure
(def quicksort (-> List List)
    (quicksort [Nil] Nil)
    (quicksort [x:xs]
        (++ (filter (< x) xs) (x) (filter (> x) xs))
    )))
```

```clojure
(def quicksort { List -> List }
    (quicksort [Nil] Nil)
    (quicksort [x:xs] {
            (filter (< x) xs) ++ (x) ++ (filter (> x) xs)
        }
    )))
```

```python
def quicksort { List -> List }
    quicksort [Nil] Nil
    quicksort [x:xs] {
        (filter (< x) xs) ++ (x) ++ (filter (> x) xs)
    }
```

Possibly `(defn <name> <signature> <body>)` could just be sugar for `(def <name> (fn <signature> <body>))`?


### Typed Racket-style signatures

With general def:

```clojure
(def quicksort (fn [xs: List]: List
    ;# quicksort takes place here
    ))
```

Without general def:

```clojure
(def quicksort [xs: List]: List
    ;# quicksort takes place here
    )
```

With more than one argument.

```clojure
(def str-concat ([a: String] [b: String]): String
    (+ a b))
```

Or, Max-style (which I still think would be tricky to parse...):

```clojure
(def str-concat ([a String] [b String]) String
    (+ a b))
```

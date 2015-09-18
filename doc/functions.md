Def syntax
----------

#### Haskell/Lux style signatures

Matching function syntax with general def
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

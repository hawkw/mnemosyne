## Sum types

I think they'll look like this:

```clojure
(def data Weekday
    (| Monday ; "|" means product type
       Tuesday
       Wednesday
       Thursday
       Friday
       Saturday
       Sunday ))
```

This means that in curly brace syntax you could write something like this:

```clojure
(def data Weekday { Monday     | Tuesday
                  | Wednesday  | Thursday
                  | Friday     | Saturday
                  | Sunday
                  })
```

Of course, the `|` operator can be used in product type fields, and sum type variants don't have to be tagwords; they can be type expressions.

```clojure
(def data (List a) { (Cons a)
                   | Nil
                   })
```

## Product types

I think they'd use the comma operator?

```clojure
(def data Date
    (, day: Weekday
       month: Month ; where "Month" and "Weekday" are extant sum types
       year: i64
    ))
```

With the curly brace syntax it looks a little more like a `struct` (which is kinda the point):

```clojure
(def data Date {
    day: Weekday,
    month: Month,
    year: i64
    })
```

The comma operator seems really awkward at first, but it looks nice with the curly-brace infix syntax. And I am not really sure what other operator makes sense for product types.

Maybe the same type annotation syntax should be used for both product types and functions? It seems weird that I use the colon syntax here and the arrow syntax in functions. But on the other hand, the colon makes _sense_ here and the arrow, I think, looks weird, as in the following:
```clojure
(def data Date
    (,  (-> day Weekday)
        (-> month Month)
        (-> year i64)
    ))
```

I think it's best to reserve the arrow syntax to specifically refer to functions. Not sure how one would make typeclass constraints make sense in a product type though. `where` looks nice in ML/Haskell but seems weird in a Lispular grammar...

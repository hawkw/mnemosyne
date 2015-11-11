## Sum types

I think they'll look like this:

```clojure
(def Weekday data
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
(def Weekday data { Monday     | Tuesday
                  | Wednesday  | Thursday
                  | Friday     | Saturday
                  | Sunday
                  })
```

Of course, the `|` operator can be used in product type fields, and sum type variants don't have to be tagwords; they can be type expressions.

```clojure
(def (List a) data { (Cons a)
                   | Nil
                   })
```

## Product types

I think they'd use the comma operator?

```clojure
(def Date data
    (, day: Weekday
       month: Month ; where "Month" and "Weekday" are extant sum types
       year: i64
    ))
```

With the curly brace syntax it looks a little more like a `struct` (which is kinda the point):

```clojure
(def Date data {
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

Alternatively, the colon could be a regular S-expression style operator that is used for creating a field in a product type. So we could have the following:

```clojure
(def Date data
    (: day Weekday)
    (: month Month)
    (: year i64))
```

This starts to look like Clojure's record syntax, and looks nice in infix mode:

```clojure
(def Date data
    {day: Weekday}
    {month: Month}
    {year: i64})
```

This still feels more alien to programmers used to the very C-like fake struct above, though...

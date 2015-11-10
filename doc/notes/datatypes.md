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

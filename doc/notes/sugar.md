This is a list of planned convenience syntax and their desugared equivalents.

+ `{a b c}` => `(b a c)`
+ `let a = b, c = d ... in e` => `(let ([a b][c d]...) e)`
+ `a.b` => `(b a)`
+ `a.b(c)` => `(b a c)`
+ `a(b, c)` => `(a b c)`

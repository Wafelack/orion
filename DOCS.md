Documentation
=============

This is not a real documentation, but more of a tutorial to learn the language syntax.

For now, there isn't a big standard library, but you can define yourself your methods in Orion.

Data Types
----------

In orion there are several data types, as listed in the table below:

| Data type | Explanation              | Example |
|-----------|--------------------------|---------|
|  Integer  | A signed 32 bits integer |   `42`  |
|  Single   | A single precision floating point number | `3.1415926535897932` |
|  String   | An UTF-8 character string | `"Hello, World !"` |
|  Tuples   | A finite sequence of elements. | `(, 4 5 6)` | 
|  Lambda   | A lambda abstraction | `(λ (x) y)` |
|   Unit    | An empty constructor | `()` |
|  Constructor | An enum variant constructor | `(Just 88)` |
|    Quote   | An expression which evaluation is delayed | `'(print "bar")` |

Syntax Definition
-----------------

```ebnf
literal := Integer | String | Single | Unit
pattern := literal | Constructor | Tuple | identifier
expression := literal | Constructor | Tuple | identifier | builtin | keyword | Quote
```

Defining Constants
------------------

Syntax: `'(' 'define' identifier expression ')'`.

The `define` keyword is used with an identifier and an expression as the constant value.

Examples: 
```scheme
(define foo 99)
(define bar "Hello")
(define moo bar)
(define bruh (, foo bar moo))
```

Lambdas
-------

Syntax: `'(' ('λ' | 'lambda') '(' identifier* ')' expression)`.

The `lambda` keyword takes zero or more arguments, enclosed by parentheses and a body.

**Note:** Orion has currying for the lambdas, it means that `(λ (x y) (+ x y))` will be converted into `(λ (x) (λ (y) (+ x y)))`.

Examples: 
```scheme
(λ (x) (printf x))
(λ (x y) (* x y))
(λ (x y z) (+ x (+ y z)))
```

Tuples
------

Syntax: `'(' ',' expression* ')'`.

The `,` keyword takes zero or more arguments.

Examples:
```scheme
(, 4 5 6)
(, "a" 42 3.1415826535897932)
(, "a" (, 4 5 6)) ; Yes, tuples can contain tuples as well.
```



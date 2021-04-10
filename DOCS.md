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

Enums
-----

Syntax: `'(' 'enum' identifier '(' identifier identifier* ')' ')'`.

Enums are used to define datatypes with multiple variants, that can contain various data.

They are composed of an enum name and of one or more variants.

**Tip:** If the enum variant has no data, you can ommit the parenteheses. Example: `Nothing <=> (Nothing)`.

Examples:
```scheme
(enum Maybe
	(Just x)
	Nothing) ;; Equivalent to (Nothing)

(enum List
	(Cons x next)
	Nil)
```

Constructors
------------

Syntax: `'(' identifier expression* ')'`.

A variant constructor is defined with the variant name and zero or more expressions.

**Tip:** If the enum variant has no data, you can ommit the parenteheses. Example: `Nothing <=> (Nothing)`.

Examples:
```scheme
(Just 99)
(Just (Just 99))
Nothing
```

Quotes
------

Syntax: `'\'' expression`.

A quote is defined with a quote symbol (`'`) followed by an epxression.

It is used to delay an expression evaluation to call.

Example:
```scheme
(define foo '(printf "Hello, World !"))
foo ;; `Hello, World !` is displayed now, because we evaluate just now.
```

Pattern Matching
----------------

Syntax: `'(' 'match' expression ('(' pattern expression ')')* ')'`.

Match is used to process pattern matching, if the pattern is matched, the expressions is evaluated.

Examples:
```scheme
(define a (Just 42))

(match a
	((Just x) (printf x))) ;; x is not in the upper scope, therefore this pattern matches all Just constructors and adds the value in the scope.

(define b 45105010501)
(match b
	(x (printf x))) ;; x matches any value, because it is not in scope
```

Contributing
------------

This tutorial is probably not very clear, if you have improvements ideas, feel free to make a pull request with your improvements.

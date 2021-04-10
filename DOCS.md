The Orion Reference
================

This "book" goes through all the Orion concepts and stands at the main documentation of the language.

Index
-------

- [Hello, World !](#hello-world-)
- [Fundamentals](#fundamentals)
	- [Prefixed notation](#prefixed-notation)
	- [Basic data types](#basic-data-types)
	- [Defining a constant](#defining-a-constant)
	- [Lambdas](#lambdas)
		- [A quick introduction to lambda calculus](#a-quick-introduction-to-lambda-calculus)
		- [Currying](#currying)
		- [And in Orion ?](#and-in-orion-)
		- [Calling lambdas](#calling-lambdas)
	- [Control flow](#control-flow)
- [Advanced topics](#advanced-topics)
	- [Tuples](#tuples)

Hello, World !
-----------------

Staying classic, here is a `Hello, World !` in Orion:

```scheme
(printf "Hello, World !")
```

Let's break down this program:
- `printf`: A function call, highlighted with the parentheses.
- `"Hello, World !"`: A string literal.

Fundamentals
-----------------

This part will show you the basics of Orion programming, *id est*:
- [Prefixed notation](#prefixed-notation) 
- [Basic data types](#basic-data-types)
- [Defining a constant](#defining-a-constant)
- [Lambdas](#lambdas)
- [Control flow](#control-flow)

### Prefixed notation

Orion, as it is a Lisp dialect, uses prefixed notation. It means that the mathematical expression `3 + 5` will be written `(+ 3 5)` in Orion ; and `3 + 5 + 6` is written `(+ 4 (+ 5 6))`.

### Basic data types

There are 4 basic data types, and we will introduce 4 other types later.
Those types are as following:
| Type | Description |
|-------|-------------
| Integer | A 32 bits signed number (part of the Z set)
| Single  | A 32 bits floating point integer (part of the R set) |
| String | An immutable, fixed length, character string |
| Unit   | An empty thing. |

**Note:** You cannot add a Single with an Integer.

How to use those types: 
```scheme
5 ;; An Integer
3.1415926535897932 ;; A Single
"Hello, World !" ;; A String
() ;; The Unit 
```
### Defining a constant

You can define a constant with the `def` keyword followed by an identifier (the constant name) and an expression.

**Tip:** In Orion, an expression can be anything.

Examples: 
```scheme
(def a 99)
(def b 3.1415)
(def c "Hello")
(def d ())
(def e a) ;; e is not a reference to a, it only contains a's value.
(def f (def a)) ;; `def` returns Unit, so you can do that.
```

### Lambdas

#### A quick introduction to lambda calculus

Lambda calculus[¹](#links) is a formal system to express computation based on function abstraction and application.

If you really don't like maths (even if the following thing is not that tricky to understand), here is the TL;DR (but you are missing something): Lambdas are a way to apply an expression to a value.

In mathematics, a lambda is noted `λ𝑥.𝑀`, where 𝑥 is the variable and 𝑀 is the expression.
With `𝘧 = λ𝑥.𝑥+1`, the lambda application of `𝘧` to the number 5 is noted `𝘧  5` and means `𝘧[𝑥:=5]` (All bound occurrences of 𝑥 replaced with the number 5).

#### Currying

As seen in the previous part, lambdas can only take one argument, but there is a way to write multiple argument lambdas, that is known as currying[²](#links). It permits to write `λ𝑥𝘺.𝘺𝑥` instead of `λ𝑥.(λ𝘺.𝘺𝑥)`.

#### And in Orion ?

Lambda notation in Orion and in math are almost the same. To create a lambda in Orion, you will write:
```scheme
(λ (x) (+ x 1))
```
Currying also applies in Orion, you can define a multiple arguments lambda too, with the following syntax:
```scheme
(λ (x y) (+ x y))
```

#### Calling lambdas

Lambdas are called using prefixed-notation, with the syntax `(function arguments...)`.

Example:
```scheme
(def f (λ (x) (+ x 1)))
(f 5) ;; 6
```

### Control flow

Control flow in Orion is permitted by the `match` keyword, with pattern matching.

Pattern matching, as the name describes, matches an expression with patterns, and if it matches, it evaluates the expression associated with the pattern.

Patterns can be variables, literals (Singles, Integers, Strings or Unit), Constructors or Tuples (we'll see those two later, in the [Advanced Topics section](#advanced-topics).

The syntax of a "pattern line" is `(pattern expression)`.
Example: 
```scheme
(def a 9)
(match a
	(9 (printf "It is nine !")))
```

If you run this code, you should see `It is nine !` appear on screen.

Pattern matching brings another interesting thing: if a pattern is a  variable that does not exist in the current scope, that means that the pattern matches any value, and it is created in the execution scope.
Example:
```scheme
(def a 9)
(match a
	(b (printf "A is " b))) ;; b does not exist in scope
				;; Therefore it matches any value
				;; It is created in the expression scope and its value
				;; is bound to a.
```

Advanced topics
--------------------

### Tuples

Tuples are ordered, fixed size collections of data. They are made using the `,` function with zero or more values in arguments.
Example:
```scheme
(def foo (, "a" 5 2.817))
```

Tuples can also be matched in pattern matching, and their content too.
Example: 
```scheme
(def some_tuple (, 5 6 7))

(match some_tuple
	((, 5 x 7) (printf x " is between 5 and 7 in this tuple !")) ;; Matched because the
								     ;; x matches any value.
	((, 5 x y) (printf "A 5 followed by " x " and " y)))
```



Links
-------

1. https://en.wikipedia.org/wiki/Lambda_calculus
2. https://en.wikipedia.org/wiki/Currying 


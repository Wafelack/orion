<div align=center >
<h1>le cukta be la .orion.</h1>
<h2>A complete Orion tutorial.</h2>
</div>

This book will teach you Orion from 0 to 1 by presenting all the Orion concepts with detailed examples.

Index
-
- [Installing Orion](#installing-orion)
- [Fundamentals](#fundamentals)
	- [Hello, World](#hello-world)
	- [Main concepts](#main-concepts)
	- [The REPL](#the-repl)
	- [Basic Datatypes](#basic-datatypes)
	- [Defining Variables](#defining-variables)
	- [Closures](#closures)
	- [Tuples](#tuples)
	- [Enumerations](#enumerations)
	- [Pattern Matching](#pattern-matching)
- [Appendix I: Macros](#appendix-i-macros)
- [Appendix II: Conventions](#appendix-ii-conventions)
    - [Naming](#naming)
    - [Formatting](#formatting)
- [Appendix III: Shorthands](#appendix-iii-shorthands)

Installing Orion
-

To install Orion, you will need [Git](https://git-scm.org), a "make" program (e.g. [GNU Make](https://www.gnu.org/software/make/)), the [Rust toolchain](https://rustup.rs/) v1.51 or later and a POSIX shell (e.g. [DaSH](http://gondor.apana.org.au/~herbert/dash/)) linked to `/bin/sh`.

First, clone the Orion repository with `git clone https://github.com/orion-lang/orion.git` (add the `--branch dev` option if you would would like to have the nightly toolchain.).
Then, enter the produced directory (`cd orion/`) and make the `configure` script executable (`chmod +x configure`).
Finally, produce a Makefile by running `./configure`, compile the program `make` and then install it with `make install PREFIX=/wherever/you/want` (Please note that root access may be necessary depending of the location.).

Fundamentals
-

In this part, you are going to learn the very basics needed to learn how to program in Orion.

### Hello, World !

Create a file named `main.orn`. Orion files always end with the *.orn* extension.
Then open it in your favourite text editor and put this content in it:
```clojure
(def 'impure main
    (\ () (putStrLn "Hello, World !")))
```
Save the file, go back to your shell and run the following command:
```shell
$ orion main.orn
```
`Hello, World !` should appear on screen.

Let's break the code down:
- `(def main` declares the main variable.
- `(\ ()` declares a closure with 0 arguments.
- `(putStrLn "Hello, World !")` displays `Hello, World !` and a newline on the standard output.

### Main concepts

Orion is a purely functional programming language, that means that it has almost no side effects, and the remaining side effects are controlled. Therefore, mutation is not possible in Orion. Now for I/O, Orion has a special system to control side effects. Haskell uses the IO monad, Pony uses an Env, and Orion uses the `impure` tag. There are 2 main rules about impurity: the top-level is pure, so you cannot call an `impure`-tagged function at the top-level, and you cannot use an `impure`-tagged function in a non-impure variable.

It means that the following codes would fail:
```clojure
(putStrLn "Hello, World !")
```
```clojure
(def foo (putStrLn "Hello, World !"))
```
Try typing this in a file and running it: you should get a compilation error.
To fix the first code, you should put this line in a `main` function. The `main` function is the entry point of every Orion program, and it allows more control on side effects.
To fix the second, you must tag `foo` as `impure`.

### The REPL

The REPL, for *Read Evaluate Print Loop* is an interactive environment to try Orion code ; unlike the files', the REPL's top-level is impure.
To enter the REPL, run `orion` in a shell, without passing it any arguments, you should see an interactive prompt. Then type `(putStrLn "Hello, World !") 4`, and hit the return key.
You should see
```
Hello, World !
=> 4
```

`Hello, World !` is the output string, and `4` is the returned value. The returned value is identified by a `=>` sign before it.

### Basic Datatypes

There are 3 basic datatypes: Integer, Single and String.

| Type | Description | Example |
|-------|---------------|------|
| Integer | A 32 bits relative number | `42` |
| Single | A 32 bits real number | `3.1415926535897932`
| String | A character string. | `"Wafelack"` |


### Defining variables

A variable is defined using the `def` keyword, followed by an identifier and an expression.
Example:
```clojure
(def a 44)
(def b 3.1415)
(def c "Hello, World !")
(def b "foo") ;; Shadowing the variable.
```

### Closures

A closure is defined with the `λ` (or `\` keyword), followed by zero or more arguments enclosed in parentheses and an expression.

Syntax: `(λ (<ident>*) expr)`
Example:
```clojure
(def square ((λ (x) (* x x)))
```
To call a closure, you just write the closure followed by the arguments.

Syntax: `(<closure> <args>*)`.
Example:
```clojure
(def square (λ (n) (* n n)))
(square 13) ;; 169
```

### Tuples

Tuples are ordered, fixed size collections of data. They are made using the `,` function with zero or more values in arguments.

Syntax: `(, <expr>*)`.
Example:
```clojure
(def x (, 4 5 6 7))
```

### Enumerations

#### Declaring an enumeration

An enumeration is a data type containing different variants, and each of these variants can contain values.
They are declared using the `enum` keyword.

Syntax: `(enum { '(' <ident> <ident>* ')' }*)`.
Example:
```clojure
(enum List
    (Cons x next)
    Nil)
```
#### Constructors

Enum constructors are initialized with the enum variant name and the values corresponding to the variant's data.

Syntax:`(<ident> <expr>*)`.
Example:
```clojure
(Cons 4 (Cons 6 Nil))
```

### Pattern matching

Pattern matching is the central feature of Orion. It allows to check if a value matches a pattern, and if it matches, it executes the associated expression.

Patterns can either be:
* A variable, which is bound to the matched value.
* A constructor, containing patterns.
* A tuple, containing patterns.
* An Integer, a Single or a String.
* An "Any" pattern, which matches any value.

Example:
```clojure
(match (, 4 5)
    ((, 4 6) (do_someth)) ;; Does not match
    ((, x y) (nice x y))  ;; Matches, x is boud to 4 and y is bound to 5
    (_ (foo)))            ;; Would match if the previous pattern hasn't been matched.
```

Appendix I: Macros
-

Macros are like functions, but they don't evaluate the arguments. They only replace the variables in the body with the given parameter.
For example, let's take the `if` macro. As a macro, it is written like this:
```clojure
(macro if (cond then else)
    (match cond
        (True then)
        (False else)))
```

If you run `(if (= 3 3) (putStrLn "All good !") (putStrLn "What is going on ?"))`, only `All good !` is displayed, but if `if` was built in a standard function, like this:
```clojure
(def if (λ (cond then else)
    (match cond
        (True then)
        (False else))))
```
, both `All good !` and `What is going on ?` would be displayed.
TL;DR: Macros allows you to manipulate the AST nodes instead of the interpreter Values.

Appendix II: Conventions
-

### Naming

* Variables names should be in snake_case
* Enumerations and variants names should be in PascalCase
* Macros names should be in kebab-case
* Builtins names are in camelCase.

### Formatting

- The indentation should be made with 4 spaces.
- If the function is impure, the closure creation should be on the next line and indented.
  ```closure
  (def 'impure main
      (λ () (putStrLn "Hello, World !")))
  ```
- Every expression should be indented one level more than the parent expression.
  ```closure
  (match a
      (5 10))
  {
      (putStrLn "Hello, World !")
      5}
  ```

Appendix III: Shorthands
-

- `()` expands to `(,)`
- `{x}` expands to `(begin x)`
- `'x` expands to `(\ () x)`

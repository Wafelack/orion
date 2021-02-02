# Getting Started - Orion

This will teach you the basics of Orion.

## Hello, World

We will start traditionnally, with a "Hello, World !".

```scheme
(print "Hello, World !")
```

## Variables

There are 8 types: `int`, `float`, `string`, `bool`, `list`, `object`, `function` and `nil`.

To define a variable, we'll use the `define` function:

```scheme
(define a 5.) ; a is of type float
(print a) ; prints 5
```

To edit a variable, we'll use the `set` function.

Try modyfying the previously defined variable.

You should get an error, this is because `defines` creates an **immutable** variable.

To create a **mutable** variable, we'll use the `var` function.

Try defining a new variable `foo` with the value `"bar"`, then assign it the value `false`.

You get an error, it is because orion is **statically typed**, it means that a variable will keep its original type until it is dropped.

## Conditions

In orion, we use keyword `if` for conditions, used with a condition (boolean) to check, a first scope to execute if the condition is true, and a second scope to execute if it is not:

```scheme
(define foo "bar")

(if (= foo "foo") { ; if it is true
    (print "foo is foo, like expected.")
} { ; else
    (print (+ "foo is not foo, it is " foo))
})
```

This code will print `foo is not foo, it is bar`, because the condition is false.

## Loops

To operate loops, we'll use the `while` function, that is used with a condition to check, and a scope to execute.

A code that will print values from 0 to 10:

```scheme
(var i 0)
(while (< i 10) {
    (set i (+ i 1))
    (print i)
})
```

## Functions

In Orion, we define a function with the `lambda` keyword, which has the following syntax:

```scheme
(lambda (arg0 arg1 ... argN) {
    ; function body
})
```

E.g. a square function:

```scheme
(define square (lambda (x) {
    (* x x) ; this is returned
}))
```

In Orion, the latest value of the function body is returned.

## Macros

Orion implements metaprogramming through **macros**, the macro syntax is as following:

```
macro pattern {replaceWith}
```

E.g:

```scheme
macro foo {(print "bar")}
foo
```

`bar` will appear on screen.

## Pattern matching

The last feature we'll see in this introduction is pattern matching.

Pattern matching is a powerful way to avoid multiple print statements.

In orion, it is implemented with the following syntax:

```scheme
(match <variable> {
    (=> <case> {
        <block>
    })
    (_ {
        <default block>
    })
})
```

E.g:

```scheme
(define foo "bar")
(match foo {
    (=> "foo" { ; block is executed if foo == "foo"
        (print "foo is foo")
    })

    (_ { ; default case
        (print (+ "foo is not foo, but " foo))
    })

    (=> "bar" { ; won't be executed because default case is above
        (print "foo is bar")
    })
})
```

Output: `foo is not foo, but bar`


## Conclusion

Well,now you know the basics of Orion, you can continue your journey with [the documentation](https://github.com/wafelack/orion-lang/wiki) and thanks for interesting in learning Orion.

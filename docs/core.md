Core Library Documentation
==========================

Enumerations
------------

### `Bool`

A boolean type.

```clojure
(enum Bool
	True
	False)
```

Constants
---------

### `#t`

A `True` shorthand.

```clojure
(def #t True)
```

### `#f`

A `False` shorthand.

```clojure
(def #f False)
```

Functions
---------

### `=`

`(=) :: Value -> Value -> Bool`

Test equality between two values

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

### `not`

`not :: Bool -> Bool`

Get the opposite of a Bool.

#### Arguments

* `val :: Bool`: The value to process.

### `/=`

`(/=) :: Value -> Value -> Value`

The opposite of `=`.

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

### `and`

`and :: Bool -> Bool -> Bool`

Test that both arguments are True.

#### Arguments

* `lhs :: Bool`: Left hand side argument.
* `rhs :: Bool`: Right hand side argument.

### `or`

`or :: Bool -> Bool -> Bool`

Test that at least one argument is True.

#### Arguments

* `lhs :: Bool`: Left hand side argument.
* `rhs :: Bool`: Right hand side argument.

### `assert_eq`

`assert_eq :: Value -> Value -> Unit`

Assert equality between two values, panic if False.

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

### `if`

`if :: Bool -> Quote -> Quote -> Value`

Evaluate a condition and do something if it is true, and something different if it is false.

#### Arguments

* `cond :: Bool`    : The condition to evaluate
* `if_expr :: Quote`: The quote to evaluate if `cond` is `True`.
* `else :: Quote`   : The quote to evaluate if `cond` is `False`.


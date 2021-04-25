Bool
====

Enumerations
------------

### `Bool`

A boolean type.

```clojure
(enum Bool
	True
	False)
```

#### Example

`(def foo False)`

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

Test structural equality between two values

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

#### Example

```clojure
(= 5 6) ;; False
(= "Hello" 99) ;; False
(= 3 3) ;; Trues
```

### `not`

`not :: Bool -> Bool`

Get the opposite of a Bool.

#### Arguments

* `val :: Bool`: The value to process.

#### Example

```clojure
(not True) ;; False
(not #f) ;; True
(not 0) ;; Panics, because it is not a Boolean.
```

### `/=`

`(/=) :: Value -> Value -> Value`

The opposite of `=`.

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

#### Examples

```clojure
(/= 3 4) ;; True
(/= "a" 42) ;; True
(/= 5 5) ;; False
```

### `and`

`and :: Bool -> Bool -> Bool`

Test that both arguments are True.

#### Arguments

* `lhs :: Bool`: Left hand side argument.
* `rhs :: Bool`: Right hand side argument.

#### Examples

```clojure
(and (= 5 5) (/= 3 4)) ;; True
(and #t #t) ;; True
(and #f #t) ;; False
```

### `or`

`or :: Bool -> Bool -> Bool`

Test that at least one argument is True.

#### Arguments

* `lhs :: Bool`: Left hand side argument.
* `rhs :: Bool`: Right hand side argument.

#### Examples

```clojure
(or (= 5 5) (/= 3 4)) ;; True
(or #t #t) ;; True
(or #f #t) ;; True
(or #f #f) ;; False
```

### `assert_eq`

`assert_eq :: Value -> Value -> Unit`

Assert equality between two values, panic if False.

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

#### Examples

```clojure
(assert_eq 4 4) ;; Unit
(assert_eq (= 3 4) #t) ;; Panics, because (= 3 4) is false.
```

### `if`

`if :: Bool -> Quote -> Quote -> Value`

Evaluate a condition and do something if it is true, and something different if it is false.

#### Arguments

* `cond :: Bool`    : The condition to evaluate
* `then :: Quote`   : The quote to evaluate if `cond` is `True`.
* `else :: Quote`   : The quote to evaluate if `cond` is `False`.

#### Examples

```clojure
(if (= 3 4)
	'#f
	'#t) ;; True
```


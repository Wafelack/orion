Math
====

Enumerations
------------

### Ordering

Ordering enumeration, used for comparisons between Integers, Singles and Strings.

```clojure
(enum Ordering
	Less
	Equal
	Greater)
```

Functions
---------

### `cmp`

`cmp :: String|Integer|Single -> String|Integer|Single -> Ordering`

Compare two values mathematically.

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

### `<`

`(<) :: String|Integer|Single -> String|Integer|Single -> Bool`

Returns true if `(= (cmp lhs rhs) Less)`

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

### `>`

`(>) :: String|Integer|Single -> String|Integer|Single -> Bool`

Returns true if `(= (cmp lhs rhs) Greater)`

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

### `>=`

`(>=) :: String|Integer|Single -> String|Integer|Single -> Bool`

Returns true if `(or (= lhs rhs) (> lhs rhs))`.

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

### `<=`

`(<=) :: String|Integer|Single -> String|Integer|Single -> Bool`

Returns true if `(or (= lhs rhs) (< lhs rhs))`.

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

### `%`

`(%) :: Integer -> Integer -> Integer`

Returns the remainder of the euclidian division of `lhs` by `rhs`.

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.


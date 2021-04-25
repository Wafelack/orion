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

`(def foo Less)`

Functions
---------

### `cmp`

`cmp :: String|Integer|Single -> String|Integer|Single -> Ordering`

Compare two values mathematically.

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

#### Example

```clojure
(cmp 4 5) ;; Less
(cmp "a" "a") ;; Equal
(cmp 3.1415 0.) ;; Greater
```

### `<`

`(<) :: String|Integer|Single -> String|Integer|Single -> Bool`

Returns true if `(= (cmp lhs rhs) Less)`

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

#### Example

```clojure
(< 3 4) ;; True
(< 3 3) ;; False
```

### `>`

`(>) :: String|Integer|Single -> String|Integer|Single -> Bool`

Returns true if `(= (cmp lhs rhs) Greater)`

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

#### Example

```clojure
(> 3 4) ;; False
(> 5 3) ;; True
```

### `>=`

`(>=) :: String|Integer|Single -> String|Integer|Single -> Bool`

Returns true if `(or (= lhs rhs) (> lhs rhs))`.

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

#### Example

```clojure
(>= 4 4) ;; True
(>= 3 4) ;; False
```

### `<=`

`(<=) :: String|Integer|Single -> String|Integer|Single -> Bool`

Returns true if `(or (= lhs rhs) (< lhs rhs))`.

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

#### Example

```clojure
(<= 3 3) ;; True
(<= 3 4) ;; True
```

### `%`

`(%) :: Integer -> Integer -> Integer`

Returns the remainder of the euclidian division of `lhs` by `rhs`.

#### Arguments

* `lhs :: Value`: Left hand side argument.
* `rhs :: Value`: Right hand side argument.

#### Example

```clojure
(% 10 3) ;; 1
(% 12 3) ;; 0
```

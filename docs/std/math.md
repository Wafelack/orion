Math
====

Functions
---------

### `<`

`(<) :: (Any a) => a -> a -> Bool`

Returns true if `(= (cmp lhs rhs) Less)`

#### Example

```clojure
(< 3 4) ;; True
(< 3 3) ;; False
```

### `>`

`(>) :: (Any a) => a -> a -> Bool`

Returns true if `(= (cmp lhs rhs) Greater)`

#### Example

```clojure
(> 3 4) ;; False
(> 5 3) ;; True
```

### `>=`

`(>=) :: (Any a) => a -> a -> Bool`

Returns true if `(or (= lhs rhs) (> lhs rhs))`.

#### Example

```clojure
(>= 4 4) ;; True
(>= 3 4) ;; False
```

### `<=`

`(<=) :: (Any a) => a -> a -> Bool`

Returns true if `(or (= lhs rhs) (< lhs rhs))`.

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

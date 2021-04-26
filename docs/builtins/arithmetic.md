Arithmetic
==========

`+`
---

`(+) :: Integer|Single... -> Integer|Single`

Adds two or more numbers.

### Example

```clojure
(+ 5 6 7 8) ;; 26
```

`-`
---

`(-) :: Integer|Single... -> Integer|Single`

Substracts one or more numbers from another number.

### Example

```clojure
(- 10 5 2) ;; 3
```


`*`
---

`(*) :: Integer|Single... -> Integer|Single`

Multiplies two or more numbers.

### Example

```clojure
(* 3 4 5) ;; 60
```

`/`
---

`(/) :: Integer|Single... -> Integer|Single`

Divides one or more numbers from another number.

### Example

```clojure
(/ 12 3 2) ;; 2
```

`!`
---

`(!) :: Integer|Single -> Integer|Single`

Returns the opposite of a number.

### Example

```clojure
(! 99) ;; -99
(! (- 0 1)) ;; 1
```

`_cmp`
------

`(_cmp) :: Integer|Single|String -> Integer|Single|String -> Integer`

Compares two Integers/Strings/Singles.

Returns `0` if `lhs < rhs`.
Returns `1` if `lhs == rhs`.
Returns `2` if `lhs > rhs`.

### Example

```clojure
(_cmp 3 4) ;; 0
(_cmp "a" "b") ;; 1
(_cmp 3.1415 3.) ;; 2
```

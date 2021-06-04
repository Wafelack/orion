Arithmetic
==========

`+`
---

`(+) :: (Number a) => a -> a -> a`


Adds two or more numbers.

### Example

```clojure
(+ 7 8) ;; 15
```

`-`
---

`(-) :: (Number a) => a -> a -> a`

Substracts one or more numbers from another number.

### Example

```clojure
(- 10 5) ;; 2
```

`*`
---

`(\*) :: (Number a) => a -> a -> a`


Multiplies two or more numbers.

### Example

```clojure
(* 3 5) ;; 15
```

`/`
---

`(\*) :: (Number a) => a -> a -> a`


Divides one or more numbers from another number.

### Example

```clojure
(/ 12 3) ;; 4
```

`neg`
---

`(neg) :: (Number a) => a -> a`


Returns the opposite of a number.

### Example

```clojure
(neg 99) ;; -99
(neg (- 0 1)) ;; 1
```

`_cmp`
------

`(\_cmp) :: (Any a) => a -> a -> Integer`


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

`cos`
-----

`cos :: Single -> Single`

Returns the cosine of an angle (in radians).

### Example

```clojure
(cos 1.047198) ;; 0.49999955
```

`sin`
-----

`sin :: Single -> Single`

Returns the sine of an angle (in radians).

### Example

```clojure
(sin 0.5235988) ;; 0.5
```

`tan`
-----

`tan :: Single -> Single`

Returns the sine of an angle (in radians).

### Example

```clojure
(tan 0.7853982) ;; 1.
```

`acos`
------

`acos :: Single -> Single`

Returns the arccosine of a number (in radians).

### Example

```clojure
(acos 0.5) ;; 1.0471976
```

`asin`
------

`asin :: Single -> Single`

Returns the arcsine of a number (in radians).

### Example

```clojure
(asin 0.5) ;; 0.5235988
```

`atan`
------

`atan :: Single -> Single`

Returns the arctangent of a number (in radians).

### Example

```clojure
(atan 1.) ;; 0.7853982
```

Bool
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

### `cmp`

`cmp :: forall a . a -> a -> Ordering`

Compare two values mathematically.

#### Example

```clojure
(cmp 4 5) ;; Less
(cmp "a" "a") ;; Equal
(cmp 3.1415 0.) ;; Greater
```


### `=`

`(=) :: forall a . a -> a -> Bool`

Test structural equality between two values

#### Example

```clojure
(= 5 6) ;; False
(= "Hello" 99) ;; False
(= 3 3) ;; Trues
```

### `not`

`not :: Bool -> Bool`

Get the opposite of a Bool.

#### Example

```clojure
(not True) ;; False
(not #f) ;; True
(not 0) ;; Panics, because it is not a Boolean.
```

### `/=`


`(/=) :: forall a . a -> a -> Bool`

The opposite of `(not (=))`.

#### Examples

```clojure
(/= 3 4) ;; True
(/= "a" 42) ;; True
(/= 5 5) ;; False
```

### `and`

`and :: Bool -> Bool -> Bool`

Test that both arguments are True.

#### Examples

```clojure
(and (= 5 5) (/= 3 4)) ;; True
(and #t #t) ;; True
(and #f #t) ;; False
```

### `or`

`or :: Bool -> Bool -> Bool`

Test that at least one argument is True.

#### Examples

```clojure
(or (= 5 5) (/= 3 4)) ;; True
(or #t #t) ;; True
(or #f #t) ;; True
(or #f #f) ;; False
```

### `assert_eq`

`assert_eq :: forall a . a -> a -> Unit`

Assert equality between two values, panic if False.

#### Examples

```clojure
(assert_eq 4 4) ;; Unit
(assert_eq (= 3 4) #t) ;; Panics, because (= 3 4) is false.
```

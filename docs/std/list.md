List
====

Enumerations
------------

### List

A linked list enumeration.

```clojure
(enum List
	(Cons x next)
	Nil)
```

#### Example

```clojure
(def foo (Cons 3 (Cons 4 (Cons 5 Nil))))
(def empty Nil)
```

Functions
---------

### `fill`

`fill :: Value -> Integer -> List`

Fills a list with a Value.

#### Arguments

* `x :: Value`: The value to fill with.
* `n :: Integer`: The amount of values to put in the list.

#### Example

```clojure
(def foo (fill "a" 5)) ;; (Cons "a" (Cons "a" (Cons "a" (Cons "a" (Cons "a" Nil)))))
```

### `car`

`car :: List -> Maybe Value`

Returns `Just` the first element if the list is not empty, and `Nothing` if the list is empty.

#### Arguments

* `list :: List`: The list to get the first value of.

#### Example

```clojure
(def foo (Cons 3 (Cons 4 (Cons 5 Nil))))
(car foo) ;; (Just 3)
```

### `cdn`

`cdn :: List -> Maybe List`

Returns `Just` the list without its first element if it is not empty, and `Nothing` if the list is empty.

#### Arguments

* `list :: List`: The list to get the tail of.

#### Example

```clojure
(def foo (Cons 3 (Cons 4 (Cons 5 Nil))))
(cdn foo) ;; (Just (Cons 4 (Cons 5 Nil)))
```

### `range`

`range :: Integer -> Integer -> List`

Returns a List with the Integers going from `start` to `end`.

#### Arguments

* `start :: Integer`: The starting Integer.
* `end :: Integer`: The ending Integer.

#### Example

```clojure
(def foo (range 0 4)) ;; (Cons 0 (Cons 1 (Cons 2 (Cons 3 Nil))))
```

### `length`

`length :: List -> Integer`

Returns the length of a List.

#### Arguments

* `list :: List`: The list to get the length of.

#### Example

```clojure
(length (Cons 3 (Cons 4 (Cons 5 (Cons 6 Nil))))) ;; 4
```
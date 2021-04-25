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

Functions
---------

### `fill`

`fill :: Value -> Integer -> List`

Fills a list with a Value.

#### Arguments

* `x :: Value`: The value to fill with.
* `n :: Integer`: The amount of values to put in the list.

### `car`

`car :: List -> Maybe Value`

Returns `Just` the first element if the list is not empty, and `Nothing` if the list is empty.

#### Arguments

* `list :: List`: The list to get the first value of.

### `cdn`

`cdn :: List -> Maybe List`

Returns `Just` the list without its first element if it is not empty, and `Nothing` if the list is empty.

#### Arguments

* `list :: List`: The list to get the tail of.

### `range`

`range :: Integer -> Integer -> List`

Returns a List with the Integers going from `start` to `end`.

#### Arguments

* `start :: Integer`: The starting Integer.
* `end :: Integer`: The ending Integer.

### `length`

`length :: List -> Integer`

Returns the length of a List.

#### Arguments

* `list :: List`: The list to get the length of.

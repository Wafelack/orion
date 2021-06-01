Maybe
=====

Enumerations
------------

### `Maybe`

```clojure
(enum Maybe
	(Just x)
	Nothing)
```

#### Examples

```clojure
(def foo (Just 99))
(def bar Nothing)
```

Functions
---------

### `and_then`

`and_then :: forall a . Maybe a -> Lambda -> Maybe Value`

Performs a lambda with the contained value of `Just`.

#### Arguments

* `optionnal :: Maybe a`: The value to process.
* `callback :: Lambda`: The callback to use.

#### Examples

```clojure
(def foo (Just 99))
(and_then foo (λ (x) (Just (+ x 1)))) ;; (Just 100)
(def bar Nothing)
(and_then bar (λ (x) (Just (- x 1)))) ;; Nothing
```

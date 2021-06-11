String
======

Functions
---------

### `show :: (Any a) => a -> String`

Transforms a value into its literal form.

```clojure
(show "a") ;; "'a'"
(show 44) ;; "44"
```

### `chars :: String -> (List String)`

Transforms a `String`  into a `List` of the chars it contains.

```clojure
(chars "Hello") ;; (Cons "H" (Cons "e" (Cons "l" (Cons "l" (Cons "o" Nil)))))
```

### `strlen :: String -> Integer`

Get the length of the string.

```clojure
(strlen "Hello") ;; 5
```

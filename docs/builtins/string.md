String
======


format
------

`format :: (Any a) => String -> (a) -> String`

Formats values into a string on a delimiter.

* Delimiter: `{}`.

### Example

```clojure
(format "Hello {} !" "there") ;; "Hello there!"
```

get
---

`get :: String -> Integer -> String`

Get the character at a position in a string. Returns `""` if the string has no character at this position.

### Example

```clojure
(get "Hello" 0)  ;; "H"
(get "Hello" 12) ;; ""
```

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

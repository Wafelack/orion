String
======


format
------

`format :: String -> Value* -> String`

Formats values into a string on a delimiter.

* Delimiter: `#v`.

### Example

```clojure
(format "Hello #v !" "there") ;; "Hello there!"
```

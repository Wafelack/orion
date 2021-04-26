Miscellanous
============

unquote
-------

`unquote :: Quote -> Value`

Unquotes a quote and returns the evaluated quote expression.

### Example

```clojure
(def foo '(putStrLn "Hello, World !"))
(unquote foo) ;; Displays "Hello, World !"
```

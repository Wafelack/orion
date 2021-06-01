IO
==

putStr
------

`putStr :: String -> ()`

Displays a String on the standard output.

### Example

```clojure
(putStr "foo")
```

putStrLn
--------

`putStrLn :: String -> ()`

Like `putStr`, but displays a newline at the end of the string and flushes the standard output.

### Example

```clojure
(putStrLn "foo")
```

getLine
-------

`getLine :: String`

Reads a line on the standard output.

### Example

```clojure
(def foo (getLine))
```

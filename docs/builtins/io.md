IO
==

putStr
------

`putStr :: String -> Unit`

Displays a String on the standard output.

### Example

```clojure
(putStr "foo")
```

putStrLn
--------

`putStrLn :: String -> Unit`

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

write
-----

`write :: String -> Value -> Integer`

Writes a value in a file and returns the number of bytes written.

### Example

```clojure
(write "/dev/stdout" "Hello, World\n") ;; 13
```

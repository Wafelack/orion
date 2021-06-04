IO
==

Functions
---------

### `putStrLn :: String -> ()`

Equivalent to `(putStr (format "{}\n" (, x)))`.

**Impure**

#### Example

```clojure
(putStrLn "Hello, World !")
```

### `print :: (Any a) => a -> ()`

Equivalent to `(putStrLn (show x))`.

**Impure**

#### Example

```clojure
(print 44) ;; 44
(print "Hello") ;; 'Hello'
```

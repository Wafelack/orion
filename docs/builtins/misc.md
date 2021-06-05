Misc
==========

type
----

`type :: (Any a) => a -> String`

Get the literal type of a value.

Example :
```clojure
(type True) ;; "Bool"
```

begin
-----

Begin allow you to put serveral expressions in a block.

Example:
```clojure
(begin
    (putStrLn "Hello, World !")
    55)
    ```

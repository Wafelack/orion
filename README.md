<div align="center">

  Orion
  ---
  <img width="150px" src="assets/logo.png">

  Orion is a lisp inspired statically typed programming language written in Rust

</div>


---

## Install

### From releases

* Download binary from the releases.

### With cargo

* Run `cargo install orion-cli`
* Run `cp ~/.cargo/bin/orion-cli ~/.cargo/bin/orion`

## Help

### To run the repl

- Run `orion` in a terminal

### To execute a file

- Run `orion <file>` in a terminal (you can add the `--debug` option to display AST)

## Examples

### Factorial

```lisp
(define factorial (lambda (n) {
    (var toret 1)
    (var i 2)
    (while (<= i n) {
        (set toret (* toret i))
        (set i (+ i 1))
    })
    (return toret)
}))
```

### More or less game

```lisp
(math:initRng)
(define toGuess (math:rand 1 100))
(var guessed 0)
(var tries 0)

(while (!= guessed toGuess) {
  (set tries (+ tries 1))
  (set guessed (static_cast "int" (input "Input number: ")))
  (if (< guessed toGuess) {
    (print "More !")
  } {
    (if (> guessed toGuess) {
      (print "Less !")
    })
  })
})
(print (+ "Well done, you found the number in " (+ (static_cast "string" tries) " tries")))

```

## Documentation

- To learn the basics: [Getting Started](./GETTING_STARTED.md)
- The standard library docs are available [on the wiki](https://github.com/Wafelack/orion-lang/wiki)


## Performances

### Ackermann(3, 3) - 500 runs

| Language |  Total  | Average | Median | Amplitude |
|----------|---------|---------|--------|-----------|
|   **Nixt**   | 63145ms |  **126ms**  |  121ms |   134ms   |
|**Orion** |  4398ms |   **8ms**   |   8ms  |  13ms         |
|**Python**| 1.1e-4ms | **2.24e-7ms** | 2.12e-7 | 3.77e-7ms

<br>

### Pushing 1000 numbers to an array - 500 runs

| Language |  Total  | Average | Median | Amplitude |
|----------|---------|---------|--------|-----------|
|   **Nixt**   | 6602ms |  **13ms**  |  12ms |   29ms   |
|**Orion** |  5473ms |   **10ms**   |   10ms  |  22ms         |
|**Python**| 5.44e-5ms | **1.08e-7ms** | 9.98e-8ms| 1.61e-7ms

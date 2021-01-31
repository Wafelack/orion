<div align="center">
    
    # Orion

    Orion is a lisp inspired programming language, strongly and statically typed, focused on flexibility.

    ---

</div>

# Install

- Download binary from the releases.
- Run `cargo install orion`

# Performances

## Ackermann(3, 3) - 500 runs

| Language |  Total  | Average | Median | Amplitude |
|----------|---------|---------|--------|-----------|
|   **Nixt**   | 63145ms |  **126ms**  |  121ms |   134ms   |
|**Orion** |  4398ms |   **8ms**   |   8ms  |  13ms         |
|**Python**| 1.1e-4ms | **2.24e-7ms** | 2.12e-7 | 3.77e-7ms

<br>

## Pushing 1000 numbers to an array - 500 runs

| Language |  Total  | Average | Median | Amplitude |
|----------|---------|---------|--------|-----------|
|   **Nixt**   | 6602ms |  **13ms**  |  12ms |   29ms   |
|**Orion** |  5473ms |   **10ms**   |   10ms  |  22ms         |
|**Python**| 5.44e-5ms | **1.08e-7ms** | 9.98e-8ms| 1.61e-7ms
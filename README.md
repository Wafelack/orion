<div align="center">

Orion
=====

---

  <img width="150px" src="assets/orion-logo.png">

  Orion is a high level, purely functional Lisp dialect written in Rust.

  ![GitHub Repo stars](https://img.shields.io/github/stars/wafelack/orion?color=%2320272c&style=for-the-badge)
  ![GitHub forks](https://img.shields.io/github/forks/wafelack/orion?color=%232c2120&style=for-the-badge)

</div>

---

Installation
------------

### MacOS, BSD and GNU/Linux

You will need: the Rust toolchain (1.50+), a "make" program, a POSIX shell (installed in `/bin/sh`) and Git.

```
$ git clone https://github.com/wafelack/orion.git
$ cd orion/
$ chmod +x configure
$ ./configure
$ make
# make install
```

### Windows

* Clone the project.
* Build with cargo.
* Copy `target/release/orion` to `C:/Program Files/Orion`.
* Copy `lib/` to `C:/Program Files/Orion/`.
* Add `C:/Program Files/Orion` to your `PATH`.

Roadmap
-------

- [x] Lexing
- [x] Parsing
- [x] Evaluation
- [x] Core functionnality
- [ ] Standard library.

Differences between Orion and Orion++
-------------------------------------

Orion++ is purely functional, has enums, tuples, pattern matching, and a tiny builtin part.
Orion is functional and imperative, has mutation, and an enormous builtin part.

Performances
------------

Speed is not Orion's main goal, but here are some benchmarks.

### Ackermann péter function - m := 3 ; n := 3 - 500 runs

| Language |  Total  | Average | Median | Amplitude |
|----------|---------|---------|--------|-----------|
|   **Nixt**   | 63145ms |  **126ms**  |  121ms |   134ms   |
|**Orion** |  4398ms |   **8ms**   |   8ms  |  13ms         |
|__**Orion++**__ | 42529ms |   **85.058ms**  | 85ms | 21ms        |   
|**Python**| 1.1e-4ms | **2.24e-7ms** | 2.12e-7 | 3.77e-7ms   |


License
-------

This software and all associated items (assets, documentation, etc) are licensed under the GNU General Public License version 3.0 and higher.

<div align="center">

Orion
=====

---

  <img width="150px" src="assets/orion-logo.png">

  Orion is a high level, purely functional programming language with a LISP based syntax.

  ![GitHub Repo stars](https://img.shields.io/github/stars/orion-lang/orion?color=%2320272c&style=for-the-badge)
  ![Continuous Build](https://img.shields.io/github/workflow/status/orion-lang/orion/Continuous%20Build?style=for-the-badge)
  ![Continuous Test](https://img.shields.io/github/workflow/status/orion-lang/orion/Continuous%20Test?label=TEST&style=for-the-badge)
  ![GitHub forks](https://img.shields.io/github/forks/orion-lang/orion?color=%232c2120&style=for-the-badge)

</div>

---

Features
--------

- Lightness: Orion source code fits under 3k SLOC and its binary is under 2MB.
- Portable: Orion code is run on a virtual machine, that avoids architecture-specific problems.
- Purely functional: Pattern matching, immutability, side effects control.
- Elegant: It embeds shorthands such as `'<expr>` for `(\ () <expr>)` or `{ <expr>* }` for `(begin <expr>*)`.


Installation
------------

### MacOS, BSD and GNU/Linux

You will need: the Rust toolchain (1.50+), a "make" program, a POSIX shell (installed in `/bin/sh`) and Git.

```bash
$ git clone https://github.com/orion-lang/orion.git
$ cd orion/
$ chmod +x configure
$ ./configure
$ make
$ make install PREFIX==/wherever/you/want/
```

### Windows

Build the project, copy the binary to your path, move the library wherever you want and set `ORION_LIB` to this location.

Documentation
-------------

You can find the standard library, the core and the builtins documentation [here](docs/).

Quick Example
-------------

Fibonnaci suite:
```clojure
(load "core/math.orn")
(def fibo (λ (n) 
	(match (< n 2) 
		(True n) 
		(_ (+ (fibo (- n 1)) (fibo (- n 2)))))))
```

Differences Between Older Orion and Newer Orion
-------------------------------------

The current Orion is purely functional, has enums, tuples, pattern matching, and a tiny builtin part.

The older Orion was functional and imperative, had mutation, and an enormous builtin part.

Performance Tests
-----------------

### `ack 3 3`

|       Language       | Average|Median |Amplitude|
|----------------------|--------|-------|---------|
|       **Nixt**       |  126ms | 121ms |  134ms  |
|**Orion Interpreter** |76.106ms| 75ms  |   21ms  |
|     **Orion VM**     | 4.168ms|  4ms  |   4ms   |   
|      **CPython**     | 0.516ms|0.482ms| 0.541ms |


Acknowledgments
---------------

* Lexer, parser, interpreter and documentation: Wafelack \<wafelack@protonmail.com>
* CI: Kreyren \<kreyren@fsfe.org>

Special thanks to [@Mesabloo](https://github.com/mesabloo) and [@felko](https://github.com/felko) for support and help about implementation details.

License
-------

This software and all associated items (assets, documentation, etc) are licensed under the GNU General Public License version 3.0 and higher.

# Changelog

All the Orion changes will be written here

# 0.1.0-alpha | 31/01/2021

## Added

### CLI

- Repl
- Debug mode
- File running

### STD

- fs:exists?
- import
- assert
- print
- eprint
- puts
- eputs
- object
- list
- push

### Types

- int
- string
- float
- function
- list
- object


### Core

- define
- var
- set
- lambda
- if
- while
- arithmetic (+ / - * %)
- boolean ( ! & | != = < <= >= >)

# 0.1.2-alpha | 01/02/2021

## Added

### CLI

- Improved repl with rustyline

### fs

- fs:readFile
- fs:writeFile
- fs:readDir

### sys

- sys:exec
- sys:breakpoint
- sys:exit
- sys:args

### collections

- slice
- pop
- foreach
- index
- length

### Core

- Macros

## Improved

- Improved unmatched closing delimiter error messages.

# 0.1.3-alpha | 02/02/2021

## Improved

- Improved list display

## Fixed

- Fixed `=` with function call problem

## Added

### Docs

- Added GETTING_STARTED.md

### CLI

- Added `(quit)` to quit the repl

### math

- math:odd
- math:cos
- math:sin
- math:tan
- math:acos
- math:asin
- math:atan
- math:max
- math:min
- math:range
- math:sqrt
- math:pow

### misc

- static_cast

# 0.1.0 | 03/02/2021

## Fixed

- Boolean and arithmetic Scopes and Function Calls bugs (index problems)
- Trigonometry with radians instead of degrees
- Input with a trim

## Added

### Core

- Pattern matching

### Math

- math:initRng
- math:rand

## Improved

- Getting started with pattern matching explanation

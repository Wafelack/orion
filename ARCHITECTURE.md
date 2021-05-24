Architecture
============

### `src/main.rs`

Entry point of the program, contains only a call to cli() and the modules declaration.

### `src/cli.rs`

Contains the REPL, the file runner and all things with user interaction.

### `src/errors.rs`

The error type and the error macro.

### `src/lexer.rs`

Contains the `Lexer` code and the `Token` enumeration definition.
The `Lexer` takes a `String` and returns a `Vec<Token>`.

### `src/parser.rs`

Contains the `Parser`, a hand made recursive parser, the `Literal` enumeration definition (an enumeration for describing literal types, such as `Integer`, `Single` or `String`), the `Pattern` enumeration definition (an enumeration for describing `match` arms, with `Tuple`, `Constr`, `Var` and `Literal` variants) and the `Expr` enumearation definition.
The `Parser` takes a `Vec<Token>` and returns a `Vec<Expr>`.

### `src/compiler.rs`

Contains the `Compiler`, that takes a `Vec<Expr>` and returns a `Bytecode`.

### `src/bytecode.rs`

Contains the `Bytecode` struct, containing the `matches`, the `Chunk`s, the `symbols`, the `constants`,  the `BytecodePattern`s, the `OpCode`s and the `constructors`.

* `matches` :: `Vec<Vec<(u16, Vec<OpCode>)>>`: The `match` expressions, each one being a `Vec<(u16, Vec<OpCode>)>`. Each element of this Vec has a pattern ID (the `u16`), part of the `patterns` field of the `Bytecode` and an instruction set, that are the `OpCode`s being executed when the pattern is matched.
* `Chunk`s :: `Vec<Chunk>`: The `chunks` of the bytecode, that represent the functions bodies. Each chunk is constitued of a reference `Vec<u16>`, representing the ID in the `symbols` of the `Bytecode` of each of the arguments, and of an instruction set, `Vec<OpCode>`, composing the function body.
* `symbols` :: `Vec<String>`: The symbol table of the bytecode, contaning the name of each variable, that is replace by an ID (`u16`) in the instructions, for size and efficiency reasons.
* `constants` :: `Vec<Literal>`: The constants table, containing the constants needed by the program, refered by ID for the same reasons as above.
* `BytecodePattern`s :: `Vec<BytecodePattern>`: The pattern table of the bytecode. `BytecodePattern` is the same as `Pattern` but with 2 exceptions: It uses IDs instead of recursive patterns and it has the `Otherwise` variant, for the `_` variable.
* `OpCode`s :: `Vec<OpCode>`: The bytecode instructions.
* `constructors` :: `Vec<u8>`: The bytecode constructors, each `u8` represents the amount of values contained in the constructor.

### `src/vm.rs`

The Orion Virtual Machine, containing the `Value` enumeration declaration and the whole virtual machine.

### `src/arithmetic.rs`

The maths builtins.

### `lib/*`

The Orion standard library and prelude.

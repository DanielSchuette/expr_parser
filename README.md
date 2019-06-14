# Expression Parser in Rust

![crates.io](https://img.shields.io/crates/v/expr_parser.svg)

## Overview
A simple program that takes expressions like `5 + 8 * (7-1)` and parses them into a syntax tree. [GRAMMAR.md](./GRAMMAR.md) explains the process in greater detail. A few things will be implemented like executing the syntax tree that lexer and parser create. This is really just an experiment to see how a context-free grammar-based parser might be implemented in `Rust`.

## Run the Code
A `Makefile` makes running the code in this repository easy:

```bash
make
make test # runs the binary with appropriate parameters (especially `-e <expr>')
make err1 # demonstrates an error; `err2' and `err3' exist, too
```

But `cargo` can be used, too. E.g., install the binary from [crates.io](https://crates.io) with:

```bash
cargo install expr_parser
expr_parser --help # validates a successful installation
```

## Create an AST Graph
`graphviz` must be installed on your system. If you `make` and `make test`, `.gv` and `.pdf` files will be created in the project root. Refer to the `Makefile` or run `make help` for available parameters.

## Direct Dependencies

1. [Clap v2.33.0](https://crates.io/crates/clap)

## To-Do:
1. execute AST via the vm's `evaluate` function
1. add additional ops to the grammar, e.g. `[]`, `log2`, `log10`
1. test parser using QuickCheck or something similar
1. allow for dynamic user input
1. allow for variable assignment and re-use of those variables (`$var` syntax), using a stack machine
1. FIXME: `$ls src/*.rs | xargs grep -ni 'FIXME'`

## Known Bugs
At this point in development, the `parser_old.rs` is quite buggy. It works with simple expressions, but it parses with right-associativity when `>= 1` token of the same level of precendence appear on the same line. This bug will be fixed soon, though (with a complete rewrite of a top-down, recursive descent parser).

## License
The code in this repository is MIT-licensed (see [LICENSE.md](./LICENSE.md)).

# Expression Parser in Rust

## Overview
A simple program that takes expressions like `5 + 8 * (7-1)` and parses them into a syntax tree. [GRAMMAR.md](./GRAMMAR.md) explains the process in greater detail. A few things will be implemented like executing the syntax tree that lexer and parser create. This is really just an experiment to see how a context-free grammar-based parser might be implemented in `Rust`.

## Run the Code
`run.sh` makes running the code in this repository easy:

```bash
make
make test # runs the binary with appropriate parameters (especially `-e <expr>')
make err1 # demonstrates an error; `err2' and `err3' exist, too
```

## Create an AST Graph
`graphviz` must be installed on your system. If you `make` and `make test`, `.gv` and `.pdf` files will be created in the project root. Refer to the `Makefile` or run `make help` for available parameters.

## License
The code in this repository is MIT-licensed (see [LICENSE.md](./LICENSE.md)).

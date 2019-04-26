# Expression Parser in Rust

## Overview
A simple program that takes expressions like `5 + 8 * (7-1)` and parses them into a syntax tree. [GRAMMAR.md](./GRAMMAR.md) explains the process in greater detail. A few things will be implemented like executing the syntax tree that lexer and parser create. This is really just an experiment to see how a context-free grammar-based parser might be implemented in `Rust`.

## Run the Code
`run.sh` makes running the code in this repository easy:

```bash
make
make test # equivalent to `$cargo run -- <expr_to_evaluate>'
# Output:
#[
#    [
#        [
#            [
#                Literal -> 18 (depth=0),
#                Literal -> 29 (depth=0)
#            ], is Sum (depth=1)
#        ], is Paren (depth=2),
#        [
#            Literal -> 50 (depth=0),
#            Literal -> 611 (depth=0)
#        ], is Mult (depth=1)
#    ], is Div (depth=3),
#    [
#        Literal -> 41 (depth=0),
#        Literal -> 12 (depth=0)
#    ], is Exp (depth=1)
#], is Sum (depth=4)

make err1 # demonstrates an error; `err2' and `err3' exist, too
# Output:
#Token 3: Unexpected character `s'.
#        2123^sdkfj(141+22-(5998)-142
#        -----^

```

## Create an AST Graph
`graphviz` must be installed on your system. If you `make` and `make test`, `.gv` and `.pdf` files will be created in the project root.

## License
The code in this repository is MIT-licensed (see [LICENSE.md](./LICENSE.md)).

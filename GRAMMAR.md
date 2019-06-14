# Grammar

## Overview
A [reference](http://pages.cs.wisc.edu/~fischer/cs536.s08/course.hold/html/NOTES/3.CFG.html#exp).

These *symbols* are recognized by the parser:

```
literal * / ^ + - %
```

The *context-free grammar* is constructed from highest to lowest precedence, i.e. following mathematics:

```
expr     --> expr + term       | expr - term   | expr % term   | term
term     --> term * factor     | term / factor | factor
factor   --> factor ^ exponent | exponent
exponent --> int_literal       | ( expr )
```

## Program logic
1. Lexing -> take a string and returns a vector of tokens
1. Parsing -> turn tokens into an executable *abstract syntax tree*, from lowest to highest precedence (`parse_expr` -> `parse_term` -> `parse_factor` -> `parse_exponent` (which might recurse to `parse_expr`)
1. Execute the AST bottom-up

## Example
The resulting parse tree for an expression like `5 - 7 / 1` looks as follows:

```
             exp
           /  |  \
         exp  -  exp
          |       |
        term    term
          |     / | \
          |  term \ term
          |    |     |
      factor factor factor
          |    |     |
        LIT=5 LIT=7 LIT=1
```

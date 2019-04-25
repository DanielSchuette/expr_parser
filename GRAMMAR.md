Grammar:
--------
A [reference](http://pages.cs.wisc.edu/~fischer/cs536.s08/course.hold/html/NOTES/3.CFG.html#exp).

*Symbols:* `literal * / ^ + - %`

The *CFG* (from highest to lowest precedence, i.e. following mathematics):

```
expr     --> expr + expr     | expr - expr   | expr % expr   | term
term     --> term * term     | term / term   | factor
factor   --> factor ^ factor | exponent
exponent --> INT_LITERAL     | ( expr )
```

*Program logic:*

1. Lexing -> takes a string and returns a vector of tokens
1. Parsing -> turn tokens into an executable AST, from lowest to highest precedence (`parse_expr` -> `parse_term` -> `parse_factor` -> `parse_exponent`)
1. Execution -> TODO!

Example:
--------
The resulting parse tree for an expression like `5 - 7 / 1` should look like this:

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

To-Do:
------
1. visualize AST
1. add additional ops to the grammar, e.g. `[]`, `log2`, `log10`
1. execute AST
1. test parser using QuickCheck or something similar
1. allow for dynamic user input
1. allow for variable assignment and re-use of those variables (`$var` syntax), using a stack machine

/*
 * Author:  Daniel Schuette
 * Date:    04/26/2019
 * License: MIT
 *          (see LICENSE.md at https://github.com/DanielSchuette/expr_parser)
 */
use std::env;
use std::fmt;
use std::iter::Peekable;
use std::process::exit;

// Language symbols are either terminals or non-terminals.
#[derive(Debug)]
enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}

/*
 * Every symbol in the language except for integer literals are non-terminals.
 * They appear as `ParseNode.kind' in the syntax tree. Leaves of the tree only
 * store the corresponding terminal with its `i64' value.
 */
#[derive(Debug)]
enum NonTerminal {
    Sum,   /* summation */
    Sub,   /* subtraction */
    Mod,   /* modulo */
    Mult,  /* multiplication */
    Div,   /* divison */
    Exp,   /* exponentiation */
    Paren, /* parenthesis */
}

#[derive(Debug)]
enum Terminal {
    Literal(i64),
}

// An expression is parsed into a `ParseNode'.
struct ParseNode {
    children: Vec<ParseNode>, /* len==0 for terminals */
    kind: Symbol,             /* type of this node in the AST */
    depth: usize,             /* depth of this node (leaf=0) */
}

impl ParseNode {
    fn new(kind: Symbol, depth: usize) -> ParseNode {
        ParseNode { children: Vec::new(),
                    kind,
                    depth }
    }
}

impl fmt::Debug for ParseNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.children.len() != 0 {
            // this is a node
            if let Symbol::NonTerminal(ref nt) = self.kind {
                write!(f,
                       "{:#?}, is {:?} (depth={})",
                       self.children, nt, self.depth)
            } else {
                write!(f, "{:#?}, is {:?}", self.children, self.kind)
            }
        } else {
            // this is a terminal
            if let Symbol::Terminal(Terminal::Literal(n)) = self.kind {
                write!(f, "Literal -> {:?} (depth={})", n, self.depth)
            } else {
                write!(f, "{:?}", self.kind)
            }
        }
    }
}

// Lexing returns the following tokens.
#[derive(Debug, Clone)]
enum Token {
    OpAdd,      /* + */
    OpSub,      /* - */
    OpMod,      /* % */
    OpMult,     /* * */
    OpDiv,      /* / */
    OpExp,      /* ^ */
    LeftParen,  /* ( */
    RightParen, /* ) */
    Number(i64),
}

// A generic error type that is used by the parser and holds a message and the
// token at which the error occured.
struct ParserError {
    msg: String,
    token_no: usize,
    lexer: Vec<Token>, /* inherited from the lexer, see below */
}

// A generic error type that is used by the lexer and holds a message and the
// token at which the error occured. A vector of tokens up to the error is
// included for better error reporting.
struct LexerError {
    msg: String,
    token_no: usize,
    tokens: Vec<Token>, /* tokens up to the error */
}

// The lexer which emits a token stream or an error.
fn lex(input: &String) -> Result<Vec<Token>, LexerError> {
    let mut progress = 0;
    let mut result = vec![];
    let mut token_stream = input.chars().peekable();

    while let Some(&c) = token_stream.peek() {
        progress += 1;

        match c {
            '0'...'9' => {
                token_stream.next();

                // pass the already consumed char and the stream to a fn that
                // parses the whole number
                let n = get_number(c, &mut token_stream);
                result.push(Token::Number(n));
            }
            '+' => {
                result.push(Token::OpAdd);
                token_stream.next();
            }
            '-' => {
                result.push(Token::OpSub);
                token_stream.next();
            }
            '%' => {
                result.push(Token::OpMod);
                token_stream.next();
            }
            '*' => {
                result.push(Token::OpMult);
                token_stream.next();
            }
            '/' => {
                result.push(Token::OpDiv);
                token_stream.next();
            }
            '^' => {
                result.push(Token::OpExp);
                token_stream.next();
            }
            '(' => {
                result.push(Token::LeftParen);
                token_stream.next();
            }
            ')' => {
                result.push(Token::RightParen);
                token_stream.next();
            }
            ' ' => {
                token_stream.next();
            }
            _ => {
                return Err(LexerError { msg:
                                            format!("Unexpected character `{}'", c),
                                        token_no: progress,
                                        tokens: result });
            }
        }
    }
    Ok(result)
}

/*
 * Get a number from a token stream. NOTE: the generic is required to force
 * static dispatch with a type of unknown size. Using a `Box' would be an
 * alternative, too.
 * FIXME: Improve this function.
 */
fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> i64 {
    // parse the character that was already consumed and passed as `c'
    let mut number = c.to_string()
                      .parse::<i64>()
                      .expect("Failed to parse `char' as `i64'");

    /*
     * Consume characters as long as parsing them to `i64' succeeds
     * by only peeking without advancing the iterator, the next non-
     * digit character.
     *
     * TODO: This could be implemented using `.position(|&c| c == ' ')'
     * and `.take()' on the iterator to avoid peeking.
     */
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<i64>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}

/*
 * The parser takes a stream of tokens from a lexer and parses it. The root
 * node of the resulting tree or an error is returned.
 */
fn parse(tokens: Result<Vec<Token>, LexerError>) -> Result<ParseNode, ParserError> {
    match tokens {
        Ok(tokens) => {
            parse_expr(&tokens, 0).and_then(|(n, i)| {
                // check if all tokens were consumed
                if i == tokens.len() {
                    Ok(n)
                } else {
                    Err(ParserError { msg: format!("Expected end of input, found {:?}",
                                                   tokens[i]),
                                      token_no: i,
                                      lexer: vec![]})
                }
            })
        }
        Err(e) => Err(ParserError { msg: e.msg,
                                    token_no: e.token_no,
                                    lexer: e.tokens }),
    }
}

// Everything is an expression, so parsing starts here.
// FIXME: replace repetition in match arm bodies with macro.
fn parse_expr(tokens: &Vec<Token>, pos: usize)
              -> Result<(ParseNode, usize), ParserError> {
    let (lhs, pos) = parse_term(tokens, pos)?;
    let c = tokens.get(pos);
    match c {
        // if the token after the term is `%', `+' or `-', parse the RHS expr
        Some(&Token::OpAdd) => {
            let mut sum =
                ParseNode::new(Symbol::NonTerminal(NonTerminal::Sum), lhs.depth + 1);
            sum.children.push(lhs);
            let (rhs, pos) = parse_expr(tokens, pos + 1)?;
            sum.children.push(rhs);
            Ok((sum, pos))
        }
        Some(&Token::OpSub) => {
            let mut sub =
                ParseNode::new(Symbol::NonTerminal(NonTerminal::Sub), lhs.depth + 1);
            sub.children.push(lhs);
            let (rhs, pos) = parse_expr(tokens, pos + 1)?;
            sub.children.push(rhs);
            Ok((sub, pos))
        }
        Some(&Token::OpMod) => {
            let mut md =
                ParseNode::new(Symbol::NonTerminal(NonTerminal::Mod), lhs.depth + 1);
            md.children.push(lhs);
            let (rhs, pos) = parse_expr(tokens, pos + 1)?;
            md.children.push(rhs);
            Ok((md, pos))
        }
        // otherwise, the expression is just a single term (recursion stops
        // here eventually)
        _ => Ok((lhs, pos)),
    }
}

// An expression consists of terms, so they are parsed next.
fn parse_term(tokens: &Vec<Token>, pos: usize)
              -> Result<(ParseNode, usize), ParserError> {
    let (lhs, pos) = parse_factor(tokens, pos)?;
    let c = tokens.get(pos);
    match c {
        Some(&Token::OpMult) => {
            let mut mult = ParseNode::new(Symbol::NonTerminal(NonTerminal::Mult),
                                          lhs.depth + 1);
            mult.children.push(lhs);
            let (rhs, pos) = parse_term(tokens, pos + 1)?;
            mult.children.push(rhs);
            Ok((mult, pos))
        }
        Some(&Token::OpDiv) => {
            let mut div =
                ParseNode::new(Symbol::NonTerminal(NonTerminal::Div), lhs.depth + 1);
            div.children.push(lhs);
            let (rhs, pos) = parse_term(tokens, pos + 1)?;
            div.children.push(rhs);
            Ok((div, pos))
        }
        _ => Ok((lhs, pos)),
    }
}

fn parse_factor(tokens: &Vec<Token>, pos: usize)
                -> Result<(ParseNode, usize), ParserError> {
    let (lhs, pos) = parse_exponent(tokens, pos)?;
    let c = tokens.get(pos);
    match c {
        Some(&Token::OpExp) => {
            let mut exp =
                ParseNode::new(Symbol::NonTerminal(NonTerminal::Exp), lhs.depth + 1);
            exp.children.push(lhs);
            let (rhs, pos) = parse_factor(tokens, pos + 1)?;
            exp.children.push(rhs);
            Ok((exp, pos))
        }
        _ => Ok((lhs, pos)),
    }
}

fn parse_exponent(tokens: &Vec<Token>, pos: usize)
                  -> Result<(ParseNode, usize), ParserError> {
    let c: &Token =
        tokens.get(pos)
              .ok_or(ParserError { msg:
                                       String::from("Unexpected end of input"),
                                   token_no: pos,
                                   lexer: vec![] })?;
    match c {
        &Token::Number(n) => {
            // this is a terminal/leaf, so `children' vec stays empty
            let terminal =
                ParseNode::new(Symbol::Terminal(Terminal::Literal(n)), 0);
            Ok((terminal, pos + 1))
        }
        &Token::LeftParen => {
            parse_expr(tokens, pos + 1).and_then(|(node, pos)| {
                if let Some(&Token::RightParen) = tokens.get(pos) {
                    let mut paren =
                        ParseNode::new(Symbol::NonTerminal(NonTerminal::Paren),
                                       node.depth + 1);
                    paren.children.push(node);
                    Ok((paren, pos + 1))
                } else {
                    Err(ParserError { msg: format!("Expected closing parenthesis but found {:?}",
                                tokens.get(pos-1)), token_no: pos, lexer: vec![] })
                }
            })
        }
        _ => Err(ParserError { msg: format!("Unexpected token {:?}", c),
                               token_no: pos,
                               lexer: vec![] }),
    }
}

fn exit_with_err(err: ParserError, input: &String) {
    // report the error back to the user
    println!("Token {}: {}.", err.token_no, err.msg);
    println!("\t{}", input);

    // print an indicator where in the input the error happened
    if err.lexer.len() != 0 {
        let indicator = "-".repeat(get_position(err.lexer));
        println!("\t{}^", indicator);
    } else {
        let indicator = "-".repeat(input.to_string().len() - 1);
        println!("\t{}^", indicator);
    }
    exit(1);
}

fn get_position(vec: Vec<Token>) -> usize {
    let mut pos = 0;
    for token in vec {
        match token {
            Token::Number(n) => {
                pos += n.to_string().len();
            }
            _ => {
                pos += 1;
            }
        }
    }
    pos
}

fn main() {
    // collect 2 command line arguments or exit
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error: need exactly two arguments.");
        exit(1);
    }

    // lex and parse the user input and report the result
    // FIXME: handle errors inbetween lexing and parsing instead of delegating
    //        things to `parse'. This involves calling `exit_with_err' when
    //        appropriate.
    let tokens = lex(&args[1]);
    let res = parse(tokens);

    if let Ok(ast) = res {
        println!("{:#?}", ast);
    } else if let Err(e) = res {
        exit_with_err(e, &args[1]);
    }
}

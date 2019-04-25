/* parser.rs: The parser. */
use crate::lexer;
use lexer::*;
use std::fmt;

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
pub struct ParseNode {
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

// A generic error type that is used by the parser and holds a message and the
// token at which the error occured.
pub struct ParserError {
    pub msg: String,
    pub token_no: usize,
    pub lexer: Vec<Token>, /* inherited from the lexer, see below */
}

/*
 * The parser takes a stream of tokens from a lexer and parses it. The root
 * node of the resulting tree or an error is returned.
 */
pub fn parse(tokens: Result<Vec<Token>, LexerError>)
             -> Result<ParseNode, ParserError> {
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

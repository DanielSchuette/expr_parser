/* parser.rs: The parser. */
use crate::lexer;
use lexer::*;

/*
 * Every symbol in the language except for integer literals and operators are
 * non-terminals. They appear as `ParseNode.kind' in the syntax tree. Leaf
 * nodes of the tree only store the corresponding terminal with its `i64' value
 * while branch nodes store their non-terminal, too.
 */
#[derive(Debug)]
enum NonTerminal {
    Expression, /* precedence 0 (lowest) */
    Term,       /* precedence 1 */
    Factor,     /* precedence 2 */
    Exponent,   /* precedence 3 (highest ) */
}

#[derive(Debug)]
pub enum Terminal {
    Sum,          /* summation */
    Sub,          /* subtraction */
    Mod,          /* modulo */
    Mult,         /* multiplication */
    Div,          /* divison */
    Exp,          /* exponentiation */
    Paren,        /* parenthesis */
    Literal(i64), /* literals are stored with their associated values */
}

#[derive(Debug)]
enum NodeType {
    Root,
    Branch,
    Leaf,
}

/* An expression is parsed into a `ParseNode'. */
#[derive(Debug)]
pub struct ParseNode {
    left_child: Option<Box<ParseNode>>, /* `None' for terminals */
    right_child: Option<Box<ParseNode>>, /* `None' for terminals & parens */
    ntype: NodeType,                    /* type of this node in the AST */
    pub terminal: Terminal,             /* the terminal type of this node */
    non_terminal: NonTerminal,          /* `IsTerminal' for terminals */
    depth: usize,                       /* depth of this node (leaf=0) */
}

impl ParseNode {
    fn new(ntype: NodeType, terminal: Terminal, non_terminal: NonTerminal,
           depth: usize)
           -> ParseNode {
        ParseNode { left_child: None,
                    right_child: None,
                    ntype,
                    terminal,
                    non_terminal,
                    depth }
    }
    pub fn get_lchild(&self) -> &Option<Box<ParseNode>> {
        &self.left_child
    }

    pub fn get_rchild(&self) -> &Option<Box<ParseNode>> {
        &self.right_child
    }

    pub fn get_long_type(&self) -> String {
        match self.terminal {
            Terminal::Literal(n) => format!("Literal={}", n),
            Terminal::Sum => format!("Op=PLUS"),
            Terminal::Sub => format!("Op=MINUS"),
            Terminal::Mod => format!("Op=MODULP"),
            Terminal::Mult => format!("Op=MULTIPLICATION"),
            Terminal::Div => format!("Op=DIVISON"),
            Terminal::Exp => format!("Op=EXPONENTIATION"),
            Terminal::Paren => format!("Parentheses"),
        }
    }

    pub fn get_short_type(&self) -> String {
        match self.terminal {
            Terminal::Literal(n) => format!("{}", n),
            Terminal::Sum => format!("+"),
            Terminal::Sub => format!("-"),
            Terminal::Mod => format!("%"),
            Terminal::Mult => format!("*"),
            Terminal::Div => format!("/"),
            Terminal::Exp => format!("^"),
            Terminal::Paren => format!("(...)"),
        }
    }

    #[allow(dead_code)]
    pub fn get_non_terminal_type(&self) -> String {
        match self.non_terminal {
            NonTerminal::Expression => format!("Expression"),
            NonTerminal::Term => format!("Term"),
            NonTerminal::Factor => format!("Factor"),
            NonTerminal::Exponent => format!("Exponent"),
        }
    }

    pub fn get_depth(&self) -> usize {
        self.depth
    }
}

/*
 * A generic error type that is used by the parser and holds a message and the
 * token at which the error occured.
 */
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
            // parse from right to left to preserve left-associativity of operations
            parse_expr(&tokens, tokens.len()-1).and_then(|(mut node, pos)| {
                // check if all tokens were consumed and append the parsing
                // result to a root node
                if pos == 0 {
                    node.ntype = NodeType::Root;
                    Ok(node)
                } else {
                    Err(ParserError { msg: format!("Expected end of input, found {:?}",
                                                   tokens[pos]),
                                      token_no: pos,
                                      lexer: vec![]})
                }
            })
        }
        Err(e) => Err(ParserError { msg: e.msg,
                                    token_no: e.token_no,
                                    lexer: e.tokens }),
    }
}

/*
 * Everything is an expression, so parsing starts here.
 */
fn parse_expr(tokens: &Vec<Token>, pos: usize)
              -> Result<(ParseNode, usize), ParserError> {
    let (lhs, pos) = parse_term(tokens, pos)?;
    let c = tokens.get(pos);
    match c {
        // if the token after the term is `%', `+' or `-', parse the RHS
        Some(&Token::OpAdd) => {
            let mut sum = ParseNode::new(NodeType::Branch,
                                         Terminal::Sum,
                                         NonTerminal::Expression,
                                         lhs.depth + 1);
            let (rhs, pos) = parse_expr(tokens, pos - 1)?;
            sum.left_child = Some(Box::new(lhs));
            sum.right_child = Some(Box::new(rhs));
            Ok((sum, pos))
        }
        Some(&Token::OpSub) => {
            let mut sub = ParseNode::new(NodeType::Branch,
                                         Terminal::Sub,
                                         NonTerminal::Expression,
                                         lhs.depth + 1);
            let (rhs, pos) = parse_expr(tokens, pos - 1)?;
            sub.left_child = Some(Box::new(lhs));
            sub.right_child = Some(Box::new(rhs));
            Ok((sub, pos))
        }
        Some(&Token::OpMod) => {
            let mut md = ParseNode::new(NodeType::Branch,
                                        Terminal::Mod,
                                        NonTerminal::Expression,
                                        lhs.depth + 1);
            let (rhs, pos) = parse_expr(tokens, pos - 1)?;
            md.left_child = Some(Box::new(lhs));
            md.right_child = Some(Box::new(rhs));
            Ok((md, pos))
        }

        // otherwise, the expression is just a single term (recursion stops
        // here eventually)
        _ => Ok((lhs, pos)),
    }
}

/* An expression consists of terms, so they are parsed next. */
fn parse_term(tokens: &Vec<Token>, pos: usize)
              -> Result<(ParseNode, usize), ParserError> {
    let (lhs, pos) = parse_factor(tokens, pos)?;
    let c = tokens.get(pos);
    match c {
        Some(&Token::OpMult) => {
            let mut mult = ParseNode::new(NodeType::Branch,
                                          Terminal::Mult,
                                          NonTerminal::Term,
                                          lhs.depth + 1);
            let (rhs, pos) = parse_term(tokens, pos - 1)?;
            mult.left_child = Some(Box::new(lhs));
            mult.right_child = Some(Box::new(rhs));
            Ok((mult, pos))
        }
        Some(&Token::OpDiv) => {
            let mut div = ParseNode::new(NodeType::Branch,
                                         Terminal::Div,
                                         NonTerminal::Term,
                                         lhs.depth + 1);
            let (rhs, pos) = parse_term(tokens, pos - 1)?;
            div.left_child = Some(Box::new(lhs));
            div.right_child = Some(Box::new(rhs));
            Ok((div, pos))
        }
        _ => Ok((lhs, pos)),
    }
}

/* Term consist of factors, which are parsed by this function. */
fn parse_factor(tokens: &Vec<Token>, pos: usize)
                -> Result<(ParseNode, usize), ParserError> {
    let (lhs, pos) = parse_exponent(tokens, pos)?;
    let c = tokens.get(pos);
    match c {
        Some(&Token::OpExp) => {
            let mut exp = ParseNode::new(NodeType::Branch,
                                         Terminal::Exp,
                                         NonTerminal::Factor,
                                         lhs.depth + 1);
            let (rhs, pos) = parse_factor(tokens, pos - 1)?;
            exp.left_child = Some(Box::new(lhs));
            exp.right_child = Some(Box::new(rhs));
            Ok((exp, pos))
        }
        _ => Ok((lhs, pos)),
    }
}

/*
 * Lastly, exponents are parsed. If parentheses are encountered, start with
 * parsing an expression again. If a literal is found, no more recursion is
 * done because literals are leaves in the parse tree.
 */
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
            // this is a leaf, so left and right child keep their `None' vals
            let leaf = ParseNode::new(NodeType::Leaf,
                                      Terminal::Literal(n),
                                      NonTerminal::Exponent,
                                      0);
            Ok((leaf, pos - 1))
        }
        &Token::RightParen => {
            parse_expr(tokens, pos - 1).and_then(|(node, pos)| {
                if let Some(&Token::LeftParen) = tokens.get(pos) {
                    // parentheses are not expected to be empty
                    let mut paren =
                        ParseNode::new(NodeType::Branch, Terminal::Paren, NonTerminal::Exponent,
                                       node.depth + 1);
                    paren.left_child = Some(Box::new(node));
                    if pos == 0 {
                        Ok((paren, pos))
                    } else {
                        Ok((paren, pos-1))
                    }
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

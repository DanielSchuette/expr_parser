/* parser.rs: The expression parser. Creates an abstract syntax tree. */
use crate::lexer;
use lexer::*;

#[derive(Debug)]
enum NonTerminal {
    Expression, /* precedence 1 */
    Term,       /* precedence 2 */
    Factor,     /* precedence 3 */
    Exponent,   /* precedence 4 (highest) */
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

    NonTerminal, /* type of non-terminal parse nodes */
}

#[derive(Debug)]
enum NodeType {
    Branch,
    Leaf,
}

/// An expression is parsed into a `ParseNode`.
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
            Terminal::NonTerminal => format!("Non-Terminal"),
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
            Terminal::NonTerminal => format!("@"),
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

/// A generic error type that is used by the parser and holds a message and the
/// token at which the error occured.
pub struct ParserError {
    pub msg: String,
    pub token_no: usize,
    pub lexer: Vec<Token>, /* inherited from the lexer, see below */
}

impl ParserError {
    fn new(msg: String, token_no: usize, lexer: Vec<Token>) -> ParserError {
        ParserError { msg,
                      token_no,
                      lexer }
    }
}

struct TokenStream {
    tokens: Vec<Token>,
    cursor: usize,
}

/// A `TokenStream` wraps a vector of tokens and a `cursor` which indicates the
/// current position of the parser. Out-of-bounds errors are not allowed
/// because the cursor can only be advanced if there are tokens left.
impl TokenStream {
    fn new(tokens: Vec<Token>) -> TokenStream {
        TokenStream { tokens, cursor: 0 }
    }

    fn get_position(&self) -> usize {
        self.cursor
    }

    fn get_current(&self) -> Token {
        self.tokens[self.cursor].clone()
    }

    fn advance(&mut self, step: usize) {
        if (self.cursor + step) < self.tokens.len() {
            self.cursor += step;
        }
    }

    fn is_finished(&self) -> bool {
        if self.tokens.len() == (self.cursor + 1) {
            true
        } else {
            false
        }
    }

    fn get_stream(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}

pub fn parse(tokens: Result<Vec<Token>, LexerError>)
             -> Result<ParseNode, ParserError> {
    if let Ok(tokens) = tokens {
        let mut stream = TokenStream::new(tokens);
        let ast = expression(&mut stream, 0)?;

        // check if all tokens were consumed
        if stream.is_finished() {
            return Ok(ast);
        } else {
            return Err(ParserError::new(format!("Expected end of input, found {:?}", stream.get_current()),
                                  stream.get_position(), stream.get_stream()));
        }
    } else if let Err(e) = tokens {
        Err(ParserError { msg: e.msg,
                          token_no: e.token_no,
                          lexer: e.tokens })
    } else {
        Err(ParserError::new("Unknown error".to_string(), 0, vec![]))
    }
}

fn expression(mut stream: &mut TokenStream, depth: usize)
              -> Result<ParseNode, ParserError> {
    let mut node = ParseNode::new(NodeType::Branch,
                                  Terminal::NonTerminal,
                                  NonTerminal::Expression,
                                  depth);
    let lchild = term(&mut stream, depth)?;
    node.left_child = Some(Box::new(lchild));
    let mut depth = depth + 1;

    // one or more term(s) can follow the appropriate token
    loop {
        let token = stream.get_current();
        match token {
            Token::OpAdd | Token::OpSub | Token::OpMod => {
                stream.advance(1);
                let rchild = term(&mut stream, depth)?;
                if let None = node.right_child {
                    node.right_child = Some(Box::new(rchild));
                } else if let Some(ref mut current) = node.right_child {
                    let mut parent = current;
                    loop {
                        if let None = parent.left_child {
                            parent.left_child = Some(Box::new(rchild));
                            break;
                        } else if let Some(ref mut new) = parent.left_child {
                            parent = new;
                        }
                    }
                }
                depth += 1;
            }

            // cursor is already advanced, break out of loop
            _ => {
                break;
            }
        }
    }
    Ok(node)
}

fn term(mut stream: &mut TokenStream, depth: usize)
        -> Result<ParseNode, ParserError> {
    let mut node = ParseNode::new(NodeType::Branch,
                                  Terminal::NonTerminal,
                                  NonTerminal::Term,
                                  depth);
    let lchild = factor(&mut stream, depth)?;
    node.left_child = Some(Box::new(lchild));
    let mut depth = depth + 1;

    // one or more factor(s) can follow the appropriate token
    loop {
        let token = stream.get_current();
        match token {
            Token::OpMult | Token::OpDiv => {
                stream.advance(1);
                let rchild = factor(&mut stream, depth)?;
                if let None = node.right_child {
                    node.right_child = Some(Box::new(rchild));
                } else if let Some(ref mut current) = node.right_child {
                    let mut parent = current;
                    loop {
                        if let None = parent.left_child {
                            parent.left_child = Some(Box::new(rchild));
                            break;
                        } else if let Some(ref mut new) = parent.left_child {
                            parent = new;
                        }
                    }
                }
                depth += 1;
            }

            // cursor is already advanced, break out of loop
            _ => {
                break;
            }
        }
    }
    Ok(node)
}

fn factor(mut stream: &mut TokenStream, depth: usize)
          -> Result<ParseNode, ParserError> {
    let mut node = ParseNode::new(NodeType::Branch,
                                  Terminal::NonTerminal,
                                  NonTerminal::Factor,
                                  depth);
    let lchild = exponent(&mut stream, depth)?;
    node.left_child = Some(Box::new(lchild));
    let depth = depth + 1;

    let token = stream.get_current();
    match token {
        Token::OpExp => {
            stream.advance(1);
            let rchild = exponent(&mut stream, depth)?;
            node.right_child = Some(Box::new(rchild));
        }
        _ => { /* don't do anything */ }
    }
    Ok(node)
}

fn exponent(stream: &mut TokenStream, depth: usize)
            -> Result<ParseNode, ParserError> {
    let mut node = ParseNode::new(NodeType::Branch,
                                  Terminal::NonTerminal,
                                  NonTerminal::Exponent,
                                  depth);

    let token = stream.get_current();
    match token {
        Token::LeftParen => {
            stream.advance(1);
            node.terminal = Terminal::Paren;
            let lchild = expression(stream, depth)?; /* parse rest of parenthesized expression */
            node.left_child = Some(Box::new(lchild));
            if let Token::RightParen = stream.get_current() {
                stream.advance(1);
            } else {
                return Err(ParserError::new(format!("Expected `)', found {:?}",
                                                    stream.get_current()),
                                            stream.get_position(),
                                            stream.get_stream()));
            }
        }
        Token::Number(i) => {
            stream.advance(1);
            node.ntype = NodeType::Leaf;
            node.terminal = Terminal::Literal(i);
        }
        _ => {
            return Err(ParserError::new("Unexpected end of input".to_string(),
                                        stream.get_position(),
                                        stream.get_stream()));
        }
    }
    Ok(node)
}

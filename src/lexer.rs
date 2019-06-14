/* lexer.rs: The lexer. */
use std::iter::Peekable;

/* Lexing can return these tokens. */
#[derive(Debug, Clone)]
pub enum Token {
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

/*
 * A generic error type that is used by the lexer and holds a message and the
 * token at which the error occured. A vector of tokens up to the error is
 * included for better error reporting.
 */
#[derive(Debug)]
pub struct LexerError {
    pub msg: String,
    pub token_no: usize,
    pub tokens: Vec<Token>, /* tokens up to the error */
}

/* The lexer which emits a token stream or an error. */
pub fn lex(input: &String) -> Result<Vec<Token>, LexerError> {
    let mut progress = 0;
    let mut result = vec![];
    let mut token_stream = input.chars().peekable();

    while let Some(&c) = token_stream.peek() {
        progress += 1;

        match c {
            '0'..='9' => {
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

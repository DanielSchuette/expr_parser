/*
 * Author:  Daniel Schuette
 * Date:    04/26/2019
 * License: MIT
 *          (see LICENSE.md at https://github.com/DanielSchuette/expr_parser)
 */
mod lexer;
mod parser;

use lexer::{lex, Token};
use parser::{parse, ParserError};
use std::env;
use std::process::exit;

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

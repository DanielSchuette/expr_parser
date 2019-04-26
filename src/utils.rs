/* utils.rs: Utility functions used by the main interpreter loop. */
use crate::lexer::Token;
use crate::parser::ParserError;
use std::env::Args;
use std::process::exit;

pub const DEBUG: bool = true;

pub fn get_expr(args: Args) -> String {
    let args: Vec<String> = args.collect();
    if args.len() != 2 {
        println!("Error: need exactly two arguments.");
        exit(1);
    }
    args[1].clone()
}

pub fn exit_with_err(err: ParserError, input: &String) {
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

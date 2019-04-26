/* utils.rs: Utility functions used by the main interpreter loop. */
extern crate clap;

use crate::draw;
use crate::lexer::Token;
use crate::parser::{ParseNode, ParserError};
use clap::{App, Arg};
use std::process::exit;

pub struct Config {
    pub expression: String,
    pub is_debug: bool,
    pub make_graph: bool, // pub graph_path: String, /* TODO: not yet implemented. */
}

pub fn get_configs() -> Config {
    // define cli arguments using clap
    let cli_args =
        App::new("Expression Parser").version("0.0.1")
                              .author("Daniel Schuette <d.schuette@online.de>")
                              .about("Parse simple arithmetic expressions.")
                              .arg(Arg::with_name("EXPR").short("e")
                                                         .long("expression")
                                                         .help("The expression to evaluate")
                                                         .takes_value(true)
                                                         .required(true))
                              .arg(Arg::with_name("GRAPH").short("g")
                                                          .long("graph")
                                                          .help("Create an AST graph")
                                                          .takes_value(false)
                                                          .required(false))
                              .arg(Arg::with_name("DEBUG").short("d")
                                                          .long("debug")
                                                          .help("Debug mode (off by default)")
                                                          .takes_value(false)
                                                          .required(false))
                              .get_matches();

    // extract arguments and return config struct for main to use
    let expression = cli_args.value_of("EXPR").unwrap().to_string();
    let is_debug = if cli_args.is_present("DEBUG") {
        true
    } else {
        false
    };
    let make_graph = if cli_args.is_present("GRAPH") {
        true
    } else {
        false
    };
    Config { expression,
             is_debug,
             make_graph }
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

// A thin wrapper around `create_graph' from the `draw' crate.
pub fn draw(ast: &ParseNode, path: &str, pdf: bool) {
    let res = draw::create_graph(&ast, path, pdf);
    match res {
        Ok(_) => println!("Successfully wrote graph data to file."),
        Err(e) => println!("Failed to create graph: {}.", e),
    }
}

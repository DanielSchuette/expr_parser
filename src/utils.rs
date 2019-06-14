/* utils.rs: Utility functions used by the main interpreter loop. */
extern crate clap;

use crate::draw;
use crate::lexer::Token;
use crate::parser::{ParseNode, ParserError};
use clap::{App, Arg};
use std::process::exit;

const VERSION: &str = "0.0.2";
const AUTHOR: &str = "Daniel Schuette <d.schuette@online.de>";
const ABOUT: &str = "Parse simple arithmetic expressions. Without any flags or options, an interactive session is started.";
const PROGNAME: &str = "expr_parser";

pub struct Config {
    pub expression: String,
    pub is_debug: bool,
    pub make_graph: bool,
    pub graph_file: String,
    pub progname: String,
}

/* Parse CLi arguments and return them, wrapped in a `Config' struct. */
pub fn get_configs() -> Config {
    // define cli arguments using clap
    let cli_args =
        App::new("Expression Parser").version(VERSION)
                              .author(AUTHOR)
                              .about(ABOUT)
                              .arg(Arg::with_name("EXPR").short("e")
                                                         .long("expression")
                                                         .help("The expression to evaluate")
                                                         .takes_value(true)
                                                         .required(false))
                              .arg(Arg::with_name("DEBUG").short("d")
                                                          .long("debug")
                                                          .help("Debug mode (off by default)")
                                                          .takes_value(false)
                                                          .required(false))
                              .arg(Arg::with_name("GRAPH").short("g")
                                                          .long("graph")
                                                          .help("Create an AST graph")
                                                          .takes_value(false)
                                                          .required(false))
                              .arg(Arg::with_name("G_FILE").short("f")
                                                           .long("graph_file")
                                                           .help("File to save the graph to")
                                                           .takes_value(true)
                                                           .required(false))
                              .get_matches();

    // extract arguments and return config struct for main to use
    let expression = if cli_args.is_present("EXPR") {
        cli_args.value_of("EXPR").unwrap().to_string()
    } else {
        String::from("")
    };

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

    let graph_file = if cli_args.is_present("G_FILE") {
        cli_args.value_of("G_FILE").unwrap().to_string()
    } else {
        String::from("")
    };

    Config { expression,
             is_debug,
             make_graph,
             graph_file,
             progname: PROGNAME.to_string() }
}

/*
 * Prints a helpful error msg, based on the `ParserError' and the user `input'.
 */
pub fn report_parser_err(err: ParserError, input: &String) {
    // report the error back to the user
    eprintln!("Token {}: {}.", err.token_no, err.msg);
    eprintln!("\t{}", input);

    // print an indicator where in the input the error happened
    if err.lexer.len() != 0 {
        let indicator = "-".repeat(get_position(err.lexer));
        eprintln!("\t{}^", indicator);
    } else {
        let input_len = input.to_string().len();
        if input_len > 1 {
            let indicator = "-".repeat(input_len - 1);
            eprintln!("\t{}^", indicator);
        } else {
            eprintln!("\t^");
        }
    }
}

/* Wraps `report_parser_err' and exits with the indicated status code. */
pub fn exit_with_err(err: ParserError, input: &String, code: i32) {
    report_parser_err(err, &input);
    exit(code);
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

/* A thin wrapper around `create_graph' from the `draw' crate. */
pub fn draw(ast: &ParseNode, path: &str, pdf: bool) {
    let res = draw::create_graph(&ast, path, pdf);
    match res {
        Ok(_) => eprintln!("Successfully wrote graph data to file."),
        Err(e) => eprintln!("Failed to create graph: {}.", e),
    }
}

/*
 * Author:  Daniel Schuette
 * Date:    04/26/2019
 * License: MIT
 *          (see LICENSE.md at https://github.com/DanielSchuette/expr_parser)
 */
mod lexer;
mod parser;
mod utils;
mod vm;

use lexer::lex;
use parser::parse;
use std::env;
use std::process::exit;
use utils::exit_with_err;

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
    //        Ultimately, the VM should handle dynamic user input and calling
    //        `lex' and `parse'. The user might indicate execution of a cli-
    //        provided expression via a flag and `vm::run' is handed a config
    //        struct with an appropriate bit field set. Other solutions are
    //        possible, too.
    let tokens = lex(&args[1]);
    let res = parse(tokens);

    if let Ok(ast) = res {
        if utils::DEBUG {
            println!("{:#?}", ast);
        }
        vm::run(ast);
    } else if let Err(e) = res {
        exit_with_err(e, &args[1]);
    }
}

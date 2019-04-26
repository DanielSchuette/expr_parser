/*
 * Author:  Daniel Schuette
 * Date:    04/26/2019
 * License: MIT
 *          (see LICENSE.md at https://github.com/DanielSchuette/expr_parser)
 */
mod draw;
mod lexer;
mod parser;
mod utils;
mod vm;

use lexer::lex;
use parser::parse;
use std::process::exit;
use utils::{exit_with_err, get_configs};

fn main() {
    let configs = get_configs();

    // if the user provided an expression via `-e', lex & parse and
    // evaluate it manually; return afterwards
    if !configs.expression.is_empty() {
        /*
         * FIXME: handle errors inbetween lexing and parsing instead of
         * delegating things to `parse'. This involves calling `exit_with_err'
         * when appropriate.
         */
        let tokens = lex(&configs.expression);
        let res = parse(tokens);

        if let Ok(ast) = res {
            if configs.is_debug {
                println!("{:#?}", ast);
            }
            if configs.make_graph {
                utils::draw(&ast, &configs.graph_file, true);
            }
            let res = vm::evaluate(&ast);
            println!("Expression result: {}", res);
        } else if let Err(e) = res {
            exit_with_err(e, &configs.expression);
        }

        exit(0);
    }

    // delegate IO, lexing & parsing, and evaluation of a resulting AST to the
    // virtual machine
    vm::run();
}

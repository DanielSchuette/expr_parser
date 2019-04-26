/*
 * Author:  Daniel Schuette
 * Date:    04/26/2019
 * License: MIT
 *          (see LICENSE.md at https://github.com/DanielSchuette/expr_parser)
 * FIXME: handle errors inbetween lexing and parsing instead of delegating
 *        things to `parse'. This involves calling `exit_with_err' when
 *        appropriate.
 *        Ultimately, the VM should handle dynamic user input and calling
 *        `lex' and `parse'. The user might indicate execution of a cli-
 *        provided expression via a flag and `vm::run' is handed a config
 *        struct with an appropriate bit field set. Other solutions are
 *        possible, too.
 */
mod draw;
mod lexer;
mod parser;
mod utils;
mod vm;

use lexer::lex;
use parser::parse;
use utils::{exit_with_err, get_configs};

fn main() {
    let configs = get_configs();
    let tokens = lex(&configs.expression);
    let res = parse(tokens);

    if let Ok(ast) = res {
        if configs.is_debug {
            println!("{:#?}", ast);
        }
        if configs.make_graph {
            utils::draw(&ast, &configs.graph_file, true);
        }
        vm::run(ast);
    } else if let Err(e) = res {
        exit_with_err(e, &configs.expression);
    }
}

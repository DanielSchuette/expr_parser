/* vm.rs: The virtual machine which executes the syntax tree. */
use crate::parser::ParseNode;
use std::io::{stdin, stdout, Write};

/* Run the virtual machine, including interpreter loop & lexing & parsing. */
pub fn run() {
    loop {
        let input = prompt_and_read("> ");
        println!("{}", input);
        if input == "exit" {
            return;
        }
    }
}

/* Evaluate an expression, represented by an abstract syntax tree. */
pub fn evaluate(_tree_root: &ParseNode) -> i64 {
    0
}

/* Print a prompt, read from `stdin' and return with new lines trimmed off. */
fn prompt_and_read(ps1: &str) -> String {
    print!("{}", ps1);
    stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

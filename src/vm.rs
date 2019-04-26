/* vm.rs: The virtual machine which executes the syntax tree. */
use crate::parser::ParseNode;
use std::io::{stdin, stdout, Write};

pub fn run(_tree_root: ParseNode) {
    loop {
        let input = prompt_and_read(String::from("> "));
        println!("{}", input);
        if input == "exit" {
            return;
        }
    }
}

// Print a prompt, read from standard input and return with new lines trimmed
// off.
fn prompt_and_read(ps1: String) -> String {
    print!("{}", ps1);
    stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

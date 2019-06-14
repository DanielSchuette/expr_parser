/* vm.rs: The virtual machine which executes the syntax tree. */
use crate::lexer::lex;
use crate::parser::{parse, ParseNode, Terminal};
use crate::utils::{report_parser_err, Config};
use std::io::{stdin, stdout, Write};

/* Stores keywords that are interpreted alongside the expressions. */
struct Keywords {
    quit: Vec<String>, /* "quit", "q" */
}

/* Run the virtual machine, including interpreter loop & lexing & parsing. */
pub fn run(configs: &Config) {
    let keywords = init();
    eprintln!("{}: Exit with ctrl+c or by typing `quit' or `q'.",
              configs.progname);

    // the read-eval-print loop
    loop {
        let input = prompt_and_read("> ");

        // check if the input is a keyword
        if matches_any(&input, &keywords.quit) {
            return; /* the only exit condition */
        }

        // lex and parse the input
        let tokens = lex(&input);
        let res = parse(tokens);

        // TODO: clean this code up
        if let Ok(ast) = res {
            let res = evaluate(&ast);
            if let Ok(res) = res {
                eprintln!("\t{}", res);
            } else if let Err(e) = res {
                eprintln!("{}: error: {}", configs.progname, e);
            }
        } else if let Err(e) = res {
            report_parser_err(e, &input);
        }
    }
}

/* Evaluate an expression, represented by an abstract syntax tree. */
// TODO: fix bugs! Simple addition works but nothing else.
pub fn evaluate(node: &ParseNode) -> Result<i64, String> {
    let mut stack: Vec<&Terminal> = vec![];
    build_exec_stack(&node, &mut stack);

    let mut result: i64 = 0;
    if let Terminal::Literal(n) = stack.pop().unwrap() {
        result += n; /* first item on stack _must_ be a literal */
    }

    // pop off the rest of the stack
    loop {
        let next = stack.pop();
        match next {
            None => break, /* here, the stack is empty */
            Some(val) => {
                match val {
                    Terminal::Literal(n) => {
                        // stack cannot be empty here, so unwrapping is save
                        let op = stack.pop().unwrap();
                        match op {
                            Terminal::Sum => {
                                result += n;
                            }
                            Terminal::Sub => {
                                result -= n;
                            }
                            Terminal::Mod => {
                                result %= n;
                            }
                            Terminal::Mult => {
                                result *= n;
                            }
                            Terminal::Div => {
                                if *n == 0 {
                                    return Err(String::from("vm: Divison by 0"));
                                }
                                result /= n;
                            }
                            Terminal::Exp => {
                                result = result.pow(*n as u32);
                            }
                            Terminal::Paren => continue, /* parens are ignored */
                            Terminal::NonTerminal => continue, /* FIXME: non-terminals are ignored */
                            Terminal::Literal(n) => {
                                return Err(String::from(
                                        format!("vm: Unexpected integer literal {}", n)
                                        ));
                            }
                        }
                    }
                    _ => {
                        return Err(String::from("vm: Expected integer literal"));
                    }
                }
            }
        }
    }
    Ok(result)
}

/*
 * Traverse parse tree recursively and push terminals onto the execution stack.
 */
fn build_exec_stack<'a>(node: &'a ParseNode, mut stack: &mut Vec<&'a Terminal>) {
    stack.push(&node.terminal);

    match node.get_lchild() {
        // this is a leaf
        None => return,
        Some(lchild) => {
            build_exec_stack(&lchild, &mut stack);
            if let Some(rchild) = node.get_rchild() {
                build_exec_stack(&rchild, &mut stack);
            }
        }
    }
}

/* Initialize data that is used by the VM. */
fn init() -> Keywords {
    Keywords { quit: vec![String::from("quit"), String::from("q")] }
}

fn matches_any(s: &String, s_vec: &Vec<String>) -> bool {
    for elem in s_vec {
        // at this point, exact matches could be avoided with `String.contains'
        if s == elem {
            return true;
        }
    }
    false
}

/* Print a prompt, read from `stdin' and return with new lines trimmed off. */
fn prompt_and_read(ps1: &str) -> String {
    print!("{}", ps1);
    stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

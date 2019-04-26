/* draw.rs: Create a graph of an abstract syntax tree. */
use crate::parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::process::Command;

macro_rules! start_branch {
    ( $graph:expr, $ast:expr, $preamble:expr ) => {
        let id = &format!("\"id={}_{}\"", $ast.get_long_type(), $ast.get_depth());
        let name = &format!("\"{}\"", $ast.get_short_type());
        $graph.push_str(&format!("\t{} -- ", id));
        $preamble.push_str(&format!("\t\t{} [label = {}]\n", id, name));
    };
}

macro_rules! append_to_branch {
    ( $graph:expr, $ast:expr, $preamble:expr ) => {
        let id = &format!("\"id={}_{}\"", $ast.get_long_type(), $ast.get_depth());
        let name = &format!("\"{}\"", $ast.get_short_type());
        $graph.push_str(&format!("{} -- ", id));
        $preamble.push_str(&format!("\t\t{} [label = {}]\n", id, name));
    };
}

macro_rules! end_branch {
    ( $graph:expr, $ast:expr, $preamble:expr ) => {
        let id = &format!("\"id={}_{}\"", $ast.get_long_type(), $ast.get_depth());
        let name = &format!("\"{}\"", $ast.get_short_type());
        $graph.push_str(&format!("{}\n\t", id));
        $preamble.push_str(&format!("\t\t{} [label = {}]\n", id, name));
    };
}

pub fn create_graph(ast: &parser::ParseNode, path: &str, pdf: bool)
                    -> std::io::Result<()> {
    // the provided path must point to a `.gv' file, otherwise replacing the
    // file extension with `.pdf' might fail later on
    if !path.ends_with(".gv") {
        return Err(Error::new(
                ErrorKind::InvalidInput,
                String::from("Provide the path to a `.gv' file (need not exist)")));
    }
    let mut file = File::create(path)?;

    // transform an ast data structure into a graph description and write the
    // result to the indicated file
    let graph = create_graph_from_ast(&ast);
    file.write_all(graph.as_bytes())?;

    // if requested, execute `dot' on the created graph description file and
    // save the output to a pdf file
    if pdf {
        let outfile = path.replace(".gv", ".pdf");
        let mut file = File::create(&outfile)?;
        let output = Command::new("dot").arg("-Tpdf")
                                        .arg(path)
                                        .output()
                                        .expect("Failed to execute dot.");
        file.write_all(&output.stdout[..])?;
    }

    Ok(())
}

fn create_graph_from_ast(ast: &parser::ParseNode) -> String {
    // `graph' holds the actual relationships between nodes and the enclosing
    // `graph { ... }' while the `preamble' remaps node IDs and readable labels
    // (it is actually append to the end of the graph description body, though)
    let mut graph = String::new();
    let mut preamble = String::new();
    graph.push_str("graph {\n");
    preamble.push_str("{\n");

    // add the root node to the tree and delegate interpretation
    // of the children
    if ast.get_children().len() == 2 {
        // LHS of the tree
        start_branch!(graph, ast, preamble);
        add_child(&ast.get_children()[0], &mut graph, &mut preamble);

        // RHS of the tree
        append_to_branch!(graph, ast, preamble);
        add_child(&ast.get_children()[1], &mut graph, &mut preamble);
    }

    // close the right curly braces, add the preamble and return
    preamble.push_str("\t}\n");
    graph.push_str(&preamble);
    graph.push_str("}");
    graph
}

fn add_child(ast_node: &parser::ParseNode, graph: &mut String,
             preamble: &mut String) {
    let children = ast_node.get_children().len();

    if children == 0 {
        end_branch!(graph, ast_node, preamble);
    } else if children == 1 {
        if ast_node.get_long_type().contains("Parentheses") {
            start_branch!(graph, ast_node, preamble);
            add_child(&ast_node.get_children()[0], graph, preamble);
        }
    } else if children == 2 {
        append_to_branch!(graph, ast_node, preamble);
        add_child(&ast_node.get_children()[0], graph, preamble);
        append_to_branch!(graph, ast_node, preamble);
        add_child(&ast_node.get_children()[1], graph, preamble);
    }
}

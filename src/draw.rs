/* draw.rs: Create a graph of an abstract syntax tree. */
use crate::parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::process::Command;
use std::str::from_utf8;

/*
 * The following three macros are used by the graph creation functions and are
 * not exposed publicly. They determine the format of graph IDs and labels. To
 * add non-terminal types to the graph, too, this set of macros needs to be
 * expanded with appropriate functionality. I.e. for every node in the AST, the
 * graph needs to have 2 nodes, one containing the non-terminal and one
 * containing the terminal type of that AST node.
 */
macro_rules! start_branch {
    ( $graph:expr, $ast:expr, $preamble:expr, $side:expr ) => {
        let id = &format!("\"id={}_{}_{}\"",
                          $ast.get_long_type(),
                          $ast.get_depth(),
                          $side);
        let name = &format!("\"{}\"", $ast.get_short_type());
        $graph.push_str(&format!("\t{} -- ", id));
        $preamble.push_str(&format!("\t\t{} [label = {}]\n", id, name));
    };
}

macro_rules! append_to_branch {
    ( $graph:expr, $ast:expr, $preamble:expr, $side:expr ) => {
        let id = &format!("\"id={}_{}_{}\"",
                          $ast.get_long_type(),
                          $ast.get_depth(),
                          $side);
        let name = &format!("\"{}\"", $ast.get_short_type());
        $graph.push_str(&format!("{} -- ", id));
        $preamble.push_str(&format!("\t\t{} [label = {}]\n", id, name));
    };
}

macro_rules! end_branch {
    ( $graph:expr, $ast:expr, $preamble:expr, $side:expr ) => {
        let id = &format!("\"id={}_{}_{}\"",
                          $ast.get_long_type(),
                          $ast.get_depth(),
                          $side);
        let name = &format!("\"{}\"", $ast.get_short_type());
        $graph.push_str(&format!("{}\n\t", id));
        $preamble.push_str(&format!("\t\t{} [label = {}]\n", id, name));
    };
}

/*
 * Based on the root node of an AST, this function writes a graphviz `.gv' file
 * to `path' and if `pdf', it also creates a PDF using the `dot' utility, which
 * will be written to `path', too (only the file extension will change to
 * `.pdf'). Currently, only the terminal types of nodes in the graph are drawn
 * out. Non-terminal types like `Expression' could be added quite easily,
 * though.
 */
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
                                        .expect("Failed to execute dot");

        // if anything was printed on `stderr', return with an error
        let err = output.stderr;
        if err.len() > 0 {
            return Err(Error::new(ErrorKind::InvalidInput,
                                  format!("Faild to execute dot: {}",
                                          from_utf8(&err[..]).unwrap())));
        }
        file.write_all(&output.stdout[..])?;
    }

    Ok(())
}

/* The syntax of a `.gv' file is described below. */
fn create_graph_from_ast(ast: &parser::ParseNode) -> String {
    /*
     * `graph' holds the actual relationships between nodes and the enclosing
     * `graph { ... }' while the `preamble' remaps node IDs and readable labels
     * (it is actually appended to the end of the graph description body,
     * though). The resulting structure of the `.gv' file is:
     * ```
     * graph {
     *      "id_node_1" -- "id_node_2" -- "id_node_3"
     *      "id_node_1" -- "id_node_4" -- "id_node_5"
     *      [...]
     *      {
     *          "id_node_1" [label = "label of this node"]
     *          [...]
     *      }
     * }
     * ```
     * All node IDs and labels are always enclosed in double-quotes to avoid
     * syntax errors (`+', `>', etc. are valid `dot' syntax).
     */
    let mut graph = String::new();
    let mut preamble = String::new();
    graph.push_str("graph {\n");
    preamble.push_str("{\n");

    // add the root node to the tree and delegate interpretation
    // of the children
    if let Some(lchild) = ast.get_lchild() {
        // LHS of the tree
        start_branch!(graph, ast, preamble, "root");
        add_child(lchild, &mut graph, &mut preamble, "left");

        if let Some(rchild) = ast.get_rchild() {
            // RHS of the tree
            append_to_branch!(graph, ast, preamble, "root");
            add_child(rchild, &mut graph, &mut preamble, "right");
        }
    }

    // close the right curly braces, add the preamble and return
    preamble.push_str("\t}\n");
    graph.push_str(&preamble);
    graph.push_str("}");
    graph
}

/*
 * NOTE: the `side' would ideally be incremented from level to level,
 * resulting in strings like 'rootleftleftsingleleft'. The current solution
 * still doesn't guarantee unique names for every node. A nicer solution would
 * probably draw graph nodes based on the non-terminal types of the AST nodes.
 */
fn add_child(ast_node: &parser::ParseNode, graph: &mut String,
             preamble: &mut String, side: &str) {
    if let None = ast_node.get_lchild() {
        end_branch!(graph, ast_node, preamble, side);
    } else if let Some(lchild) = ast_node.get_lchild() {
        if let None = ast_node.get_rchild() {
            // this node has only one child, which means this must be
            // parentheses (currently this is a somewhat dirty hack)
            if ast_node.get_long_type().contains("Parentheses") {
                start_branch!(graph, ast_node, preamble, side);
                add_child(lchild, graph, preamble, "single");
            }
        } else if let Some(rchild) = ast_node.get_rchild() {
            append_to_branch!(graph, ast_node, preamble, side);
            add_child(lchild, graph, preamble, "left");
            append_to_branch!(graph, ast_node, preamble, side);
            add_child(rchild, graph, preamble, "right");
        }
    }
}

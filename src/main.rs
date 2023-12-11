use anyhow::{Context, Result};
use clap::Parser;
use tree_sitter::{Tree, Node};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

trait Walkable {
    fn walk_node(&self) -> tree_sitter::TreeCursor;
}

impl Walkable for Tree {
    fn walk_node(&self) -> tree_sitter::TreeCursor {
        self.walk()
    }
}

impl Walkable for Node<'_> {
    fn walk_node(&self) -> tree_sitter::TreeCursor {
        self.walk()
    }
}

/// Get an iterator walking the given tree in depth-first prefix order
fn tree_nodes<W: Walkable>(tree: &W) -> impl IntoIterator<Item=Node> {
    let mut cursor = tree.walk_node();
    let mut ascending = false;
    let mut done = false;

    std::iter::from_fn(move || {
        if done {
            return None;
        }

        let node = cursor.node();

        // advance to the next entry
        loop {
            // advance cursor
            if !ascending && cursor.goto_first_child() {
                // descending into this node
                break;
            }
            if cursor.goto_next_sibling() {
                ascending = false;
                break;
            }

            // we need to try moving to the parent; set the ascending flag so we don't go back into
            // this node
            if cursor.goto_parent() {
                // no further nodes to visit - terminate iteration
                ascending = true;
                continue;
            }

            // no further nodes to visit - terminate iteration
            done = true;
            break;
        }

        Some(node)
    })
}

/// Traverse tree
/// pull out functions
/// and generate control flow graph (CFG)
/// every function has it's own CFG.
/// Returns an iterator of all functions inside of it
fn tree_functions<'a>(data: &'a [u8], tree: &'a Tree) -> impl IntoIterator<Item=FunctionDef<'a>> + 'a {
    tree_nodes(tree)
        .into_iter()
        .filter(|n| n.kind() == "function_definition")
        .map(|n| {
            let name = n.child_by_field_name("name").expect("Missing function name");
            let params = n.child_by_field_name("parameters").expect("Missing function params");
            let body = n.child_by_field_name("body").expect("Missing function body");

            let name = name.utf8_text(data).unwrap();
            let params = params.named_children(&mut n.walk())
                        .map(|child| match child.kind() {
                            "identifier" => Parameter::Ident(child.utf8_text(data).unwrap()),
                            _ => unimplemented!("unknown parameter kind"),
                        })
                        .collect::<Vec<_>>();

            FunctionDef { name, params, body }
        })
}

#[derive(Debug)]
struct FunctionDef<'d> {
    name: &'d str,
    params: Vec<Parameter<'d>>,
    body: Node<'d>,
}

#[derive(Debug)]
enum Parameter<'a> {
    Ident(&'a str),
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read(&args.path)
    .with_context(|| format!("Could not read file: {}", args.path.display()))?;

    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_python::language()).expect("Error loading Python grammar");
    let parsed = parser.parse(&content, None)
                .context("Failed to unwrap tree, it's christmas after all you unwrap presents not trees")?;
    //println!("{:?}", parsed);

    for def in tree_functions(&content, &parsed) {
        dbg!(&def);
        for child in def.body.children(&mut def.body.walk()) {
            dbg!(child);
        }
    }

    Ok(())
}

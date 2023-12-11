use anyhow::{Context, Result};
use clap::Parser;
use tree_sitter::{Tree, Node};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}
/// Traverse tree
/// pull out functions
/// and generate control flow graph (CFG)
/// every function has it's own CFG.
/// Returns an iterator of all functions inside of it
fn tree_functions(tree:&Tree) -> impl IntoIterator<Item=&Node>{
    vec![]
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
    .with_context(|| format!("Could not read file: {}", args.path.display()))?;
    for line in content.lines() {
        println!("{}", line)
    }
    //let parser = enderpy_python_parser::Parser::new(content).parse();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_python::language()).expect("Error loading Python grammar");
    let parsed = parser.parse(content, None).context("Failed to unwrap tree, it's christmas after all you unwrap presents not trees")?;
    println!("{:?}", parsed);

    Ok(())
}

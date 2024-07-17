use clap::Parser;
use rust_bf::Program;
use std::fs;
use std::io::{stdin, stdout};

/// Brainfuck interpreter in Rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the Brainfuck file to execute
    filename: String,
}

fn main() {
    let args = Args::parse();
    let source = fs::read_to_string(args.filename).expect("Unable to read file");
    Program::parse(&source)
        .unwrap()
        .run(&mut stdin(), &mut stdout())
        .unwrap();
}

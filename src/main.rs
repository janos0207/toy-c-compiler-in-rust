mod tinyc;

use std::{env, process};

use tinyc::codegen::generate;
use tinyc::lexer::Tokenizer;
use tinyc::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("wrong arguments number \n");
        process::exit(1);
    }

    let tokenizer = Tokenizer::tokenize(&args[1]);
    let node = Parser::parse(tokenizer);
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    generate(*node.unwrap());

    println!("  pop rax");
    println!("  ret");
}

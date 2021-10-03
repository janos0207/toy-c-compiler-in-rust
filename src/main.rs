mod tinyc;

use std::{env, process};

use tinyc::codegen::generate;
use tinyc::lexer::Tokenizer;
use tinyc::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("wrong arguments number");
        process::exit(1);
    }

    let tokenizer = Tokenizer::tokenize(&args[1]);
    let parser = Parser::parse(tokenizer);
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    // prologue: allocate the space of 26 variables
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    for tree in parser.code {
        if tree.is_none() {
            break;
        }
        generate(*tree.unwrap());
        println!("  pop rax");
    }

    // epilogue: return the value of the last expression at RAX
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}

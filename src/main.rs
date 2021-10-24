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
    let mut parser = Parser::parse(tokenizer);
    generate(&mut parser)
}

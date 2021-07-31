use std::io::{self, Write};
use std::iter::Peekable;
use std::{env, process};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        io::stderr().write_all(b"wrong arguments number \n")?;
        process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let mut chars = args[1].chars().peekable();
    println!("  mov rax, {}", parse_int(&mut chars));
    let mut next_char: Option<char>;
    loop {
        next_char = chars.next();
        match next_char {
            Some('+') => println!("  add rax, {}", parse_int(&mut chars)),
            Some('-') => println!("  sub rax, {}", parse_int(&mut chars)),
            Some(_) => {
                io::stderr().write_all(b"unexpected character")?;
                process::exit(1);
            }
            None => break,
        }
    }
    println!("  ret");

    Ok(())
}

fn parse_int<'a>(chars: &'a mut Peekable<std::str::Chars>) -> String {
    let mut integer = String::from("");
    while let Some(next_char) = chars.peek() {
        if next_char.is_numeric() {
            integer.push(chars.next().unwrap())
        } else {
            break;
        }
    }
    return integer;
}

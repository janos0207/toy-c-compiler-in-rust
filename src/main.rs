use std::cell::RefCell;
use std::iter::Peekable;
use std::rc::Rc;
use std::{env, process};

#[derive(Debug, PartialEq, Clone)]
enum TokenKind {
    TkReserved,
    TkNum,
    TkEOF,
}

type TokenLink = Option<Rc<RefCell<Token>>>;

type PeekableString<'a> = Peekable<std::str::Chars<'a>>;

#[derive(Debug, Clone)]
struct Token {
    kind: TokenKind,
    next: TokenLink,
    val: Option<String>,
    character: Option<char>,
}

#[derive(Debug, Clone)]
struct Tokenizer<'a> {
    current: TokenLink,
    chars: Peekable<std::str::Chars<'a>>,
    head: TokenLink,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(string: &'a str) -> Tokenizer {
        let mut tokenizer = Tokenizer::new_empty(string.chars().peekable());
        let mut next_char: Option<&char>;
        loop {
            next_char = tokenizer.chars.peek();
            match next_char {
                Some(' ') => {
                    tokenizer.chars.next();
                    continue;
                }
                Some('+') | Some('-') => {
                    tokenizer.new_token(TokenKind::TkReserved);
                    continue;
                }
                Some('0'..='9') => {
                    tokenizer.new_token(TokenKind::TkNum);
                    continue;
                }
                Some(_) => {
                    eprintln!("{}", string);
                    eprintln!("Not implemented");
                    process::exit(1);
                }
                None => {
                    tokenizer.new_token(TokenKind::TkEOF);
                    break;
                }
            }
        }

        tokenizer
    }

    fn new_empty(chars: PeekableString<'a>) -> Tokenizer {
        Tokenizer {
            current: None,
            chars: chars,
            head: None,
        }
    }

    fn new_token(&mut self, kind: TokenKind) {
        let mut val: Option<String> = None;
        let mut character: Option<char> = None;
        if kind == TokenKind::TkNum {
            val = self.parse_int();
        } else {
            character = self.chars.next();
        }

        let token = Token {
            kind: kind,
            next: None,
            val: val,
            character: character,
        };
        let token_pointer = Rc::new(RefCell::new(token));

        match self.current.take() {
            Some(curr) => {
                curr.borrow_mut().next = Some(token_pointer.clone());
            }
            None => {
                self.head = Some(token_pointer.clone());
            }
        }

        self.current = Some(token_pointer);
    }

    fn expect(&mut self, op: char) {
        if let Some(head) = self.head.as_ref() {
            let head_ref = head.borrow();
            if head_ref.kind != TokenKind::TkReserved || head_ref.character != Some(op) {
                eprintln!("the character is not {}, got={:?}", op, head_ref.character);
                process::exit(1);
            }
        } else {
            eprintln!("head is None");
            process::exit(1);
        }

        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            }
        });
    }

    fn consume(&mut self, op: char) -> bool {
        if let Some(head) = self.head.as_ref() {
            let head_ref = head.borrow();
            if head_ref.kind != TokenKind::TkReserved || head_ref.character != Some(op) {
                return false;
            }
        } else {
            eprintln!("head is None");
            process::exit(1);
        }

        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            }
        });
        return true;
    }

    fn expect_number(&mut self) -> Option<String> {
        let val = self.head.take().map(|head| {
            let mut head_ref = head.borrow_mut();
            if let Some(next) = head_ref.next.take() {
                if head_ref.kind != TokenKind::TkNum {
                    eprintln!("Not a number");
                    process::exit(1);
                }
                self.head = Some(next);
                head_ref.val.clone()
            } else {
                None
            }
        });
        return val.unwrap_or(Some(String::from("")));
    }

    fn at_eof(&mut self) -> bool {
        if let Some(ref head) = self.head {
            return head.borrow().kind == TokenKind::TkEOF;
        } else {
            eprintln!("tokenizer's head is None");
            process::exit(1);
        }
    }

    fn parse_int(&mut self) -> Option<String> {
        let mut integer = String::from("");

        while let Some(next_char) = self.chars.peek() {
            if next_char.is_numeric() {
                integer.push(self.chars.next().unwrap())
            } else if next_char == &' ' {
                self.chars.next();
                continue;
            } else {
                break;
            }
        }

        return Some(integer);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("wrong arguments number \n");
        process::exit(1);
    }

    let mut tokenizer = Tokenizer::tokenize(&args[1]);

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    println!("  mov rax, {}", tokenizer.expect_number().unwrap());

    while !tokenizer.at_eof() {
        if tokenizer.consume('+') {
            println!("  add rax, {}", tokenizer.expect_number().unwrap());
            continue;
        };
        tokenizer.expect('-');
        println!("  sub rax, {}", tokenizer.expect_number().unwrap());
    }

    println!("  ret");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_int() {
        let mut tokenizer = Tokenizer::new_empty("42".chars().peekable());
        assert_eq!(tokenizer.parse_int().unwrap(), String::from("42"));
    }
}

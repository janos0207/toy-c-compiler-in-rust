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
    string: String, // token string
}

#[derive(Debug, Clone)]
struct Tokenizer<'a> {
    current: TokenLink,
    chars: Peekable<std::str::Chars<'a>>,
    head: TokenLink,
}

impl<'a> Tokenizer<'a> {
    fn tokenize(string: &'a str) -> Self {
        let mut tokenizer = Tokenizer::new_empty(string.chars().peekable());
        let mut next_char: Option<&char>;
        loop {
            next_char = tokenizer.chars.peek();
            match next_char {
                Some(' ') => {
                    tokenizer.chars.next();
                }
                // is there a better notation?
                Some('+') | Some('-') | Some('*') | Some('/') | Some('(') | Some(')') => {
                    let string = tokenizer.chars.next().unwrap().to_string();
                    tokenizer.new_token(TokenKind::TkReserved, string);
                }
                Some('=') => {
                    let mut string = tokenizer.chars.next().unwrap().to_string();
                    string = Tokenizer::peek_and_append_char(&mut tokenizer, string, '=');
                    tokenizer.new_token(TokenKind::TkReserved, string);
                }
                Some('>') => {
                    let mut string = tokenizer.chars.next().unwrap().to_string();
                    string = Tokenizer::peek_and_append_char(&mut tokenizer, string, '=');
                    tokenizer.new_token(TokenKind::TkReserved, string);
                }
                Some('<') => {
                    let mut string = tokenizer.chars.next().unwrap().to_string();
                    string = Tokenizer::peek_and_append_char(&mut tokenizer, string, '=');
                    tokenizer.new_token(TokenKind::TkReserved, string);
                }
                Some('!') => {
                    let mut string = tokenizer.chars.next().unwrap().to_string();
                    let next_char = tokenizer.chars.next().unwrap_or('\0');
                    if next_char != '=' {
                        eprintln!("tokenizer: unexpected token '!'");
                        process::exit(1);
                    } else {
                        string.push(next_char);
                    }
                    tokenizer.new_token(TokenKind::TkReserved, string);
                }
                Some('0'..='9') => {
                    tokenizer.new_token(TokenKind::TkNum, String::from(""));
                }
                Some(_) => {
                    eprintln!("{}", string);
                    eprintln!("tokenizer: Not implemented");
                    process::exit(1);
                }
                None => {
                    tokenizer.new_token(TokenKind::TkEOF, String::from(""));
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

    fn new_token(&mut self, kind: TokenKind, string: String) {
        let mut val: Option<String> = None;
        if kind == TokenKind::TkNum {
            val = self.parse_int();
        }

        let token = Token {
            kind: kind,
            next: None,
            val: val,
            string: string,
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

    fn expect(&mut self, op: &str) {
        if let Some(head) = self.head.as_ref() {
            let head_ref = head.borrow();
            if head_ref.kind != TokenKind::TkReserved || head_ref.string != op {
                eprintln!("the character is not {}, got={:?}", op, head_ref.string);
                process::exit(1);
            }
        } else {
            eprintln!("head is None");
            process::exit(1);
        }

        self.head.take().map(|head| {
            if let Some(next) = head.borrow().next.clone() {
                self.head = Some(next);
            }
        });
    }

    fn consume(&mut self, op: &str) -> bool {
        if let Some(head) = self.head.as_ref() {
            let head_ref = head.borrow();
            if head_ref.kind != TokenKind::TkReserved || head_ref.string != op.to_string() {
                return false;
            }
        } else {
            eprintln!("head is None");
            process::exit(1);
        }

        self.head.take().map(|head| {
            if let Some(next) = head.borrow().next.clone() {
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

    fn peek_and_append_char(
        tokenizer: &mut Tokenizer,
        mut string: String,
        expected: char,
    ) -> String {
        let next_char = tokenizer.chars.peek();
        match next_char {
            Some(c) if c == &expected => string.push(tokenizer.chars.next().unwrap()),
            None => {}
            _ => eprintln!("tokenizer: not implemented operator"),
        }
        return string;
    }
}

#[derive(Debug, Clone, PartialEq)]
enum NodeKind {
    NodeAdd,
    NodeSub,
    NodeMul,
    NodeDiv,
    NodeNum,
    NodeEQ,
    NodeNE,
    NodeLT,
    NodeLE,
}

type Tree = Option<Box<Node>>;

#[derive(Debug, Clone)]
struct Node {
    kind: NodeKind,
    lhs: Tree,
    rhs: Tree,
    val: Option<String>,
}

#[derive(Debug, Clone)]
struct Parser<'a> {
    lexer: Tokenizer<'a>,
    tree: Tree,
}

impl<'a> Parser<'a> {
    pub fn parse(lexer: Tokenizer<'a>) -> Tree {
        let mut parser = Parser {
            lexer: lexer,
            tree: None,
        };
        parser.expr()
    }

    fn new_node(&mut self, kind: NodeKind, lhs: Tree, rhs: Tree) -> Tree {
        let node = Node {
            kind: kind,
            lhs: lhs,
            rhs: rhs,
            val: None,
        };
        Some(Box::new(node))
    }

    fn new_node_num(&self, val: String) -> Tree {
        let node = Node {
            kind: NodeKind::NodeNum,
            lhs: None,
            rhs: None,
            val: Some(val),
        };
        Some(Box::new(node))
    }

    // expr = equality
    fn expr(&mut self) -> Tree {
        return self.equality();
    }

    // equality = relational ("==" relational | "!=" relational)*
    fn equality(&mut self) -> Tree {
        let mut node = self.relational();

        loop {
            if self.lexer.consume("==") {
                let rhs = self.relational();
                node = self.new_node(NodeKind::NodeEQ, node, rhs)
            } else if self.lexer.consume("!=") {
                let rhs = self.relational();
                node = self.new_node(NodeKind::NodeNE, node, rhs)
            } else {
                return node;
            }
        }
    }

    // relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    fn relational(&mut self) -> Tree {
        let mut node = self.add();

        loop {
            if self.lexer.consume("<") {
                let rhs = self.add();
                node = self.new_node(NodeKind::NodeLT, node, rhs);
            } else if self.lexer.consume("<=") {
                let rhs = self.add();
                node = self.new_node(NodeKind::NodeLE, node, rhs);
            } else if self.lexer.consume(">") {
                let lhs = self.add();
                node = self.new_node(NodeKind::NodeLT, lhs, node);
            } else if self.lexer.consume(">=") {
                let lhs = self.add();
                node = self.new_node(NodeKind::NodeLE, lhs, node);
            } else {
                return node;
            }
        }
    }

    // add = mul ("+" mul | "-" mul)*
    fn add(&mut self) -> Tree {
        let mut node = self.mul();

        loop {
            if self.lexer.consume("+") {
                let rhs = self.mul();
                node = self.new_node(NodeKind::NodeAdd, node, rhs);
            } else if self.lexer.consume("-") {
                let rhs = self.mul();
                node = self.new_node(NodeKind::NodeSub, node, rhs);
            } else {
                return node;
            }
        }
    }

    // mul = unary ("*" unary | "/" unary)*
    fn mul(&mut self) -> Tree {
        let mut node = self.unary();

        loop {
            if self.lexer.consume("*") {
                let rhs = self.unary();
                node = self.new_node(NodeKind::NodeMul, node, rhs);
            } else if self.lexer.consume("/") {
                let rhs = self.unary();
                node = self.new_node(NodeKind::NodeDiv, node, rhs);
            } else {
                return node;
            }
        }
    }

    // unary = ("+" | "-")? primary
    fn unary(&mut self) -> Tree {
        if self.lexer.consume("+") {
            return self.primary();
        }
        if self.lexer.consume("-") {
            let zero = self.new_node_num(String::from("0"));
            let rhs = self.primary();
            return self.new_node(NodeKind::NodeSub, zero, rhs);
        }
        return self.primary();
    }

    // primary = num | "(" expr ")"
    fn primary(&mut self) -> Tree {
        if self.lexer.consume("(") {
            let node = self.expr();
            self.lexer.expect(")");
            return node;
        }
        if let Some(val) = self.lexer.expect_number() {
            self.new_node_num(val)
        } else {
            eprintln!("parser: expected number");
            process::exit(1);
        }
    }
}

fn generate(node: Node) {
    if node.kind == NodeKind::NodeNum {
        println!("  push {}", node.val.unwrap());
        return;
    }

    generate(*node.lhs.unwrap());
    generate(*node.rhs.unwrap());

    println!("  pop rdi");
    println!("  pop rax");

    match node.kind {
        NodeKind::NodeAdd => println!("  add rax, rdi"),
        NodeKind::NodeSub => println!("  sub rax, rdi"),
        NodeKind::NodeMul => println!(" imul rax, rdi"),
        NodeKind::NodeDiv => {
            println!("  cqo");
            println!("  idiv rdi");
        }
        NodeKind::NodeEQ => {
            println!("  cmp rax, rdi");
            println!("  sete al");
            println!("  movzb rax, al");
        }
        NodeKind::NodeNE => {
            println!("  cmp rax, rdi");
            println!("  setne al");
            println!("  movzb rax, al");
        }
        NodeKind::NodeLT => {
            println!("  cmp rax, rdi");
            println!("  setl al");
            println!("  movzb rax, al");
        }
        NodeKind::NodeLE => {
            println!("  cmp rax, rdi");
            println!("  setle al");
            println!("  movzb rax, al");
        }
        _ => eprintln!("Unsupported token kind!"),
    }

    println!("  push rax");
}

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_int() {
        let mut tokenizer = Tokenizer::new_empty("42".chars().peekable());
        assert_eq!(tokenizer.parse_int().unwrap(), String::from("42"));
    }
}

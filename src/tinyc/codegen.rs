use super::parser::{Node, NodeKind};
use std::process;

fn gen_lval(node: &Node) {
    if node.kind != NodeKind::NodeLVar {
        eprintln!("the left side value of assignment is not a variable");
        process::exit(1);
    }
    println!("  mov rax, rbp");
    println!("  sub rax, {}", node.offset);
    println!("  push rax");
}

pub fn generate(node: Node) {
    match node.kind {
        NodeKind::NodeNum => {
            println!("  push {}", node.val.unwrap());
            return;
        }
        NodeKind::NodeLVar => {
            self::gen_lval(&node);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            return;
        }
        NodeKind::NodeAssign => {
            self::gen_lval(&node.lhs.unwrap());
            generate(*node.rhs.unwrap());
            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
            return;
        }
        _ => {}
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

use super::parser::{Node, NodeKind, Parser};
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

pub fn gen_stmt(node: Node) {
    match node.kind {
        NodeKind::NodeNum => {
            println!("  push {}", node.val.unwrap());
            return;
        }
        NodeKind::NodeLVar => {
            gen_lval(&node);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            return;
        }
        NodeKind::NodeAssign => {
            gen_lval(&node.lhs.unwrap());
            gen_stmt(*node.rhs.unwrap());
            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
            return;
        }
        _ => {}
    }

    gen_stmt(*node.lhs.unwrap());
    gen_stmt(*node.rhs.unwrap());

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

pub fn generate(parser: &Parser) {
    println!(".intel_syntax noprefix");

    let stack_size = calculate_total_offsets(parser);

    println!(".global main");
    println!("main:");

    // prologue: allocate the space of 26 variables
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, {}", stack_size);

    let code = parser.code.clone();

    for tree in code {
        if tree.is_none() {
            break;
        }
        gen_stmt(*tree.unwrap());
        println!("  pop rax");
    }

    // epilogue: return the value of the last expression at RAX
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}

// Round up `n` to the nearest multiple of `align`
fn align_to(n: usize, align: usize) -> usize {
    (n + align - 1) / align * align
}

fn calculate_total_offsets(parser: &Parser) -> usize {
    let offset = parser.locals.len() * 8;
    align_to(offset, 16)
}

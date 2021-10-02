use super::parser::{Node, NodeKind};

pub fn generate(node: Node) {
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

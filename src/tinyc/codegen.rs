use super::parser::{Node, NodeKind, Parser};
use std::process;

pub struct CodeGen {
    jmp_counter: i64,
}

impl CodeGen {
    pub fn init() -> Self {
        CodeGen { jmp_counter: 0 }
    }

    fn gen_lval(node: &Node) {
        if node.kind != NodeKind::NodeLVar {
            eprintln!("the left side value of assignment is not a variable");
            process::exit(1);
        }
        println!("  mov rax, rbp");
        println!("  sub rax, {}", node.offset);
        println!("  push rax");
    }

    fn gen_stmt(&mut self, node: Node) {
        match node.kind {
            NodeKind::NodeNum => {
                println!("  push {}", node.val.unwrap());
                return;
            }
            NodeKind::NodeLVar => {
                CodeGen::gen_lval(&node);
                println!("  pop rax");
                println!("  mov rax, [rax]");
                println!("  push rax");
                return;
            }
            NodeKind::NodeAssign => {
                CodeGen::gen_lval(&node.lhs.unwrap());
                self.gen_stmt(*node.rhs.unwrap());
                println!("  pop rdi");
                println!("  pop rax");
                println!("  mov [rax], rdi");
                println!("  push rdi");
                return;
            }
            NodeKind::NodeBlock => {
                for stmt in node.body.into_iter() {
                    self.gen_stmt(*stmt.unwrap());
                    println!("  pop rax");
                }
                println!("  push rax");
                return;
            }
            NodeKind::NodeIf => {
                self.gen_stmt(*node.cond.unwrap());
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je .L.else.{}", self.jmp_counter);
                self.gen_stmt(*node.then.unwrap());
                println!("  jmp .L.end.{}", self.jmp_counter);
                println!(".L.else.{}:", self.jmp_counter);
                if let Some(els) = node.els {
                    self.gen_stmt(*els);
                }
                println!(".L.end.{}:", self.jmp_counter);
                self.jmp_counter += 1;
                return;
            }
            NodeKind::NodeFor => {
                if let Some(init) = node.init {
                    self.gen_stmt(*init);
                }
                println!(".L.begin.{}:", self.jmp_counter);
                if let Some(cond) = node.cond {
                    self.gen_stmt(*cond);
                    println!("  cmp rax, 0");
                    println!("  je .L.end.{}", self.jmp_counter);
                }
                self.gen_stmt(*node.then.unwrap());
                if let Some(inc) = node.inc {
                    self.gen_stmt(*inc);
                }
                println!("  jmp .L.begin.{}", self.jmp_counter);
                println!(".L.end.{}:", self.jmp_counter);
                self.jmp_counter += 1;
                return;
            }
            NodeKind::NodeReturn => {
                self.gen_stmt(*node.lhs.unwrap());
                println!("  pop rax");
                println!("  mov rsp, rbp");
                println!("  pop rbp");
                println!("  ret");
                return;
            }
            _ => {}
        }

        self.gen_stmt(*node.lhs.unwrap());
        self.gen_stmt(*node.rhs.unwrap());

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

    pub fn generate(&mut self, parser: &Parser) {
        println!(".intel_syntax noprefix");

        let stack_size = CodeGen::calculate_total_offsets(parser);

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
            self.gen_stmt(*tree.unwrap());
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
        CodeGen::align_to(offset, 16)
    }
}

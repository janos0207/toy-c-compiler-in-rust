use super::lexer::Tokenizer;
use std::process;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    NodeAdd,
    NodeSub,
    NodeMul,
    NodeDiv,
    NodeNum,
    NodeEQ,
    NodeNE,
    NodeLT,
    NodeLE,
    NodeAssign,
    NodeReturn,
    NodeLVar,
}

type Tree = Option<Box<Node>>;

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Tree,
    pub rhs: Tree,
    pub val: Option<String>,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub struct LVar {
    name: String,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lexer: Tokenizer<'a>,
    tree: Tree,
    pub code: Vec<Tree>,
    pub locals: Vec<LVar>,
}

impl<'a> Parser<'a> {
    pub fn parse(lexer: Tokenizer<'a>) -> Self {
        let mut parser = Parser {
            lexer: lexer,
            tree: None,
            code: vec![None; 100],
            locals: Vec::new(),
        };
        parser.program();
        parser
    }

    fn new_node(&mut self, kind: NodeKind, lhs: Tree, rhs: Tree) -> Tree {
        let node = Node {
            kind: kind,
            lhs: lhs,
            rhs: rhs,
            val: None,
            offset: 0,
        };
        Some(Box::new(node))
    }

    fn new_node_num(&self, val: String) -> Tree {
        let node = Node {
            kind: NodeKind::NodeNum,
            lhs: None,
            rhs: None,
            val: Some(val),
            offset: 0,
        };
        Some(Box::new(node))
    }

    pub fn find_var(&mut self, val: String) -> usize {
        let mut locals = self.locals.clone();
        loop {
            if let Some(local) = locals.pop() {
                if local.name == val {
                    return local.offset;
                }
            } else {
                let offset = (self.locals.len() + 1) * 8;
                let new_var = LVar {
                    name: val,
                    offset: offset,
                };
                self.locals.push(new_var.clone());
                return offset;
            }
        }
    }

    // program = stmt*
    fn program(&mut self) {
        let mut i = 0;
        while !self.lexer.at_eof() {
            self.code[i] = self.stmt();
            i += 1;
        }
    }

    // stmt = expr ";"
    //      | "return" expr ";"
    fn stmt(&mut self) -> Tree {
        let node: Tree;
        if self.lexer.consume("return") {
            let lhs = self.expr();
            node = self.new_node(NodeKind::NodeReturn, lhs, None);
            self.lexer.expect(";");
            return node;
        }
        node = self.expr();
        self.lexer.expect(";");
        return node;
    }

    // expr = assign
    fn expr(&mut self) -> Tree {
        return self.assign();
    }

    // assign = equality ("=" assign)?
    fn assign(&mut self) -> Tree {
        let mut node = self.equality();
        if self.lexer.consume("=") {
            let rhs = self.assign();
            node = self.new_node(NodeKind::NodeAssign, node, rhs);
        }
        return node;
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

    // primary = num | ident | "(" expr ")"
    fn primary(&mut self) -> Tree {
        if self.lexer.consume("(") {
            let node = self.expr();
            self.lexer.expect(")");
            return node;
        }
        if let Some(val) = self.lexer.is_ident_token() {
            let node = Node {
                kind: NodeKind::NodeLVar,
                lhs: None,
                rhs: None,
                val: None,
                offset: self.find_var(val),
            };
            return Some(Box::new(node));
        }

        if let Some(val) = self.lexer.expect_number() {
            self.new_node_num(val)
        } else {
            eprintln!("parser: expected number");
            process::exit(1);
        }
    }
}

use std::collections::HashMap;
use std::fmt::Write;

pub struct NodeExprIntLit {
    pub int_lit: i64,
}

pub struct NodeExprIdent {
    pub ident: String,
}

pub enum NodeExpr {
    IntLit(NodeExprIntLit),
    Ident(NodeExprIdent),
}

pub struct NodeStmtExit {
    pub expr: NodeExpr,
}

pub struct NodeStmtLet {
    pub ident: NodeExprIdent,
    pub expr: NodeExpr,
}

pub enum NodeStmt {
    Exit(NodeStmtExit),
    Let(NodeStmtLet),
}

pub struct NodeProg {
    pub stmts: Vec<NodeStmt>,
}

pub struct Generator {
    m_prog: NodeProg,
    m_output: String,
    m_stack_size: usize,
    m_vars: HashMap<String, Var>,
}

impl Generator {
    pub fn new(prog: NodeProg) -> Self {
        Generator {
            m_prog: prog,
            m_output: String::new(),
            m_stack_size: 0,
            m_vars: HashMap::new(),
        }
    }
    pub fn gen_prog(&mut self) -> String {
        write!(self.m_output, "global _start\n_start:\n").unwrap();

        // for stmt in &self.m_prog.stmts {
        //     self.gen_stmt(stmt);
        // }

        write!(self.m_output, "    mov rax, 60\n").unwrap();
        write!(self.m_output, "    mov rdi, 0\n").unwrap();
        write!(self.m_output, "    syscall\n").unwrap();

        self.m_output.clone()
    }

    fn push(&mut self, reg: &str) {
        write!(self.m_output, "    push {}\n", reg).unwrap();
        self.m_stack_size += 1;
    }

    fn pop(&mut self, reg: &str) {
        write!(self.m_output, "    pop {}\n", reg).unwrap();
        self.m_stack_size -= 1;
    }
}

struct Var {
    stack_loc: usize,
}

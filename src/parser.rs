#[derive(Debug)]
pub struct NodeExprIntLit {
    pub int_lit: i64,
}

#[derive(Debug)]
pub struct NodeExprIdent {
    pub ident: String,
}

#[derive(Debug)]
pub enum NodeExpr {
    IntLit(NodeExprIntLit),
    Ident(NodeExprIdent),
}
#[derive(Debug)]
pub struct NodeStmtExit {
    pub expr: NodeExpr,
}

#[derive(Debug)]
pub struct NodeStmtLet {
    pub ident: NodeExprIdent,
    pub expr: NodeExpr,
}

#[derive(Debug)]
pub enum NodeStmt {
    Exit(NodeStmtExit),
    Let(NodeStmtLet),
}

pub struct NodeProg {
    pub stmts: Vec<NodeStmt>,
    pub exit_code: String,
}

pub struct Parser {
    // pub tokens: Vec<Token>,
    pub file_content: String,
}

impl Parser {
    pub fn new(file_content: String) -> Self {
        Parser { file_content }
    }

    pub fn parse(&mut self) -> NodeProg {
        // the file has one return with a value, parse the value
        let mut stmts = Vec::new();
        let mut pos = 0;

        // parse the return keyword
        let return_keyword = &self.file_content[pos..pos + 6];
        if return_keyword != "return" {
            panic!("Expected return keyword");
        }

        pos += 6;

        // parse the value
        let mut value = String::new();
        while pos < self.file_content.len() {
            let c = self.file_content.chars().nth(pos).unwrap();
            if c == ';' {
                break;
            }
            if c == ' ' {
                pos += 1;
                continue;
            }
            value.push(c);
            pos += 1;
        }

        NodeProg {
            stmts,
            exit_code: value,
        }
    }

    // fn parse_stmt(&mut self) -> NodeStmt {
    //     match self.tokens[self.pos] {
    //         Token::Exit => {
    //             self.pos += 1;
    //             NodeStmt::Exit(NodeStmtExit {
    //                 expr: self.parse_expr(),
    //             })
    //         }
    //         Token::Let => {
    //             self.pos += 1;
    //             let ident = self.parse_ident();
    //             self.expect(Token::Equal);
    //             let expr = self.parse_expr();
    //             NodeStmt::Let(NodeStmtLet { ident, expr })
    //         }
    //         _ => panic!("Unexpected token"),
    //     }
    // }

    // fn parse_expr(&mut self) -> NodeExpr {
    //     match self.tokens[self.pos] {
    //         Token::IntLit(int_lit) => {
    //             self.pos += 1;
    //             NodeExpr::IntLit(NodeExprIntLit { int_lit })
    //         }
    //         Token::Ident(ref ident) => {
    //             self.pos += 1;
    //             NodeExpr::Ident(NodeExprIdent {
    //                 ident: ident.clone(),
    //             })
    //         }
    //         _ => panic!("Unexpected token"),
    //     }
    // }

    // fn parse_ident(&mut self) -> NodeExprIdent {
    //     match self.tokens[self.pos] {
    //         Token::Ident(ref ident) => {
    //             self.pos += 1;
    //             NodeExprIdent {
    //                 ident: ident.clone(),
    //             }
    //         }
    //         _ => panic!("Unexpected token"),
    //     }
    // }

    // fn expect(&mut self, token: Token) {
    //     if self.tokens[self.pos] != token {
    //         panic!("Unexpected token");
    //     }
    //     self.pos += 1;
    // }
}

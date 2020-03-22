use crate::lexer::{Token, TokenKind};

#[derive(Debug, PartialEq)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num,
    Eq,
    Ne,
    Lt,
    Le,
    Assign,
    Lvar
}

#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub val: Option<i32>,
    pub offset: Option<u32>,
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            position: 0,
        }
    }
    pub fn parse(&mut self) -> Result<Vec<Node>, String> {
        self.program()
    }
    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }
    fn at_eof(&self) -> bool {
        self.current_token().kind == TokenKind::EOF
    }
    fn consume(&mut self, op: String) -> bool {
        if self.current_token().kind != TokenKind::Reserved || self.current_token().str != op {
            false
        } else {
            self.position += 1;
            true
        }
    }
    fn expect(&mut self, op: String) -> Result<(), String> {
        if self.current_token().kind != TokenKind::Reserved || self.current_token().str != op {
            Err(format!("expected {}", op))
        } else {
            self.position += 1;
            Ok(())
        }
    }
    fn consume_ident(&mut self) -> Option<String> {
        if self.current_token().kind != TokenKind::Ident {
            None
        } else {
            let val = self.current_token().str.clone();
            self.position += 1;
            Some(val)
        }
    }
    fn expect_number(&mut self) -> Result<i32, String> {
        if self.current_token().kind != TokenKind::Num {
            Err("expected a number".to_string())
        } else {
            let val = self.current_token().val;
            self.position += 1;
            val.ok_or("system error".to_string())
        }
    }
    // program = stmt*
    fn program(&mut self) -> Result<Vec<Node>, String> {
        let mut nodes = Vec::new();
        while !self.at_eof() {
            match self.stmt() {
                Ok(v) => nodes.push(v),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(nodes)
    }
    // stmt = expr ";"
    fn stmt(&mut self) -> Result<Node, String> {
        let ret = self.expr();
        if let Err(e) = self.expect(";".to_string()) {
            return Err(e)
        }
        ret
    }
    // expr = assign
    fn expr(&mut self) -> Result<Node, String> {
        self.assign()
    }
    // assign = equality ("=" assign)?
    fn assign(&mut self) -> Result<Node, String> {
        let mut ret = match self.equality() {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };
        if self.consume("=".to_string()) {
            ret = match self.assign() {
                Ok(v) => Node {
                    kind: NodeKind::Assign,
                    lhs: Some(Box::new(ret)),
                    rhs: Some(Box::new(v)),
                    val: None,
                    offset: None,
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(ret)
    }
    // equality = relational ("==" relational | "!=" relational)*
    fn equality(&mut self) -> Result<Node, String> {
        let mut ret = match self.relational() {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };
        loop {
            if self.consume("==".to_string()) {
                ret = match self.relational() {
                    Ok(v) => Node {
                        kind: NodeKind::Eq,
                        lhs: Some(Box::new(ret)),
                        rhs: Some(Box::new(v)),
                        val: None,
                        offset: None,
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            } else if self.consume("!=".to_string()) {
                ret = match self.relational() {
                    Ok(v) => Node {
                        kind: NodeKind::Ne,
                        lhs: Some(Box::new(ret)),
                        rhs: Some(Box::new(v)),
                        val: None,
                        offset: None,
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            } else {
                return Ok(ret);
            }
        }
    }
    // relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    fn relational(&mut self) -> Result<Node, String> {
        let mut ret = match self.add() {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };
        loop {
            if self.consume("<".to_string()) {
                ret = match self.add() {
                    Ok(v) => Node {
                        kind: NodeKind::Lt,
                        lhs: Some(Box::new(ret)),
                        rhs: Some(Box::new(v)),
                        val: None,
                        offset: None,
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            } else if self.consume("<=".to_string()) {
                ret = match self.add() {
                    Ok(v) => Node {
                        kind: NodeKind::Le,
                        lhs: Some(Box::new(ret)),
                        rhs: Some(Box::new(v)),
                        val: None,
                        offset: None,
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            } else if self.consume(">".to_string()) {
                ret = match self.add() {
                    Ok(v) => Node {
                        kind: NodeKind::Lt,
                        lhs: Some(Box::new(v)),
                        rhs: Some(Box::new(ret)),
                        val: None,
                        offset: None,
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            } else if self.consume(">=".to_string()) {
                ret = match self.add() {
                    Ok(v) => Node {
                        kind: NodeKind::Le,
                        lhs: Some(Box::new(v)),
                        rhs: Some(Box::new(ret)),
                        val: None,
                        offset: None,
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            } else {
                return Ok(ret);
            }
        }
    }
    // add = mul ("+" mul | "-" mul)*
    fn add(&mut self) -> Result<Node, String> {
        let mut ret = match self.mul() {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };
        loop {
            if self.consume("+".to_string()) {
                ret = match self.mul() {
                    Ok(v) => Node {
                        kind: NodeKind::Add,
                        lhs: Some(Box::new(ret)),
                        rhs: Some(Box::new(v)),
                        val: None,
                        offset: None,
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            } else if self.consume("-".to_string()) {
                ret = match self.mul() {
                    Ok(v) => Node {
                        kind: NodeKind::Sub,
                        lhs: Some(Box::new(ret)),
                        rhs: Some(Box::new(v)),
                        val: None,
                        offset: None,
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            } else {
                return Ok(ret);
            }
        }
    }
    // mul = unary ("*" unary | "/" unary)*
    fn mul(&mut self) -> Result<Node, String> {
        let mut ret = match self.unary() {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };
        loop {
            if self.consume("*".to_string()) {
                ret = match self.unary() {
                    Ok(v) => Node {
                        kind: NodeKind::Mul,
                        lhs: Some(Box::new(ret)),
                        rhs: Some(Box::new(v)),
                        val: None,
                        offset: None,
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            } else if self.consume("/".to_string()) {
                ret = match self.unary() {
                    Ok(v) => Node {
                        kind: NodeKind::Div,
                        lhs: Some(Box::new(ret)),
                        rhs: Some(Box::new(v)),
                        val: None,
                        offset: None,
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            } else {
                return Ok(ret);
            }
        }
    }
    // unary = ("+" | "-")? unary | primary
    fn unary(&mut self) -> Result<Node, String> {
        if self.consume("+".to_string()) {
            self.unary()
        } else if self.consume("-".to_string()) {
            match self.unary() {
                Ok(v) => Ok(Node {
                    kind: NodeKind::Sub,
                    lhs: Some(Box::new(self.new_num(0))),
                    rhs: Some(Box::new(v)),
                    val: None,
                    offset: None,
                }),
                Err(e) => {
                    return Err(e);
                }
            }
        } else {
            self.primary()
        }
    }
    // primary = "(" expr ")" | num
    fn primary(&mut self) -> Result<Node, String> {
        if self.consume("(".to_string()) {
            let ret = self.expr();
            if let Err(e) = self.expect(")".to_string()) {
                return Err(e)
            }
            ret
        } else if let Some(s) = self.consume_ident() {
            Ok(Node {
                kind: NodeKind::Lvar,
                lhs: None,
                rhs: None,
                val: None,
                offset: Some((s.chars().collect::<Vec<_>>()[0].to_digit(16).unwrap() - 'a'.to_digit(16).unwrap() + 1) * 8),
            })
        } else {
            match self.expect_number() {
                Ok(v) => Ok(self.new_num(v)),
                Err(e) => Err(e)
            }
        }
    }
    fn new_num(&mut self, num: i32) -> Node {
        Node {
            kind: NodeKind::Num,
            lhs: None,
            rhs: None,
            val: Some(num),
            offset: None,
        }
    }
}

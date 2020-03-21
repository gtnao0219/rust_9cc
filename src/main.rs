use std::env;
use std::process;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
enum TokenKind {
    Reserved,
    Num,
    EOF,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    val: Option<i32>,
    str: String
}

fn tokenize(iter: &mut Peekable<Chars>) -> Vec<Token> {
    let mut tokens = Vec::new();
    loop {
        match iter.peek() {
            Some(c) if c.is_whitespace() => {
                iter.next();
            }
            Some(c) if c.is_digit(10) => {
                let mut ret = 0;
                loop {
                    match iter.peek() {
                        Some(n) if n.is_digit(10) => {
                            ret = ret * 10 + n.to_digit(10).unwrap() as i32;
                            iter.next();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                tokens.push(Token { kind: TokenKind::Num, val: Some(ret), str: ret.to_string() });
            }
            Some('=') => {
                iter.next();
                if let Some('=') = iter.peek() {
                    tokens.push(Token { kind: TokenKind::Reserved, val: None, str: "==".to_string() });
                    iter.next();
                } else {
                    eprintln!("invalid token");
                    process::exit(1);
                }
            }
            Some('!') => {
                iter.next();
                if let Some('=') = iter.peek() {
                    tokens.push(Token { kind: TokenKind::Reserved, val: None, str: "!=".to_string() });
                    iter.next();
                } else {
                    eprintln!("invalid token");
                    process::exit(1);
                }
            }
            Some('<') => {
                iter.next();
                if let Some('=') = iter.peek() {
                    tokens.push(Token { kind: TokenKind::Reserved, val: None, str: "<=".to_string() });
                    iter.next();
                } else {
                    tokens.push(Token { kind: TokenKind::Reserved, val: None, str: "<".to_string() });
                }
            }
            Some('>') => {
                iter.next();
                if let Some('=') = iter.peek() {
                    tokens.push(Token { kind: TokenKind::Reserved, val: None, str: ">=".to_string() });
                    iter.next();
                } else {
                    tokens.push(Token { kind: TokenKind::Reserved, val: None, str: ">".to_string() });
                }
            }
            Some(c) if vec!['+', '-', '*', '/', '(', ')'].contains(c) => {
                tokens.push(Token { kind: TokenKind::Reserved, val: None, str: c.to_string() });
                iter.next();
            }
            None => {
                tokens.push(Token { kind: TokenKind::EOF, val: None, str: "".to_string() });
                break;
            }
            _ => {
                eprintln!("invalid token");
                process::exit(1);
            }
        }
    }
    tokens
}

#[derive(Debug, PartialEq)]
enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num,
    Eq,
    Ne,
    Lt,
    Le
}

#[derive(Debug)]
struct Node {
    kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
    val: Option<i32>,
}

struct Parser {
    tokens: Vec<Token>,
    position: usize
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, position: 0 }
    }
    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }
    pub fn consume(&mut self, op: String) -> bool {
        if self.current_token().kind != TokenKind::Reserved || self.current_token().str != op {
            false
        } else {
            self.position += 1;
            true
        }
    }
    pub fn expect(&mut self, op: String) {
        if self.current_token().kind != TokenKind::Reserved || self.current_token().str != op {
            eprintln!("expected {}", op);
            process::exit(1);
        } else {
            self.position += 1;
        }
    }
    pub fn expect_number(&mut self) -> i32 {
        if self.current_token().kind != TokenKind::Num {
            eprintln!("expected a number");
            process::exit(1);
        } else {
            let val = self.current_token().val;
            self.position += 1;
            val.unwrap()
        }
    }
    // pub fn at_eof(&self) -> bool {
    //     self.current_token().kind == TokenKind::EOF
    // }
    // expr = equality
    pub fn expr(&mut self) -> Node {
        self.equality()
    }
    // equality = relational ("==" relational | "!=" relational)*
    pub fn equality(&mut self) -> Node {
        let mut ret = self.relational();
        loop {
            if self.consume("==".to_string()) {
                ret = Node {
                    kind: NodeKind::Eq,
                    lhs: Some(Box::new(ret)),
                    rhs: Some(Box::new(self.relational())),
                    val: None
                }
            } else if self.consume("!=".to_string()) {
                ret = Node {
                    kind: NodeKind::Ne,
                    lhs: Some(Box::new(ret)),
                    rhs: Some(Box::new(self.relational())),
                    val: None
                }
            } else {
                return ret
            }
        }
    }
    // relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    pub fn relational(&mut self) -> Node {
        let mut ret = self.add();
        loop {
            if self.consume("<".to_string()) {
                ret = Node {
                    kind: NodeKind::Lt,
                    lhs: Some(Box::new(ret)),
                    rhs: Some(Box::new(self.add())),
                    val: None
                }
            } else if self.consume("<=".to_string()) {
                ret = Node {
                    kind: NodeKind::Le,
                    lhs: Some(Box::new(ret)),
                    rhs: Some(Box::new(self.add())),
                    val: None
                }
            } else if self.consume(">".to_string()) {
                ret = Node {
                    kind: NodeKind::Lt,
                    lhs: Some(Box::new(self.add())),
                    rhs: Some(Box::new(ret)),
                    val: None
                }
            } else if self.consume(">=".to_string()) {
                ret = Node {
                    kind: NodeKind::Le,
                    lhs: Some(Box::new(self.add())),
                    rhs: Some(Box::new(ret)),
                    val: None
                }
            } else {
                return ret
            }
        }
    }
    // add = mul ("+" mul | "-" mul)*
    pub fn add(&mut self) -> Node {
        let mut ret = self.mul();
        loop {
            if self.consume("+".to_string()) {
                ret = Node {
                    kind: NodeKind::Add,
                    lhs: Some(Box::new(ret)),
                    rhs: Some(Box::new(self.mul())),
                    val: None
                }
            } else if self.consume("-".to_string()) {
                ret = Node {
                    kind: NodeKind::Sub,
                    lhs: Some(Box::new(ret)),
                    rhs: Some(Box::new(self.mul())),
                    val: None
                }
            } else {
                return ret
            }
        }
    }
    // mul = unary ("*" unary | "/" unary)*
    pub fn mul(&mut self) -> Node {
        let mut ret = self.unary();
        loop {
            if self.consume("*".to_string()) {
                ret = Node {
                    kind: NodeKind::Mul,
                    lhs: Some(Box::new(ret)),
                    rhs: Some(Box::new(self.unary())),
                    val: None
                }
            } else if self.consume("/".to_string()) {
                ret = Node {
                    kind: NodeKind::Div,
                    lhs: Some(Box::new(ret)),
                    rhs: Some(Box::new(self.unary())),
                    val: None
                }
            } else {
                return ret
            }
        }
    }
    // unary = ("+" | "-")? unary | primary
    pub fn unary(&mut self) -> Node {
        if self.consume("+".to_string()) {
            self.unary()
        } else if self.consume("-".to_string()) {
            Node {
                kind: NodeKind::Sub,
                lhs: Some(Box::new(self.new_num(0))),
                rhs: Some(Box::new(self.unary())),
                val: None
            }
        } else {
            self.primary()
        }
    }
    // primary = "(" expr ")" | num
    pub fn primary(&mut self) -> Node {
        if self.consume("(".to_string()) {
            let ret = self.expr();
            self.expect(")".to_string());
            ret
        } else {
            let val = self.expect_number();
            self.new_num(val)
        }
    }
    fn new_num(&mut self, num: i32) -> Node {
        Node {
            kind: NodeKind::Num,
            lhs: None,
            rhs: None,
            val: Some(num)
        }
    }
}

fn gen(node: Node) {
    if node.kind == NodeKind::Num {
        println!("  push {}", node.val.unwrap());
        return;
    }
    gen(*node.lhs.unwrap());
    gen(*node.rhs.unwrap());
    println!("  pop rdi");
    println!("  pop rax");
    match node.kind {
        NodeKind::Add => println!("  add rax, rdi"),
        NodeKind::Sub => println!("  sub rax, rdi"),
        NodeKind::Mul => println!("  imul rax, rdi"),
        NodeKind::Div => {
            println!("  cqo");
            println!("  idiv rdi");
        }
        NodeKind::Eq => {
            println!("  cmp rax, rdi");
            println!("  sete al");
            println!("  movzb rax, al");
        }
        NodeKind::Ne => {
            println!("  cmp rax, rdi");
            println!("  setne al");
            println!("  movzb rax, al");
        }
        NodeKind::Lt => {
            println!("  cmp rax, rdi");
            println!("  setl al");
            println!("  movzb rax, al");
        }
        NodeKind::Le => {
            println!("  cmp rax, rdi");
            println!("  setle al");
            println!("  movzb rax, al");
        }
        _ => {
            eprintln!("unexpected token");
            process::exit(1);
        }
    }
    println!("  push rax");
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("{}: invalid number of arguments", args[0]);
        process::exit(1);
    }
    let mut user_input = args[1].chars().peekable();
    let tokens = tokenize(&mut user_input);
    // println!("{:?}", tokens);
    let mut parser = Parser::new(tokens);
    let node = parser.expr();
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    gen(node);
    println!("  pop rax");
    println!("  ret");
}

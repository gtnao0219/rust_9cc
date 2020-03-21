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
    str: String,
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
                // iter.next();
            }
            Some(c) if *c == '+' || *c == '-' => {
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
    pub fn at_eof(&self) -> bool {
        self.current_token().kind == TokenKind::EOF
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("{}: invalid number of arguments", args[0]);
        process::exit(1);
    }
    let mut chars = args[1].chars().peekable();
    let tokens = tokenize(&mut chars);
    // println!("{:?}", tokens);
    let mut parser = Parser::new(tokens);
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", parser.expect_number());
    while !parser.at_eof() {
        if parser.consume("+".to_string()) {
            println!("  add rax, {}", parser.expect_number());
            continue;
        }
        parser.expect("-".to_string());
        println!("  sub rax, {}", parser.expect_number());
    }
}

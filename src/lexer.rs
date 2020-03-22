use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Reserved,
    Num,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub val: Option<i32>,
    pub str: String,
}

pub fn tokenize(iter: &mut Peekable<Chars>) -> Result<Vec<Token>, &'static str> {
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
                tokens.push(Token {
                    kind: TokenKind::Num,
                    val: Some(ret),
                    str: ret.to_string(),
                });
            }
            Some('=') => {
                iter.next();
                if let Some('=') = iter.peek() {
                    tokens.push(Token {
                        kind: TokenKind::Reserved,
                        val: None,
                        str: "==".to_string(),
                    });
                    iter.next();
                } else {
                    return Err("invalid token");
                }
            }
            Some('!') => {
                iter.next();
                if let Some('=') = iter.peek() {
                    tokens.push(Token {
                        kind: TokenKind::Reserved,
                        val: None,
                        str: "!=".to_string(),
                    });
                    iter.next();
                } else {
                    return Err("invalid token");
                }
            }
            Some('<') => {
                iter.next();
                if let Some('=') = iter.peek() {
                    tokens.push(Token {
                        kind: TokenKind::Reserved,
                        val: None,
                        str: "<=".to_string(),
                    });
                    iter.next();
                } else {
                    tokens.push(Token {
                        kind: TokenKind::Reserved,
                        val: None,
                        str: "<".to_string(),
                    });
                }
            }
            Some('>') => {
                iter.next();
                if let Some('=') = iter.peek() {
                    tokens.push(Token {
                        kind: TokenKind::Reserved,
                        val: None,
                        str: ">=".to_string(),
                    });
                    iter.next();
                } else {
                    tokens.push(Token {
                        kind: TokenKind::Reserved,
                        val: None,
                        str: ">".to_string(),
                    });
                }
            }
            Some(c) if vec!['+', '-', '*', '/', '(', ')'].contains(c) => {
                tokens.push(Token {
                    kind: TokenKind::Reserved,
                    val: None,
                    str: c.to_string(),
                });
                iter.next();
            }
            None => {
                tokens.push(Token {
                    kind: TokenKind::EOF,
                    val: None,
                    str: "".to_string(),
                });
                break;
            }
            _ => {
                return Err("invalid token");
            }
        }
    }
    Ok(tokens)
}

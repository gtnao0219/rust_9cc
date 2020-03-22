extern crate rust_9cc;
use rust_9cc::lexer;
use rust_9cc::parser::{Parser};
use rust_9cc::generator;

use std::env;
use std::process;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("{}: invalid number of arguments", args[0]);
        process::exit(1);
    }
    let mut user_input = args[1].chars().peekable();
    let tokens = match lexer::tokenize(&mut user_input) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("failed tokenize: {}", e);
            process::exit(1);
        }
    };
    let mut parser = Parser::new(tokens);
    let nodes = match parser.parse() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("failed parse: {}", e);
            process::exit(1);
        }
    };
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    for node in nodes {
        generator::generate(node);
        println!("  pop rax");
    }

    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}

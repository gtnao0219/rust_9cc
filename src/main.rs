use std::env;
use std::process;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("{}: invalid number of arguments", args[0]);
        process::exit(1);
    }
    let chars = args[1].chars();
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    
    let mut n = 0;
    let mut op = '.';
    for c in chars {
        if c.is_digit(10) {
            n = n * 10 + c.to_digit(10).unwrap();
        } else {
            if op == '+' {
                println!("  add rax, {}", n);
            } else if op == '-' {
                println!("  sub rax, {}", n);
            } else {
                println!("  mov rax, {}", n);
            }
            n = 0;
            op = c;
        }
        // eprintln!("unexpected character: '{}'", c);
        // process::exit(1);
    }
    if op == '+' {
        println!("  add rax, {}", n);
    } else if op == '-' {
        println!("  sub rax, {}", n);
    } else {
        println!("  mov rax, {}", n);
    }

    println!("  ret");
}
use std::io;
use std::str;

fn compile_number(first_char: char, chars: str::Chars) {
    let mut n = first_char.to_digit(10).unwrap();
    for c in chars {
        if c.is_whitespace() {
            break;
        }
        if !c.is_digit(10) {
            panic!("Invalid character in number: '{}'", c);
        }
        n = n * 10 + c.to_digit(10).unwrap();
    }
    println!("\t.text");
    println!("\t.global mymain");
    println!("mymain:");
    println!("\tmov ${}, %eax", n);
    println!("\tret");
}

fn compile() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.pop().unwrap();

    let mut chars = input.chars();
    let first_char = chars.next().unwrap();
    if first_char.is_digit(10) {
        println!("digit");
        compile_number(first_char, chars)
    }
    if first_char == '"' {
        println!("string");
    }
}

fn main() {
    compile();
}
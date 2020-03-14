use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.pop().unwrap();
    let val: i32 = input.parse().unwrap();
    
    println!("\t.text");
    println!("\t.global mymain");
    println!("mymain:");
    println!("\tmov ${}, %eax", val);
    println!("\tret");
}

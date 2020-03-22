use crate::parser::{Node, NodeKind};

use std::process;

fn generate_lval(node: Node) {
    if node.kind != NodeKind::Lvar {
        eprintln!("failed generate");
        process::exit(1);
    }
    println!("  mov rax, rbp");
    println!("  sub rax, {}", node.offset.unwrap());
    println!("  push rax");
}

pub fn generate(node: Node) {
    if node.kind == NodeKind::Num {
        println!("  push {}", node.val.unwrap());
        return;
    }
    if node.kind == NodeKind::Lvar {
        generate_lval(node);
        println!("  pop rax");
        println!("  mov rax, [rax]");
        println!("  push rax");
        return;
    }
    if node.kind == NodeKind::Assign {
        generate_lval(*node.lhs.unwrap());
        generate(*node.rhs.unwrap());
        println!("  pop rdi");
        println!("  pop rax");
        println!("  mov [rax], rdi");
        println!("  push rdi");
        return;
    }
    generate(*node.lhs.unwrap());
    generate(*node.rhs.unwrap());
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
            eprintln!("failed generate");
            process::exit(1);
        }
    }
    println!("  push rax");
}

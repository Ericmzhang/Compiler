#![allow(unused)]
#[allow(unused_variables)]

use std::any::Any;
use std::fs::{File,OpenOptions};
use std::io::{Write};
use std::env;
use std::process::Command;
mod lex;
mod tokens;
mod parser;

use crate::lex::lex;
use crate::parser::parse_func;
use crate::tokens::{Token, Program, Func, Statement, Exp, Term, Factor};
use crate::tokens::{ExpType, TermType, FactorType};
fn generate_ass (root: &dyn Any, output_file: &String)-> std::io::Result<()> { 
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(output_file)?;
    if let Some(node) = root.downcast_ref::<Program>() {
        let _ = generate_ass(&node.func, output_file);
        Ok(())
    } 
    else if let Some(node) = root.downcast_ref::<Func>() {
        println!("Func");
        file.write_all(b".global ")?;
        file.write_all(node.identifier.as_bytes())?;
        file.write_all(b"\n");
        file.write_all(node.identifier.as_bytes())?;
        file.write_all(b":\n");
        let _ = generate_ass(&node.statement, output_file);
        Ok(())
    } 
    else if let Some(node) = root.downcast_ref::<Statement>() {
        println!("Statement");
        let _ = generate_ass(&node.exp, output_file);
        let err = file.write_all(b"   ret\n");
        Ok(())
    } 
    else if let Some(node) = root.downcast_ref::<Exp>() {
        println!("Expression");
        generate_exp_ass(&node.value, output_file)
    } 
    else if let Some(node) = root.downcast_ref::<ExpType>() {
        println!("Exp Type");
        generate_exp_ass(node, output_file)
    } 
    else if let Some(node) = root.downcast_ref::<Term>() {
        println!("Term");
        generate_term_ass(&node.value, output_file)
    } 
    else if let Some(node) = root.downcast_ref::<TermType>() {
        println!("Term Type");
        generate_term_ass(node, output_file)
    } 
    else if let Some(node) = root.downcast_ref::<Factor>() {
        println!("Factor"); 
        match &node.value {
            FactorType::Exp(exp) => {
                // let yo = **exp;
                generate_ass(&**exp, output_file);
            }
            FactorType::Constant(c) => {
                file.write_all(b"   movl    $")?;
                file.write_all(c.constant.to_string().as_bytes())?;
                file.write_all(b", %eax\n")?;
            }
            FactorType::Unop(unop) => {
                let _ = generate_ass(&(*unop.exp), output_file);
                match unop.operator {
                    Token::Negation => {
                        file.write_all(b"   neg     %eax")?;
                        file.write_all(b"\n");
                    },
                    Token::BitwiseComplement => {
                        file.write_all(b"   not     %eax")?;
                        file.write_all(b"\n");
                    },
                    Token::LogicalNegation => {
                        file.write_all(b"   cmpl   $0, %eax \n")?;
                        file.write_all(b"   neg     %eax \n")?;
                        file.write_all(b"   sete   %al\n")?;
                        file.write_all(b"\n");
                    },
                    _ => panic!()

                }
                // You might want to recursively process `unop.exp`
            }
            _ => {}
        }
        Ok(())
    } 
    else{
        Ok(())
    }
    
}

fn generate_term_ass(node: &TermType, output_file: &String) -> std::io::Result<()>{
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(output_file)?;
    match &node {
        TermType::BinOp(bin_op) => {
            let _ = generate_ass(&*bin_op.left, output_file);
            file.write_all(b"   push    %eax\n")?;
            let _ = generate_ass(&*bin_op.right, output_file);
            
            match &bin_op.operator {
                Token::Multiplication => {
                    file.write_all(b"   pop     %ecx\n")?;
                    file.write_all(b"   imul    %ecx,   %eax\n")?;
                } 
                Token::Division => {
                    file.write_all(b"   movl    %eax,   %ecx\n")?;
                    file.write_all(b"   pop     %eax\n")?;
                    file.write_all(b"   cdq\n")?;
                    file.write_all(b"   idivl    %ecx\n")?;
                }
                _ => panic!("Unsupported binary operator"),
            }
        }
        TermType::Factor(factor) => {
            let _ = generate_ass(&**factor, output_file);
        }
        _ => {}
    }
    Ok(())
}

fn generate_exp_ass(node: &ExpType, output_file: &String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(output_file)?;
    match &node {
        ExpType::BinOp(bin_op) => {
            generate_ass(&*bin_op.left, output_file)?;
            file.write_all(b"   push    %eax\n")?;
            generate_ass(&*bin_op.right, output_file)?;
            match &bin_op.operator {
                Token::Addition => {
                    file.write_all(b"   pop     %ecx\n")?;
                    file.write_all(b"   addl    %ecx,   %eax\n")?;
                }
                Token::Negation => {
                    file.write_all(b"   movl    %eax,   %ecx\n")?;
                    file.write_all(b"   pop     %eax\n")?;
                    file.write_all(b"   subl    %ecx,   %eax\n")?;
                }
                _ => panic!("Unsupported binary operator"),
            }
        }
        ExpType::Term(term) => {
            generate_ass(&**term, output_file)?;
        }
        _ => {}
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file =  &args[1];

    println!("PRINTING TOKENS\n");
    let tokens: Vec<Token>= lex(input_file);
    
    // PARSING
    println!("STARTING PARSING\n");
    let func = parse_func(&tokens);
    let prog = Program::new(func);
    // println!("{}", prog.func.statement.exp.value.value);
    let output_file = input_file.replace(".c", ".s");
    let program_name = input_file.replace(".c", "");
    //println!("{}",output_file);
    File::create(output_file.clone());
    println!("\nGENERATING ASSEMBLY\n");
    generate_ass(&prog, &output_file);
    let _ = Command::new("gcc")
        .arg("-m32")
        .arg(output_file)
        .arg("-o")
        .arg(program_name)
        .output(); // Execute the command and capture the output
}


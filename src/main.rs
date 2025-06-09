#![allow(unused)]
use core::panic;
#[allow(unused_variables)]

use std::any::Any;
use std::fs::{File,OpenOptions, remove_file};
use std::io::{Write};
use std::env;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
mod lex;
mod tokens;
mod parser;

use crate::lex::lex;
use crate::parser::parse_func;
use crate::tokens::{Token, Program, Func, Statement, Exp, Term, Factor};
use crate::tokens::{ExpType, TermType, FactorType, StatementType};

fn panic_and_delete(output_file: &String){
    let _ = remove_file(output_file);
    panic!();
}

fn generate_ass (root: &dyn Any, output_file: &String, stack_ind: i32, var_map: &HashMap<String,i32>)-> std::io::Result<()> { 
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(output_file)?;
    if let Some(node) = root.downcast_ref::<Program>() {
        let _ = generate_ass(&node.func, output_file, stack_ind, var_map);
        Ok(())
    } 
    else if let Some(node) = root.downcast_ref::<Func>() {
        println!("Func");
        file.write_all(b".global ")?;
        file.write_all(node.identifier.as_bytes())?;
        file.write_all(b"\n");
        file.write_all(node.identifier.as_bytes())?;
        file.write_all(b":\n");
        file.write_all(b"   push   %ebp\n");
        file.write_all(b"   movl   %esp, %ebp\n");
        let has_return = generate_statements_ass(&node.statements, output_file, stack_ind, var_map);
        file.write_all(b"   movl   %ebp, %esp\n");
        file.write_all(b"   pop    %ebp \n");
        if(!has_return){
            file.write_all(b"   movl   $0, %eax\n")?;
        }
        file.write_all(b"   ret\n");
        Ok(())
    } 
    else if let Some(node) = root.downcast_ref::<Exp>() {
        println!("Expression");
        generate_exp_ass(&node.value, output_file, stack_ind, var_map)
    } 
    else if let Some(node) = root.downcast_ref::<ExpType>() {
        println!("Exp Type");
        generate_exp_ass(node, output_file, stack_ind, var_map)
    } 
    else if let Some(node) = root.downcast_ref::<Term>() {
        println!("Term");
        generate_term_ass(&node.value, output_file, stack_ind, var_map)
    } 
    else if let Some(node) = root.downcast_ref::<TermType>() {
        println!("Term Type");
        generate_term_ass(node, output_file, stack_ind, var_map)
    } 
    else if let Some(node) = root.downcast_ref::<Factor>() {
        println!("Factor"); 
        match &node.value {
            FactorType::Exp(exp) => {
                // let yo = **exp;
                generate_ass(&**exp, output_file, stack_ind, var_map);
            }
            FactorType::Constant(c) => {
                file.write_all(b"   movl    $")?;
                file.write_all(c.constant.to_string().as_bytes())?;
                file.write_all(b", %eax\n")?;
            }
            FactorType::Id(v) => {
                // println!("factor type {}",v);
                match var_map.get(v) {
                    Some(var_offset) => {
                        let assignment = format!("   movl   {}(%ebp), %eax\n", var_offset);
                        file.write_all(assignment.as_bytes())?;
                    },
                    None => panic_and_delete(output_file)
                }
            }
            FactorType::Unop(unop) => {
                let _ = generate_ass(&(*unop.exp), output_file, stack_ind, var_map);
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
                    _ => panic_and_delete(output_file)

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

fn generate_term_ass(node: &TermType, output_file: &String, stack_ind: i32, var_map: &HashMap<String,i32>) -> std::io::Result<()>{
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(output_file)?;
    match &node {
        TermType::BinOp(bin_op) => {
            let _ = generate_ass(&*bin_op.left, output_file, stack_ind, var_map);
            file.write_all(b"   push    %eax\n")?;
            let _ = generate_ass(&*bin_op.right, output_file, stack_ind, var_map);
            
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
                _ => panic_and_delete(output_file)
            }
        }
        TermType::Factor(factor) => {
            let _ = generate_ass(&**factor, output_file, stack_ind, var_map);
        }
        _ => {}
    }
    Ok(())
}

fn generate_exp_ass(node: &ExpType, output_file: &String, stack_ind: i32, var_map: &HashMap<String,i32>) -> std::io::Result<()> { 
    static CLAUSE: AtomicUsize = AtomicUsize::new(2);
    static END: AtomicUsize = AtomicUsize::new(1);
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(output_file)?;
    // println!("{:?}", node);
    match &node {
        ExpType::BinOp(bin_op) => {
            match &bin_op.operator {
                Token::Or | Token::And =>{
                    generate_ass(&*bin_op.left, output_file, stack_ind, var_map)?;
                    match &bin_op.operator{
                        Token::Or =>{
                            let clause = CLAUSE.fetch_add(1, Ordering::SeqCst);
                            let end = END.fetch_add(1, Ordering::SeqCst);
                            file.write_all(b"   cmpl    $0,   %eax\n")?;
                            let je2clause = format!("   je      _clause{}\n", clause);
                            file.write_all(je2clause.as_bytes())?;
                            file.write_all(b"   movl    $1,   %eax\n")?;
                            let jmp2end = format!("   jmp      _end{}\n", end);
                            file.write_all(jmp2end.as_bytes())?;    
                            let clause_label = format!("_clause{}:\n", clause);
                            file.write_all(clause_label.as_bytes())?;
                            generate_ass(&*bin_op.right, output_file, stack_ind, var_map)?;
                            file.write_all(b"   cmpl $0, %eax\n")?;
                            file.write_all(b"   movl $0, %eax\n")?;
                            file.write_all(b"   setne %al \n")?;
                            let end_label = format!("_end{}:\n", end);
                            file.write_all(end_label.as_bytes())?;
                        }   
                        Token::And =>{
                            let clause = CLAUSE.fetch_add(1, Ordering::SeqCst);
                            let end = END.fetch_add(1, Ordering::SeqCst);
                            file.write_all(b"   cmpl    $0,   %eax\n")?;
                            let je2clause = format!("   jne      _clause{}\n", clause);
                            file.write_all(je2clause.as_bytes())?;
                            let jmp2end = format!("   jmp      _end{}\n", end);
                            file.write_all(jmp2end.as_bytes())?;    
                            let clause_label = format!("_clause{}:\n", clause);
                            file.write_all(clause_label.as_bytes())?;
                            generate_ass(&*bin_op.right, output_file, stack_ind, var_map)?;
                            file.write_all(b"   cmpl $0, %eax\n")?;
                            file.write_all(b"   movl $0, %eax\n")?;
                            file.write_all(b"   setne %al \n")?;
                            let end_label = format!("_end{}:\n", end);
                            file.write_all(end_label.as_bytes())?;
                        }
                        _ => panic_and_delete(output_file)               
                    }
                 
                }
                Token::Eq | Token::Neq | Token::Lt | Token::Leq | Token::Gt | Token::Geq | Token::Addition | Token::Negation =>{
                    generate_ass(&*bin_op.left, output_file, stack_ind, var_map)?;
                    file.write_all(b"   push    %eax\n")?;
                    generate_ass(&*bin_op.right, output_file, stack_ind, var_map)?;
                    match &bin_op.operator {
                        Token::Eq =>{
                            file.write_all(b"   pop    %ecx\n")?;
                            file.write_all(b"   cmpl   %eax, %ecx\n")?;
                            file.write_all(b"   movl   $0, %eax\n")?;
                            file.write_all(b"   sete   %al  \n")?;
                        }
                        Token::Neq =>{
                            file.write_all(b"   pop    %ecx\n")?;
                            file.write_all(b"   cmpl   %eax, %ecx\n")?;
                            file.write_all(b"   movl   $0, %eax\n")?;
                            file.write_all(b"   setne   %al  \n")?;
                        }
                        Token::Lt =>{
                            file.write_all(b"   pop    %ecx\n")?;
                            file.write_all(b"   cmpl   %eax, %ecx\n")?;
                            file.write_all(b"   movl   $0, %eax\n")?;
                            file.write_all(b"   setl   %al  \n")?;
                        }
                        Token::Leq =>{
                            file.write_all(b"   pop    %ecx\n")?;
                            file.write_all(b"   cmpl   %eax, %ecx\n")?;
                            file.write_all(b"   movl   $0, %eax\n")?;
                            file.write_all(b"   setle   %al  \n")?;
                        }
                        Token::Gt =>{
                            file.write_all(b"   pop    %ecx\n")?;
                            file.write_all(b"   cmpl   %eax, %ecx\n")?;
                            file.write_all(b"   movl   $0, %eax\n")?;
                            file.write_all(b"   setg   %al  \n")?;
                        }
                        Token::Geq =>{
                            file.write_all(b"   pop    %ecx\n")?;
                            file.write_all(b"   cmpl   %eax, %ecx\n")?;
                            file.write_all(b"   movl   $0, %eax\n")?;
                            file.write_all(b"   setge   %al  \n")?;
                        }
                        Token::Addition => {
                            file.write_all(b"   pop     %ecx\n")?;
                            file.write_all(b"   addl    %ecx,   %eax\n")?;
                        }
                        Token::Negation => {
                            file.write_all(b"   movl    %eax,   %ecx\n")?;
                            file.write_all(b"   pop     %eax\n")?;
                            file.write_all(b"   subl    %ecx,   %eax\n")?;
                        }
                        _ => panic_and_delete(output_file)
                    }
                }
                
                _ => panic_and_delete(output_file)
            }
        }
        ExpType::Assign(assign)=>{
            println!("Expression assign");
            match assign.exp.as_ref() {
                Some(exp) => {
                    generate_exp_ass(&exp.value, output_file, stack_ind, &var_map);
                        match var_map.get(&assign.id) {
                            Some(var_offset) => {
                                let assignment = format!("   movl   %eax, {}(%ebp)\n", var_offset);
                                file.write_all(assignment.as_bytes())?;
                            },
                            None => panic_and_delete(output_file)
                    }
                }
                None => {
                    println!("Error in exp assign");
                }
            }
        }
        ExpType::Term(term) => {
            generate_ass(&**term, output_file, stack_ind, var_map)?;
        }
        ExpType::LogAndExp(term)|ExpType::EqExp(term)|ExpType::RelationalExp(term)|ExpType::AdditiveExp(term) => {
            generate_ass(&**term, output_file, stack_ind, var_map)?;
        }
        // ExpType::EqExp(term) => {
        //     generate_ass(&**term, output_file, stack_ind, var_map)?;
        // }
        // ExpType::RelationalExp(term) => {
        //     generate_ass(&**term, output_file, stack_ind, var_map)?;
        // }
        // ExpType::AdditiveExp(term) => {
        //     generate_ass(&**term, output_file, stack_ind, var_map)?;
        // }
        _ => {}
    }
    Ok(())
}

fn generate_statements_ass(statements: &Vec<Statement>, output_file: &String, mut stack_ind: i32, var_map: &HashMap<String,i32>) -> bool { 
    println!("Statements");
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(output_file)
    .expect("Failed to open file");
    // println!("{:?}", node);
    let mut new_map = var_map.clone();
    let mut statement_found = false;
    for statement in statements.iter() {
        match &statement.value {
            StatementType::Return(exp) =>{
                println!("Statement return");
                generate_exp_ass(&exp.value, output_file, stack_ind, &new_map);
                statement_found = true;
            }
            
            StatementType::Exp(exp) => {
                println!("Statement return");
                generate_exp_ass(&exp.value, output_file, stack_ind, &new_map);
            }
            StatementType::Declaration(assign) => {
                println!("Statement declaration");
                if new_map.contains_key(&assign.id){
                    panic_and_delete(output_file)//shouldn't declare var twice
                }
                match &assign.exp{
                    Some(exp) => {
                        generate_ass(&**exp, output_file, stack_ind, &new_map);}
                    None => {file.write_all(b"   movl   $0, %eax\n");}
                }
                file.write_all(b"   push %eax\n");
                new_map.insert(assign.id.clone(), stack_ind);
                stack_ind = stack_ind -4;
            }
            _ => {}
        }
    }
    statement_found
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
    let var_map = HashMap::new();
    generate_ass(&prog, &output_file, -4, &var_map);
    let _ = Command::new("gcc")
        .arg("-m32")
        .arg(output_file)
        .arg("-o")
        .arg(program_name)
        .output(); // Execute the command and capture the output
}


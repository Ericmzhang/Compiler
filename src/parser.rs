use crate::tokens::{Token, Func, Statement, Exp, Term, Factor, Constant};
use crate::tokens::{UnOp, BinOp, ExpBinOp, TermBinOp};
use crate::tokens::{ExpType, TermType, FactorType};
pub fn parse_func(tokens: &Vec<Token>) -> Func {
    let mut tok = tokens.iter().peekable();
    if *tok.next().unwrap() != Token::Int {   //check function starts with "int"
        panic!();
    }

    let func_tok = match *tok.next().unwrap() {  //get function name
        Token::Identifier(ref v) => v.clone(), 
        _ => panic!()
    };

    if *tok.next().unwrap() != Token::LeftParen {  
        panic!();
    }

    if *tok.next().unwrap() != Token::RightParen { 
        panic!();
    }

    if *tok.next().unwrap() != Token::LeftBrace { 
        panic!();
    }

    let statement = parse_statement(&mut tok); //get return statement

    if *tok.next().unwrap() != Token::RightBrace {
        panic!();
    }
    let func = Func::new(statement, func_tok);
    return func
}

fn parse_statement(tok: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> Statement{
    if *tok.next().unwrap() != Token::Return {
        panic!();
    }
    let expr = parse_expression(tok);
    
    if *tok.next().unwrap() != Token::SemiColon {
        panic!();
    }
    let statement = Statement::new(expr);
    statement
}

fn parse_expression(tok: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> Exp{
    println!("parse expresson");
    let mut exp1= ExpType::Term(Box::new(parse_term(tok)));
    while let Some(token) = tok.peek() {
        if **token == Token::Addition || **token == Token::Negation {
            let op = (*token).clone();
            tok.next(); // advance manually if needed
            let exp2 = parse_term(tok);
            let bin_op = ExpBinOp::new(op,
                Box::new(exp1),
                Box::new(exp2));
            exp1 = ExpType::BinOp(bin_op);
        }
        else{
            break;
        }
    }
    let exp = Exp::new(exp1);
    exp
}

fn parse_term(tok: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> Term{
    println!("parse term");
    let mut factor1= TermType::Factor(Box::new(parse_factor(tok)));
    while let Some(token) = tok.peek() {
        if **token == Token::Multiplication || **token == Token::Division {
            let op = (*token).clone();
            tok.next(); // advance manually if needed
            let factor2 = parse_factor(tok);
            let bin_op = TermBinOp::new(op,
                Box::new(factor1),
                Box::new(factor2));
            factor1 = TermType::BinOp(bin_op);
        }
        else{
            break;
        }
    }
    let term = Term::new(factor1);
    term
}

fn parse_factor(tok: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> Factor{
    let factor = match tok.next().unwrap() {
        Token::LeftParen => {  //exp
            let exp = parse_expression(tok);
            if tok.next().unwrap() != &Token::RightParen {
                panic!()
            }
            FactorType::Exp(Box::new(exp))
        }
        Token::Number(v)  => {  //int
            println!("parse factor: {}", v);
            FactorType::Constant(Constant::new(*v))
        },
        token if token.is_un_op() => {
            FactorType::Unop(UnOp::new(token.clone(), Box::new(parse_factor(tok))))
        }
        _ => panic!()
    };
    let factor = Factor::new(factor);
    factor
}
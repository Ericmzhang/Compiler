use crate::tokens::{Token, Func, Statement, Exp, Term, Factor, Constant};
use crate::tokens::{UnOp, BinOp, ExpBinOp, TermBinOp, Assign};
use crate::tokens::{ExpType, TermType, FactorType, StatementType};
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

    let mut statements: Vec<Statement> = Vec::new();
    while(**tok.peek().unwrap() != Token::RightBrace){
        statements.push(parse_statement(&mut tok)); //get return statement
    }
    let func = Func::new(statements, func_tok);
    return func
}

fn parse_statement(tok: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> Statement{
    let statement = if (**tok.peek().unwrap() == Token::Return) {
        println!("statement return");
        tok.next();
        let expr = parse_expression(tok);
        Statement::new(StatementType::Return(Box::new(expr)))
    }
    else if(**tok.peek().unwrap() == Token::Int){
        println!("statement declare");
        tok.next();
        let id = match *tok.next().unwrap() {  //get function name
            Token::Identifier(ref v) => v.clone(), 
            _ => panic!()
        };
        let inner_statement = if(**tok.peek().unwrap() == Token::Assignment){
            tok.next();
            let expr = parse_expression(tok);   
            Statement::new(StatementType::Declaration(Assign::new(id, Some(Box::new(expr)))))
        }
        else{
            Statement::new(StatementType::Declaration(Assign::new(id, None)))
        };
        inner_statement
    }
    else{
        println!("statement expression");
        let expr = parse_expression(tok);
        Statement::new(StatementType::Exp(Box::new(expr)))
    };           
    if *tok.next().unwrap() != Token::SemiColon {
        panic!();
    }
    statement
}

fn parse_expression_general <F>(tok: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>, subparser: F, operators: &[Token], wrap_rhs: fn(Box<ExpType>)->ExpType) -> ExpType where
F: Fn(&mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> ExpType, {  //general function for parsing different types of expression
    let mut exp1= subparser(tok);
    while let Some(token) = tok.peek() {
        if(operators.contains(token)){
            let op = (*token).clone();
            tok.next();
            let exp2 = wrap_rhs(Box::new(subparser(tok)));
            let bin_op = ExpBinOp::new(op,
                Box::new(exp1),
                Box::new(exp2));
            exp1 = ExpType::BinOp(bin_op);
        }
        else{
            break;
        }
    }
    exp1
}

fn parse_expression(tok: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> Exp{
    let exp = if let Token::Identifier(ref v) = **tok.peek().unwrap() {  //get function name
        let mut iter = tok.clone();
        iter.next();
        let inner_exp = if (**iter.peek().unwrap() == Token::Assignment) {
            tok.next();
            tok.next();
            println!("parse expression assign");
            Exp::new(ExpType::Assign(Assign::new(v.clone(),Some(Box::new(parse_expression(tok))))))
        } 
        else{
            Exp::new(parse_or(tok))
        };
        inner_exp
    } 
    else{
        Exp::new(parse_or(tok))
    };
    exp
}

fn parse_or(tok: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> ExpType {
    println!("parse expression or");
    parse_expression_general(tok, parse_logical_and, &[Token::Or], ExpType::LogAndExp)
}

fn parse_logical_and(tok: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> ExpType {
    println!("parse expression and");
    parse_expression_general(tok, parse_equality, &[Token::And], ExpType::EqExp)
}

fn parse_equality(tok: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> ExpType {
    println!("parse expression equality");
    parse_expression_general(tok, parse_relational, &[Token::Eq, Token::Neq], ExpType::RelationalExp)
}

fn parse_relational(tok: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> ExpType {
    println!("parse expression relational");
    parse_expression_general(tok,parse_additive,&[Token::Lt, Token::Leq, Token::Gt, Token::Geq],ExpType::AdditiveExp)
}

fn parse_additive(tok: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> ExpType {
    println!("parse expression additive");
    parse_expression_general(tok,parse_term,&[Token::Addition, Token::Negation],ExpType::AdditiveExp)
}

fn parse_term(tok: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> ExpType{
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
    let term = ExpType::Term(Box::new(Term::new(factor1)));
    term
}

fn parse_factor(tok: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> Factor{
    println!("parse factor");
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
        },
        Token::Identifier(ref v) =>{
            println!("parse factor: {}", v);
            FactorType::Id(v.clone())
        }
        _ => panic!()
    };
    let factor = Factor::new(factor);
    factor
}
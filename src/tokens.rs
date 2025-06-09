#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(i64),
    Identifier(String),
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    SemiColon,
    Int,
    Return,
    Negation, //-
    Addition,
    Multiplication,
    Division,
    BitwiseComplement, //~
    LogicalNegation, // !
    And,  //&&
    Or,
    Eq,   //==
    Neq,  
    Lt,
    Leq,  //<=
    Gt,
    Geq,
    Assignment, //=
    EOF,
}

#[derive(Debug)]
pub enum StatementType {
    Return(Box<Exp>),
    Exp(Box<Exp>),
    Declaration(Assign),
}

#[derive(Debug)]
pub enum ExpType {
    Assign(Assign),
    Term(Box<Term>),
    LogAndExp(Box<ExpType>),
    EqExp(Box<ExpType>),
    RelationalExp(Box<ExpType>),
    AdditiveExp(Box<ExpType>),
    BinOp(ExpBinOp)
}

#[derive(Debug)]
pub enum TermType {
    Factor(Box<Factor>),
    BinOp(TermBinOp)
}

#[derive(Debug)]
pub enum FactorType {
    Exp(Box<Exp>),
    Unop(UnOp),
    Constant(Constant),
    Id(String),
}

#[derive(Debug)]
pub struct UnOp {
    pub operator: Token,
    pub exp: Box<Factor>,

}

#[derive(Debug)]
pub struct BinOp<T> {
    pub operator: Token,
    pub left: Box<T>,
    pub right: Box<T>,
}

#[derive(Debug)]
pub struct ExpBinOp {
    pub operator: Token,
    pub left: Box<ExpType>,
    pub right: Box<ExpType>,
}

#[derive(Debug)]
pub struct TermBinOp {
    pub operator: Token,
    pub left: Box<TermType>,
    pub right: Box<Factor>,
}

#[derive(Debug)]
pub struct Assign{ //used for both assigning and declaring vars
    pub id: String,
    pub exp: Option<Box<Exp>>
}

#[derive(Debug)]
pub struct Program {
    pub func: Func
}

#[derive(Debug)]
pub struct Func {
    pub statements: Vec<Statement>, //function statement (ie return 0)
    pub identifier: String,   //function name
}

#[derive(Debug)]
pub struct Statement {
    pub value: StatementType
}

#[derive(Debug)]
pub struct Exp {
    pub value: ExpType
}

#[derive(Debug)]
pub struct Term {
    pub value: TermType
}

#[derive(Debug)]
pub struct Factor {
    pub value: FactorType
}

#[derive(Debug)]
pub struct Constant {
    pub constant: i64,
}

impl UnOp {
    pub fn new(operator: Token, exp: Box<Factor>) -> UnOp {
        UnOp{
            operator: operator,
            exp: exp
        }
    }
}

impl<T> BinOp<T> {
    pub fn new(operator: Token, left: Box<T>, right: Box<T>) -> BinOp<T> {
        BinOp{
            operator: operator,
            left: left,
            right: right
        }
    }
}

impl ExpBinOp {
    pub fn new(operator: Token, left: Box<ExpType>, right: Box<ExpType>) -> ExpBinOp {
        ExpBinOp{
            operator: operator,
            left: left,
            right: right
        }
    }
}

impl TermBinOp {
    pub fn new(operator: Token, left: Box<TermType>, right: Box<Factor>) -> TermBinOp {
        TermBinOp{
            operator: operator,
            left: left,
            right: right
        }
    }
}

impl Assign {
    pub fn new(id: String, exp: Option<Box<Exp>>) -> Assign {
        Assign{
            id: id,
            exp: exp,
        }
    }
}

impl Token {
    pub fn is_un_op(&self) -> bool {
        matches!(self, Token::Negation | Token::BitwiseComplement | Token::LogicalNegation)
    }
}

impl Program {
    // Define a method on the `Person` struct
    pub fn new(func: Func) -> Program {
        Program{
            func: func,
        }
    }
}

impl Func {
    // Define a method on the `Person` struct
    pub fn new(statements: Vec<Statement>, identifier: String) -> Func {
        Func{
            statements: statements,
            identifier: identifier.to_string()
        }
    }
}

impl Statement {
    // Define a method on the `Person` struct
    pub fn new(value: StatementType) -> Statement {
        Statement{
            value: value
        }
    }
}

impl Exp {
    pub fn new(value: ExpType) -> Exp {
        Exp{
            value: value
        }
    }
}

impl Term {
    pub fn new(value: TermType) -> Term {
        Term{
            value: value
        }
    }
}

impl Factor {
    pub fn new(value: FactorType) -> Factor {
        Factor{
            value: value
        }
    }
}


impl Constant {
    pub fn new(constant: i64) -> Constant {
        Constant{
            constant: constant
        }
    }
}
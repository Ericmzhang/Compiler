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
    EOF,
}

pub enum ExpType {
    Term(Box<Term>),
    BinOp(ExpBinOp)
}
pub enum TermType {
    Factor(Box<Factor>),
    BinOp(TermBinOp)
}

pub enum FactorType {
    Exp(Box<Exp>),
    Unop(UnOp),
    Constant(Constant)
}

pub struct UnOp {
    pub operator: Token,
    pub exp: Box<Factor>,
}

pub struct BinOp<T> {
    pub operator: Token,
    pub left: Box<T>,
    pub right: Box<T>,
}

pub struct ExpBinOp {
    pub operator: Token,
    pub left: Box<ExpType>,
    pub right: Box<Term>,
}

pub struct TermBinOp {
    pub operator: Token,
    pub left: Box<TermType>,
    pub right: Box<Factor>,
}

pub struct Program {
    pub func: Func
}

pub struct Func {
    pub statement: Statement, //function statement (ie return 0)
    pub identifier: String,   //function name
}

pub struct Statement {
    pub exp: Exp
}

pub struct Exp {
    pub value: ExpType
}

pub struct Term {
    pub value: TermType
}

pub struct Factor {
    pub value: FactorType
}

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
    pub fn new(operator: Token, left: Box<ExpType>, right: Box<Term>) -> ExpBinOp {
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
    pub fn new(statement: Statement, identifier: String) -> Func {
        Func{
            statement: statement,
            identifier: identifier.to_string()
        }
    }
}

impl Statement {
    // Define a method on the `Person` struct
    pub fn new(exp: Exp) -> Statement {
        Statement{
            exp: exp
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
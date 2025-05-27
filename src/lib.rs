pub mod scanner;
pub mod parser;


pub struct TokenUnit {
    pub token_type: TokenType,
    pub table_ptr: usize,
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    // 标识符
    Id,

    // 常量
    Const,

    // 特殊形式
    Define,
    If,
    List,
    Cons,
    Lambda,
    Display,
    Quote,
    QuoteMark,

    // 算术运算符
    PlusOp,
    MulOp,
    MinusOp,
    DivOp,

    // 逻辑运算符
    LessThan,
    GreaterThan,
    LessEq,
    GreaterEq,
    Eq,

    // 界定符（不含双引号）
    LParen,
    RParen,
}


#[derive(Debug)]
pub struct TableItem {
    pub index: (usize, usize),
    pub value: Option<ValueType>,
}


#[derive(Debug)]
pub enum ValueType {
    Int(isize),
    Float(f64),
    Str(String),
    Bool(bool),
}


pub enum ScanError {
    // 不会出现在Lisp中的字符
    InvalidCharacter((usize, usize)),
    // 不符合词法规则的串
    InvalidToken((usize, usize)),
}


pub enum ParseError {
    UnexpectedToken((usize, usize)),
    UnexpectedEndOfInput,
    UnexpectedRemains,
    UnknownScanError,
}

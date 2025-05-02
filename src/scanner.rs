mod utils;

use utils::{tokenize, chars2bytes};

pub struct TokenUnit {
    pub token_type: TokenType,
    pub table_ptr: usize,
}

pub enum ScanError {
    // 不会出现在Lisp中的字符
    InvalidCharacter((usize, usize)),
    // 不期望在该处出现的字符
    UnexpectedCharacter((usize, usize)),
}

#[derive(Debug)]
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

    // 其他
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

pub fn scan(input: &str) -> Result<(Vec<TokenUnit>, Vec<TableItem>), ScanError> {
    let mut token_table: Vec<TableItem> = Vec::new();
    let mut tokens: Vec<TokenUnit> = Vec::new();

    for (row, mut line) in input.lines().enumerate() {
        let mut column = whitespace_cnt(line);
        line = &line[column..];

        while !line.is_empty() {
            match tokenize(line, row, column) {
                Ok((mut token, table_item)) => {
                    // 更新column
                    column += token.table_ptr;

                    // 计算切片索引
                    let token_bytes = chars2bytes(line, token.table_ptr);

                    // 添加token序列
                    token.table_ptr = token_table.len();
                    tokens.push(token);

                    // 添加符号表条目
                    token_table.push(table_item);

                    // 切片并去除前导空白符
                    line = &line[token_bytes..];
                    let ws_cnt = whitespace_cnt(line);
                    column += ws_cnt;
                    line = &line[ws_cnt..];
                },

                Err(scan_error) => match scan_error {
                    ScanError::InvalidCharacter((row, column)) => {
                        panic!("tokenize() failed at row {} column {}: Invalid Character", row + 1, column + 1);
                    },

                    ScanError::UnexpectedCharacter((row, column)) => {
                        panic!("tokenize() failed at row {} column {}: Unexpected Character", row + 1, column + 1);
                    }
                },
            }
        }
    }

    Ok((tokens, token_table))
}

fn whitespace_cnt(line: &str) -> usize {
    let mut ws_cnt = 0;

    for ch in line.chars() {
        if !ch.is_whitespace() {
            return ws_cnt;
        }
        ws_cnt += 1;
    }

    ws_cnt
}

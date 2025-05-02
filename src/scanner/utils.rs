use super::{ScanError, TokenUnit, TokenType, TableItem, ValueType};


// 功能：
// 传入一个以非空白字符开头的字符串，识别出第一个词法单元，
// 返回词法单元和符号表条目。（词法单元字符数暂存于TokenUnit.table_ptr中）

// 识别顺序：
// (1) 左右括号（即除双引号外的界定符）
// (2) 常量（含字符串）
// (3) 特殊形式关键字
// (4) 用户自定义标识符
// (5) 算术运算符（过程）
// (6) 逻辑运算符（过程）
pub fn tokenize(line: &str, row: usize, column: usize) -> Result<(TokenUnit, TableItem), ScanError> {
    if let Some(result) = recog_mark(line, row, column) {
        return Ok(result);
    }

    if let Some(result) = recog_const(line, row, column) {
        return Ok(result);
    }

    if let Some(result) = recog_reserved(line, row, column) {
        return Ok(result);
    }

    if let Some(result) = recog_id(line, row, column) {
        return Ok(result);
    }

    if let Some(result) = recog_op(line, row, column) {
        return Ok(result);
    }

    if let Some(result) = recog_cmp(line, row, column) {
        return Ok(result);
    }

    Err(ScanError::InvalidCharacter((row, column)))
}


fn recog_mark(line: &str, row: usize, column: usize) -> Option<(TokenUnit, TableItem)> {
    let token_unit = if let Some(ch) = line.chars().next() {
        match ch {
            '(' => Some(TokenUnit { token_type: TokenType::LParen, table_ptr: 1 }),
            ')' => Some(TokenUnit { token_type: TokenType::RParen, table_ptr: 1 }),
            _ => None,
        }
    } else {
        None
    };

    match token_unit {
        Some(token_unit) => Some((token_unit, TableItem {
            index: (row, column),
            value: None,
        })),
        _ => None,
    }
}


fn recog_const(line: &str, row: usize, column: usize) -> Option<(TokenUnit, TableItem)> {
    // 字符串识别（双引号的特性使得字符串须单独写识别逻辑）
    if line.starts_with("\"") {
        let mut pre_escape = false;
        for (i, ch) in line.chars().enumerate() {
            if i != 0 && ch == '\"' && !pre_escape {
                return Some((TokenUnit {
                    token_type: TokenType::Const,
                    table_ptr: i + 1
                }, TableItem {
                    index: (row, column),
                    value: Some(ValueType::Str(String::from(&line[1..i])))
                }))
            }
            pre_escape = ch == '\\';
        }
        return None;
    }

    // 整型、浮点型、布尔型常量识别
    if let Some(first) = line.split_whitespace().next() {
        if let Some(first) = first.split(|c| c == '(' || c == ')' || c == '\"').next() {
            match parse_const(first) {
                Some((value_type, token_len)) => Some((TokenUnit {
                    token_type: TokenType::Const,
                    table_ptr: token_len,
                }, TableItem {
                    index: (row, column),
                    value: Some(value_type),
                })),
                _ => None,
            }
        } else { None }
    } else { None }
}


// 先于recog_id()调用
fn recog_reserved(line: &str, row: usize, column: usize) -> Option<(TokenUnit, TableItem)> {
    let token_unit = if let Some(first) = line.split_whitespace().next() {
        match first.split(|c| c == '(' || c == ')' || c == '\"').next() {
            Some("define") => Some(TokenUnit { token_type: TokenType::Define, table_ptr: "define".len() }),
            Some("if") => Some(TokenUnit { token_type: TokenType::If, table_ptr: "if".len() }),
            Some("list") => Some(TokenUnit { token_type: TokenType::List, table_ptr: "list".len() }),
            Some("cons") => Some(TokenUnit { token_type: TokenType::Cons, table_ptr: "cons".len() }),
            Some("lambda") => Some(TokenUnit { token_type: TokenType::Lambda, table_ptr: "lambda".len() }),
            Some("display") => Some(TokenUnit { token_type: TokenType::Lambda, table_ptr: "display".len() }),
            Some("quote") => Some(TokenUnit { token_type: TokenType::Quote, table_ptr: "quote".len() }),
            Some("\'") => Some(TokenUnit { token_type: TokenType::QuoteMark, table_ptr: "\'".len() }),
            Some(other) => {
                if let Some(start_ch) = other.chars().next() {
                    match start_ch {
                        '\'' => Some(TokenUnit { token_type: TokenType::QuoteMark, table_ptr: 1 }),
                        _ => None,
                    }
                } else { None }
            },

            _ => None,
        }
    } else { None };

    match token_unit {
        Some(token_unit) => Some((token_unit, TableItem { index: (row, column), value: None })),
        _ => None,
    }
}


fn recog_id(line: &str, row: usize, column: usize) -> Option<(TokenUnit, TableItem)> {
    if let Some(first) = line.split_whitespace().next() {
        if let Some(first) = first.split(|c| c == '(' || c == ')' || c == '\"').next() {
            if first == "" {
                None
            } else {
                for (i, ch) in first.chars().enumerate() {
                    if i == 0 && ch.is_digit(10) {
                        return None;
                    }
                    if !ch.is_alphabetic() && !ch.is_numeric() && ch != '_' {
                        return None;
                    }
                }
                Some((TokenUnit {
                    token_type: TokenType::Id,
                    table_ptr: first.len(),
                }, TableItem {
                    index: (row, column),
                    value: Some(ValueType::Str(String::from(first))),
                }))
            }
        } else { None }
    } else { None }
}


fn recog_op(line: &str, row: usize, column: usize) -> Option<(TokenUnit, TableItem)> {
    let token_unit = if let Some(first) = line.split_whitespace().next() {
        if let Some(first) = first.split(|c| c == '(' || c == ')' || c == '\"').next() {
            match first {
                "+" => Some(TokenUnit { token_type: TokenType::PlusOp, table_ptr: "+".len(), }),
                "-" => Some(TokenUnit { token_type: TokenType::MinusOp, table_ptr: "-".len(), }),
                "*" => Some(TokenUnit { token_type: TokenType::MulOp, table_ptr: "*".len(), }),
                "/" => Some(TokenUnit { token_type: TokenType::DivOp, table_ptr: "/".len(), }),

                _ => None,
            }
        } else { None }
    } else { None };

    match token_unit {
        Some(token_unit) => Some((token_unit, TableItem {
            index: (row, column),
            value: None,
        })),

        _ => None,
    }
}


fn recog_cmp(line: &str, row: usize, column: usize) -> Option<(TokenUnit, TableItem)> {
    let token_unit = if let Some(first) = line.split_whitespace().next() {
        if let Some(first) = first.split(|c| c == '(' || c == ')' || c == '\"').next() {
            match first {
                "<=" => Some(TokenUnit { token_type: TokenType::LessEq, table_ptr: "<=".len(), }),
                ">=" => Some(TokenUnit { token_type: TokenType::GreaterEq, table_ptr: ">=".len(), }),
                "<" => Some(TokenUnit { token_type: TokenType::LessThan, table_ptr: "<".len(), }),
                ">" => Some(TokenUnit { token_type: TokenType::GreaterThan, table_ptr: ">".len(), }),
                "=" => Some(TokenUnit { token_type: TokenType::Eq, table_ptr: "=".len(), }),

                _ => None,
            }
        } else { None }
    } else { None };

    match token_unit {
        Some(token_unit) => Some((token_unit, TableItem {
            index: (row, column),
            value: None,
        })),

        _ => None,
    }
}


pub fn chars2bytes(input: &str, charcnt: usize) -> usize {
    let mut bytecnt = 0;
    let mut input_iter = input.chars();

    for _ in 0..charcnt {
        if let Some(ch) = input_iter.next() {
            bytecnt += ch.len_utf8();
        }
    }

    bytecnt
}


// 将字符串转换为整型、浮点型或布尔型常量
fn parse_const(input: &str) -> Option<(ValueType, usize)> {
    let token_len= input.len();

    if let Ok(int_val) = input.parse::<isize>() {
        return Some((ValueType::Int(int_val), token_len));
    }

    if let Ok(float_val) = input.parse::<f64>() {
        return Some((ValueType::Float(float_val), token_len));
    }

    match input {
        "#t" => Some((ValueType::Bool(true), token_len)),
        "#f" => Some((ValueType::Bool(false), token_len)),
        _ => None,
    }
}

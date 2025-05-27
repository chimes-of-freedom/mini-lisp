use crate::{ParseError::{self, *}, TableItem, TokenType::{self, *}, TokenUnit};


// 开始符号的子程序：`start -> (list) | atom`
pub fn parse_start<'a>(tokens: &'a [TokenUnit], token_table: &Vec<TableItem>) -> Result<&'a [TokenUnit], ParseError> {
    if let Some(first) = tokens.get(0) {
        if is_atom(first.token_type) {
            expect_ts(tokens, token_table, first.token_type)
        } else if first.token_type == LParen {
            let tokens = expect_ts(tokens, token_table, LParen)?;
            let tokens = parse_list(tokens, token_table)?;
            Ok(expect_ts(tokens, token_table, RParen)?)
        } else {
            match token_table.get(first.table_ptr) {
                Some(table_item) => Err(UnexpectedToken(table_item.index)),
                None => Err(UnknownScanError)
            }
        }
    } else {
        Err(UnexpectedEndOfInput)
    }
}


// 非终结符list的子程序：`list -> start list | epsilon`
fn parse_list<'a>(tokens: &'a [TokenUnit], token_table: &Vec<TableItem>) -> Result<&'a [TokenUnit], ParseError> {
    match tokens.get(0) {
        Some(token_unit) => {
            if token_unit.token_type == LParen || is_atom(token_unit.token_type) {
                let tokens = parse_start(tokens, token_table)?;
                Ok(parse_list(tokens, token_table)?)
            } else if token_unit.token_type == RParen {
                Ok(tokens)
            } else {
                match token_table.get(token_unit.table_ptr) {
                    Some(table_item) => Err(UnexpectedToken(table_item.index)),
                    None => Err(UnknownScanError)
                }
            }
        },
        None => Err(UnexpectedEndOfInput)
    }
}


// 试图匹配1个指定的终结符
fn expect_ts<'a>(tokens: &'a [TokenUnit], token_table: &Vec<TableItem>, ts: TokenType) -> Result<&'a [TokenUnit], ParseError> {
    if let Some(token_unit) = tokens.get(0) {
        if token_unit.token_type == ts {
            Ok(&tokens[1..])
        } else {
            match token_table.get(token_unit.table_ptr) {
                Some(table_item) => Err(UnexpectedToken(table_item.index)),
                None => Err(UnknownScanError)
            }
        }
    } else {
        Err(UnexpectedEndOfInput)
    }
}


// 检验token是否为终结符atom
fn is_atom(token_type: TokenType) -> bool {
    token_type != LParen && token_type != RParen
}

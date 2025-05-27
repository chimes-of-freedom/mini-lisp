use crate::{ParseError::{self, *}, TokenType::{self, *}, TokenUnit};


// 开始符号的子程序：`start -> (list) | atom`
pub fn parse_start(tokens: &[TokenUnit]) -> Result<&[TokenUnit], ParseError> {
    if let Some(first) = tokens.get(0) {
        if is_atom(first.token_type) {
            expect_ts(tokens, first.token_type)
        } else if first.token_type == LParen {
            let tokens = expect_ts(tokens, LParen)?;
            let tokens = parse_list(tokens)?;
            Ok(expect_ts(tokens, RParen)?)
        } else {
            Err(UnexpectedToken)
        }
    } else {
        Err(UnexpectedEndOfInput)
    }
}


// 非终结符list的子程序：`list -> start list | epsilon`
fn parse_list(tokens: &[TokenUnit]) -> Result<&[TokenUnit], ParseError> {
    match tokens.get(0) {
        Some(token_unit) => {
            if token_unit.token_type == LParen || is_atom(token_unit.token_type) {
                let tokens = parse_start(tokens)?;
                Ok(parse_list(tokens)?)
            } else if token_unit.token_type == RParen {
                Ok(tokens)
            } else {
                Err(UnexpectedToken)
            }
        },
        None => Err(UnexpectedEndOfInput)
    }
}


// 试图匹配1个指定的终结符
fn expect_ts(tokens: &[TokenUnit], ts: TokenType) -> Result<&[TokenUnit], ParseError> {
    if let Some(token_unit) = tokens.get(0) {
        if token_unit.token_type == ts {
            Ok(&tokens[1..])
        } else {
            Err(UnexpectedToken)
        }
    } else {
        Err(UnexpectedEndOfInput)
    }
}


// 检验token是否为终结符atom
fn is_atom(token_type: TokenType) -> bool {
    token_type != LParen && token_type != RParen
}

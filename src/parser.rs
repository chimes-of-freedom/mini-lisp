use utils::parse_start;

use crate::{ParseError, TableItem, TokenUnit};
mod utils;


pub fn parse(tokens: &Vec<TokenUnit>, token_table: &Vec<TableItem>) -> Result<(), ParseError> {
    let mut current_tokens = &tokens[..];
    loop {
        current_tokens = parse_start(current_tokens, token_table)?;
        if current_tokens.is_empty() {
            break Ok(());
        }
    }
}

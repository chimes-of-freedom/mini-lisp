use utils::parse_start;

use crate::{ParseError, TokenUnit};
mod utils;


pub fn parse(tokens: &Vec<TokenUnit>) -> Result<(), ParseError> {
    let mut current_tokens = &tokens[..];
    loop {
        current_tokens = parse_start(current_tokens)?;
        if current_tokens.is_empty() {
            break Ok(());
        }
    }
}

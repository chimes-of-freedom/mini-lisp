use super::{ScanError, TokenType};

pub fn tokenize(input: &str, table: &mut Vec<TokenType>) -> Result<usize, ScanError> {
    
}

fn scan_id(input: &str, table: &Vec<TokenType>) -> Result<usize, ScanError> {
    let mut input_iter = input.chars();
}
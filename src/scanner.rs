mod utils;

pub enum ScanError {
    InvalidToken(String),
    UnexpectedCharacter(char),
    CharsToBytesCntFailed,
}

enum TokenType {
    Id(String),
    Number(NumberType),
    String(String),
    Boolean(bool),
    Punct(PunctType),
}

enum NumberType {
    Integer(isize),
    Float(f64),
}

enum PunctType {
    OpenParen,
    CloseParen,
    DoubleQuote,
}

pub fn scan(mut input: &str) -> Result<(), ScanError> {
    let mut token_table: Vec<TokenType> = Vec::new();
    while !input.is_empty() {
        let token_len = utils::tokenize(input, &mut token_table)?;
        input = charscnt_to_bytescnt(input, token_len)?;
    }
    Ok(())
}

pub fn charscnt_to_bytescnt(line: &str, chars_cnt: usize) -> Result<&str, ScanError> {
    let mut line_iter = line.chars();
    let mut bytes_cnt = 0;
    for _ in 0..chars_cnt {
        if let Some(ch) = line_iter.next() {
            bytes_cnt += ch.len_utf8();
        } else {
            return Err(ScanError::CharsToBytesCntFailed);
        }
    }
    Ok(&line[bytes_cnt..])
}
mod utils;
use utils::{tokenize, chars2bytes};
use crate::{TokenType, TokenUnit, ValueType, TableItem};


pub fn scan(input: &str) -> Result<(Vec<TokenUnit>, Vec<TableItem>), ScanError> {
    let mut token_table: Vec<TableItem> = Vec::new();
    let mut tokens: Vec<TokenUnit> = Vec::new();

    for (row, mut line) in input.lines().enumerate() {
        let mut column = whitespace_cnt(line);
        line = &line[column..];

        while !line.is_empty() {
            let (mut token, table_item) = tokenize(line, row, column)?;

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


pub enum ScanError {
    // 不会出现在Lisp中的字符
    InvalidCharacter((usize, usize)),
    // 不符合词法规则的串
    InvalidToken((usize, usize)),
}

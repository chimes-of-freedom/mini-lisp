mod utils;
use utils::{tokenize, chars2bytes};
use crate::{TokenUnit, TableItem, ScanError};


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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ScanError, TokenType, ValueType};

    fn token_types(input: &str) -> Result<Vec<TokenType>, ScanError> {
        let (tokens, _) = scan(input)?;
        Ok(tokens.iter().map(|t| t.token_type).collect())
    }

    // --- 界定符 ---

    #[test]
    fn test_scan_parens() {
        let types = token_types("()").unwrap();
        assert_eq!(types, vec![TokenType::LParen, TokenType::RParen]);
    }

    // --- 常量 ---

    #[test]
    fn test_scan_integer() {
        let (tokens, table) = scan("42").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Const);
        assert!(matches!(table[tokens[0].table_ptr].value, Some(ValueType::Int(42))));
    }

    #[test]
    fn test_scan_negative_integer() {
        let (tokens, table) = scan("-7").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Const);
        assert!(matches!(table[tokens[0].table_ptr].value, Some(ValueType::Int(-7))));
    }

    #[test]
    fn test_scan_float() {
        let (tokens, table) = scan("3.14").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Const);
        if let Some(ValueType::Float(f)) = &table[tokens[0].table_ptr].value {
            assert!((f - 3.14).abs() < 1e-9);
        } else {
            panic!("expected Float");
        }
    }

    #[test]
    fn test_scan_bool_true() {
        let (tokens, table) = scan("#t").unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Const);
        assert!(matches!(table[tokens[0].table_ptr].value, Some(ValueType::Bool(true))));
    }

    #[test]
    fn test_scan_bool_false() {
        let (tokens, table) = scan("#f").unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Const);
        assert!(matches!(table[tokens[0].table_ptr].value, Some(ValueType::Bool(false))));
    }

    #[test]
    fn test_scan_string() {
        let (tokens, table) = scan("\"hello\"").unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Const);
        if let Some(ValueType::Str(s)) = &table[tokens[0].table_ptr].value {
            assert_eq!(s, "hello");
        } else {
            panic!("expected Str");
        }
    }

    #[test]
    fn test_scan_string_with_escape() {
        let (tokens, table) = scan("\"a\\\"b\"").unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Const);
        if let Some(ValueType::Str(s)) = &table[tokens[0].table_ptr].value {
            assert_eq!(s, "a\\\"b");
        } else {
            panic!("expected Str");
        }
    }

    // --- 保留关键字 ---

    #[test]
    fn test_scan_reserved_keywords() {
        let cases = vec![
            ("define", TokenType::Define),
            ("if", TokenType::If),
            ("list", TokenType::List),
            ("cons", TokenType::Cons),
            ("lambda", TokenType::Lambda),
            ("display", TokenType::Display),
            ("quote", TokenType::Quote),
        ];
        for (src, expected) in cases {
            let types = token_types(src).unwrap();
            assert_eq!(types, vec![expected], "failed for keyword: {}", src);
        }
    }

    #[test]
    fn test_scan_quote_mark_standalone() {
        let types = token_types("'").unwrap();
        assert_eq!(types, vec![TokenType::QuoteMark]);
    }

    #[test]
    fn test_scan_quote_mark_before_atom() {
        let types = token_types("'x").unwrap();
        assert_eq!(types, vec![TokenType::QuoteMark, TokenType::Id]);
    }

    // --- 用户标识符 ---

    #[test]
    fn test_scan_identifier() {
        let (tokens, table) = scan("my-var?").unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Id);
        if let Some(ValueType::Str(s)) = &table[tokens[0].table_ptr].value {
            assert_eq!(s, "my-var?");
        } else {
            panic!("expected Str");
        }
    }

    #[test]
    fn test_scan_identifier_with_exclaim() {
        let types = token_types("set!").unwrap();
        assert_eq!(types, vec![TokenType::Id]);
    }

    // --- 算术运算符 ---

    #[test]
    fn test_scan_arithmetic_ops() {
        // '+', '*', '/' 识别为算术运算符
        let cases = vec![
            ("+", TokenType::PlusOp),
            ("*", TokenType::MulOp),
            ("/", TokenType::DivOp),
        ];
        for (src, expected) in cases {
            let types = token_types(src).unwrap();
            assert_eq!(types, vec![expected], "failed for op: {}", src);
        }

        // 独立的 '-' 由于 recog_id 先于 recog_op 执行，被识别为 Id
        // 负数如 '-7' 则由 recog_const 正确识别为 Const
        let types = token_types("-").unwrap();
        assert_eq!(types, vec![TokenType::Id]);
    }

    // --- 比较运算符 ---

    #[test]
    fn test_scan_comparison_ops() {
        let cases = vec![
            ("<", TokenType::LessThan),
            (">", TokenType::GreaterThan),
            ("<=", TokenType::LessEq),
            (">=", TokenType::GreaterEq),
            ("=", TokenType::Eq),
        ];
        for (src, expected) in cases {
            let types = token_types(src).unwrap();
            assert_eq!(types, vec![expected], "failed for cmp: {}", src);
        }
    }

    // --- 完整表达式 ---

    #[test]
    fn test_scan_define_expression() {
        let types = token_types("(define x 10)").unwrap();
        assert_eq!(types, vec![
            TokenType::LParen,
            TokenType::Define,
            TokenType::Id,
            TokenType::Const,
            TokenType::RParen,
        ]);
    }

    #[test]
    fn test_scan_arithmetic_expression() {
        let types = token_types("(+ x 5)").unwrap();
        assert_eq!(types, vec![
            TokenType::LParen,
            TokenType::PlusOp,
            TokenType::Id,
            TokenType::Const,
            TokenType::RParen,
        ]);
    }

    #[test]
    fn test_scan_if_expression() {
        let types = token_types("(if (< x 20) 1 2)").unwrap();
        assert_eq!(types, vec![
            TokenType::LParen,
            TokenType::If,
            TokenType::LParen,
            TokenType::LessThan,
            TokenType::Id,
            TokenType::Const,
            TokenType::RParen,
            TokenType::Const,
            TokenType::Const,
            TokenType::RParen,
        ]);
    }

    #[test]
    fn test_scan_multiline() {
        let input = "(define x 10)\n(+ x 5)";
        let types = token_types(input).unwrap();
        assert_eq!(types, vec![
            TokenType::LParen, TokenType::Define, TokenType::Id, TokenType::Const, TokenType::RParen,
            TokenType::LParen, TokenType::PlusOp, TokenType::Id, TokenType::Const, TokenType::RParen,
        ]);
    }

    // --- 错误情况 ---

    #[test]
    fn test_scan_invalid_character() {
        // '@' 不在合法字符集中，产生 InvalidCharacter 错误
        assert!(matches!(scan("@"), Err(ScanError::InvalidCharacter(_))));
        // '#' 不在合法字符集中，接在未识别前缀后也产生 InvalidCharacter 错误
        assert!(matches!(scan("#x"), Err(ScanError::InvalidCharacter(_))));
    }

    #[test]
    fn test_scan_invalid_token() {
        // 全部由合法字符组成但无法识别的串，产生 InvalidToken 错误
        // 例如：以数字开头但不能被解析为数字的串（如仅含字母的前缀被数字打断）
        assert!(matches!(scan("1abc"), Err(ScanError::InvalidToken(_))));
    }

    // --- 符号表位置 ---

    #[test]
    fn test_scan_token_table_index() {
        let (_, table) = scan("(define x 10)").unwrap();
        // '(' is at row 0, col 0
        assert_eq!(table[0].index, (0, 0));
        // 'define' is at row 0, col 1
        assert_eq!(table[1].index, (0, 1));
        // 'x' is at row 0, col 8
        assert_eq!(table[2].index, (0, 8));
        // '10' is at row 0, col 10
        assert_eq!(table[3].index, (0, 10));
        // ')' is at row 0, col 12
        assert_eq!(table[4].index, (0, 12));
    }
}

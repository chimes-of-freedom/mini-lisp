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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ParseError, scanner::scan};

    fn parse_str(input: &str) -> Result<(), ParseError> {
        let (tokens, token_table) = scan(input).map_err(|_| ParseError::UnknownScanError)?;
        parse(&tokens, &token_table)
    }

    // --- 原子（atom）---

    #[test]
    fn test_parse_integer_atom() {
        assert!(parse_str("42").is_ok());
    }

    #[test]
    fn test_parse_float_atom() {
        assert!(parse_str("3.14").is_ok());
    }

    #[test]
    fn test_parse_bool_atoms() {
        assert!(parse_str("#t").is_ok());
        assert!(parse_str("#f").is_ok());
    }

    #[test]
    fn test_parse_string_atom() {
        assert!(parse_str("\"hello world\"").is_ok());
    }

    #[test]
    fn test_parse_identifier_atom() {
        assert!(parse_str("my-var?").is_ok());
    }

    #[test]
    fn test_parse_operator_atom() {
        assert!(parse_str("+").is_ok());
        assert!(parse_str("<=").is_ok());
    }

    // --- 引号形式 ---

    #[test]
    fn test_parse_quoted_atom() {
        assert!(parse_str("'x").is_ok());
    }

    #[test]
    fn test_parse_quoted_integer() {
        assert!(parse_str("'42").is_ok());
    }

    #[test]
    fn test_parse_quoted_list() {
        assert!(parse_str("'(1 2 3)").is_ok());
    }

    #[test]
    fn test_parse_quoted_nested_list() {
        assert!(parse_str("'(a (b c))").is_ok());
    }

    // --- 空列表 ---

    #[test]
    fn test_parse_empty_list() {
        assert!(parse_str("()").is_ok());
    }

    // --- 简单 S 表达式 ---

    #[test]
    fn test_parse_define() {
        assert!(parse_str("(define x 10)").is_ok());
    }

    #[test]
    fn test_parse_arithmetic() {
        assert!(parse_str("(+ 1 2)").is_ok());
    }

    #[test]
    fn test_parse_display() {
        assert!(parse_str("(display \"hello\")").is_ok());
    }

    // --- 嵌套表达式 ---

    #[test]
    fn test_parse_if_expression() {
        assert!(parse_str("(if (< x 20) 1 2)").is_ok());
    }

    #[test]
    fn test_parse_lambda() {
        assert!(parse_str("(lambda (x) (+ x 1))").is_ok());
    }

    #[test]
    fn test_parse_list_expression() {
        assert!(parse_str("(list 1 2 3)").is_ok());
    }

    #[test]
    fn test_parse_cons_expression() {
        assert!(parse_str("(cons 1 (list 2 3))").is_ok());
    }

    #[test]
    fn test_parse_nested_arithmetic() {
        assert!(parse_str("(+ (* 2 3) (- 10 4))").is_ok());
    }

    // --- 多个顶层表达式 ---

    #[test]
    fn test_parse_multiple_top_level() {
        assert!(parse_str("(define x 10) (+ x 5)").is_ok());
    }

    #[test]
    fn test_parse_multiline() {
        let input = "(define x 10)\n(+ x 5)\n(if (< x 20) 1 2)";
        assert!(parse_str(input).is_ok());
    }

    // --- 错误情况 ---

    #[test]
    fn test_parse_unmatched_open_paren() {
        assert!(matches!(
            parse_str("(define x"),
            Err(ParseError::UnexpectedEndOfInput)
        ));
    }

    #[test]
    fn test_parse_unmatched_close_paren() {
        assert!(matches!(
            parse_str("(define x 10))"),
            Err(ParseError::UnexpectedToken(_))
        ));
    }

    #[test]
    fn test_parse_empty_input_is_error() {
        assert!(matches!(
            parse_str(""),
            Err(ParseError::UnexpectedEndOfInput)
        ));
    }

    #[test]
    fn test_parse_nested_unmatched() {
        // 内层括号未闭合
        assert!(matches!(
            parse_str("(+ (1 2)"),
            Err(ParseError::UnexpectedEndOfInput)
        ));
    }

    #[test]
    fn test_parse_deeply_nested() {
        assert!(parse_str("(a (b (c (d (e 1)))))").is_ok());
    }
}

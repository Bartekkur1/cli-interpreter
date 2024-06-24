use crate::lexer::{ Token, TokenVariant };

pub fn analyze_syntax(tokens: Vec<Token>) -> Result<(), String> {
    let mut i: i32 = 0;
    while i < (tokens.len() as i32) {
        let cursor = tokens.get(i as usize).unwrap();
        let prev_value_index = i - 1;
        let next_value_index = i + 1;
        if cursor.variant() == TokenVariant::Operator {
            if prev_value_index < 0 {
                return Err(
                    format!("Syntax Error! Operator missing value at position {}", prev_value_index)
                );
            }
            if next_value_index > (tokens.len() as i32) - 1 {
                return Err(
                    format!("Syntax Error! Operator missing value at position {}", next_value_index)
                );
            }
            let prev_token = tokens.get(prev_value_index as usize).unwrap();
            if prev_token.variant() != TokenVariant::Value {
                return Err(
                    format!("Syntax Error! Invalid token variant at {}, expected value", prev_value_index)
                );
            }
            let next_token = tokens.get(next_value_index as usize).unwrap();
            if next_token.variant() != TokenVariant::Value {
                return Err(
                    format!("Syntax Error! Invalid token variant at {}, expected value", next_value_index)
                );
            }
        }
        if cursor.variant() == TokenVariant::Value {
            let left_token_is_operator =
                prev_value_index > 0 &&
                tokens
                    .get(prev_value_index as usize)
                    .unwrap()
                    .variant() == TokenVariant::Operator;
            let right_token_is_operator =
                next_value_index < (tokens.len() as i32) &&
                tokens
                    .get(next_value_index as usize)
                    .unwrap()
                    .variant() == TokenVariant::Operator;
            if !left_token_is_operator && !right_token_is_operator {
                return Err(
                    format!("Syntax Error! Value token is missing operator at position {}", i)
                );
            }
        }

        i += 1;
    }
    Ok(())
}

#[test]
fn test_should_error_on_missing_next_operation_value() {
    let mut tokens: Vec<Token> = Vec::new();
    tokens.push(Token::Value { value: String::from("1") });
    tokens.push(Token::Operator { value: '*' });

    let syntax_result = analyze_syntax(tokens);
    assert!(syntax_result.is_err());
    assert_eq!(syntax_result.unwrap_err(), "Syntax Error! Operator missing value at position 2");
}

#[test]
fn test_should_error_on_missing_next_operation_value_reverse() {
    let mut tokens: Vec<Token> = Vec::new();
    tokens.push(Token::Operator { value: '*' });
    tokens.push(Token::Value { value: String::from("1") });

    let syntax_result = analyze_syntax(tokens);
    assert!(syntax_result.is_err());
    assert_eq!(syntax_result.unwrap_err(), "Syntax Error! Operator missing value at position -1");
}

#[test]
fn test_should_error_on_missing_prev_operation_value() {
    let mut tokens: Vec<Token> = Vec::new();
    tokens.push(Token::Operator { value: '*' });
    tokens.push(Token::Value { value: String::from("1") });

    let syntax_result = analyze_syntax(tokens);
    assert!(syntax_result.is_err());
    assert_eq!(syntax_result.unwrap_err(), "Syntax Error! Operator missing value at position -1");
}

#[test]
fn test_should_error_on_invalid_command() {
    let mut tokens: Vec<Token> = Vec::new();
    tokens.push(Token::Value { value: String::from("1") });
    tokens.push(Token::Operator { value: '*' });
    tokens.push(Token::Operator { value: '*' });

    let syntax_result = analyze_syntax(tokens);
    assert!(syntax_result.is_err());
    assert_eq!(
        syntax_result.unwrap_err(),
        "Syntax Error! Invalid token variant at 2, expected value"
    );
}

#[test]
fn test_should_fail_on_missing_operator() {
    let mut tokens: Vec<Token> = Vec::new();
    tokens.push(Token::Value { value: String::from("11") });

    let syntax_result = analyze_syntax(tokens);
    assert!(syntax_result.is_err());
    assert_eq!(
        syntax_result.unwrap_err(),
        "Syntax Error! Value token is missing operator at position 0"
    );
}

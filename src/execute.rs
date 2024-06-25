use core::panic;
use std::vec;

use crate::lexer::{ OperatorVariant, Token, TokenVariant };

fn get_slice<T: Clone>(vec: &[T], start: usize, end: usize) -> Vec<T> {
    vec[start..end].to_vec()
}

fn execute_operator(operator: &Token, left: &Token, right: &Token) -> Token {
    let left = left.token_value().parse::<f32>().unwrap();
    let right = right.token_value().parse::<f32>().unwrap();

    let result = match operator.operator_variant() {
        OperatorVariant::Mul => left * right,
        OperatorVariant::Div => left / right,
        OperatorVariant::Add => left + right,
        OperatorVariant::Sub => left - right,
    };

    return Token::value(result.to_string());
}

pub fn execute(tokens: Vec<Token>) -> f32 {
    // first is score, second is position
    let mut max_operator: (&u8, Option<usize>) = (&0, None);
    for (index, token) in tokens.iter().enumerate() {
        if token.variant() == TokenVariant::Operator {
            if max_operator.1 == None {
                max_operator.1 = Some(index);
                continue;
            }
            if token.score() > max_operator.0 {
                max_operator.0 = &token.score();
                max_operator.1 = Some(index);
            }
        }
    }

    if !max_operator.1.is_some() {
        let value = tokens.get(0).unwrap().token_value().parse::<f32>();
        return match value {
            Ok(value) => value,
            _ => panic!(),
        };
    }

    let index = max_operator.1.unwrap();
    let operator = tokens.get(index).unwrap();
    let left = tokens.get(index - 1).unwrap();
    let right = tokens.get(index + 1).unwrap();
    let operator_result = execute_operator(&operator, &left, &right);

    let right_side = get_slice(&tokens, index + 2, tokens.len());
    let left_side = get_slice(&tokens, 0, index - 1);

    let outgoing_tokens: Vec<Token> = [
        &left_side[..],
        &vec![operator_result],
        &right_side[..],
    ].concat();

    return execute(outgoing_tokens);
}

#[test]
fn test_basic_tokens() {
    let tokens = vec![
        Token::value("1".to_string()),
        Token::operator("+".to_string(), 0),
        Token::value("1".to_string())
    ];

    let result = execute(tokens);
    assert_eq!(result, 2.0);
}

#[test]
fn test_operator_order() {
    let tokens = vec![
        Token::value("2".to_string()),
        Token::operator("+".to_string(), 0),
        Token::value("2".to_string()),
        Token::operator("*".to_string(), 1),
        Token::value("2".to_string())
    ];

    let result = execute(tokens);
    assert_eq!(result, 6.0);
}

#[test]
fn test_handle_floats() {
    let tokens = vec![
        Token::value("1".to_string()),
        Token::operator("/".to_string(), 0),
        Token::value("2".to_string())
    ];

    let result = execute(tokens);
    assert_eq!(result, 0.5);
}

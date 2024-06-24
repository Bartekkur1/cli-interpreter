use crate::lexer::{ Token, TokenVariant };

pub fn analyze_syntax(mut tokens: Vec<Token>) -> Result<(), String> {
    println!("Tokens len: {}", tokens.len());

    // tokens.reverse();
    let mut i = 0i32;

    while i < (tokens.len() as i32) {
        let cursor = tokens.get(i as usize).unwrap();
        println!("{}", cursor);

        if cursor.variant() == TokenVariant::Operator {
            let prev_value_index = i - 1;
            let next_value_index = i + 1;
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

        i += 1;
    }
    Ok(())
}

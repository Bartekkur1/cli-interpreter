use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Value {
        value: String,
    },
    Operation {
        value: String,
    },
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Value { value } => write!(f, "Value: {}", value),
            Token::Operation { value } => write!(f, "Operation: {}", value),
        }
    }
}

pub fn lexer(input: &String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut input: Vec<char> = input.chars().collect();
    let mut mem = String::new();

    while let Some(c) = input.pop() {
        if Regex::new(r"\d").unwrap().is_match(&c.to_string()) {
            mem.insert(0, c);
        } else if Regex::new(r"(\+|\-|\/|\*)").unwrap().is_match(&c.to_string()) {
            if mem.len() > 0 {
                tokens.push(Token::Value { value: mem.clone() });
                mem = String::new();
            }
            tokens.push(Token::Operation { value: c.to_string().clone() });
        } else {
            return Err(format!("Unrecognized char {}!", c));
        }
    }

    if mem.len() > 0 {
        tokens.push(Token::Value { value: mem.clone() });
    }

    tokens.reverse();
    Ok(tokens)
}

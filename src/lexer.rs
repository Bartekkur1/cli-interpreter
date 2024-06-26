use core::panic;

use regex::Regex;

#[derive(Debug, Clone)]
pub enum Token {
    Value {
        value: String,
    },
    Operator {
        value: String,
        score: u8,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenVariant {
    Value,
    Operator,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OperatorVariant {
    Mul,
    Div,
    Add,
    Sub,
}

impl Token {
    pub fn value(value: String) -> Self {
        Token::Value { value }
    }

    pub fn operator(value: String, score: u8) -> Self {
        Token::Operator { value, score }
    }

    pub fn variant(&self) -> TokenVariant {
        match self {
            Token::Value { .. } => TokenVariant::Value,
            Token::Operator { .. } => TokenVariant::Operator,
        }
    }

    pub fn operator_variant(&self) -> OperatorVariant {
        match self {
            Token::Operator { value, .. } => {
                match value.as_str() {
                    "*" => OperatorVariant::Mul,
                    "/" => OperatorVariant::Div,
                    "+" => OperatorVariant::Add,
                    "-" => OperatorVariant::Sub,
                    _ => panic!("Unknown operator!"),
                }
            }
            // Handle other token variants if necessary
            _ => panic!("Expected an operator token!"),
        }
    }

    pub fn token_value(&self) -> &str {
        match self {
            Token::Value { value } => value,
            Token::Operator { value, score: _ } => value,
        }
    }

    pub fn score(&self) -> &u8 {
        match self {
            Token::Operator { value: _, score } => score,
            Token::Value { value: _ } => &0,
        }
    }
}

impl std::fmt::Display for TokenVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenVariant::Value => write!(f, "Value"),
            TokenVariant::Operator => write!(f, "Operator"),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Value { value } => write!(f, "Value: {}", value),
            Token::Operator { value, score } => write!(f, "Operator: {} Score: {}", value, score),
        }
    }
}

fn get_token_score(token: &String) -> u8 {
    if *token == String::from("*") || *token == String::from("/") {
        return 1;
    } else {
        return 0;
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
            let c = c.to_string();
            let token_score = get_token_score(&c);
            tokens.push(Token::Operator { value: c, score: token_score });
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

#[test]
fn test_lexer_simple_value() {
    let expected_value = String::from("1");
    let tokens = lexer(&expected_value).unwrap();
    assert_eq!(tokens.len(), 1);
    if let Some(Token::Value { value }) = tokens.get(0) {
        // assert!(true);
        assert_eq!(*value, expected_value);
    }
}

#[test]
fn test_lexer_value_with_operator() {
    let input = String::from("1+");
    let tokens: Vec<Token> = lexer(&input).unwrap();

    assert_eq!(tokens.len(), 2);
    if let Token::Value { value } = &tokens[0] {
        assert_eq!(*value, String::from("1"));
    } else {
        panic!("Expected a Value token");
    }

    if let Token::Operator { value, score: 0 } = &tokens[1] {
        println!("{}", value);
        assert_eq!(*value, "+");
    } else {
        panic!("Expected an Operator token");
    }
}

#[test]
fn test_lexer_simple_sentence() {
    let input = String::from("1+232+33*40");
    let tokens = lexer(&input).unwrap();
    assert_eq!(tokens.len(), 7);

    if let Token::Value { value } = &tokens[2] {
        assert_eq!(*value, String::from("232"));
    } else {
        panic!("Expected a Value token");
    }

    if let Token::Operator { value, score: 1 } = &tokens[5] {
        println!("{}", value);
        assert_eq!(*value, "*");
    } else {
        panic!("Expected an Operator token");
    }
}

#[test]
fn test_empty_input() {
    let input = String::from("");
    let tokens = lexer(&input).unwrap();
    assert_eq!(tokens.len(), 0);
}

#[test]
fn test_should_fail_on_unrecognized_char() {
    let input = String::from("B");
    let tokens = lexer(&input);
    assert!(tokens.is_err());
    assert_eq!(tokens.unwrap_err(), "Unrecognized char B!");
}

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
    if let Some(Token::Value { value }) = tokens.get(0) {
        // assert!(true);
        assert_eq!(*value, String::from("1"));
    }
    if let Some(Token::Operation { value }) = tokens.get(0) {
        // assert!(true);
        assert_eq!(*value, String::from("+"));
    }
}

#[test]
fn test_lexer_simple_sentence() {
    let input = String::from("1+232+33*40");
    let tokens = lexer(&input).unwrap();
    assert_eq!(tokens.len(), 7);
    if let Some(Token::Value { value }) = tokens.get(2) {
        // assert!(true);
        assert_eq!(*value, String::from("232"));
    }
    if let Some(Token::Operation { value }) = tokens.get(5) {
        // assert!(true);
        assert_eq!(*value, String::from("*"));
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

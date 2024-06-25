mod lexer;
mod syntax_analysis;
mod execute;

use std::io::{ self, BufRead };
use execute::execute;
use lexer::lexer;
use syntax_analysis::analyze_syntax;

fn read_input() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    if line.ends_with('\n') {
        line.pop();
    }
    line
}

fn print_help() {
    println!("- To exit type 'exit'");
    println!("This interpreter handles basic math operations (*/+-) with given numbers");
}

fn main() {
    println!("Welcome in rust cli interpreter!");
    println!("Type 'help' to see whats possible");

    loop {
        let input = read_input();
        if input == "exit" {
            break;
        } else if input == "help" {
            print_help();
        } else {
            let tokens = lexer(&input).unwrap();
            let syntax_result = analyze_syntax(&tokens);
            if let Err(error) = syntax_result {
                println!("Syntax Error! {}", error);
            }
            let result = execute(tokens);
            println!("Result: {}", result);
        }
    }
}

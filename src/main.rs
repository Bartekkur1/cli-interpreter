mod lexer;
mod syntax_analysis;

use std::io::{ self, BufRead };
use lexer::lexer;

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
        }

        let tokens = lexer(&input).unwrap();
        for item in &tokens {
            println!("{}", item);
        }
    }
}

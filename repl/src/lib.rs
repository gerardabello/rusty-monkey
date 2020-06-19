use std::iter::Iterator;
use lexer::{ Lexer, Token };

use std::io::{self, Read};

pub fn run() {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let tokens : Vec<Token> = Lexer::new(line.chars()).collect();
        println!("{:?}", tokens);
    }
}

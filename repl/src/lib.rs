use parser::Parser;

use std::io;

pub fn run() {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let program = Parser::new(line.chars()).parse_program();
        println!("{:?}", program);
    }
}

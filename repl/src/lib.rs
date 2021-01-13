use evaluator::{eval_statements, new_environment};

use parser::Parser;

use std::io;

pub fn run() {
    let env = new_environment();

    loop {
        print!("> ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        match Parser::new(line.chars()).parse_program() {
            Err(e) => {
                println!("There was an error parsing the program");
                println!("{:?}", e)
            }
            Ok(statements) => match eval_statements(&env, &statements) {
                Err(e) => {
                    println!("There was an error evaluating the program");
                    println!("{:?}", e)
                }
                Ok(o) => println!("{}", o),
            },
        };
    }
}

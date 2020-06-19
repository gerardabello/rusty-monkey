use std::env;

fn print_help() {
    println!("Usage: rustymonkey [command] ...");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Missing command");
        print_help();
        return;
    }

    let command = &args[1];

    match &command[..] {
        "repl" => repl::run(),
        unknown => {
            println!("Unknown command {}", unknown);
            print_help();
        }
    }
}

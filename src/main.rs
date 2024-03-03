use std::env;
use std::fs;
use std::io::{self, Write};

mod lexer;
mod parser;

/// Enter the REPL
fn run_repl() {
    let mut input = String::new();
    loop {
        print!("> ");
        // Flush to print the output
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read from stdin");
        run(&input);
        // Remembder to clear the input, otherwise the last insertion will be
        // read again
        input.clear();
    }
}

/// Scan the input program
fn run(program: &String) {
    let mut scanner: lexer::Lexer = lexer::Lexer::init(program);
    if let Err(e) = scanner.scan() {
        eprintln!("Error while scanning: {}", e);
    }
    println!("{:?}\n\n", scanner.tokens);
    let mut parser: parser::Parser = parser::Parser::init(scanner.tokens);
    match parser.parse() {
        Ok(expr) => println!("{:?}", expr),
        Err(e) => eprintln!("Error while parsing: {}", e),
    }
}

fn main() {
    let mut args = env::args();
    if args.len() == 2 {
        // Try to parse the input file
        let content = fs::read_to_string(args.nth(1).expect("Never happen!"))
            .unwrap_or_else(|_| panic!("Invalid string while reading"));
        run(&content)
    } else {
        // Start the REPL
        run_repl()
    }
}

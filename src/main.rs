use std::{
    env::args,
    fs::read,
    io::{self, stdin, stdout, BufRead, Write},
    process::exit,
};

enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EOF,
}



fn main() {
    let mut had_error = false;
    let args: Vec<String> = args().collect();
    println!("{}", args.len());
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(&args[1], &had_error);
    } else {
        run_prompt(&mut had_error);
        println!("> {}", had_error);
    }
}

fn run_file(path: &String, had_error: &bool) {
    let bytes = read(path).expect("Can't read file");
    //run();

    if had_error.to_owned() {
        exit(65);
    }
}

fn run_prompt(had_error: &mut bool) {
    let mut reader = stdin().lock();
    loop {
        println!("> {}", had_error);
        let mut input = String::new();
        reader.read_line(&mut input).expect("Unable to read");
        run(input.trim());
        *had_error = false;
    }
}

fn run(source: &str) {
    //println!("{}", source);
}

fn error(line: u32, message: String) {
    report(line, String::new(), message);
}

fn report(line: u32, location: String, message: String) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
}

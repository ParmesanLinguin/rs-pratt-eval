use crate::eval;
use crate::parse::{Expr, Lexer, Parser, Token};
use std::io::Write;

pub fn repl() {
    let mut print_tree = false;
    let mut print_tokens = false;

    loop {
        print!("> ");
        std::io::stdout().flush();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        if line.starts_with("#tree") {
            print_tree = !print_tree;
            let text = if print_tree { "enabled" } else { "disabled" };
            println!("tree printing {}.", text);
        }
        else if line.starts_with("#tokens") {
            print_tokens = !print_tokens;
            let text = if print_tokens { "enabled" } else { "disabled" };
            println!("token printing {}.", text);
        }
        else if line.starts_with("#") {
            println!("unrecognized command {}", line);
        } else {
            let lexed = lex(line);
            if let Err(_) = lexed {
                println!("failed to lex.");
                continue;
            }
            let lexed = lexed.unwrap();

            if print_tokens { debug_tokens(&lexed); }
            
            let parsed = parse(lexed);
            if let Err(_) = parsed {
                println!("failed to parse.");
                continue;
            }
            let parsed = parsed.unwrap();
            if print_tree { debug_tree(&parsed) };

            let eval = eval::interp(&parsed);
            if let Err(_) = eval {
                println!("failed to eval.");
                continue;
            }
            let eval = eval.unwrap();
            println!("{eval}")
        }
    }
}

fn lex(input: String) -> Result<Vec<Token>, ()> {
    let mut lexer = Lexer::new(&input);
    let lexed = lexer.lex()?;
    Ok(lexed)
}

fn parse(input: Vec<Token>) -> Result<Expr, ()> {
    let mut parser = Parser::new(input);
    let parsed = parser.parse()?;
    Ok (parsed)
}

fn debug_tokens(tokens: &Vec<Token>) {    
    println!("token stream: \x1b[90m");
    for (i, t) in tokens.iter().enumerate() {
        println!("{:0>4} {}", i, t);
    }
    println!("\x1b[0m");
}

fn debug_tree(root: &Expr) {
    println!("parse tree: \x1b[90m");
    root.pretty_print();
    println!("\x1b[0m");
}
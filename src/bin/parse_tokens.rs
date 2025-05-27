use std::{env, process, fs};
use mini_lisp::scanner::{ScanError, scan};
use mini_lisp::parser::parse;
use mini_lisp::ParseError::*;


fn main() {
    // 读取命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        if args.len() < 2 {
            eprintln!("not enough arguments");
        } else if args.len() > 2 {
            eprintln!("too many arguments");
        }

        eprintln!("usage: mini_lisp <filename>");
        process::exit(1);
    }

    let filename = &args[1];
    let input = match fs::read_to_string(filename) {
        Ok(input) => input,
        
        Err(_) => {
            eprintln!("Something went wrong reading the file");
            process::exit(1);
        }
    };

    match scan(input.as_str()) {
        Ok((token_sequence, _)) => {
            match parse(&token_sequence) {
                Ok(()) => println!("parsing success"),
                Err(e) => {
                    match e {
                        UnexpectedToken => eprintln!("parsing failed: Unexpected Token"),
                        UnexpectedRemains => eprintln!("parsing failed: Unexpected Remains"),
                        UnexpectedEndOfInput => eprintln!("parsing failed: Unexpected End Of Input")
                    }
                    process::exit(1);
                }
            }
        },

        Err(e) => {
            match e {
                ScanError::InvalidCharacter((row, column)) => {
                    eprintln!(
                        "tokenize() failed at row {} column {}: Invalid Character",
                        row + 1, column + 1
                    );
                },

                ScanError::InvalidToken((row, column)) => {
                    eprintln!(
                        "tokenize() failed at row {} column {}: Invalid Token",
                        row + 1, column + 1
                    );
                }
            };

            process::exit(1);
        }
    }
}
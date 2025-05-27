use std::{env, fs, io::{self, Write}, process};
use mini_lisp::ScanError;
use mini_lisp::scanner::scan;
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
        Ok((token_sequence, token_table)) => {
            // 输出词法分析结果
            println!("=====================");
            println!("====== Scanner ======");
            println!("=====================");
            println!("tokens:");
            for token in token_sequence.iter() {
                print!("<{:?}, {}> ", token.token_type, token.table_ptr);
                io::stdout().flush().expect("flush failed");
            }
            println!("\n");

            println!("token table:");
            for (i, table_item) in token_table.iter().enumerate() {
                println!("{:>3}: {:?}", i, table_item);
            }

            // 输出语法分析结果
            println!("\n");
            println!("====================");
            println!("====== Parser ======");
            println!("====================");
            match parse(&token_sequence, &token_table) {
                Ok(()) => println!("parsing success"),
                Err(e) => {
                    match e {
                        UnexpectedToken((x, y)) => eprintln!("parse() failed at row {} column {}: Unexpected Token", x + 1, y + 1),
                        UnexpectedEndOfInput => eprintln!("parse() failed: Unexpected End Of Input"),
                        UnknownScanError => eprintln!("parse() failed: Unknown Scan Error"),
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
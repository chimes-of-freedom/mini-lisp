use std::io::{self, Write};
use std::{env, process, fs};
use mini_lisp::scanner::{ScanError, scan};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("not enough arguments");
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
            println!("tokens:");
            for token in token_sequence {
                print!("<{:?}, {}> ", token.token_type, token.table_ptr);
                io::stdout().flush().expect("flush failed");
            }
            println!("\n");

            println!("token table:");
            for (i, table_item) in token_table.iter().enumerate() {
                println!("{:>3}: {:?}", i, table_item);
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

use std::io::{self, Write};
use std::{env, process, fs};

use mini_lisp::scanner::scan;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments");
    }

    let filename = &args[1];
    let input = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    match scan(input.as_str()) {
        Ok((token_sequence, token_table)) => {
            for token in token_sequence {
                print!("<{:?}, {}> ", token.token_type, token.table_ptr);
                io::stdout().flush().expect("flush failed");
            }
            println!("");
            for (i, table_item) in token_table.iter().enumerate() {
                println!("{}: {:?}", i, table_item);
            }
        },
        _ => {
            eprintln!("ScanError");
            process::exit(1);
        }
    }
}
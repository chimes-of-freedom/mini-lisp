use std::io::{self, Write};

use mini_lisp::scanner::scan;

fn main() {
    print!("input string: ");
    io::stdout().flush().expect("flush failed");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read line error");

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
        _ => { println!("ScanError"); }
    }
}
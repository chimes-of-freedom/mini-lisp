use std::{ path::PathBuf, fs, io::{self, Write}, process };

use clap::{ Args, Parser, Subcommand };

use mini_lisp::{ ScanError, ParseError::*, scanner::scan, parser::parse };

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {   
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug)]
struct CommonArgs {
    name: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    Scan(CommonArgs),
    Parse(CommonArgs),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Scan(args) => {
            let path = &args.name;

            let Ok(input) = fs::read_to_string(path) else {
                eprintln!("Something went wrong reading the file");
                process::exit(1);
            };

            // 对 `filename` 做词法分析
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
        },

        Commands::Parse(args) => {
            let path = &args.name;
            
            let Ok(input) = fs::read_to_string(path) else {
                eprintln!("Something went wrong reading the file");
                process::exit(1);
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
                },
            }
        },
    }
}

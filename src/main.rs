pub mod lexer;
pub mod move_tree;

use std::fs;

use lexer::{Lexer, Token};

fn main() {
    for val in fs::read_dir("./data").expect("could not open data directory") {
        let Ok(entry) = val else {
            continue;
        };
        let Ok(data) = fs::read_to_string(entry.path()) else {
            continue;
        };
        let mut lex = Lexer::new(data);
        println!(
            "=== tokenizing: {} ===",
            entry.file_name().to_string_lossy()
        );
        loop {
            match lex.next_token() {
                Token::EOF => {
                    println!("\t{}", Token::EOF);
                    break;
                }
                x => println!("\t{x}"),
            }
        }
        println!("=== end of file ===");
    }
}

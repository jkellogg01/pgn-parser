pub mod lexer;

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
        println!(
            "=== tokenizing: {} ===",
            entry.file_name().to_string_lossy()
        );
        Lexer::new(data).for_each(|tok| {
            match tok {
                Token::EOF => {
                    println!("\t{}", Token::EOF);
                    return;
                }
                x => println!("\t{x}"),
            }
        });
        println!("=== end of file ===");
    }
}

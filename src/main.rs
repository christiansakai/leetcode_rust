#![allow(dead_code)]

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::path::Path;

mod easy;
mod medium;
mod hard;

const HELP: &'static str = r#"
No category and file_name provided.

Usage:

$ cargo run -- --easy path_to_file
"#;

const FILE_TEMPLATE: &'static str = r#"pub fn run() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(run());
    }
}
"#;

fn main() {
    if env::args().len() < 3 {
        println!("{}", HELP);
        process::exit(1);
    }

    let mut category = String::new();
    let mut file_name = String::new();

    for arg in env::args().skip(1) {
        match arg.as_ref() {
            "--easy" => {
                category.push_str("easy");
            },
            "--medium" => {
                category.push_str("medium");
            },
            "--hard" => {
                category.push_str("hard");
            },
            _ => {
                file_name.push_str(&arg);
            }
        }
    }

    let file_path = format!(
        "./src/{}/{}.rs",
        category,
        file_name,
    );

    if Path::new(&file_path).exists() {
        println!("\nThe file {} already exists", file_path);
        process::exit(1);
    } else {
        match generate_file(&file_path) {
            Ok(_) => {
                println!("\n Created {}", file_path);
            },
            Err(err) => {
                println!("\nFailed to create {}", file_path);
                println!("{:?}", err);
                process::exit(1);
            }
        }
    }
}

fn generate_file(file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(FILE_TEMPLATE.as_bytes())?;
    Ok(())
}

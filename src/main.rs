use std::env;
// use std::io;
// use std::io::Read;
// use std::io::File;
// use std::path::Path;

mod easy;

const HELP: &'static str = r#"
No category and file_name provided.

Usage:

$ cargo run -- --easy path_to_file
"#;

fn main() {
    if env::args().len() < 3 {
        println!("{}", HELP);
    }

    let mut category: String;
    let mut file_name: String;

    for arg in env::args().skip(1) {
        match arg.as_ref() {
            "--easy" => {
                category = String::from("easy");
            },
            "--medium" => {
                category = String::from("medium");
            },
            "--hard" => {
                category = String::from("hard");
            },
            _ => {
                file_name = arg;
            }
        }
    }

    generate_file(category, file_name);
}

fn generate_file(category: String, file_name: String) {
}

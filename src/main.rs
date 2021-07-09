pub mod parser;

use core::panic;
use parser::{Parser, Token};
use std::env::args;

fn parse_markdown_file(file_path: &String) {
    let mut parser = Parser::new(file_path);

    //TODO: Come up with a better idea for this
    let it = parser.clone();
    let mut it = it.input_lines.iter();

    while let Some(line) = it.next() {
        //TODO: Come up with a better idea for this
        if !line.is_empty() {
            parser.current_line = line.clone();
            let token = Token::new(
                line.chars()
                    .take_while(|ch| *ch != ' ')
                    .collect::<String>()
                    .as_str(),
            );

            parser.parse(token);
        }
    }

    parser.write_to_file("test.html");
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Please provide a path to an existing file.")
    }

    parse_markdown_file(&args[1]);
}

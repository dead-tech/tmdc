pub mod fs;
pub mod parsable;
pub mod parser;
pub mod tests;
pub mod util;

use core::panic;
use fs::file_handler::FileHandler;
use parser::{Parser, ParserState, Token};
use std::{env::args, vec};

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Please provide a path to an existing file.")
    }

    let lines = FileHandler::read(&args[1]);
    let mut output_lines = vec![];

    let mut state = ParserState {
        input_lines: lines.clone(),
        token: Token::General,
        current_line: (0, String::from("")),
        code_blocks: vec![],
        next_ul: 0,
    };

    for (line_number, line) in lines.iter().enumerate() {
        state.token = Token::new(
            line.chars()
                .take_while(|ch| *ch != ' ')
                .collect::<String>()
                .as_str(),
        );

        state.current_line = (line_number, line.clone());

        output_lines.push(Parser::parse(&mut state));
    }

    println!("{:#?}", output_lines);

    FileHandler::write(&args[1].replace(".md", ".html"), output_lines);
}

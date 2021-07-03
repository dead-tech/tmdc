use core::panic;
use regex::Regex;
use std::collections::VecDeque;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn parse_token(mut token: VecDeque<char>, mut line: String) -> String {
    // println!("Token: {:#?}\n Line: {:#?}", &token, &line);

    if line == "---" {
        let s = format!("<hr />");
        return s;
    }

    //TODO: Bold and italics get applied also to code_blocks
    //Actually no idea of how this could be solved.

    let bold_regex = Regex::new(r"[*_]{2}(?P<w>[\w\d\s]+)[*_]{2}").unwrap();
    line = bold_regex
        .replace_all(&line, "<strong>$w</strong>")
        .to_string();

    let italic_regex = Regex::new(r"[*_]{1}(?P<w>[\w\d\s]+)[*_]{1}").unwrap();
    line = italic_regex.replace_all(&line, "<em>$w</em>").to_string();

    let ret: String = match token.pop_front() {
        Some('#') => {
            let s = format!(
                "<h{}>{}</h{}>\n",
                token.len() + 1,
                &line[token.len() + 1..],
                token.len() + 1
            );
            s
        }
        Some('`') => {
            //TODO: There's only one know problem for this implementation:
            // - newlines aren't kept so the text in the code tag will always appear on the same line.
            // This happens because i replace \n with " ", should be pretty easy to fix but tedious.
            if line.matches("`").count() == 2 {
                // println!("Line: {:?} => {:?}", line, "Single");
                let single_line_code_regex = Regex::new(r"[`]{1}(?P<w>.+)[`]{1}").unwrap();
                line = single_line_code_regex
                    .replace_all(&line, "<code>$w</code>\n")
                    .to_string();
                // println!("Replaced with => {:?}", line);
                return line;
            } else {
                "".to_string()
            }
        }
        Some('*') => {
            //TODO: Now creates a <ul> tag for each item even if they sould be part of the same unordered list.
            //Probably needs to know of the next lines in order to handle this correctly
            //So something like I did for the multiline regione of text could work.
            let unordered_list_regex = Regex::new(r"[*]{1}\s(?P<w>.+)(?m)").unwrap();

            line = unordered_list_regex
                .replace_all(&line, "<ul>\n<li>$w</li>\n</ul>\n")
                .to_string();
            return line;
        }
        _ => {
            // println!("{:#?}", token);
            let s = format!("<p>{}</p>\n", &line);
            s
        }
    };

    ret
}

fn parse_code_block(lines: &Vec<String>) -> VecDeque<Vec<String>> {
    let code_blocks_indices: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, el)| el.starts_with("```"))
        .map(|el| el.0)
        .collect();

    let code_blocks = code_blocks_indices
        .chunks(2)
        .map(|el| {
            lines
                .iter()
                .skip(el[0] + 1)
                .take(el[1] - el[0] - 1)
                .map(String::clone)
                .collect::<Vec<_>>()
        })
        .collect();

    code_blocks
}

fn parse_markdown_file(file_path: &String) {
    let file = File::open(file_path).expect("Could not open file.");
    let input_lines: Vec<_> = BufReader::new(file.try_clone().unwrap())
        .lines()
        .map(Result::unwrap)
        .collect();

    let mut output_lines: Vec<String> = Vec::new();
    let mut code_blocks = parse_code_block(&input_lines);

    let mut it = input_lines.iter();

    // enum Token {
    //     CodeBlock(Vec<String>),
    //     List(Vec<String>),
    // }

    // let test = Token::code_block(vec![]);

    // match test {
    //     Token::code_block(content) => {}
    // }

    while let Some(line) = it.next() {
        let token: VecDeque<char> = line.chars().take_while(|ch| *ch != ' ').collect();

        if line.starts_with("```") {
            if code_blocks.len() > 0 {
                let block = code_blocks.pop_front().unwrap();

                it.nth(block.len());

                output_lines.push(format!(
                    "<code>\n<pre>\n{}</pre>\n</code>\n",
                    block.join("\n")
                ));
            }
        } else if token.len() > 0 {
            output_lines.push(parse_token(token, line.clone()));
        }
    }

    let mut outfile = File::create("test.html").expect("Could not create output file");

    for line in &output_lines {
        outfile
            .write_all(line.as_bytes())
            .expect("Could not write to output file.")
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Please provide a path to an existing file.")
    }

    parse_markdown_file(&args[1]);
}

pub mod parser;

use core::panic;
use parser::{Parser, Token};
use std::env::args;

// fn parse_token(mut token: String, mut line: String) -> String {
//     // println!("Token: {:#?}\n Line: {:#?}", &token, &line);

//     if line == "---" {
//         let s = format!("<hr />");
//         return s;
//     }

//     //TODO: Bold and italics get applied also to code_blocks
//     //Actually no idea of how this could be solved.

//     let bold_regex = Regex::new(r"[*_]{2}(?P<w>[\w\d\s]+)[*_]{2}").unwrap();
//     line = bold_regex
//         .replace_all(&line, "<strong>$w</strong>")
//         .to_string();

//     let italic_regex = Regex::new(r"[*_]{1}(?P<w>[\w\d\s]+)[*_]{1}").unwrap();
//     line = italic_regex.replace_all(&line, "<em>$w</em>").to_string();

//     let ret: String = match token.chars().nth(0) {
//         Some('#') => {
//             let s = format!(
//                 "<h{}>{}</h{}>\n",
//                 token.len() + 1,
//                 &line[token.len() + 1..],
//                 token.len() + 1
//             );
//             s
//         }
//         Some('`') => {
//             //TODO: There's only one know problem for this implementation:
//             // - newlines aren't kept so the text in the code tag will always appear on the same line.
//             // This happens because i replace \n with " ", should be pretty easy to fix but tedious.
//             if line.matches("`").count() == 2 {
//                 // println!("Line: {:?} => {:?}", line, "Single");
//                 let single_line_code_regex = Regex::new(r"[`]{1}(?P<w>.+)[`]{1}").unwrap();
//                 line = single_line_code_regex
//                     .replace_all(&line, "<code>$w</code>\n")
//                     .to_string();
//                 // println!("Replaced with => {:?}", line);
//                 return line;
//             } else {
//                 "".to_string()
//             }
//         }
//         Some('*') => {
//             //TODO: Now creates a <ul> tag for each item even if they sould be part of the same unordered list.
//             //Probably needs to know of the next lines in order to handle this correctly
//             //So something like I did for the multiline regione of text could work.
//             let unordered_list_regex = Regex::new(r"[*]{1}\s(?P<w>.+)(?m)").unwrap();

//             line = unordered_list_regex
//                 .replace_all(&line, "<ul>\n<li>$w</li>\n</ul>\n")
//                 .to_string();
//             return line;
//         }
//         _ => {
//             // println!("{:#?}", token);
//             let s = format!("<p>{}</p>\n", &line);
//             s
//         }
//     };

//     ret
// }

fn parse_markdown_file(file_path: &String) {
    let mut parser = Parser::new(file_path);

    let it = parser.clone();
    let mut it = it.input_lines.iter();

    while let Some(line) = it.next() {
        let token = Token::new(
            line.chars()
                .take_while(|ch| *ch != ' ')
                .collect::<String>()
                .as_str(),
        );

        parser.parse(token);
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

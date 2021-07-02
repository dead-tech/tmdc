use core::panic;
use regex::{Regex, RegexBuilder};
use std::collections::VecDeque;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::slice;

fn parse_token(mut token: VecDeque<char>, mut line: String) -> String {
    println!("Token: {:#?}\n Line: {:#?}", &token, &line);

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
            if line.matches("`").count() == 6 {
                // println!("Actual Line: {:?}", line);
                let multiline_code_regex = RegexBuilder::new(r"[`]{3}\n(?P<w>.+)\n[`]{3}")
                    .dot_matches_new_line(true)
                    .build()
                    .unwrap();

                line = multiline_code_regex
                    .replace_all(&line, "<code>\n$w</code>\n")
                    .to_string();

                return line;
            } else if line.matches("`").count() == 2 {
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

fn new_parse_code_block(reader: &mut BufReader<File>) -> (Vec<String>, Vec<String>) {
    let mut read_buffer: Vec<u8> = vec![];
    let mut buffers: Vec<Vec<u8>> = Vec::new();

    let mut code_blocks: Vec<String> = Vec::new();
    let mut inner_codeblocks: Vec<String> = Vec::new();

    let mut bytes_read: usize = 1;

    // let lines: Vec<_> = reader.lines().map(|el| el.unwrap()).collect();

    // let test: Vec<_> = lines
    //     .iter()
    //     .enumerate()
    //     .filter(|(_, el)| el.starts_with("```"))
    //     .map(|el| el.0)
    //     .collect();

    // println!("{:?}", test);

    // let code_blocks: Vec<_> = test
    //     .chunks(2)
    //     .map(|el| {
    //         lines
    //             .iter()
    //             .skip(el[0] + 1)
    //             .take(el[1] - el[0] - 1)
    //             .collect::<Vec<_>>()
    //     })
    //     .collect();

    // println!("{:?}", code_blocks);

    while bytes_read != 0 {
        read_buffer.clear();

        let mut line = String::new();

        let _ = reader
            .by_ref()
            .read_line(&mut line)
            .unwrap_or(usize::max_value());
        bytes_read = reader
            .by_ref()
            .read_until(b'`', &mut read_buffer)
            .unwrap_or(usize::max_value());

        if line.contains("``") {
            buffers.push(read_buffer.clone());
        }
    }

    // WARN: This is an hack
    buffers.pop();

    for buffer in buffers {
        if buffer.len() > 1 {
            let mut codeblock = String::from_utf8(buffer.clone()).unwrap();
            codeblock = codeblock[..&codeblock.len() - 1].to_string();
            codeblock.insert_str(0, "```\n");
            codeblock.push_str("\n```");

            code_blocks.push(codeblock)
        }
    }

    let inner_code_blocks_cleaned: Vec<_> = code_blocks
        .clone()
        .iter()
        .map(|el| el.replace("```", "").replace("\n", ""))
        .collect();

    for t in inner_code_blocks_cleaned {
        let split = t.split("\r").filter(|el| *el != "");

        for s in split {
            inner_codeblocks.push(s.to_string());
        }
    }

    (code_blocks, inner_codeblocks)
}

fn parse_markdown_file(file_path: &String) {
    let mut file = File::open(file_path).expect("Could not open file.");
    let mut reader = BufReader::new(file.try_clone().unwrap());

    let mut lines: Vec<String> = Vec::new();

    let mut codeblock_tags_counter = 0;
    let mut current_codeblock = 0;
    let (codeblocks, inner_codeblocks) = new_parse_code_block(reader.by_ref());

    let _ = file.seek(SeekFrom::Start(0));

    for line in reader.by_ref().lines() {
        let mut line: String = match line {
            Ok(contents) => contents,
            Err(err) => panic!("Invalid token: {}", err.to_string()),
        };
        let token: VecDeque<char> = line.chars().take_while(|ch| *ch != ' ').collect();

        if line.matches("`").count() == 3 && codeblock_tags_counter % 2 == 0 {
            line = codeblocks[current_codeblock].clone();
            codeblock_tags_counter += 1;
            current_codeblock += 1;
        } else if line.matches("`").count() == 3 {
            codeblock_tags_counter += 1;
        }

        if token.len() > 0 && !inner_codeblocks.iter().any(|el| *el == line) {
            lines.push(parse_token(token, line));
        }
    }

    let mut outfile = File::create("test.html").expect("Could not create output file");

    for line in &lines {
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

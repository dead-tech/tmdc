use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

trait Parsable<T> {
    fn parse(parser: &mut Parser) -> T;
}

pub enum Token {
    Heading,
    CodeBlock,
    UnorderedList,
    General,
}

#[derive(Debug, Clone)]
struct Heading;

#[derive(Debug, Clone)]
struct Paragraph;

#[derive(Debug, Clone)]
struct CodeBlocks {
    code_blocks: VecDeque<Vec<String>>,
    counter: usize,
    skip: bool,
}

#[derive(Debug, Clone)]
struct UnorderedLists {
    unordered_lists: VecDeque<Vec<String>>,
    counter: usize,
    max_counter: usize,
}

#[derive(Debug, Clone)]
pub struct Parser {
    pub input_lines: Vec<String>,
    output_lines: Vec<String>,
    code_blocks: CodeBlocks,
    unordered_lists: UnorderedLists,
    pub current_line: String,
}

impl Token {
    pub fn new(s: &str) -> Token {
        // TODO: Come up with a better way of doing this, if there is any
        if s.starts_with("#") {
            return Token::Heading;
        }

        match s {
            "```" => Token::CodeBlock,
            "*" => Token::UnorderedList,
            _ => Token::General,
        }
    }
}

impl Parsable<String> for Heading {
    fn parse(parser: &mut Parser) -> String {
        let heading_number = parser.current_line.matches("#").count();
        let content = parser.current_line[heading_number + 1..].to_string();
        format!("<h{}>{}</h{}>\n", heading_number, content, heading_number)
    }
}

impl Parsable<String> for Paragraph {
    fn parse(parser: &mut Parser) -> String {
        //TODO: Only do this if it isn't a codeblock.
        if !parser.code_blocks.is_code_block(&parser.current_line) {
            return format!("<p>{}</p>\n", parser.current_line);
        }
        "".to_string()
    }
}

impl CodeBlocks {
    fn is_code_block(&self, line: &String) -> bool {
        for blocks in &self.code_blocks {
            for code_line in blocks {
                if line == code_line {
                    return true;
                }
            }
        }
        return false;
    }
}

impl Parsable<Vec<String>> for CodeBlocks {
    fn parse(parser: &mut Parser) -> Vec<String> {
        let mut res = Vec::new();

        if parser.code_blocks.code_blocks.len() > 0 && !parser.code_blocks.skip {
            let blocks = parser
                .code_blocks
                .code_blocks
                .get(parser.code_blocks.counter);

            match blocks {
                Some(inner) => {
                    res.push(format!(
                        "<code>\n<pre>\n{}\n</pre>\n</code>\n",
                        &inner.join("\n")
                    ));
                    parser.code_blocks.counter += 1;
                }
                None => {}
            }
        }
        res
    }
}

impl Parsable<Vec<String>> for UnorderedLists {
    fn parse(parser: &mut Parser) -> Vec<String> {
        let mut res = Vec::new();

        if parser.unordered_lists.unordered_lists.len() > 0 {
            if parser.unordered_lists.counter == 0 {
                let blocks = parser.unordered_lists.unordered_lists.pop_front().unwrap();

                res.push(format!(
                    "<ul>{}\n</ul>\n",
                    blocks
                        .iter()
                        .map(|s| format!("\n<li>\n{}\n</li>", s))
                        .collect::<String>()
                ));
            }

            parser.unordered_lists.counter += 1;

            if parser.unordered_lists.counter == parser.unordered_lists.max_counter {
                parser.unordered_lists.max_counter =
                    parser.unordered_lists.unordered_lists[0].len();
                parser.unordered_lists.counter = 0;
            }
        }
        res
    }
}

impl Parser {
    pub fn new(file_path: &String) -> Parser {
        let file = Parser::open_file(file_path);
        let input_lines: Vec<String> = Parser::read_file(&file);
        let code_blocks = Parser::parse_code_blocks(&input_lines);
        let unordered_lists = Parser::parse_unordered_lists(&input_lines);

        Parser {
            input_lines,
            output_lines: Vec::new(),
            code_blocks: CodeBlocks {
                code_blocks,
                counter: 0,
                skip: false,
            },
            unordered_lists: UnorderedLists {
                unordered_lists: unordered_lists.clone(),
                counter: 0,
                max_counter: unordered_lists.get(0).unwrap_or(&vec![]).len(),
            },
            current_line: String::from(""),
        }
    }

    fn open_file(file_path: &String) -> File {
        let file = match File::open(file_path) {
            Ok(file_content) => file_content,
            Err(err) => {
                panic!("Unable to open file. Reason: {}", err.to_string())
            }
        };

        file
    }

    fn read_file(file: &File) -> Vec<String> {
        let input_lines: Vec<String> =
            BufReader::new(file.try_clone().expect("Failed to clone file!"))
                .lines()
                .map(|s| s.unwrap())
                .collect();

        input_lines
    }

    pub fn write_to_file(&self, file_path: &str) {
        let mut file = File::create(file_path).expect("Failed to create file!");

        for line in &self.output_lines {
            file.write_all(line.as_bytes())
                .expect(format!("Failed to write {}", line).as_str());
        }
    }

    fn parse_code_blocks(lines: &Vec<String>) -> VecDeque<Vec<String>> {
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
                    .map(|s| s.clone().to_string())
                    .collect::<Vec<_>>()
            })
            .collect();

        code_blocks
    }

    // TODO: Check whether this function could become simpler
    fn parse_unordered_lists(lines: &Vec<String>) -> VecDeque<Vec<String>> {
        let mut unordered_lists_indices: Vec<_> = lines
            .iter()
            .enumerate()
            .filter(|(_, s)| s.starts_with("*"))
            .map(|s| s.0)
            .collect();

        let mut consecutives = Vec::new();

        let mut i = 0;
        while let (Some(curr), Some(next)) = (
            unordered_lists_indices.get(i),
            unordered_lists_indices.get(i + 1),
        ) {
            if next - curr != 1 {
                let (left, right) = unordered_lists_indices.split_at(i + 1);
                consecutives.push(left.to_vec());
                unordered_lists_indices = right.to_vec();
                i = 0;
            }
            i += 1;
        }

        let mut unordered_lists_blocks = VecDeque::new();

        if unordered_lists_indices.len() > 0 {
            consecutives.push(unordered_lists_indices);

            unordered_lists_blocks = consecutives
                .iter()
                .map(|el| {
                    lines
                        .iter()
                        .skip(el[0])
                        .take(el[el.len() - 1] - el[0] + 1)
                        .map(|s| s.clone()[2..].to_string())
                        .collect::<Vec<_>>()
                })
                .collect();
        }
        unordered_lists_blocks
    }

    pub fn parse(&mut self, token: Token) {
        match token {
            Token::Heading => {
                let res = Heading::parse(self);
                self.output_lines.push(res);
            }
            Token::CodeBlock => {
                let mut res = CodeBlocks::parse(self);
                self.code_blocks.skip = !self.code_blocks.skip;
                self.output_lines.append(&mut res);
            }
            Token::UnorderedList => {
                let mut res = UnorderedLists::parse(self);
                self.output_lines.append(&mut res);
            }
            Token::General => {
                let res = Paragraph::parse(self);
                self.output_lines.push(res);
            }
        }
    }
}

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

trait Parsable {
    fn parse(parser: &mut Parser) -> Vec<String>;
}

pub enum Token {
    CodeBlock,
    UnorderedList,
    General,
}

#[derive(Debug, Clone)]
struct CodeBlocks {
    code_blocks: VecDeque<Vec<String>>,
}

#[derive(Debug, Clone)]
struct UnorderedLists {
    unordered_lists: VecDeque<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Parser {
    pub input_lines: Vec<String>,
    output_lines: Vec<String>,
    code_blocks: CodeBlocks,
    unordered_lists: UnorderedLists,
}

impl Token {
    pub fn new(s: &str) -> Token {
        match s {
            "```" => Token::CodeBlock,
            "*" => Token::UnorderedList,
            _ => Token::General,
        }
    }
}

impl Parsable for CodeBlocks {
    fn parse(parser: &mut Parser) -> Vec<String> {
        let mut res = Vec::new();
        let mut it = parser.input_lines.iter();

        if parser.code_blocks.code_blocks.len() > 0 {
            let block = parser.code_blocks.code_blocks.pop_front().unwrap();

            it.nth(block.len());

            res.push(format!(
                "<code>\n<pre>\n{}\n</pre>\n</code>\n",
                block.join("\n")
            ));
        }
        res
    }
}

impl Parsable for UnorderedLists {
    fn parse(parser: &mut Parser) -> Vec<String> {
        let mut res = Vec::new();
        let mut it = parser.input_lines.iter();

        if parser.unordered_lists.unordered_lists.len() > 0 {
            let block = parser.unordered_lists.unordered_lists.pop_front().unwrap();

            it.nth(block.len());

            res.push(format!(
                "<ul>{}\n</ul>\n",
                block
                    .iter()
                    .map(|s| format!("\n<li>\n{}\n</li>", s))
                    .collect::<String>()
            ));
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
            code_blocks: CodeBlocks { code_blocks },
            unordered_lists: UnorderedLists { unordered_lists },
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

        consecutives.push(unordered_lists_indices);

        let unordered_lists_blocks: VecDeque<_> = consecutives
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

        unordered_lists_blocks
    }

    pub fn parse(&mut self, token: Token) {
        match token {
            Token::CodeBlock => {
                let mut res = CodeBlocks::parse(self);
                self.output_lines.append(&mut res);
            }
            Token::UnorderedList => {
                let mut res = UnorderedLists::parse(self);
                self.output_lines.append(&mut res);
            }
            _ => {}
        }
    }
}

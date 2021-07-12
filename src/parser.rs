use crate::parsable::code_blocks::CodeBlocks;
use crate::parsable::heading::Heading;
use crate::parsable::paragraph::Paragraph;
use crate::parsable::unordered_lists::UnorderedLists;
use crate::parsable::Parsable;

#[derive(Debug)]
pub enum Token {
    Heading,
    CodeBlock,
    UnorderedList,
    General,
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

pub struct ParserState {
    pub input_lines: Vec<String>,
    pub token: Token,
    pub current_line: (usize, String),
    pub code_blocks: Vec<usize>,
    pub next_ul: usize,
}

pub struct Parser;

impl Parser {
    pub fn parse(mut state: &mut ParserState) -> String {
        match state.token {
            Token::Heading => Heading::parse(state),
            Token::CodeBlock => CodeBlocks::parse(&mut state),
            Token::UnorderedList => UnorderedLists::parse(&mut state),
            Token::General => Paragraph::parse(&mut state),
        }
    }

    pub fn parse_bold_italic(line: String) -> String {
        let mut res = crate::util::util::replace_html_tag_n(line, "**", "strong");
        res = crate::util::util::replace_html_tag_n(res, "__", "strong");
        res = crate::util::util::replace_html_tag_n(res, "*", "italic");
        res = crate::util::util::replace_html_tag_n(res, "_", "italic");

        res
    }
}

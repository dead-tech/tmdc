use crate::parsable::code_blocks::CodeBlocks;

use super::{Parsable, Parser, ParserState};

pub struct Paragraph;

impl Parsable<String> for Paragraph {
    fn parse(state: &mut ParserState) -> String {
        if state.current_line.1.is_empty() || CodeBlocks::is_code_block(state) {
            return String::from("");
        }
        return format!(
            "<p>{}</p>\n",
            Parser::parse_bold_italic(state.current_line.1.clone())
        );
    }
}

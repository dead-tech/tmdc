use super::{Parsable, Parser, ParserState};

pub struct Heading;

impl Parsable<String> for Heading {
    fn parse(parser: &mut ParserState) -> String {
        let heading_number = parser.current_line.1.matches("#").count();
        let content = parser.current_line.1[heading_number + 1..].to_string();
        format!(
            "<h{}>{}</h{}>\n",
            heading_number,
            Parser::parse_bold_italic(content),
            heading_number
        )
    }
}

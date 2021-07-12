use super::{Parsable, Parser, ParserState};

pub struct UnorderedLists;

impl Parsable<String> for UnorderedLists {
    fn parse(state: &mut ParserState) -> String {
        if state.next_ul == 0 {
            let token_indexes: Vec<_> = state
                .input_lines
                .iter()
                .enumerate()
                .skip_while(|(line_number, _)| *line_number != state.current_line.0)
                .take_while(|(_, line)| line.starts_with("*"))
                .collect();

            if token_indexes.len() < 1 {
                return String::from("");
            }

            let block: Vec<_> = state
                .input_lines
                .iter()
                .enumerate()
                .skip_while(|(line_number, _)| *line_number != state.current_line.0)
                // .inspect(|(line_number, line)| println!("{:?}", line_number))
                .take_while(|(line_number, line)| {
                    line_number.checked_sub(1).unwrap_or(*line_number)
                        < token_indexes.last().unwrap().0
                        && line.starts_with("*")
                })
                .map(|(_, line)| Parser::parse_bold_italic(line.replacen("* ", "", 1).to_string()))
                .collect();

            state.next_ul = block.len() - 1;

            return format!(
                "<ul>\n{}\n</ul>\n",
                block
                    .iter()
                    .map(|line| format!("\t<li>{}</li>\n", line))
                    .collect::<String>()
            );
        } else {
            state.next_ul -= 1;
        }
        String::from("")
    }
}

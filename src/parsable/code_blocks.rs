use super::{Parsable, ParserState};
// use std::collections::VecDeque;

pub struct CodeBlocks;

impl CodeBlocks {
    pub fn is_code_block(state: &mut ParserState) -> bool {
        state.code_blocks.contains(&state.current_line.0)
    }
}

impl Parsable<String> for CodeBlocks {
    // TODO: not finished
    fn parse(state: &mut ParserState) -> String {
        let text_between_tokens: Vec<_> = state
            .input_lines
            .iter()
            .enumerate()
            .skip_while(|(line_number, _)| *line_number != state.current_line.0)
            .skip(1)
            .take_while(|(_, line)| !line.starts_with("```") && !line.is_empty())
            .collect();

        if text_between_tokens.len() < 1 {
            return String::from("");
        }

        for (idx, _) in &text_between_tokens {
            state.code_blocks.push(*idx);
        }

        let block: Vec<_> = state
            .input_lines
            .iter()
            .enumerate()
            .skip_while(|(line_number, _)| *line_number != state.current_line.0)
            .skip(1)
            .take_while(|(line_number, line)| {
                *line_number - 1 < text_between_tokens.last().unwrap().0
                    && **line != "```"
                    && !line.is_empty()
            })
            .map(|(_, line)| line.to_string())
            .collect();

        if block.len() > 0 && !block.contains(&String::from("")) {
            return format!("<code>\n\t<pre>\n{}\n\t</pre>\n</code>\n", block.join("\n"));
        }

        String::from("")
    }
}

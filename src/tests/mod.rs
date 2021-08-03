#[cfg(test)]
#[test]
fn test_regular_paragraph() {
    use std::vec;

    use crate::parsable::paragraph::Paragraph;
    use crate::parsable::Parsable;
    use crate::parser::ParserState;
    use crate::parser::Token;

    let mut state = ParserState {
        input_lines: vec![],
        token: Token::General,
        current_line: (1, String::from("lorem ipsum dolor sit amet")),
        code_blocks: vec![],
        next_ul: 0,
    };

    let res = Paragraph::parse(&mut state);

    assert_eq!(res, "<p>lorem ipsum dolor sit amet</p>\n");
}

#[test]
fn test_bold_and_italic_paragraph() {
    use std::vec;

    use crate::parsable::paragraph::Paragraph;
    use crate::parsable::Parsable;
    use crate::parser::ParserState;
    use crate::parser::Token;

    let mut state = ParserState {
        input_lines: vec![],
        token: Token::General,
        current_line: (1, String::from("lorem **ipsum** dolor _sit_ amet")),
        code_blocks: vec![],
        next_ul: 0,
    };

    let res = Paragraph::parse(&mut state);

    assert_eq!(
        res,
        "<p>lorem <strong>ipsum</strong> dolor <italic>sit</italic> amet</p>\n"
    );
}

#[test]
fn test_code_block_paragraph() {
    use std::vec;

    use crate::parsable::paragraph::Paragraph;
    use crate::parsable::Parsable;
    use crate::parser::ParserState;
    use crate::parser::Token;

    let mut state = ParserState {
        input_lines: vec![],
        token: Token::General,
        current_line: (14, String::from("lorem **ipsum** dolor _sit_ amet")),
        code_blocks: vec![14],
        next_ul: 0,
    };

    let res = Paragraph::parse(&mut state);

    assert_eq!(res, "");
}

#[test]
fn test_heading() {
    use std::vec;

    use crate::parsable::heading::Heading;
    use crate::parsable::Parsable;
    use crate::parser::ParserState;
    use crate::parser::Token;

    let mut state = ParserState {
        input_lines: vec![],
        token: Token::General,
        current_line: (14, String::from("# Lorem ipsum dolor sit amet")),
        code_blocks: vec![14],
        next_ul: 0,
    };

    let h1 = Heading::parse(&mut state);
    state.current_line = (14, String::from("## Lorem ipsum dolor sit amet"));
    let h2 = Heading::parse(&mut state);
    state.current_line = (14, String::from("### Lorem ipsum dolor sit amet"));
    let h3 = Heading::parse(&mut state);
    state.current_line = (14, String::from("#### Lorem ipsum dolor sit amet"));
    let h4 = Heading::parse(&mut state);
    state.current_line = (14, String::from("##### Lorem ipsum dolor sit amet"));
    let h5 = Heading::parse(&mut state);
    state.current_line = (14, String::from("###### Lorem ipsum dolor sit amet"));
    let h6 = Heading::parse(&mut state);

    assert_eq!(h1, "<h1>Lorem ipsum dolor sit amet</h1>\n");
    assert_eq!(h2, "<h2>Lorem ipsum dolor sit amet</h2>\n");
    assert_eq!(h3, "<h3>Lorem ipsum dolor sit amet</h3>\n");
    assert_eq!(h4, "<h4>Lorem ipsum dolor sit amet</h4>\n");
    assert_eq!(h5, "<h5>Lorem ipsum dolor sit amet</h5>\n");
    assert_eq!(h6, "<h6>Lorem ipsum dolor sit amet</h6>\n");
}

#[test]
fn test_bold_and_italic_heading() {
    use std::vec;

    use crate::parsable::heading::Heading;
    use crate::parsable::Parsable;
    use crate::parser::ParserState;
    use crate::parser::Token;

    let mut state = ParserState {
        input_lines: vec![],
        token: Token::General,
        current_line: (14, String::from("# _Lorem_ ipsum dolor sit amet")),
        code_blocks: vec![14],
        next_ul: 0,
    };

    let h1 = Heading::parse(&mut state);
    state.current_line = (14, String::from("## Lorem **ipsum** dolor sit amet"));
    let h2 = Heading::parse(&mut state);
    state.current_line = (14, String::from("### __Lorem__ ipsum dolor sit amet"));
    let h3 = Heading::parse(&mut state);
    state.current_line = (14, String::from("#### Lorem ipsum *dolor* sit amet"));
    let h4 = Heading::parse(&mut state);
    state.current_line = (14, String::from("##### Lorem ipsum dolor **sit** amet"));
    let h5 = Heading::parse(&mut state);
    state.current_line = (14, String::from("###### Lorem ipsum dolor sit __amet__"));
    let h6 = Heading::parse(&mut state);

    assert_eq!(h1, "<h1><italic>Lorem</italic> ipsum dolor sit amet</h1>\n");
    assert_eq!(h2, "<h2>Lorem <strong>ipsum</strong> dolor sit amet</h2>\n");
    assert_eq!(h3, "<h3><strong>Lorem</strong> ipsum dolor sit amet</h3>\n");
    assert_eq!(h4, "<h4>Lorem ipsum <italic>dolor</italic> sit amet</h4>\n");
    assert_eq!(h5, "<h5>Lorem ipsum dolor <strong>sit</strong> amet</h5>\n");
    assert_eq!(h6, "<h6>Lorem ipsum dolor sit <strong>amet</strong></h6>\n");
}

#[test]
fn test_code_blocks() {
    use std::vec;

    use crate::parsable::code_blocks::CodeBlocks;
    use crate::parsable::Parsable;
    use crate::parser::ParserState;
    use crate::parser::Token;

    let mut state = ParserState {
        input_lines: vec![
            String::from("```"),
            String::from("i hope"),
            String::from("the test"),
            String::from("will pass"),
            String::from("```"),
        ],
        token: Token::General,
        current_line: (0, String::from("```")),
        code_blocks: vec![],
        next_ul: 0,
    };

    let res = CodeBlocks::parse(&mut state);

    assert_eq!(
        res,
        "<code>\n\t<pre>\ni hope\nthe test\nwill pass\n\t</pre>\n</code>\n"
    );
}

#[test]
fn test_no_code_block() {
    use std::vec;

    use crate::parsable::code_blocks::CodeBlocks;
    use crate::parsable::Parsable;
    use crate::parser::ParserState;
    use crate::parser::Token;

    let mut state = ParserState {
        input_lines: vec![String::from("no codeblocks")],
        token: Token::General,
        current_line: (0, String::from("no codeblocks")),
        code_blocks: vec![],
        next_ul: 0,
    };

    let res = CodeBlocks::parse(&mut state);

    assert_eq!(res, "");
}

#[test]
fn test_unordered_list() {
    use std::vec;

    use crate::parsable::unordered_lists::UnorderedLists;
    use crate::parsable::Parsable;
    use crate::parser::ParserState;
    use crate::parser::Token;

    let mut state = ParserState {
        input_lines: vec![
            String::from("* finish writing tests"),
            String::from("* push to github"),
            String::from("* update ci"),
        ],
        token: Token::General,
        current_line: (0, String::from("* finish writing tests")),
        code_blocks: vec![],
        next_ul: 0,
    };

    let res = UnorderedLists::parse(&mut state);

    assert_eq!(
        res,
        "<ul>\n\t<li>finish writing tests</li>\n\t<li>push to github</li>\n\t<li>update ci</li>\n\n</ul>\n"
    );
}

#[test]
fn test_bold_and_italic_unordered_list() {
    use std::vec;

    use crate::parsable::unordered_lists::UnorderedLists;
    use crate::parsable::Parsable;
    use crate::parser::ParserState;
    use crate::parser::Token;

    let mut state = ParserState {
        input_lines: vec![
            String::from("* __finish writing__ tests"),
            String::from("* push **to** github"),
            String::from("* update _ci_"),
        ],
        token: Token::General,
        current_line: (0, String::from("* finish writing tests")),
        code_blocks: vec![],
        next_ul: 0,
    };

    let res = UnorderedLists::parse(&mut state);

    assert_eq!(
        res,
        "<ul>\n\t<li><strong>finish writing</strong> tests</li>\n\t<li>push <strong>to</strong> github</li>\n\t<li>update <italic>ci</italic></li>\n\n</ul>\n"
    );
}

#[test]
fn test_no_unordered_list() {
    use std::vec;

    use crate::parsable::unordered_lists::UnorderedLists;
    use crate::parsable::Parsable;
    use crate::parser::ParserState;
    use crate::parser::Token;

    let mut state = ParserState {
        input_lines: vec![String::from("no unoredered list")],
        token: Token::General,
        current_line: (0, String::from("no unordered list")),
        code_blocks: vec![],
        next_ul: 0,
    };

    let res = UnorderedLists::parse(&mut state);

    assert_eq!(res, "");
}

#[test]
fn test_line_breaks() {
    use std::vec;

    use crate::parser::Parser;
    use crate::parser::ParserState;
    use crate::parser::Token;

    let mut state = ParserState {
        input_lines: vec![String::from("line break here:  ")],
        token: Token::General,
        current_line: (0, String::from("line break here:  ")),
        code_blocks: vec![],
        next_ul: 0,
    };

    let res = Parser::parse(&mut state);

    assert_eq!(res, "<p>line break here:  <br /></p>\n")
}

#[test]
fn test_no_line_breaks() {
    use std::vec;

    use crate::parser::Parser;
    use crate::parser::ParserState;
    use crate::parser::Token;

    let mut state = ParserState {
        input_lines: vec![String::from("no line break here")],
        token: Token::General,
        current_line: (0, String::from("no line break here")),
        code_blocks: vec![],
        next_ul: 0,
    };

    let res = Parser::parse(&mut state);

    assert_eq!(res, "<p>no line break here</p>\n")
}

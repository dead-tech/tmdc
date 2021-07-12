pub mod code_blocks;
pub mod heading;
pub mod paragraph;
pub mod unordered_lists;

use super::{Parser, ParserState};

pub trait Parsable<T> {
    fn parse(state: &mut ParserState) -> T;
}

//! # Demonstration
//!
//! The following examples demonstrate the basic ideal of the ACE parser:
//!
//! - [Mini Ace](../examples/mini_ace.rs)
//!
//! # References
//!
//! - https://eyalkalderon.com/blog/nom-error-recovery/

use nom::character::complete::multispace1;
use nom::combinator::eof;
use nom::Parser;
use nom::{branch::alt, bytes::complete::take_while, combinator::opt, multi::many0, IResult};

use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::{tag, tag_no_case};
use nom_supreme::ParserExt;

use nom_locate::LocatedSpan;

pub trait AceParseTree: std::fmt::Debug {
    fn range(&self) -> Range;
    fn query(&self, pos: Position) -> Result<&dyn AceParseTree, ()>;
    fn show_completions(&self) -> Vec<&'static str> {
        vec![]
    }
}

/// Position in a text document expressed as zero-based line and character offset.
/// A position is between two characters like an 'insert' cursor in a editor.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Default)]
pub struct Position {
    /// Line position in a document (zero-based).
    pub line: u32,
    /// Character offset on a line in a document (zero-based). The meaning of this
    /// offset is determined by the negotiated `PositionEncodingKind`.
    ///
    /// If the character value is greater than the line length it defaults back
    /// to the line length.
    pub character: u32,
}

impl Position {
    pub fn new(line: u32, character: u32) -> Self {
        Position { line, character }
    }
}

impl From<LocatedSpan<&str>> for Position {
    fn from(p: LocatedSpan<&str>) -> Self {
        let line = p.location_line() - 1;
        let character = p.get_column() as u32 - 1;
        Position { line, character }
    }
}

/// A range in a text document expressed as (zero-based) start and end positions.
/// A range is comparable to a selection in an editor. Therefore the end position is exclusive.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub struct Range {
    /// The range's start position.
    pub start: Position,
    /// The range's end position.
    pub end: Position,
}

impl Range {
    pub fn contains(&self, pos: Position) -> bool {
        self.start <= pos && pos < self.end
    }
}

#[derive(Debug)]
pub struct Ace<'a> {
    pub statements: Vec<Statement<'a>>,
}

pub fn ace<'a>(input: &'a str) -> Ace<'a> {
    let (_, statements) = many0(statement)
        .parse(input.into())
        .expect("parser should not fail.");
    Ace { statements }
}

impl Ace<'_> {
    pub fn query(&self, pos: Position) -> Result<&dyn AceParseTree, ()> {
        self.statements
            .iter()
            .find_map(|s| {
                if s.range().contains(pos) {
                    s.query(pos).ok()
                } else {
                    None
                }
            })
            .ok_or(())
    }
}

#[derive(Debug)]
pub enum Statement<'a> {
    Cpu(statement::Cpu<'a>),
    Config(statement::Config<'a>),
}

pub fn statement<'a>(
    input: LocatedSpan<&'a str>,
) -> IResult<LocatedSpan<&'a str>, Statement<'a>, ErrorTree<LocatedSpan<&'a str>>> {
    alt((
        statement::cpu.map(Statement::Cpu),
        statement::config.map(Statement::Config),
    ))(input)
}

impl AceParseTree for Statement<'_> {
    fn range(&self) -> Range {
        match self {
            Statement::Cpu(cpu) => cpu.range(),
            Statement::Config(config) => config.range(),
        }
    }
    fn query(&self, pos: Position) -> Result<&dyn AceParseTree, ()> {
        match self {
            Statement::Cpu(cpu) => cpu.query(pos),
            Statement::Config(config) => config.query(pos),
        }
    }
}

mod attribute;
mod statement;
mod token;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ace() {
        let input = r#"
            cpu {
                name = NX45V;
                vlen = 128;
            };
        "#
        .trim();

        let parse_tree = ace(input);
        println!("{:#?}", parse_tree);
    }
}

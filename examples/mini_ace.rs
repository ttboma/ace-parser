//! ace                      <- statement*
//! statement                <- statement::cpu
//! statement::cpu           <- CPU LEFT_BRACE (statement::cpu_attribute)* RIGHT_BRACE SEMICOLON
//! statement::cpu_attribute <- attribute::name / attribute::vlen
//! attribute::name          <- NAME EQUAL IDENTIFIER SEMICOLON
//! attribute::vlen          <- VLEN EQUAL number SEMICOLON
//! number                   <- HEX_NUMBER / DEC_NUMBER
//!
//! CPU                      <- 'cpu' spacing
//! LEFT_BRACE               <- '{' spacing
//! RIGHT_BRACE              <- '}' spacing
//! SEMICOLON                <- ';' spacing
//! EQUAL                    <- '=' spacing
//! NAME                     <- 'name' spacing
//! VLEN                     <- 'vlen' spacing
//! IDENTIFIER               <- [a-zA-Z_][a-zA-Z0-9_]* spacing
//! HEX_NUMBER               <- '0x' [0-9a-fA-F]+ spacing
//! DEC_NUMBER               <- [1-9][0-9]* spacing
//!
//! spacing                  <- eof / (comment / multispace1)*
//! comment                  <- '//' (!end_of_line .)* end_of_line?
//! multispace1              <- (' ' / '\t' / end_of_line)+
//! end_of_line              <- '\r\n' / '\n' / '\r'
//! eof                      <- !.

use nom::character::complete::multispace1;
use nom::combinator::eof;
use nom::Parser;
use nom::{branch::alt, bytes::complete::take_while, combinator::opt, multi::many0, IResult};

use nom_locate::LocatedSpan;

use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::{tag, tag_no_case};
use nom_supreme::ParserExt;

fn main() {
    unimplemented!()
}

mod grammar {
    use super::*;
    use getset::Getters;

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

    pub fn semicolon<'a>(
    ) -> impl Parser<LocatedSpan<&'a str>, Semicolon<'a>, ErrorTree<LocatedSpan<&'a str>>> {
        SemicolonParser {
            label_completion: None,
        }
    }

    #[derive(Debug)]
    pub struct Semicolon<'a> {
        range: Range,
        label_completion: Option<marker::LabelCompletion>,
        pub token: LocatedSpan<&'a str>,
        pub spacing: Vec<LocatedSpan<&'a str>>,
    }

    pub struct SemicolonParser {
        label_completion: Option<marker::LabelCompletion>,
    }

    impl SemicolonParser {
        pub fn set_label_completion(
            &mut self,
            label_completion: marker::LabelCompletion,
        ) -> &mut Self {
            self.label_completion = Some(label_completion);
            self
        }
    }

    impl<'a, 'b> Parser<LocatedSpan<&'a str>, Semicolon<'a>, ErrorTree<LocatedSpan<&'a str>>>
        for SemicolonParser
    {
        fn parse(
            &mut self,
            input: LocatedSpan<&'a str>,
        ) -> IResult<LocatedSpan<&'a str>, Semicolon<'a>, ErrorTree<LocatedSpan<&'a str>>> {
            let (s, token) = (tag(";")).parse(input)?;
            let (s, spacing) = spacing(s)?;
            Ok((
                s,
                Semicolon {
                    range: Range {
                        start: input.into(),
                        end: s.into(),
                    },
                    label_completion: self.label_completion,
                    token,
                    spacing,
                },
            ))
        }
    }

    pub fn spacing(
        input: LocatedSpan<&str>,
    ) -> IResult<LocatedSpan<&str>, Vec<LocatedSpan<&str>>, ErrorTree<LocatedSpan<&str>>> {
        eof.map(|eof| vec![eof])
            .or(many0(alt((comment, multispace1))))
            .parse(input)
    }

    fn comment(
        input: LocatedSpan<&str>,
    ) -> IResult<LocatedSpan<&str>, LocatedSpan<&str>, ErrorTree<LocatedSpan<&str>>> {
        tag("//")
            .and(take_while(|c| c != '\n' && c != '\r'))
            .and(opt(end_of_line))
            .recognize()
            .parse(input)
    }

    fn end_of_line(
        input: LocatedSpan<&str>,
    ) -> IResult<LocatedSpan<&str>, LocatedSpan<&str>, ErrorTree<LocatedSpan<&str>>> {
        alt((tag("\r\n"), tag("\r"), tag("\n")))(input)
    }
}

mod marker {

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum LabelCompletion {
        None,
        Statement,
        Attribute,
    }
}

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
use nom::{
    branch::alt,
    bytes::complete::take_while,
    combinator::opt,
    multi::many0,
    IResult,
};

use nom_locate::LocatedSpan;

use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::{tag, tag_no_case};
use nom_supreme::ParserExt;


#[derive(Debug)]
struct Ace<'a> {
    pub statements: Vec<Statement<'a>>,
}

fn ace<'a>(
    input: LocatedSpan<&'a str>,
) -> IResult<LocatedSpan<&'a str>, Ace<'a>, ErrorTree<LocatedSpan<&'a str>>> {
    let (input, statements) = many0(statement)(input)?;
    Ok((input, Ace { statements }))
}

#[derive(Debug)]
enum Statement<'a> {
    Cpu(statement::Cpu<'a>),
    Config(statement::Config<'a>),
}

fn statement<'a>(
    input: LocatedSpan<&'a str>,
) -> IResult<LocatedSpan<&'a str>, Statement<'a>, ErrorTree<LocatedSpan<&'a str>>> {
    alt((
        statement::cpu.map(Statement::Cpu),
        statement::config.map(Statement::Config),
    ))(input)
}

mod statement {
    use super::*;

    #[derive(Debug)]
    pub struct Cpu<'a> {
        pub pragma: token::Token<LocatedSpan<&'a str>>,
        pub left_brace: token::Token<LocatedSpan<&'a str>>,
        pub attributes: Vec<CpuAttribute<'a>>,
        pub right_brace: token::Token<LocatedSpan<&'a str>>,
        pub semicolon: token::Token<LocatedSpan<&'a str>>,
    }

    pub fn cpu<'a>(
        input: LocatedSpan<&'a str>,
    ) -> IResult<LocatedSpan<&'a str>, Cpu<'a>, ErrorTree<LocatedSpan<&'a str>>> {
        let (input, pragma) = token::cpu(input)?;
        let (input, left_brace) = token::left_brace(input)?;
        let (input, attributes) = many0(cpu_attribute)(input)?;
        let (input, right_brace) = token::right_brace(input)?;
        let (input, semicolon) = token::semicolon(input)?;
        Ok((
            input,
            Cpu {
                pragma,
                left_brace,
                attributes,
                right_brace,
                semicolon,
            },
        ))
    }

    #[derive(Debug)]
    pub enum CpuAttribute<'a> {
        Name(attribute::Name<'a>),
        Vlen(attribute::Vlen<'a>),
    }

    fn cpu_attribute<'a>(
        input: LocatedSpan<&'a str>,
    ) -> IResult<LocatedSpan<&'a str>, CpuAttribute<'a>, ErrorTree<LocatedSpan<&'a str>>> {
        alt((
            attribute::name.map(CpuAttribute::Name),
            attribute::vlen.map(CpuAttribute::Vlen),
        ))(input)
    }

    #[derive(Debug)]
    pub struct Config<'a> {
        pub pragma: token::Token<LocatedSpan<&'a str>>,
        pub left_brace: token::Token<LocatedSpan<&'a str>>,
        pub attributes: Vec<ConfigAttribute<'a>>,
        pub right_brace: token::Token<LocatedSpan<&'a str>>,
        pub semicolon: token::Token<LocatedSpan<&'a str>>,
    }

    pub fn config<'a>(
        input: LocatedSpan<&'a str>,
    ) -> IResult<LocatedSpan<&'a str>, Config<'a>, ErrorTree<LocatedSpan<&'a str>>> {
        let (input, pragma) = token::config(input)?;
        let (input, left_brace) = token::left_brace(input)?;
        let (input, attributes) = many0(config_attribute)(input)?;
        let (input, right_brace) = token::right_brace(input)?;
        let (input, semicolon) = token::semicolon(input)?;
        Ok((
            input,
            Config {
                pragma,
                left_brace,
                attributes,
                right_brace,
                semicolon,
            },
        ))
    }

    #[derive(Debug)]
    pub enum ConfigAttribute<'a> {
        TimeoutCycle(attribute::TimeoutCycle<'a>),
    }

    fn config_attribute<'a>(
        input: LocatedSpan<&'a str>,
    ) -> IResult<LocatedSpan<&'a str>, ConfigAttribute<'a>, ErrorTree<LocatedSpan<&'a str>>> {
        attribute::timeout_cycle
            .map(ConfigAttribute::TimeoutCycle)
            .parse(input)
    }
}

mod attribute {

    use super::*;

    #[derive(Debug)]
    pub struct Name<'a> {
        pub pragma: token::Token<LocatedSpan<&'a str>>,
        pub equal: token::Token<LocatedSpan<&'a str>>,
        pub identifier: token::Token<LocatedSpan<&'a str>>,
        pub semicolon: token::Token<LocatedSpan<&'a str>>,
    }

    pub fn name<'a>(
        input: LocatedSpan<&'a str>,
    ) -> IResult<LocatedSpan<&'a str>, Name<'a>, ErrorTree<LocatedSpan<&'a str>>> {
        let (input, pragma) = token::name(input)?;
        let (input, equal) = token::equal(input)?;
        let (input, identifier) = token::identifier(input)?;
        let (input, semicolon) = token::semicolon(input)?;
        Ok((
            input,
            Name {
                pragma,
                equal,
                identifier,
                semicolon,
            },
        ))
    }

    #[derive(Debug)]
    pub struct Vlen<'a> {
        pub pragma: token::Token<LocatedSpan<&'a str>>,
        pub equal: token::Token<LocatedSpan<&'a str>>,
        pub length: Number<'a>,
        pub semicolon: token::Token<LocatedSpan<&'a str>>,
    }

    pub fn vlen<'a>(
        input: LocatedSpan<&'a str>,
    ) -> IResult<LocatedSpan<&'a str>, Vlen<'a>, ErrorTree<LocatedSpan<&'a str>>> {
        let (input, pragma) = token::vlen(input)?;
        let (input, equal) = token::equal(input)?;
        let (input, length) = number(input)?;
        let (input, semicolon) = token::semicolon(input)?;
        Ok((
            input,
            Vlen {
                pragma,
                equal,
                length,
                semicolon,
            },
        ))
    }

    #[derive(Debug)]
    pub enum Number<'a> {
        HexNumber(token::Token<LocatedSpan<&'a str>>),
        DecNumber(token::Token<LocatedSpan<&'a str>>),
    }

    fn number<'a>(
        input: LocatedSpan<&'a str>,
    ) -> IResult<LocatedSpan<&'a str>, Number<'a>, ErrorTree<LocatedSpan<&'a str>>> {
        alt((
            token::hex_number.map(Number::HexNumber),
            token::dec_number.map(Number::DecNumber),
        ))(input)
    }

    #[derive(Debug)]
    pub struct TimeoutCycle<'a> {
        pub pragma: token::Token<LocatedSpan<&'a str>>,
        pub equal: token::Token<LocatedSpan<&'a str>>,
        pub length: Number<'a>,
        pub semicolon: token::Token<LocatedSpan<&'a str>>,
    }

    pub fn timeout_cycle<'a>(
        input: LocatedSpan<&'a str>,
    ) -> IResult<LocatedSpan<&'a str>, TimeoutCycle<'a>, ErrorTree<LocatedSpan<&'a str>>> {
        let (input, pragma) = token::timeout_cycle(input)?;
        let (input, equal) = token::equal(input)?;
        let (input, length) = number(input)?;
        let (input, semicolon) = token::semicolon(input)?;
        Ok((
            input,
            TimeoutCycle {
                pragma,
                equal,
                length,
                semicolon,
            },
        ))
    }
}

mod token {
    use super::*;

    #[derive(Debug)]
    pub struct Token<I> {
        pub token: I,
        pub spacing: Vec<I>,
    }

    fn token<'a, F>(
        mut f: F,
    ) -> impl FnMut(
        LocatedSpan<&'a str>,
    ) -> IResult<
        LocatedSpan<&'a str>,
        Token<LocatedSpan<&'a str>>,
        ErrorTree<LocatedSpan<&'a str>>,
    >
    where
        F: Parser<LocatedSpan<&'a str>, LocatedSpan<&'a str>, ErrorTree<LocatedSpan<&'a str>>>,
    {
        move |input: LocatedSpan<&'a str>| {
            let (input, token) = f.parse(input)?;
            let (input, spacing) = spacing(input)?;
            Ok((input, Token { token, spacing }))
        }
    }

    pub fn cpu(
        input: LocatedSpan<&str>,
    ) -> IResult<LocatedSpan<&str>, Token<LocatedSpan<&str>>, ErrorTree<LocatedSpan<&str>>> {
        token(tag("cpu"))(input)
    }

    pub fn config(
        input: LocatedSpan<&str>,
    ) -> IResult<LocatedSpan<&str>, Token<LocatedSpan<&str>>, ErrorTree<LocatedSpan<&str>>> {
        token(tag("config"))(input)
    }

    pub fn timeout_cycle(
        input: LocatedSpan<&str>,
    ) -> IResult<LocatedSpan<&str>, Token<LocatedSpan<&str>>, ErrorTree<LocatedSpan<&str>>> {
        token(tag("timeout_cycle"))(input)
    }

    pub fn left_brace(
        input: LocatedSpan<&str>,
    ) -> IResult<LocatedSpan<&str>, Token<LocatedSpan<&str>>, ErrorTree<LocatedSpan<&str>>> {
        token(tag("{"))(input)
    }

    pub fn right_brace(
        input: LocatedSpan<&str>,
    ) -> IResult<LocatedSpan<&str>, Token<LocatedSpan<&str>>, ErrorTree<LocatedSpan<&str>>> {
        token(tag("}"))(input)
    }

    pub fn equal(
        input: LocatedSpan<&str>,
    ) -> IResult<LocatedSpan<&str>, Token<LocatedSpan<&str>>, ErrorTree<LocatedSpan<&str>>> {
        token(tag("="))(input)
    }

    pub fn semicolon(
        input: LocatedSpan<&str>,
    ) -> IResult<LocatedSpan<&str>, Token<LocatedSpan<&str>>, ErrorTree<LocatedSpan<&str>>> {
        token(tag(";"))(input)
    }

    pub fn name(
        input: LocatedSpan<&str>,
    ) -> IResult<LocatedSpan<&str>, Token<LocatedSpan<&str>>, ErrorTree<LocatedSpan<&str>>> {
        token(tag("name"))(input)
    }

    pub fn vlen(
        input: LocatedSpan<&str>,
    ) -> IResult<LocatedSpan<&str>, Token<LocatedSpan<&str>>, ErrorTree<LocatedSpan<&str>>> {
        token(tag("vlen"))(input)
    }

    pub fn identifier(
        input: LocatedSpan<&str>,
    ) -> IResult<LocatedSpan<&str>, Token<LocatedSpan<&str>>, ErrorTree<LocatedSpan<&str>>> {
        token(
            take_while(|c: char| c.is_alphabetic() || c == '_')
                .and(take_while(|c: char| c.is_alphanumeric() || c == '_'))
                .recognize(),
        )(input)
    }

    pub fn hex_number(
        input: LocatedSpan<&str>,
    ) -> IResult<LocatedSpan<&str>, Token<LocatedSpan<&str>>, ErrorTree<LocatedSpan<&str>>> {
        token(
            tag_no_case("0x")
                .and(take_while(|c: char| c.is_ascii_hexdigit()))
                .recognize(),
        )(input)
    }

    pub fn dec_number(
        input: LocatedSpan<&str>,
    ) -> IResult<LocatedSpan<&str>, Token<LocatedSpan<&str>>, ErrorTree<LocatedSpan<&str>>> {
        token(
            take_while(|c: char| c.is_ascii_digit())
                .verify(|s: &LocatedSpan<&str>| !s.starts_with('0'))
                .recognize(),
        )(input)
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

pub trait ParserRecoveryExt<I, O, E>: Parser<I, O, E> + Sized {
    #[inline]
    fn recover<G, L>(self, g: G, l: L) -> Recovery<Self, G, L>
    where
        G: Parser<I, O, E>,
        Self: core::marker::Sized,
    {
        Recovery { f: self, g, l }
    }
}

pub struct Recovery<F, G, L> {
    f: F,
    g: G,
    l: L,
}

impl<'a, I, O, E, F, G, L> Parser<I, O, E> for Recovery<F, G, L>
where
    F: Parser<I, O, E>,
    G: Parser<I, O, E>,
    L: Fn(E) -> (I, O),
{
    fn parse(&mut self, i: I) -> IResult<I, O, E> {
        match self.f.parse(i) {
            Ok(v) => Ok(v),
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Ok((self.l)(e)),
            Err(nom::Err::Incomplete(n)) => Err(nom::Err::Incomplete(n)),
        }
    }
}

fn main() {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use cool_asserts::assert_matches;
    use nom_supreme::error::GenericErrorTree;
    use token::{spacing, Token};

    #[test]
    fn spacing_never_fail() {
        spacing("".into()).unwrap();
        spacing("//".into()).unwrap();
        spacing("//hello".into()).unwrap();
        spacing("//\n".into()).unwrap();
        spacing("//hello\n".into()).unwrap();
        spacing("//hello\n \n\t\r".into()).unwrap();
        spacing(" \n\t\r".into()).unwrap();
        spacing(" \n\t\r//hello\n \n\t\r".into()).unwrap();
    }

    use miette::{IntoDiagnostic, Result};

    #[test]
    fn test_token() {
        let input = "cpu  \n\t\r// some comment here..\n second line here..\n\t\r".into();
        let (input, cpu) = token::cpu(input).unwrap();
        assert_eq!(*input.fragment(), "second line here..\n\t\r");
        assert_matches!(cpu, Token { token, spacing } => {
            assert_eq!(*token.fragment(), "cpu");
            assert_eq!(spacing.iter().map(|span|*span.fragment()).collect::<Vec<_>>().join(""), "  \n\t\r// some comment here..\n ");
        });

        let input = "cpuhello\n \n\t\r".into();
        token::cpu(input).unwrap_err();
    }

    #[test]
    fn test_ace() {
        let input = r#"
            cpu {
                name = nx45v;
                vlen = 512;
            };
        "#
        .trim()
        .into();
        let (input, cpu) = statement::cpu(input).unwrap();
        dbg!(cpu);
    }
}

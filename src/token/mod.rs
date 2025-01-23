use super::*;

macro_rules! define_lexical_terminal {
    ($struct_name:ident, $parser:expr) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct $struct_name<'a> {
                range: Range,
                pub token: LocatedSpan<&'a str>,
                pub spacing: Vec<LocatedSpan<&'a str>>,
            }

            pub fn [<$struct_name:snake>](
                input: LocatedSpan<&str>,
            ) -> IResult<LocatedSpan<&str>, $struct_name, ErrorTree<LocatedSpan<&str>>> {
                let (s, token) = $parser.parse(input)?;
                let (s, spacing) = spacing(s)?;
                let range = Range {
                    start: input.into(),
                    end: s.into(),
                };
                Ok((
                    s,
                    $struct_name {
                        token,
                        spacing,
                        range,
                    },
                ))
            }

            impl AceParseTree for $struct_name<'_> {
                fn range(&self) -> Range {
                    self.range
                }
                fn query(&self, pos: Position) -> Result<&dyn AceParseTree, ()> {
                    if self.range().contains(pos) {
                        Ok(self)
                    } else {
                        Err(())
                    }
                }
            }
        }
    };
}

define_lexical_terminal!(Cpu, tag("cpu"));
define_lexical_terminal!(Config, tag("config"));
define_lexical_terminal!(TimeoutCycle, tag("timeout_cycle"));
define_lexical_terminal!(LeftBrace, tag("{"));
define_lexical_terminal!(RightBrace, tag("}"));
define_lexical_terminal!(Equal, tag("="));
define_lexical_terminal!(Semicolon, tag(";"));
define_lexical_terminal!(Name, tag("name"));
define_lexical_terminal!(Vlen, tag("vlen"));
define_lexical_terminal!(
    Identifier,
    take_while(|c: char| c.is_alphabetic() || c == '_')
        .and(take_while(|c: char| c.is_alphanumeric() || c == '_'))
        .recognize()
);
define_lexical_terminal!(
    HexNumber,
    tag_no_case("0x")
        .and(take_while(|c: char| c.is_ascii_hexdigit()))
        .recognize()
);
define_lexical_terminal!(
    DecNumber,
    take_while(|c: char| c.is_ascii_digit())
        .verify(|s: &LocatedSpan<&str>| !s.starts_with('0'))
        .recognize()
);

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

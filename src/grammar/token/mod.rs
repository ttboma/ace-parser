use super::*;

/// spacing <- eof / (comment / multispace1)*
/// eof     <- !.
pub fn spacing(
    input: LocatedSpan<&str>,
) -> IResult<LocatedSpan<&str>, Vec<LocatedSpan<&str>>, ErrorTree<LocatedSpan<&str>>> {
    eof.map(|eof| vec![eof])
        .or(many0(alt((comment, multispace1))))
        .parse(input)
}

/// comment     <- '//' (!end_of_line .)* end_of_line?
/// multispace1 <- (' ' / '\t' / end_of_line)+
fn comment(
    input: LocatedSpan<&str>,
) -> IResult<LocatedSpan<&str>, LocatedSpan<&str>, ErrorTree<LocatedSpan<&str>>> {
    tag("//")
        .and(take_while(|c| c != '\n' && c != '\r'))
        .and(opt(end_of_line))
        .recognize()
        .parse(input)
}

/// end_of_line <- '\r\n' / '\n' / '\r'
fn end_of_line(
    input: LocatedSpan<&str>,
) -> IResult<LocatedSpan<&str>, LocatedSpan<&str>, ErrorTree<LocatedSpan<&str>>> {
    alt((tag("\r\n"), tag("\r"), tag("\n")))(input)
}

macro_rules! define_lexical_terminal {
    ($struct_name:ident, $parser:expr) => {
        paste::paste! {
            #[doc = paste::paste! {
                concat!(
                    stringify!( [<$struct_name:upper>] ),
                    " <- '", stringify!( [<$struct_name:snake>] ), "' [`spacing`]\n\n",
                    "This function returns a parser for the `", stringify!($struct_name), "` lexical terminal.\n",
                    "\n### Example Usage\n",
                    "```\n",
                    "let parser = ", stringify!( [<$struct_name:snake>] ), "();\n",
                    "// Use parser.parse(input) to parse tokens\n",
                    "```\n"
                )
            }]
            pub fn [<$struct_name:snake>]<'a>(
            ) -> [<$struct_name Parser>] {
                [<$struct_name Parser>] {
                    label_completion: None,
                }
            }

            #[derive(Debug, Getters)]
            pub struct $struct_name<'a> {
                range: Range,
                label_completion: Option<marker::LabelCompletion>,
                #[getset(get = "pub")]
                token: LocatedSpan<&'a str>,
                #[getset(get = "pub")]
                spacing: Vec<LocatedSpan<&'a str>>,
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

                fn show_completions(&self) -> Vec<&'static str> {
                    match self.label_completion {
                        Some(m) => m.completion(),
                        None => vec![],
                    }
                }
            }

            #[derive(Getters)]
            pub struct [<$struct_name Parser>] {
                #[getset(set = "pub")]
                label_completion: Option<marker::LabelCompletion>,
            }

            impl [<$struct_name Parser>] {
                pub fn set_label_completion(
                    &mut self,
                    label_completion: marker::LabelCompletion,
                ) -> &mut Self {
                    self.label_completion = Some(label_completion);
                    self
                }
            }

            impl<'a, 'b> Parser<LocatedSpan<&'a str>, $struct_name<'a>, ErrorTree<LocatedSpan<&'a str>>>
                for [<$struct_name Parser>]
            {
                fn parse(
                    &mut self,
                    input: LocatedSpan<&'a str>,
                ) -> IResult<LocatedSpan<&'a str>, $struct_name<'a>, ErrorTree<LocatedSpan<&'a str>>> {
                    let (s, token) = $parser.parse(input)?;
                    let (s, spacing) = spacing(s)?;
                    Ok((
                        s,
                        $struct_name {
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
        }
    };
}

define_lexical_terminal!(Cpu, tag(keyword::statement::CPU));
define_lexical_terminal!(Config, tag(keyword::statement::CONFIG));
define_lexical_terminal!(TimeoutCycle, tag(keyword::attribute::TIMEOUT_CYCLE));
define_lexical_terminal!(Name, tag(keyword::attribute::NAME));
define_lexical_terminal!(Vlen, tag(keyword::attribute::VLEN));
define_lexical_terminal!(LeftBrace, tag(keyword::symbol::LEFT_BRACE));
define_lexical_terminal!(RightBrace, tag(keyword::symbol::RIGHT_BRACE));
define_lexical_terminal!(Equal, tag(keyword::symbol::EQUAL));
define_lexical_terminal!(Semicolon, tag(keyword::symbol::SEMICOLON));
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

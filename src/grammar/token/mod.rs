use super::*;
use paste::paste;

/// Parses spacing, which consists of either the end of the input ([`eof`])
/// or a sequence of comments and whitespace characters ([`comment`] or [`multispace1`]).
///
/// ## Grammar
///
/// - [`spacing`] <- [`eof`] / ([`comment`] / [`multispace1`])*
/// - [`eof`]       <- !.
pub fn spacing(
    input: LocatedSpan<&str>,
) -> IResult<LocatedSpan<&str>, Vec<LocatedSpan<&str>>, ErrorTree<LocatedSpan<&str>>> {
    eof.map(|eof| vec![eof])
        .or(many0(alt((comment, multispace1))))
        .parse(input)
}

/// Parses a comment, which starts with `//` and continues until the end of the line.
///
/// ## Grammar
///
/// - [`comment`] <- '//' (! [`end_of_line`] .)* [`end_of_line`]?
pub fn comment(
    input: LocatedSpan<&str>,
) -> IResult<LocatedSpan<&str>, LocatedSpan<&str>, ErrorTree<LocatedSpan<&str>>> {
    tag("//")
        .and(take_while(|c| c != '\n' && c != '\r'))
        .and(opt(end_of_line))
        .recognize()
        .parse(input)
}

/// Parses an end-of-line (EOL) sequence, which can be any of `\r\n`, `\n`, or `\r`.
///
/// ## Grammar
///
/// - [`end_of_line`]` <- '\r\n' / '\n' / '\r'
pub fn end_of_line(
    input: LocatedSpan<&str>,
) -> IResult<LocatedSpan<&str>, LocatedSpan<&str>, ErrorTree<LocatedSpan<&str>>> {
    alt((tag("\r\n"), tag("\r"), tag("\n")))(input)
}

/// This macro generates a Rust parser for recognizing a 
/// specific lexical token in a parsing context. It creates a function to instantiate 
/// the parser, a parser struct with an optional label completion feature, and a struct 
/// representing the parsed token. The parser follows a specified PEG grammar rule and 
/// includes spacing handling. The generated parser implements parsing logic, autocompletion, 
/// and position querying for parsed tokens.
macro_rules! define_lexical_terminal {
    ($struct_name:ident, $token:expr, $parser:expr, $test_input:literal) => {
        paste! {
            #[doc = concat!(
                " This function returns a [`", stringify!($struct_name), "`], which is a type of [`Parser`] trait.\n\n",
                " The [`", stringify!([<$struct_name Parser>]), "`] is initialized without any label completion.\n\n",
                " ## Grammar\n\n",
                " It's based on the following PEG grammar rule of the **ACE** grammar:\n\n",
                " - [`", stringify!([<$struct_name:snake>]), "`] <- [`", stringify!($token), "`] [`spacing`]\n\n",
                " ## Example Usage\n\n",
                " To create a parser and use it to parse input:\n\n",
                " ```\n",
                " let input = ", stringify!($test_input), ";\n",
                " let (remained_input, token) = ", stringify!([<$struct_name:snake>]), "().parse(input).unwrap();\n",
                " ```\n\n",
                " To create a parser with a specific label completion:\n\n",
                " ```\n",
                " use marker::LabelCompletion;\n\n",
                " let input = ", stringify!($test_input), ";\n",
                " let (remained_input, token) = ", stringify!([<$struct_name:snake>]), "().set_label_completion(LabelCompletion::Statement).parse(input).unwrap();\n",
                " ```"
            )]
            pub fn [<$struct_name:snake>]<'a>() -> [<$struct_name Parser>] {
                [<$struct_name Parser>]::default()
            }

            #[doc = concat!(
                " A parser for recognizing the regular expression [`", stringify!($token), "`] token in the input stream."
            )]
            #[derive(Getters, Default)]
            pub struct [<$struct_name Parser>] {
                #[doc = " Optional label completion for autocompletion hints."]
                #[getset(set = "pub")]
                label_completion: marker::LabelCompletion,
            }

            impl [<$struct_name Parser>] {
                #[doc = concat!(
                    " Sets the label completion for this parser.\n\n",
                    " This can be used to provide autocompletion hints when parsing.\n\n",
                    " ## Example Usage\n\n",
                    " ```\n",
                    " use ace::grammar::marker::LabelCompletion;\n",
                    " let mut parser = ", stringify!([<$struct_name:snake>]), "();\n",
                    " parser.set_label_completion(LabelCompletion::None);\n",
                    " ```"
                )]
                pub fn set_label_completion(&mut self, label_completion: marker::LabelCompletion) -> &mut Self {
                    self.label_completion = label_completion;
                    self
                }
            }

            impl<'a, 'b> Parser<LocatedSpan<&'a str>, $struct_name<'a>, ErrorTree<LocatedSpan<&'a str>>> for [<$struct_name Parser>] {
                #[doc = concat!(
                    " Parses a regular expression [`", stringify!($struct_name), "`] token from the input.\n\n",
                    "This function attempts to match the regular expression [`", stringify!($token), "`] against the input stream, capturing any trailing spaces. ",
                    "If the match is successful, it returns a [`", stringify!($struct_name), "`] struct containing the parsed token and its position."
                )]
                fn parse(
                    &mut self,
                    input: LocatedSpan<&'a str>,
                ) -> IResult<LocatedSpan<&'a str>, $struct_name<'a>, ErrorTree<LocatedSpan<&'a str>>> {
                    let (s, token) = ($parser).parse(input)?;
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

            #[doc = concat!(
                " Represents the [`", stringify!([<$struct_name:snake>]), "`] lexical terminal in the parsing process.\n\n",
                " This structure stores information about a parsed [`", stringify!($token), "`] token, including its position in the input, spacing, and optional label completion."
            )]
            #[derive(Debug, Getters)]
            pub struct $struct_name<'a> {
                #[doc = concat!(
                    " The range of the regular expression [`", stringify!($token), "`] token in the input."
                )]
                range: Range,

                #[doc = " Optional label completion associated with this token."]
                label_completion: marker::LabelCompletion,

                #[doc = concat!(
                    " The actual regular expression [`", stringify!($token), "`] token from the input."
                )]
                #[getset(get = "pub")]
                token: LocatedSpan<&'a str>,

                #[doc = " Any spacing and comments found after the token."]
                #[getset(get = "pub")]
                spacing: Vec<LocatedSpan<&'a str>>,
            }

            impl ParseTree for $struct_name<'_> {
                #[doc = concat!(
                    " Returns the range of this regular expression [`", stringify!($struct_name), "`] token in the parsed input."
                )]
                fn range(&self) -> Range {
                    self.range
                }

                #[doc = concat!(
                    " Queries whether the given position is within this regular expression [`", stringify!($struct_name), "`] token's range.\n\n",
                    " Returns `Ok(self)` if the position is within the range, otherwise `Err(())`."
                )]
                fn query(&self, pos: Position) -> Result<&dyn ParseTree, ()> {
                    if self.range().contains(pos) {
                        Ok(self)
                    } else {
                        Err(())
                    }
                }

                #[doc = concat!(
                    " Returns a list of possible autocompletion suggestions for this token.\n\n",
                    " The autocompletion suggestions are based on the [`label_completion`] associated with this token.\n\n",
                    " ## Example Usage\n\n",
                    " ```\n",
                    " use marker::LabelCompletion;\n\n",
                    " let input = ", stringify!($test_input), ";\n",
                    " let (remained_input, token) = ", stringify!([<$struct_name:snake>]), "().set_label_completion(LabelCompletion::Statement).parse(input).unwrap();\n",
                    " let completions = token.show_completions();\n",
                    " println!(\"Suggested completions: {:?}\", completions);\n",
                    " ```"
                )]
                fn show_completions(&self) -> Vec<&'static str> {
                    self.label_completion.completion()
                }
            }
        }
    };
}

define_lexical_terminal!(
    Cpu,
    literal::statement::CPU,
    tag(literal::statement::CPU),
    "cpu \n"
);
define_lexical_terminal!(
    Config,
    literal::statement::CONFIG,
    tag(literal::statement::CONFIG),
    "config \n"
);
define_lexical_terminal!(
    TimeoutCycle,
    literal::attribute::TIMEOUT_CYCLE,
    tag(literal::attribute::TIMEOUT_CYCLE),
    "timeout_cycle \n"
);
define_lexical_terminal!(
    Name,
    literal::attribute::NAME,
    tag(literal::attribute::NAME),
    "name \n"
);
define_lexical_terminal!(
    Vlen,
    literal::attribute::VLEN,
    tag(literal::attribute::VLEN),
    "vlen \n"
);
define_lexical_terminal!(
    LeftBrace,
    literal::token::LEFT_BRACE,
    tag(literal::token::LEFT_BRACE),
    "{ \n"
);
define_lexical_terminal!(
    RightBrace,
    literal::token::RIGHT_BRACE,
    tag(literal::token::RIGHT_BRACE),
    "} \n"
);
define_lexical_terminal!(
    Equal,
    literal::token::EQUAL,
    tag(literal::token::EQUAL),
    "= \n"
);
define_lexical_terminal!(
    Semicolon,
    literal::token::SEMICOLON,
    tag(literal::token::SEMICOLON),
    "; \n"
);
define_lexical_terminal!(
    Identifier,
    literal::token::IDENTIFIER,
    take_while(|c: char| c.is_alphabetic() || c == '_')
        .and(take_while(|c: char| c.is_alphanumeric() || c == '_'))
        .recognize(),
    "NX45V \n"
);
define_lexical_terminal!(
    HexNumber,
    literal::token::HEX_NUMBER,
    tag_no_case("0x")
        .and(take_while(|c: char| c.is_ascii_hexdigit()))
        .recognize(),
    "0x1234 \n"
);
define_lexical_terminal!(
    DecNumber,
    literal::token::DEC_NUMBER,
    take_while(|c: char| c.is_ascii_digit())
        .verify(|s: &LocatedSpan<&str>| !s.starts_with('0'))
        .recognize(),
    "1234 \n"
);

use super::*;

#[derive(Debug, Getters)]
pub struct Cpu<'a> {
    range: Range,
    #[getset(get = "pub")]
    pragma: token::Cpu<'a>,
    #[getset(get = "pub")]
    left_brace: token::LeftBrace<'a>,
    #[getset(get = "pub")]
    attributes: Vec<CpuAttribute<'a>>,
    #[getset(get = "pub")]
    right_brace: token::RightBrace<'a>,
    #[getset(get = "pub")]
    semicolon: token::Semicolon<'a>,
}

pub fn cpu<'a>(
    input: LocatedSpan<&'a str>,
) -> IResult<LocatedSpan<&'a str>, Cpu<'a>, ErrorTree<LocatedSpan<&'a str>>> {
    let (s, pragma) = token::cpu().parse(input)?;
    let (s, left_brace) = token::left_brace().parse(s)?;
    let (s, attributes) = many0(cpu_attribute)(s)?;
    let (s, right_brace) = token::right_brace().parse(s)?;
    let (s, semicolon) = token::semicolon()
        .set_label_completion(marker::LabelCompletion::Statement)
        .parse(s)?;
    let range = Range {
        start: input.into(),
        end: s.into(),
    };
    Ok((
        s,
        Cpu {
            pragma,
            left_brace,
            attributes,
            right_brace,
            semicolon,
            range,
        },
    ))
}

impl AceParseTree for Cpu<'_> {
    fn range(&self) -> Range {
        self.range
    }

    fn query(&self, pos: Position) -> Result<&dyn AceParseTree, ()> {
        if self.pragma.range().contains(pos) {
            self.pragma.query(pos)
        } else if self.left_brace.range().contains(pos) {
            self.left_brace.query(pos)
        } else if self.right_brace.range().contains(pos) {
            self.right_brace.query(pos)
        } else if self.semicolon.range().contains(pos) {
            self.semicolon.query(pos)
        } else {
            self.attributes
                .iter()
                .find_map(|attr| {
                    if attr.range().contains(pos) {
                        attr.query(pos).ok()
                    } else {
                        None
                    }
                })
                .ok_or(())
        }
    }
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

impl AceParseTree for CpuAttribute<'_> {
    fn range(&self) -> Range {
        match self {
            CpuAttribute::Name(name) => name.range(),
            CpuAttribute::Vlen(vlen) => vlen.range(),
        }
    }
    fn query(&self, pos: Position) -> Result<&dyn AceParseTree, ()> {
        match self {
            CpuAttribute::Name(name) => name.query(pos),
            CpuAttribute::Vlen(vlen) => vlen.query(pos),
        }
    }
}

#[derive(Debug, Getters)]
pub struct Config<'a> {
    range: Range,
    #[getset(get = "pub")]
    pragma: token::Config<'a>,
    #[getset(get = "pub")]
    left_brace: token::LeftBrace<'a>,
    #[getset(get = "pub")]
    attributes: Vec<ConfigAttribute<'a>>,
    #[getset(get = "pub")]
    right_brace: token::RightBrace<'a>,
    #[getset(get = "pub")]
    semicolon: token::Semicolon<'a>,
}

pub fn config<'a>(
    input: LocatedSpan<&'a str>,
) -> IResult<LocatedSpan<&'a str>, Config<'a>, ErrorTree<LocatedSpan<&'a str>>> {
    let (s, pragma) = token::config().parse(input)?;
    let (s, left_brace) = token::left_brace().parse(s)?;
    let (s, attributes) = many0(config_attribute)(s)?;
    let (s, right_brace) = token::right_brace().parse(s)?;
    let (s, semicolon) = token::semicolon()
        .set_label_completion(marker::LabelCompletion::Statement)
        .parse(s)?;
    let range = Range {
        start: input.into(),
        end: s.into(),
    };
    Ok((
        s,
        Config {
            pragma,
            left_brace,
            attributes,
            right_brace,
            semicolon,
            range,
        },
    ))
}

impl AceParseTree for Config<'_> {
    fn range(&self) -> Range {
        self.range
    }

    fn query(&self, pos: Position) -> Result<&dyn AceParseTree, ()> {
        if self.pragma.range().contains(pos) {
            self.pragma.query(pos)
        } else if self.left_brace.range().contains(pos) {
            self.left_brace.query(pos)
        } else if self.right_brace.range().contains(pos) {
            self.right_brace.query(pos)
        } else if self.semicolon.range().contains(pos) {
            self.semicolon.query(pos)
        } else {
            self.attributes
                .iter()
                .find_map(|attr| {
                    if attr.range().contains(pos) {
                        attr.query(pos).ok()
                    } else {
                        None
                    }
                })
                .ok_or(())
        }
    }
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

impl AceParseTree for ConfigAttribute<'_> {
    fn range(&self) -> Range {
        match self {
            ConfigAttribute::TimeoutCycle(timeout_cycle) => timeout_cycle.range(),
        }
    }

    fn query(&self, pos: Position) -> Result<&dyn AceParseTree, ()> {
        match self {
            ConfigAttribute::TimeoutCycle(timeout_cycle) => timeout_cycle.query(pos),
        }
    }
}

use super::*;

#[derive(Debug, Getters)]
pub struct Name<'a> {
    range: Range,
    #[getset(get = "pub")]
    pragma: token::Name<'a>,
    #[getset(get = "pub")]
    equal: token::Equal<'a>,
    #[getset(get = "pub")]
    identifier: token::Identifier<'a>,
    #[getset(get = "pub")]
    semicolon: token::Semicolon<'a>,
}

pub fn name<'a>(
    input: LocatedSpan<&'a str>,
) -> IResult<LocatedSpan<&'a str>, Name<'a>, ErrorTree<LocatedSpan<&'a str>>> {
    let (s, pragma) = token::name().parse(input)?;
    let (s, equal) = token::equal().parse(s)?;
    let (s, identifier) = token::identifier().parse(s)?;
    let (s, semicolon) = token::semicolon()
        .set_label_completion(marker::LabelCompletion::Attribute)
        .parse(s)?;
    let range = Range {
        start: input.into(),
        end: s.into(),
    };
    Ok((
        s,
        Name {
            pragma,
            equal,
            identifier,
            semicolon,
            range,
        },
    ))
}

impl ParseTree for Name<'_> {
    fn range(&self) -> Range {
        self.range
    }

    fn query(&self, pos: Position) -> Result<&dyn ParseTree, ()> {
        if self.range().start <= pos && pos < self.range().end {
            self.pragma
                .query(pos)
                .or_else(|_| self.equal.query(pos))
                .or_else(|_| self.identifier.query(pos))
                .or_else(|_| self.semicolon.query(pos))
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Getters)]
pub struct Vlen<'a> {
    range: Range,
    #[getset(get = "pub")]
    pragma: token::Vlen<'a>,
    #[getset(get = "pub")]
    equal: token::Equal<'a>,
    #[getset(get = "pub")]
    length: Number<'a>,
    #[getset(get = "pub")]
    semicolon: token::Semicolon<'a>,
}

pub fn vlen<'a>(
    input: LocatedSpan<&'a str>,
) -> IResult<LocatedSpan<&'a str>, Vlen<'a>, ErrorTree<LocatedSpan<&'a str>>> {
    let (s, pragma) = token::vlen().parse(input)?;
    let (s, equal) = token::equal().parse(s)?;
    let (s, length) = number(s)?;
    let (s, semicolon) = token::semicolon()
        .set_label_completion(marker::LabelCompletion::Attribute)
        .parse(s)?;
    let range = Range {
        start: input.into(),
        end: s.into(),
    };
    Ok((
        s,
        Vlen {
            pragma,
            equal,
            length,
            semicolon,
            range,
        },
    ))
}

impl ParseTree for Vlen<'_> {
    fn range(&self) -> Range {
        self.range
    }

    fn query(&self, pos: Position) -> Result<&dyn ParseTree, ()> {
        if self.range().contains(pos) {
            self.pragma
                .query(pos)
                .or_else(|_| self.equal.query(pos))
                .or_else(|_| self.length.query(pos))
                .or_else(|_| self.semicolon.query(pos))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
pub enum Number<'a> {
    HexNumber(token::HexNumber<'a>),
    DecNumber(token::DecNumber<'a>),
}

fn number<'a>(
    input: LocatedSpan<&'a str>,
) -> IResult<LocatedSpan<&'a str>, Number<'a>, ErrorTree<LocatedSpan<&'a str>>> {
    alt((
        token::hex_number().map(Number::HexNumber),
        token::dec_number().map(Number::DecNumber),
    ))(input)
}

impl ParseTree for Number<'_> {
    fn range(&self) -> Range {
        match self {
            Number::HexNumber(hex_number) => hex_number.range(),
            Number::DecNumber(dec_number) => dec_number.range(),
        }
    }

    fn query(&self, pos: Position) -> Result<&dyn ParseTree, ()> {
        match self {
            Number::HexNumber(hex_number) => hex_number.query(pos),
            Number::DecNumber(dec_number) => dec_number.query(pos),
        }
    }
}

#[derive(Debug, Getters)]
pub struct TimeoutCycle<'a> {
    range: Range,
    #[getset(get = "pub")]
    pragma: token::TimeoutCycle<'a>,
    #[getset(get = "pub")]
    equal: token::Equal<'a>,
    #[getset(get = "pub")]
    length: Number<'a>,
    #[getset(get = "pub")]
    semicolon: token::Semicolon<'a>,
}

pub fn timeout_cycle<'a>(
    input: LocatedSpan<&'a str>,
) -> IResult<LocatedSpan<&'a str>, TimeoutCycle<'a>, ErrorTree<LocatedSpan<&'a str>>> {
    let (s, pragma) = token::timeout_cycle().parse(input)?;
    let (s, equal) = token::equal().parse(s)?;
    let (s, length) = number(s)?;
    let (s, semicolon) = token::semicolon()
        .set_label_completion(marker::LabelCompletion::Attribute)
        .parse(s)?;
    let range = Range {
        start: input.into(),
        end: s.into(),
    };
    Ok((
        s,
        TimeoutCycle {
            pragma,
            equal,
            length,
            semicolon,
            range,
        },
    ))
}

impl ParseTree for TimeoutCycle<'_> {
    fn range(&self) -> Range {
        self.range
    }

    fn query(&self, pos: Position) -> Result<&dyn ParseTree, ()> {
        if self.range().contains(pos) {
            self.pragma
                .query(pos)
                .or_else(|_| self.equal.query(pos))
                .or_else(|_| self.length.query(pos))
                .or_else(|_| self.semicolon.query(pos))
        } else {
            Err(())
        }
    }
}

use super::*;

#[derive(Debug)]
pub struct Name<'a> {
    range: Range,
    pub pragma: token::Name<'a>,
    pub equal: token::Equal<'a>,
    pub identifier: token::Identifier<'a>,
    pub semicolon: token::Semicolon<'a>,
}

pub fn name<'a>(
    input: LocatedSpan<&'a str>,
) -> IResult<LocatedSpan<&'a str>, Name<'a>, ErrorTree<LocatedSpan<&'a str>>> {
    let (s, pragma) = token::name(input)?;
    let (s, equal) = token::equal(s)?;
    let (s, identifier) = token::identifier(s)?;
    let (s, semicolon) = token::semicolon(s)?;
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

impl AceParseTree for Name<'_> {
    fn range(&self) -> Range {
        self.range
    }

    fn query(&self, pos: Position) -> Result<&dyn AceParseTree, ()> {
        if self.range().start <= pos && pos < self.range().end {
            self
                .pragma
                .query(pos)
                .or_else(|_| self.equal.query(pos))
                .or_else(|_| self.identifier.query(pos))
                .or_else(|_| self.semicolon.query(pos))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
pub struct Vlen<'a> {
    range: Range,
    pub pragma: token::Vlen<'a>,
    pub equal: token::Equal<'a>,
    pub length: Number<'a>,
    pub semicolon: token::Semicolon<'a>,
}

pub fn vlen<'a>(
    input: LocatedSpan<&'a str>,
) -> IResult<LocatedSpan<&'a str>, Vlen<'a>, ErrorTree<LocatedSpan<&'a str>>> {
    let (s, pragma) = token::vlen(input)?;
    let (s, equal) = token::equal(s)?;
    let (s, length) = number(s)?;
    let (s, semicolon) = token::semicolon(s)?;
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

impl AceParseTree for Vlen<'_> {
    fn range(&self) -> Range {
        self.range
    }

    fn query(&self, pos: Position) -> Result<&dyn AceParseTree, ()> {
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
        token::hex_number.map(Number::HexNumber),
        token::dec_number.map(Number::DecNumber),
    ))(input)
}

impl AceParseTree for Number<'_> {
    fn range(&self) -> Range {
        match self {
            Number::HexNumber(hex_number) => hex_number.range(),
            Number::DecNumber(dec_number) => dec_number.range(),
        }
    }

    fn query(&self, pos: Position) -> Result<&dyn AceParseTree, ()> {
        match self {
            Number::HexNumber(hex_number) => hex_number.query(pos),
            Number::DecNumber(dec_number) => dec_number.query(pos),
        }
    }
}

#[derive(Debug)]
pub struct TimeoutCycle<'a> {
    range: Range,
    pub pragma: token::TimeoutCycle<'a>,
    pub equal: token::Equal<'a>,
    pub length: Number<'a>,
    pub semicolon: token::Semicolon<'a>,
}

pub fn timeout_cycle<'a>(
    input: LocatedSpan<&'a str>,
) -> IResult<LocatedSpan<&'a str>, TimeoutCycle<'a>, ErrorTree<LocatedSpan<&'a str>>> {
    let (s, pragma) = token::timeout_cycle(input)?;
    let (s, equal) = token::equal(s)?;
    let (s, length) = number(s)?;
    let (s, semicolon) = token::semicolon(s)?;
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

impl AceParseTree for TimeoutCycle<'_> {
    fn range(&self) -> Range {
        self.range
    }

    fn query(&self, pos: Position) -> Result<&dyn AceParseTree, ()> {
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
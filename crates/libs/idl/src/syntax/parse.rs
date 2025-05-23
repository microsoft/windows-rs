use super::*;

use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*, *,
};

fn parse_ident(input: &str) -> IResult<&str, String> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))
    .map(|s: &str| s.to_string())
    .parse(input)
}

fn parse_type(input: &str) -> IResult<&str, String> {
    recognize(pair(parse_ident, opt(pair(tag("*"), multispace0))))
        .map(|s: &str| s.replace(" ", "").to_string())
        .parse(input)
}

fn parse_comment(input: &str) -> IResult<&str, String> {
    alt((
        // Single-line comment: // ... until newline or EOF
        preceded(
            tag("//"),
            take_till(|c| c == '\n' || c == '\r').map(|s: &str| s.trim_end().to_string()),
        ),
        // Multi-line comment: /* ... */
        delimited(
            tag("/*"),
            take_until("*/").map(|s: &str| s.to_string()),
            tag("*/"),
        ),
    ))
    .parse(input)
}

impl File {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        many0(preceded(multispace0, Item::parse))
            .map(|items| Self { items })
            .parse(input)
    }
}

impl ItemEnum {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        (
            preceded(multispace0, tag("enum")),
            preceded(multispace1, parse_ident),
            delimited(
                preceded(multispace0, tag("{")),
                many0(preceded(
                    multispace0,
                    terminated(EnumVariant::parse, opt(preceded(multispace0, tag(",")))),
                )),
                preceded(multispace0, tag("}")),
            ),
        )
            .map(|(_, name, variants)| Self { name, variants })
            .parse(input)
    }
}

impl EnumVariant {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        (
            preceded(multispace0, parse_ident),
            opt(preceded(
                preceded(multispace0, tag("=")),
                preceded(multispace0, i64),
            )),
        )
            .map(|(name, value)| Self { name, value })
            .parse(input)
    }
}

impl ItemInterface {
    fn parse(input: &str) -> IResult<&str, Self> {
        (
            preceded(multispace0, tag("interface")),
            preceded(multispace1, parse_ident),
            delimited(
                preceded(multispace0, tag("{")),
                many0(Method::parse),
                preceded(multispace0, tag("}")),
            ),
        )
            .map(|(_, name, methods)| Self { name, methods })
            .parse(input)
    }
}

// impl ItemStruct {
//     fn parse(_input: &str) -> IResult<&str, Self> {
//         todo!()
//     }
// }

impl Item {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(ItemInterface::parse, Self::Interface),
            map(ItemEnum::parse, Self::Enum),
            map(parse_comment, Self::Comment),
        ))
        .parse(input)
    }
}

impl Method {
    fn parse(input: &str) -> IResult<&str, Self> {
        (
            preceded(multispace0, parse_type),
            preceded(multispace1, parse_ident),
            delimited(
                preceded(multispace0, tag("(")),
                many0(preceded(
                    multispace0,
                    terminated(Param::parse, opt(preceded(multispace0, tag(",")))),
                )),
                preceded(multispace0, tag(")")),
            ),
            preceded(multispace0, tag(";")),
        )
            .map(|(return_type, name, params, _)| Self {
                return_type,
                name,
                params,
            })
            .parse(input)
    }
}

impl Param {
    fn parse(input: &str) -> IResult<&str, Self> {
        (
            preceded(multispace0, parse_type),
            preceded(multispace1, parse_ident),
        )
            .map(|(ty, name)| Self { ty, name })
            .parse(input)
    }
}

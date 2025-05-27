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
    recognize(pair(parse_ident, many0(tag("*"))))
        .map(|s: &str| s.to_string())
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

fn parse_value(input: &str) -> IResult<&str, String> {
    alt((
        // Quoted string (e.g., "1234-ABCD")
        delimited(tag("\""), take_till(|c| c == '"'), tag("\"")).map(|s: &str| s.to_string()),
        // General token (e.g., 41f3632b-5ef4-404f-ad82-2d606c5a9a21, unique, 1234)
        take_while1(|c: char| !",()[]={}; \t\n\r".contains(c)).map(|s: &str| s.to_string()),
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

impl Enum {
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

impl Interface {
    fn parse(input: &str) -> IResult<&str, Self> {
        (
            Attribute::parse,
            preceded(multispace0, tag("interface")),
            preceded(multispace1, parse_ident),
            delimited(
                preceded(multispace0, tag("{")),
                many0(Method::parse),
                preceded(multispace0, tag("}")),
            ),
        )
            .map(|(attributes, _, name, methods)| Self {
                attributes,
                name,
                methods,
            })
            .parse(input)
    }
}

impl Import {
    fn parse(input: &str) -> IResult<&str, Self> {
        (
            preceded(multispace0, tag("import")),
            preceded(
                multispace0,
                delimited(
                    tag("\""),
                    take_till(|c| c == '"').map(|s: &str| s.to_string()),
                    tag("\""),
                ),
            ),
            preceded(multispace0, tag(";")),
        )
            .map(|(_, name, _)| Self { name })
            .parse(input)
    }
}

// impl Struct {
//     fn parse(_input: &str) -> IResult<&str, Self> {
//         todo!()
//     }
// }

impl Attribute {
    fn parse_one(input: &str) -> IResult<&str, Self> {
        (
            preceded(multispace0, parse_ident),
            opt(delimited(
                preceded(multispace0, tag("(")),
                separated_list0(
                    preceded(multispace0, tag(",")),
                    preceded(
                        multispace0,
                        alt((
                            // name=value
                            (parse_ident, preceded(multispace0, tag("=")), parse_value)
                                .map(|(name, _, value)| (name, value)),
                            // Simple value
                            parse_value.map(|value| ("".to_string(), value)),
                        )),
                    ),
                ),
                preceded(multispace0, tag(")")),
            )),
        )
            .map(|(name, params_opt)| Attribute {
                name,
                parameters: params_opt.unwrap_or_default(),
            })
            .parse(input)
    }

    fn parse(input: &str) -> IResult<&str, Vec<Self>> {
        many0(delimited(
            preceded(multispace0, tag("[")),
            separated_list0(
                preceded(multispace0, tag(",")),
                preceded(multispace0, Self::parse_one),
            ),
            preceded(multispace0, tag("]")),
        ))
        .map(|attr_lists| attr_lists.into_iter().flatten().collect())
        .parse(input)
    }
}

impl Item {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(Interface::parse, Self::Interface),
            map(Enum::parse, Self::Enum),
            map(parse_comment, Self::Comment),
            map(Import::parse, Self::Import),
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
            Attribute::parse,
            preceded(multispace0, parse_type),
            preceded(multispace1, parse_ident),
        )
            .map(|(attributes, ty, name)| Self {
                attributes,
                ty,
                name,
            })
            .parse(input)
    }
}

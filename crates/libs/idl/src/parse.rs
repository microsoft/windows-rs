use super::*;

use nom::{
    bytes::complete::{tag, take_until, take_till, take_while1},
    character::complete::{ alpha1, alphanumeric1, multispace1, i64},
    combinator::{recognize, eof, opt, value, map},
    multi::{many1, many0, separated_list1, separated_list0},
    sequence::{delimited, terminated, preceded, pair},
    branch::alt,
    error::context,
     IResult, Parser,
};

type ParseResult<'a, T> = IResult<Input<'a>, T>;

fn parse_ident(input: Input) -> ParseResult<String> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))
    .map(|s: Input| s.to_string())
    .parse(input)
}

fn parse_type(input: Input) -> ParseResult<String> {
    recognize(pair(
        parse_ident,
        many0(preceded(parse_whitespace0, tag("*"))),
    ))
    .map(|s: Input| s.to_string())
    .parse(input)
}

fn parse_whitespace0(input: Input) -> ParseResult<()> {
    value(
        (),
        many0(alt((
            multispace1,
            delimited(tag("/*"), take_until("*/"), tag("*/")),
            preceded(tag("//"), take_till(|c| c == '\n' || c == '\r')),
        ))),
    )
    .parse(input)
}

fn parse_whitespace1(input: Input) -> ParseResult<()> {
    value(
        (),
        many1(alt((
            multispace1,
            delimited(tag("/*"), take_until("*/"), tag("*/")),
            preceded(tag("//"), take_till(|c| c == '\n' || c == '\r')),
        ))),
    )
    .parse(input)
}

fn parse_value(input: Input) -> ParseResult<String> {
    alt((
        // Quoted string (e.g., "1234-ABCD")
        delimited(tag("\""), take_till(|c| c == '"'), tag("\"")).map(|s: Input| s.to_string()),
        // General token (e.g., 41f3632b-5ef4-404f-ad82-2d606c5a9a21, unique, 1234)
        take_while1(|c: char| !",()[]={}; \t\n\r".contains(c)).map(|s: Input| s.to_string()),
    ))
    .parse(input)
}

fn parse_cpp_quote(input: Input) -> ParseResult<String> {
    map(
        preceded(
            parse_whitespace0,
            delimited(
                preceded(parse_whitespace0, tag("cpp_quote")),
                delimited(
                    preceded(parse_whitespace0, tag("(")),
                    delimited(tag("\""), take_till(|c| c == '"'), tag("\"")),
                    preceded(parse_whitespace0, tag(")")),
                ),
                opt(preceded(parse_whitespace0, tag(";"))),
            ),
        ),
        |s: Input| s.to_string(),
    )
    .parse(input)
}

impl File {
    pub fn parse(input: Input) -> FileResult<Self> {
        terminated(
        terminated(
            many0(preceded(parse_whitespace0, Item::parse)),
            parse_whitespace0,
        )
        ,eof)
        .map(|items| File { items })
        .parse(input)
        .map(|(_, ok)|ok)
    }
}

impl Library {
    pub fn parse(input: Input) -> ParseResult<Self> {
        context("library",
        (
            Attribute::parse,
            preceded(parse_whitespace0, tag("library")),
            preceded(parse_whitespace1, parse_ident),
            delimited(
                preceded(parse_whitespace0, tag("{")),
                many0(preceded(parse_whitespace0, Item::parse)),
                preceded(parse_whitespace0, tag("}")),
            ),
        ))
            .map(|(attributes, _, name, items)| Library {
                attributes,
                name,
                items,
            })
            .parse(input)
    }
}

impl Enum {
    pub fn parse(input: Input) -> ParseResult<Self> {
        alt((
            // Regular format: enum NAME { ... };
            (
                Attribute::parse,
                preceded(parse_whitespace0, tag("enum")),
                preceded(parse_whitespace1, parse_ident),
                delimited(
                    preceded(parse_whitespace0, tag("{")),
                    many0(preceded(
                        parse_whitespace0,
                        terminated(
                            EnumVariant::parse,
                            opt(preceded(parse_whitespace0, tag(","))),
                        ),
                    )),
                    preceded(parse_whitespace0, tag("}")),
                ),
                preceded(parse_whitespace0, tag(";")),
            )
                .map(|(attributes, _, name, variants, _)| Enum {
                    attributes,
                    name,
                    variants,
                }),
            // C-style format: typedef enum IGNORE_THIS { ... } NAME;
            (
                Attribute::parse,
                preceded(parse_whitespace0, tag("typedef")),
                preceded(parse_whitespace1, tag("enum")),
                preceded(parse_whitespace1, parse_ident), // IGNORE_THIS (ignored)
                delimited(
                    preceded(parse_whitespace0, tag("{")),
                    many0(preceded(
                        parse_whitespace0,
                        terminated(
                            EnumVariant::parse,
                            opt(preceded(parse_whitespace0, tag(","))),
                        ),
                    )),
                    preceded(parse_whitespace0, tag("}")),
                ),
                preceded(parse_whitespace1, parse_ident), // NAME
                preceded(parse_whitespace0, tag(";")),
            )
                .map(|(attributes, _, _, _, variants, name, _)| Enum {
                    attributes,
                    name,
                    variants,
                }),
        ))
        .parse(input)
    }
}

impl EnumVariant {
    pub fn parse(input: Input) -> ParseResult<Self> {
        (
            preceded(parse_whitespace0, parse_ident),
            opt(preceded(
                preceded(parse_whitespace0, tag("=")),
                preceded(parse_whitespace0, i64),
            )),
        )
            .map(|(name, value)| Self { name, value })
            .parse(input)
    }
}

impl Interface {
    fn parse(input: Input) -> ParseResult<Self> {
        (
            Attribute::parse,
            preceded(parse_whitespace0, tag("interface")),
            preceded(parse_whitespace1, parse_ident),
            opt(preceded(
                preceded(parse_whitespace0, tag(":")),
                separated_list1(
                    preceded(parse_whitespace0, tag(",")),
                    preceded(
                        parse_whitespace0,
                        (Attribute::parse, parse_type)
                            .map(|(attributes, name)| InterfaceImpl { attributes, name }),
                    ),
                ),
            )),
            delimited(
                preceded(parse_whitespace0, tag("{")),
                many0(Method::parse),
                preceded(parse_whitespace0, tag("}")),
            ),
        )
            .map(|(attributes, _, name, implements, methods)| Self {
                attributes,
                name,
                implements: implements.unwrap_or_default(),
                methods,
            })
            .parse(input)
    }
}

fn parse_forward_interface(input: Input) -> ParseResult<String> {
    (
        preceded(parse_whitespace0, tag("interface")),
        preceded(parse_whitespace1, parse_ident),
        preceded(parse_whitespace0, tag(";")),
    )
        .map(|(_, name, _)| name)
        .parse(input)
}

fn parse_forward_struct(input: Input) -> ParseResult<String> {
    (
        preceded(parse_whitespace0, tag("struct")),
        preceded(parse_whitespace1, parse_ident),
        preceded(parse_whitespace0, tag(";")),
    )
        .map(|(_, name, _)| name)
        .parse(input)
}
fn parse_forward_enum(input: Input) -> ParseResult<String> {
    (
        preceded(parse_whitespace0, tag("enum")),
        preceded(parse_whitespace1, parse_ident),
        preceded(parse_whitespace0, tag(";")),
    )
        .map(|(_, name, _)| name)
        .parse(input)
}

impl Import {
    fn parse(input: Input) -> ParseResult<Self> {
        (
            preceded(parse_whitespace0, tag("import")),
            preceded(
                parse_whitespace0,
                delimited(
                    tag("\""),
                    take_till(|c| c == '"').map(|s: Input| s.to_string()),
                    tag("\""),
                ),
            ),
            preceded(parse_whitespace0, tag(";")),
        )
            .map(|(_, name, _)| Self { name })
            .parse(input)
    }
}

impl Struct {
    fn parse(input: Input) -> ParseResult<Self> {
        alt((
            // Regular format: struct NAME { ... };
            (
                Attribute::parse,
                preceded(parse_whitespace0, tag("struct")),
                preceded(parse_whitespace1, parse_ident),
                delimited(
                    preceded(parse_whitespace0, tag("{")),
                    many0(preceded(
                        parse_whitespace0,
                        terminated(
                            (
                                Attribute::parse,
                                preceded(parse_whitespace0, parse_type),
                                preceded(parse_whitespace1, parse_ident),
                            )
                                .map(
                                    |(attributes, field_type, name)| StructField {
                                        attributes,
                                        field_type,
                                        name,
                                    },
                                ),
                            preceded(parse_whitespace0, tag(";")),
                        ),
                    )),
                    preceded(parse_whitespace0, tag("}")),
                ),
                preceded(parse_whitespace0, tag(";")),
            )
                .map(|(attributes, _, name, fields, _)| Struct {
                    attributes,
                    name,
                    fields,
                }),
            // C-style format: typedef struct IGNORE { ... } NAME;
            (
                Attribute::parse,
                preceded(parse_whitespace0, tag("typedef")),
                preceded(parse_whitespace1, tag("struct")),
                preceded(parse_whitespace1, parse_ident), // IGNORE (ignored)
                delimited(
                    preceded(parse_whitespace0, tag("{")),
                    many0(preceded(
                        parse_whitespace0,
                        terminated(
                            (
                                Attribute::parse,
                                preceded(parse_whitespace0, parse_type),
                                preceded(parse_whitespace1, parse_ident),
                            )
                                .map(
                                    |(attributes, field_type, name)| StructField {
                                        attributes,
                                        field_type,
                                        name,
                                    },
                                ),
                            preceded(parse_whitespace0, tag(";")),
                        ),
                    )),
                    preceded(parse_whitespace0, tag("}")),
                ),
                preceded(parse_whitespace1, parse_ident), // NAME
                preceded(parse_whitespace0, tag(";")),
            )
                .map(|(attributes, _, _, _, fields, name, _)| Struct {
                    attributes,
                    name,
                    fields,
                }),
        ))
        .parse(input)
    }
}

impl Attribute {
    fn parse_one(input: Input) -> ParseResult<Self> {
        (
            preceded(parse_whitespace0, parse_ident),
            opt(delimited(
                preceded(parse_whitespace0, tag("(")),
                separated_list0(
                    preceded(parse_whitespace0, tag(",")),
                    preceded(
                        parse_whitespace0,
                        alt((
                            // name=value
                            (
                                parse_ident,
                                preceded(parse_whitespace0, tag("=")),
                                parse_value,
                            )
                                .map(|(name, _, value)| (name, value)),
                            // Simple value
                            parse_value.map(|value| ("".to_string(), value)),
                        )),
                    ),
                ),
                preceded(parse_whitespace0, tag(")")),
            )),
        )
            .map(|(name, params_opt)| Attribute {
                name,
                parameters: params_opt.unwrap_or_default(),
            })
            .parse(input)
    }

    fn parse(input: Input) -> ParseResult<Vec<Self>> {
        many0(delimited(
            preceded(parse_whitespace0, tag("[")),
            separated_list0(
                preceded(parse_whitespace0, tag(",")),
                preceded(parse_whitespace0, Self::parse_one),
            ),
            preceded(parse_whitespace0, tag("]")),
        ))
        .map(|attr_lists| attr_lists.into_iter().flatten().collect())
        .parse(input)
    }
}

impl Item {
    fn parse(input: Input) -> ParseResult<Self> {
        alt((
            map(Interface::parse, Self::Interface),
            map(Enum::parse, Self::Enum),
            map(Import::parse, Self::Import),
            map(Struct::parse, Self::Struct),
            map(Library::parse, Self::Library),
            map(parse_forward_interface, Self::ForwardInterface),
            map(parse_forward_struct, Self::ForwardStruct),
            map(parse_forward_enum, Self::ForwardEnum),
            map(parse_cpp_quote, Self::CppQuote),
        ))
        .parse(input)
    }
}

impl Method {
    fn parse(input: Input) -> ParseResult<Self> {
        (
            Attribute::parse,
            preceded(parse_whitespace0, parse_type),
            preceded(parse_whitespace1, parse_ident),
            delimited(
                preceded(parse_whitespace0, tag("(")),
                many0(preceded(
                    parse_whitespace0,
                    terminated(Param::parse, opt(preceded(parse_whitespace0, tag(",")))),
                )),
                preceded(parse_whitespace0, tag(")")),
            ),
            preceded(parse_whitespace0, tag(";")),
        )
            .map(|(attributes, return_type, name, params, _)| Self {
                attributes,
                return_type,
                name,
                params,
            })
            .parse(input)
    }
}

impl Param {
    fn parse(input: Input) -> ParseResult<Self> {
        (
            Attribute::parse,
            preceded(parse_whitespace0, parse_type),
            preceded(parse_whitespace1, parse_ident),
        )
            .map(|(attributes, ty, name)| Self {
                attributes,
                ty,
                name,
            })
            .parse(input)
    }
}

use super::*;

mod nom {
    pub use ::nom::{
        branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*,
        sequence::*, *,
    };
}

use nom::Parser;

fn parse_ident(input: &str) -> nom::IResult<&str, String> {
    nom::map(
        nom::recognize(nom::pair(
            nom::alt((nom::alpha1, nom::tag("_"))),
            nom::many0(nom::alt((nom::alphanumeric1, nom::tag("_")))),
        )),
        |s: &str| s.to_string(),
    )
    .parse(input)
}

fn parse_type(input: &str) -> nom::IResult<&str, String> {
    nom::recognize(nom::pair(
        parse_ident,
        nom::opt(nom::pair(nom::tag("*"), nom::multispace0)),
    ))
    .map(|s: &str| s.replace(" ", "").to_string())
    .parse(input)
}

impl File {
    pub fn parse(input: &str) -> nom::IResult<&str, Self> {
        nom::map(
            nom::many0(nom::preceded(nom::multispace0, Item::parse)),
            |items| Self { items },
        )
        .parse(input)
    }
}

impl ItemEnum {
    pub fn parse(input: &str) -> nom::IResult<&str, Self> {
        todo!()
    }
}

impl ItemInterface {
    pub fn parse(input: &str) -> nom::IResult<&str, Self> {
        nom::map(
            (
                nom::preceded(nom::multispace0, nom::tag("interface")),
                nom::preceded(nom::multispace1, parse_ident),
                nom::delimited(
                    nom::preceded(nom::multispace0, nom::tag("{")),
                    nom::many0(Method::parse),
                    nom::preceded(nom::multispace0, nom::tag("}")),
                ),
            ),
            |(_, name, methods)| Self { name, methods },
        )
        .parse(input)
    }
}

impl ItemStruct {
    pub fn parse(input: &str) -> nom::IResult<&str, Self> {
        todo!()
    }
}

impl Item {
    pub fn parse(input: &str) -> nom::IResult<&str, Self> {
        nom::alt((
            nom::map(ItemInterface::parse, Self::Interface),
            //    nom::map(ItemEnum::parse, Self::Enum),
        ))
        .parse(input)
    }
}

impl Method {
    pub fn parse(input: &str) -> nom::IResult<&str, Self> {
        (
            nom::preceded(nom::multispace0, parse_type),
            nom::preceded(nom::multispace1, parse_ident),
            nom::delimited(
                nom::preceded(nom::multispace0, nom::tag("(")),
                nom::many0(nom::preceded(
                    nom::multispace0,
                    nom::terminated(
                        Param::parse,
                        nom::opt(nom::preceded(nom::multispace0, nom::tag(","))),
                    ),
                )),
                nom::preceded(nom::multispace0, nom::tag(")")),
            ),
            nom::preceded(nom::multispace0, nom::tag(";")),
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
    pub fn parse(input: &str) -> nom::IResult<&str, Self> {
        (
            nom::preceded(nom::multispace0, parse_type),
            nom::preceded(nom::multispace1, parse_ident),
        )
            .map(|(ty, name)| Self { ty, name })
            .parse(input)
    }
}

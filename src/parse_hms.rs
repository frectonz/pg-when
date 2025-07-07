use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, verify},
    sequence::preceded,
    Parser,
};

use crate::NomResult;

pub fn parse_24(input: &str) -> NomResult<&str, u8> {
    verify(map_res(digit1, |s: &str| s.parse::<u8>()), |hour| {
        *hour < 24
    })
    .parse(input)
}

fn parse_12(input: &str) -> NomResult<&str, u8> {
    verify(map_res(digit1, |s: &str| s.parse::<u8>()), |hour| {
        (1..=12).contains(hour)
    })
    .parse(input)
}

pub fn parse_60(input: &str) -> NomResult<&str, u8> {
    verify(map_res(digit1, |s: &str| s.parse::<u8>()), |num| *num < 60).parse(input)
}

#[derive(Debug, Clone, Copy)]
pub enum HmsFormat {
    H24,
    H12,
}

pub fn parse_hms(format: HmsFormat) -> impl Fn(&str) -> NomResult<&str, (u8, u8, u8)> {
    move |input: &str| parse_hms_inner(input, format)
}

fn parse_hms_inner(input: &str, format: HmsFormat) -> NomResult<&str, (u8, u8, u8)> {
    let (input, hour) = match format {
        HmsFormat::H24 => parse_24(input)?,
        HmsFormat::H12 => parse_12(input)?,
    };

    if !input.starts_with(':') {
        return Ok((input, (hour, 0, 0)));
    }

    let (input, minute) = preceded(tag(":"), parse_60).parse(input)?;
    if !input.starts_with(':') {
        return Ok((input, (hour, minute, 0)));
    }

    let (input, second) = preceded(tag(":"), parse_60).parse(input)?;
    Ok((input, (hour, minute, second)))
}

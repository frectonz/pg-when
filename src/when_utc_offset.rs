use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, verify},
    sequence::preceded,
    IResult, Parser,
};

#[derive(Debug)]
pub struct WhenUtcOffset {
    sign: WhenUtcOffsetSign,
    hour: u8,
    minute: u8,
    second: u8,
}

#[derive(Debug)]
pub enum WhenUtcOffsetSign {
    Plus,
    Minus,
}

fn utc(input: &str) -> IResult<&str, &str> {
    tag("UTC").parse(input)
}

fn sign(input: &str) -> IResult<&str, WhenUtcOffsetSign> {
    alt((
        map(tag("+"), |_| WhenUtcOffsetSign::Plus),
        map(tag("-"), |_| WhenUtcOffsetSign::Minus),
    ))
    .parse(input)
}

fn parse_24(input: &str) -> IResult<&str, u8> {
    verify(map_res(digit1, |s: &str| s.parse::<u8>()), |hour| {
        *hour < 24
    })
    .parse(input)
}

fn parse_60(input: &str) -> IResult<&str, u8> {
    verify(map_res(digit1, |s: &str| s.parse::<u8>()), |num| *num < 60).parse(input)
}

fn parse_hms(input: &str) -> IResult<&str, (u8, u8, u8)> {
    let (input, hour) = parse_24(input)?;

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

pub fn parse_when_utc_offset(input: &str) -> IResult<&str, WhenUtcOffset> {
    all_consuming(map(
        (utc, sign, parse_hms),
        |(_, sign, (hour, minute, second))| WhenUtcOffset {
            sign,
            hour,
            minute,
            second,
        },
    ))
    .parse(input)
}

#[cfg(test)]
mod test {
    use crate::when_utc_offset::{parse_when_utc_offset, WhenUtcOffset, WhenUtcOffsetSign};

    #[test]
    fn parse_hour() {
        let out = parse_when_utc_offset("UTC+1");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenUtcOffset {
                    sign: WhenUtcOffsetSign::Plus,
                    hour: 1,
                    minute: 0,
                    second: 0
                }
            ))
        ));
    }

    #[test]
    fn parse_hour_and_minute() {
        let out = parse_when_utc_offset("UTC-1:30");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenUtcOffset {
                    sign: WhenUtcOffsetSign::Minus,
                    hour: 1,
                    minute: 30,
                    second: 0
                }
            ))
        ));
    }

    #[test]
    fn parse_all() {
        let out = parse_when_utc_offset("UTC+1:30:24");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenUtcOffset {
                    sign: WhenUtcOffsetSign::Plus,
                    hour: 1,
                    minute: 30,
                    second: 24
                }
            ))
        ));
    }

    #[test]
    fn parse_invalid_hour() {
        let out = parse_when_utc_offset("UTC+24");
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "24",
                code: nom::error::ErrorKind::Verify,
            }))
        ));
    }

    #[test]
    fn parse_invalid_minute() {
        let out = parse_when_utc_offset("UTC+12:60");
        dbg!(&out);
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "60",
                code: nom::error::ErrorKind::Verify,
            }))
        ));
    }

    #[test]
    fn parse_invalid_second() {
        let out = parse_when_utc_offset("UTC+12:58:60");
        dbg!(&out);
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "60",
                code: nom::error::ErrorKind::Verify,
            }))
        ));
    }
}

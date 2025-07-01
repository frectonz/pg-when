use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res, verify},
    IResult, Parser,
};

#[derive(Debug)]
pub struct WhenExactDate {
    pub year: u32,
    pub month: u8,
    pub day: u8,
}

fn parse_with_dashes(input: &str) -> IResult<&str, WhenExactDate> {
    map(
        (
            verify(map_res(digit1, |s: &str| s.parse::<u8>()), |&day| {
                day >= 1 && day <= 31
            }),
            tag("-"),
            verify(map_res(digit1, |s: &str| s.parse::<u8>()), |&month| {
                month >= 1 && month <= 12
            }),
            tag("-"),
            map_res(digit1, |s: &str| s.parse::<u32>()),
        ),
        |(day, _, month, _, year)| WhenExactDate { year, month, day },
    )
    .parse(input)
}

fn parse_with_slashes(input: &str) -> IResult<&str, WhenExactDate> {
    map(
        (
            verify(map_res(digit1, |s: &str| s.parse::<u8>()), |&day| {
                day >= 1 && day <= 31
            }),
            tag("/"),
            verify(map_res(digit1, |s: &str| s.parse::<u8>()), |&month| {
                month >= 1 && month <= 12
            }),
            tag("/"),
            map_res(digit1, |s: &str| s.parse::<u32>()),
        ),
        |(day, _, month, _, year)| WhenExactDate { year, month, day },
    )
    .parse(input)
}

pub fn parse_when_exact_date(input: &str) -> IResult<&str, WhenExactDate> {
    alt((parse_with_dashes, parse_with_slashes)).parse(input)
}

#[cfg(test)]
mod tests {
    use crate::when_exact_date::{parse_when_exact_date, WhenExactDate};

    #[test]
    fn parse_dashes() {
        let out = parse_when_exact_date("01-01-2004");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenExactDate {
                    year: 2004,
                    month: 1,
                    day: 1
                }
            ))
        ));
    }

    #[test]
    fn parse_slashes() {
        let out = parse_when_exact_date("01/01/2004");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenExactDate {
                    year: 2004,
                    month: 1,
                    day: 1
                }
            ))
        ));
    }

    #[test]
    fn parse_invalid_month() {
        let out = parse_when_exact_date("01/13/2004");
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "13/2004",
                code: nom::error::ErrorKind::Verify,
            }))
        ));

        let out = parse_when_exact_date("01/00/2004");
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "00/2004",
                code: nom::error::ErrorKind::Verify,
            }))
        ));
    }

    #[test]
    fn parse_invalid_day() {
        let out = parse_when_exact_date("32/12/2004");
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "32/12/2004",
                code: nom::error::ErrorKind::Verify,
            }))
        ));

        let out = parse_when_exact_date("00/01/2004");
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "00/01/2004",
                code: nom::error::ErrorKind::Verify,
            }))
        ));
    }

    #[test]
    fn parse_unknown() {
        let out = parse_when_exact_date("unknown");

        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "unknown",
                code: nom::error::ErrorKind::Digit,
            }))
        ));
    }
}

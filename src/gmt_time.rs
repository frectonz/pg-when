use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{map, map_res, verify},
    IResult, Parser,
};

#[derive(Debug)]
pub struct GmtTime {
    pub hour: u8, // 0-23
    pub minute: u8,
    pub second: u8,
}

fn gmt(input: &str) -> IResult<&str, &str> {
    tag("GMT").parse(input)
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

fn parse_hour_only(input: &str) -> IResult<&str, GmtTime> {
    map((parse_24, space1, gmt), |(hour, _, _)| GmtTime {
        hour,
        minute: 0,
        second: 0,
    })
    .parse(input)
}

fn parse_hour_minute(input: &str) -> IResult<&str, GmtTime> {
    map(
        (parse_24, tag(":"), parse_60, space1, gmt),
        |(hour, _, minute, _, _)| GmtTime {
            hour,
            minute,
            second: 0,
        },
    )
    .parse(input)
}

fn parse_all(input: &str) -> IResult<&str, GmtTime> {
    map(
        (
            parse_24,
            tag(":"),
            parse_60,
            tag(":"),
            parse_60,
            space1,
            gmt,
        ),
        |(hour, _, minute, _, second, _, _)| GmtTime {
            hour,
            minute,
            second,
        },
    )
    .parse(input)
}

pub fn parse_gmt_time(input: &str) -> IResult<&str, GmtTime> {
    alt((parse_hour_only, parse_hour_minute, parse_all)).parse(input)
}

#[cfg(test)]
mod tests {
    use crate::gmt_time::{parse_gmt_time, GmtTime};

    #[test]
    fn parse_hout() {
        let out = parse_gmt_time("1 GMT");
        assert!(matches!(
            out,
            Ok((
                "",
                GmtTime {
                    hour: 1,
                    minute: 0,
                    second: 0
                }
            ))
        ));
    }

    #[test]
    fn parse_hour_and_minute() {
        let out = parse_gmt_time("1:30 GMT");
        assert!(matches!(
            out,
            Ok((
                "",
                GmtTime {
                    hour: 1,
                    minute: 30,
                    second: 0
                }
            ))
        ));
    }

    #[test]
    fn parse_all() {
        let out = parse_gmt_time("1:30:24 GMT");
        assert!(matches!(
            out,
            Ok((
                "",
                GmtTime {
                    hour: 1,
                    minute: 30,
                    second: 24
                }
            ))
        ));
    }

    #[test]
    fn parse_invalid_hour() {
        let out = parse_gmt_time("24 GMT");
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "24 GMT",
                code: nom::error::ErrorKind::Verify,
            }))
        ));
    }

    #[test]
    fn parse_invalid_minute() {
        let out = parse_gmt_time("12:60 GMT");
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "60 GMT",
                code: nom::error::ErrorKind::Verify,
            }))
        ));
    }

    #[test]
    fn parse_invalid_second() {
        let out = parse_gmt_time("12:58:60 GMT");
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "60 GMT",
                code: nom::error::ErrorKind::Verify,
            }))
        ));
    }
}

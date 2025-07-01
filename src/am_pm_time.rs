use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{map, map_res, verify},
    IResult, Parser,
};

use crate::am_pm::{parse_am_pm, AmPm};

#[derive(Debug)]
pub struct AmPmTime {
    hour: u8, // 1-12
    minute: u8,
    second: u8,
    period: AmPm,
}

fn parse_12(input: &str) -> IResult<&str, u8> {
    verify(map_res(digit1, |s: &str| s.parse::<u8>()), |hour| {
        (1..=12).contains(hour)
    })
    .parse(input)
}

fn parse_60(input: &str) -> IResult<&str, u8> {
    verify(map_res(digit1, |s: &str| s.parse::<u8>()), |num| *num < 60).parse(input)
}

fn parse_hour_only(input: &str) -> IResult<&str, AmPmTime> {
    map((parse_12, space1, parse_am_pm), |(hour, _, period)| {
        AmPmTime {
            hour,
            minute: 0,
            second: 0,
            period,
        }
    })
    .parse(input)
}

fn parse_hour_minute(input: &str) -> IResult<&str, AmPmTime> {
    map(
        (parse_12, tag(":"), parse_60, space1, parse_am_pm),
        |(hour, _, minute, _, period)| AmPmTime {
            hour,
            minute,
            second: 0,
            period,
        },
    )
    .parse(input)
}

fn parse_all(input: &str) -> IResult<&str, AmPmTime> {
    map(
        (
            parse_12,
            tag(":"),
            parse_60,
            tag(":"),
            parse_60,
            space1,
            parse_am_pm,
        ),
        |(hour, _, minute, _, second, _, period)| AmPmTime {
            hour,
            minute,
            second,
            period,
        },
    )
    .parse(input)
}

pub fn parse_am_pm_time(input: &str) -> IResult<&str, AmPmTime> {
    alt((parse_hour_only, parse_hour_minute, parse_all)).parse(input)
}

#[cfg(test)]
mod tests {
    use crate::{
        am_pm::AmPm,
        am_pm_time::{parse_am_pm_time, AmPmTime},
    };

    #[test]
    fn parse_hour() {
        let out = parse_am_pm_time("1 pm");
        assert!(matches!(
            out,
            Ok((
                "",
                AmPmTime {
                    hour: 1,
                    minute: 0,
                    second: 0,
                    period: AmPm::Pm
                }
            ))
        ));
    }

    #[test]
    fn parse_hour_and_minute() {
        let out = parse_am_pm_time("1:30 pm");
        assert!(matches!(
            out,
            Ok((
                "",
                AmPmTime {
                    hour: 1,
                    minute: 30,
                    second: 0,
                    period: AmPm::Pm
                }
            ))
        ));
    }

    #[test]
    fn parse_all() {
        let out = parse_am_pm_time("1:30:24 pm");
        assert!(matches!(
            out,
            Ok((
                "",
                AmPmTime {
                    hour: 1,
                    minute: 30,
                    second: 24,
                    period: AmPm::Pm
                }
            ))
        ));
    }

    #[test]
    fn parse_invalid_hour() {
        let out = parse_am_pm_time("13 am");
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "13 am",
                code: nom::error::ErrorKind::Verify,
            }))
        ));
    }

    #[test]
    fn parse_invalid_minute() {
        let out = parse_am_pm_time("12:60 am");
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "60 am",
                code: nom::error::ErrorKind::Verify,
            }))
        ));
    }

    #[test]
    fn parse_invalid_second() {
        let out = parse_am_pm_time("12:58:60 am");
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "60 am",
                code: nom::error::ErrorKind::Verify,
            }))
        ));
    }
}

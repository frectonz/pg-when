use nom::{
    character::complete::space1,
    combinator::{all_consuming, map},
    IResult, Parser,
};

use crate::{
    am_pm::{parse_am_pm, AmPm},
    parse_hms::{parse_hms, HmsFormat},
};

#[derive(Debug)]
pub struct AmPmTime {
    pub hour: u8, // 1-12
    pub minute: u8,
    pub second: u8,
    pub period: AmPm,
}

pub fn parse_am_pm_time(input: &str) -> IResult<&str, AmPmTime> {
    all_consuming(map(
        (parse_hms(HmsFormat::H12), space1, parse_am_pm),
        |((hour, minute, second), _, period)| AmPmTime {
            hour,
            minute,
            second,
            period,
        },
    ))
    .parse(input)
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

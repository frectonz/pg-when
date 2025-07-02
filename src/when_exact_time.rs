use nom::{branch::alt, combinator::map, IResult, Parser};

use crate::{
    am_pm_time::AmPmTime,
    gmt_time::{parse_gmt_time, GmtTime},
};

#[derive(Debug)]
pub enum WhenExactTime {
    AmPm(AmPmTime),
    Gmt(GmtTime),
}

impl WhenExactTime {
    pub fn parse(input: &str) -> IResult<&str, WhenExactTime> {
        alt((
            map(AmPmTime::parse, WhenExactTime::AmPm),
            map(parse_gmt_time, WhenExactTime::Gmt),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        am_pm::AmPm, am_pm_time::AmPmTime, gmt_time::GmtTime, when_exact_time::WhenExactTime,
    };

    #[test]
    fn parse_am_pm() {
        let out = WhenExactTime::parse("01:00:00 AM");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenExactTime::AmPm(AmPmTime {
                    hour: 1,
                    minute: 0,
                    second: 0,
                    period: AmPm::Am
                })
            ))
        ));
    }

    #[test]
    fn parse_gmt() {
        let out = WhenExactTime::parse("01:00:00 GMT");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenExactTime::Gmt(GmtTime {
                    hour: 1,
                    minute: 0,
                    second: 0,
                })
            ))
        ));
    }

    #[test]
    fn parse_unknow() {
        let out = WhenExactTime::parse("unknown");
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "unknown",
                code: nom::error::ErrorKind::Digit,
            }))
        ));
    }
}

use nom::{branch::alt, combinator::map, IResult, Parser};

use crate::{
    am_pm_time::{parse_am_pm_time, AmPmTime},
    gmt_time::{parse_gmt_time, GmtTime},
};

#[derive(Debug)]
pub enum WhenExactTime {
    AmPm(AmPmTime),
    Gmt(GmtTime),
}

pub fn parse_when_exact_time(input: &str) -> IResult<&str, WhenExactTime> {
    alt((
        map(parse_am_pm_time, |am_pm| WhenExactTime::AmPm(am_pm)),
        map(parse_gmt_time, |gmt| WhenExactTime::Gmt(gmt)),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::{
        am_pm::AmPm,
        am_pm_time::AmPmTime,
        gmt_time::GmtTime,
        when_exact_time::{parse_when_exact_time, WhenExactTime},
    };

    #[test]
    fn parse_am_pm() {
        let out = parse_when_exact_time("01:00:00 AM");
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
        let out = parse_when_exact_time("01:00:00 GMT");
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
        let out = parse_when_exact_time("unknown");
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "unknown",
                code: nom::error::ErrorKind::Digit,
            }))
        ));
    }
}

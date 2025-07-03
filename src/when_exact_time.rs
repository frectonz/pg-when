use nom::{branch::alt, combinator::map, IResult, Parser};

use crate::{AmPmTime, GmtTime};

#[derive(Debug)]
pub enum WhenExactTime {
    AmPm(AmPmTime),
    Gmt(GmtTime),
}

impl WhenExactTime {
    pub fn parse(input: &str) -> IResult<&str, WhenExactTime> {
        alt((
            map(AmPmTime::parse, WhenExactTime::AmPm),
            map(GmtTime::parse, WhenExactTime::Gmt),
        ))
        .parse(input)
    }

    pub fn with_zoned(&self, zoned: jiff::Zoned) -> Result<jiff::Zoned, jiff::Error> {
        match self {
            WhenExactTime::AmPm(am_pm_time) => am_pm_time.with_zoned(zoned),
            WhenExactTime::Gmt(gmt_time) => gmt_time.with_zoned(zoned),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{AmPm, AmPmTime, GmtTime, WhenExactTime};

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

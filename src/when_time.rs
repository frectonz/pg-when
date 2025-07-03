use nom::{branch::alt, combinator::map, IResult, Parser};

use crate::{WhenExactTime, WhenRelativeTime};

#[derive(Debug)]
pub enum WhenTime {
    Relative(WhenRelativeTime),
    Exact(WhenExactTime),
}

impl WhenTime {
    pub fn parse(input: &str) -> IResult<&str, WhenTime> {
        alt((
            map(WhenRelativeTime::parse, WhenTime::Relative),
            map(WhenExactTime::parse, WhenTime::Exact),
        ))
        .parse(input)
    }

    pub fn to_timestamp(&self, timezone: jiff::tz::TimeZone) -> Result<jiff::Zoned, jiff::Error> {
        let now = jiff::Zoned::new(jiff::Timestamp::now(), timezone);
        self.with_zoned(now)
    }

    pub fn with_zoned(&self, zoned: jiff::Zoned) -> Result<jiff::Zoned, jiff::Error> {
        match self {
            WhenTime::Relative(when_relative_time) => when_relative_time.with_zoned(zoned),
            WhenTime::Exact(when_exact_time) => when_exact_time.with_zoned(zoned),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{GmtTime, TimeDuration, WhenExactTime, WhenRelativeTime, WhenTime};

    #[test]
    fn parse_relative() {
        let out = WhenTime::parse("the previous 10 mins");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenTime::Relative(WhenRelativeTime::PreviousDuration(TimeDuration::Minutes(
                    10
                )))
            ))
        ));
    }

    #[test]
    fn parse_exact() {
        let out = WhenTime::parse("10:01:30 GMT");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenTime::Exact(WhenExactTime::Gmt(GmtTime {
                    hour: 10,
                    minute: 1,
                    second: 30
                }))
            ))
        ));
    }
}

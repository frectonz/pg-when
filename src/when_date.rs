use nom::{branch::alt, combinator::map, IResult, Parser};

use crate::{WhenExactDate, WhenRelativeDate};

#[derive(Debug)]
pub enum WhenDate {
    Relative(WhenRelativeDate),
    Exact(WhenExactDate),
}

impl WhenDate {
    pub fn parse(input: &str) -> IResult<&str, WhenDate> {
        alt((
            map(WhenRelativeDate::parse, WhenDate::Relative),
            map(WhenExactDate::parse, WhenDate::Exact),
        ))
        .parse(input)
    }

    pub fn to_timestamp(&self, timezone: jiff::tz::TimeZone) -> Result<jiff::Zoned, jiff::Error> {
        match self {
            WhenDate::Relative(when_relative_date) => when_relative_date.to_timestamp(timezone),
            WhenDate::Exact(when_exact_date) => when_exact_date.to_timestamp(timezone),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{DateDuration, WhenDate, WhenExactDate, WhenRelativeDate};

    #[test]
    fn parse_relative() {
        let out = WhenDate::parse("yesterday");
        assert!(matches!(
            out,
            Ok(("", WhenDate::Relative(WhenRelativeDate::Yesterday)))
        ));

        let out = WhenDate::parse("10 days ago");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenDate::Relative(WhenRelativeDate::Ago(DateDuration::Days(10)))
            ))
        ));
    }

    #[test]
    fn parse_exact() {
        let out = WhenDate::parse("10/10/2001");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenDate::Exact(WhenExactDate {
                    year: 2001,
                    month: 10,
                    day: 10
                })
            ))
        ));

        let out = WhenDate::parse("10-10-2001");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenDate::Exact(WhenExactDate {
                    year: 2001,
                    month: 10,
                    day: 10
                })
            ))
        ));
    }

    #[test]
    fn parse_unknown() {
        let out = WhenDate::parse("unknown");

        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "unknown",
                code: nom::error::ErrorKind::Digit,
            }))
        ));
    }
}

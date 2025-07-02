use nom::{branch::alt, combinator::map, IResult, Parser};

use crate::{
    when_exact_date::{parse_when_exact_date, WhenExactDate},
    when_relative_date::WhenRelativeDate,
};

#[derive(Debug)]
pub enum WhenDate {
    Relative(WhenRelativeDate),
    Exact(WhenExactDate),
}

impl WhenDate {
    pub fn parse(input: &str) -> IResult<&str, WhenDate> {
        alt((
            map(WhenRelativeDate::parse, WhenDate::Relative),
            map(parse_when_exact_date, WhenDate::Exact),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        date_duration::DateDuration, when_date::WhenDate, when_exact_date::WhenExactDate,
        when_relative_date::WhenRelativeDate,
    };

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

use nom::{branch::alt, combinator::map, IResult, Parser};

use crate::{
    when_exact_date::{parse_when_exact_date, WhenExactDate},
    when_relative_date::{parse_when_relative_date, WhenRelativeDate},
};

#[derive(Debug)]
pub enum WhenDate {
    Relative(WhenRelativeDate),
    Exact(WhenExactDate),
}

pub fn parse_when_date(input: &str) -> IResult<&str, WhenDate> {
    alt((
        map(parse_when_relative_date, |r| WhenDate::Relative(r)),
        map(parse_when_exact_date, |e| WhenDate::Exact(e)),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::{
        date_duration::DateDuration,
        when_date::{parse_when_date, WhenDate},
        when_exact_date::WhenExactDate,
        when_relative_date::WhenRelativeDate,
    };

    #[test]
    fn parse_relative() {
        let out = parse_when_date("yesterday");
        assert!(matches!(
            out,
            Ok(("", WhenDate::Relative(WhenRelativeDate::Yesterday)))
        ));

        let out = parse_when_date("10 days ago");
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
        let out = parse_when_date("10/10/2001");
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

        let out = parse_when_date("10-10-2001");
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
        let out = parse_when_date("unkown");

        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "unkown",
                code: nom::error::ErrorKind::Digit,
            }))
        ));
    }
}

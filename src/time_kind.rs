use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult, Parser};

#[derive(Debug)]
pub enum TimeKind {
    Hour,
    Minute,
    Second,
}

impl TimeKind {
    pub fn parse(input: &str) -> IResult<&str, TimeKind> {
        alt((
            map(tag("hour"), |_| TimeKind::Hour),
            map(tag("minute"), |_| TimeKind::Minute),
            map(tag("second"), |_| TimeKind::Second),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::time_kind::TimeKind;

    #[test]
    fn parse_week() {
        let out = TimeKind::parse("hour");
        assert!(matches!(out, Ok(("", TimeKind::Hour))));
    }

    #[test]
    fn parse_month() {
        let out = TimeKind::parse("minute");
        assert!(matches!(out, Ok(("", TimeKind::Minute))));
    }

    #[test]
    fn parse_year() {
        let out = TimeKind::parse("second");
        assert!(matches!(out, Ok(("", TimeKind::Second))));
    }

    #[test]
    fn parse_unknown() {
        let out = TimeKind::parse("unknown");

        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "unknown",
                code: nom::error::ErrorKind::Tag,
            }))
        ));
    }
}

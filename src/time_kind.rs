use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult, Parser};

#[derive(Debug)]
pub enum TimeKind {
    Hour,
    Minute,
    Second,
}

pub fn parse_time_kind(input: &str) -> IResult<&str, TimeKind> {
    alt((
        map(tag("hour"), |_| TimeKind::Hour),
        map(tag("minute"), |_| TimeKind::Minute),
        map(tag("second"), |_| TimeKind::Second),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::time_kind::{parse_time_kind, TimeKind};

    #[test]
    fn parse_week() {
        let out = parse_time_kind("hour");
        assert!(matches!(out, Ok(("", TimeKind::Hour))));
    }

    #[test]
    fn parse_month() {
        let out = parse_time_kind("minute");
        assert!(matches!(out, Ok(("", TimeKind::Minute))));
    }

    #[test]
    fn parse_year() {
        let out = parse_time_kind("second");
        assert!(matches!(out, Ok(("", TimeKind::Second))));
    }

    #[test]
    fn parse_unknown() {
        let out = parse_time_kind("unknown");

        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "unknown",
                code: nom::error::ErrorKind::Tag,
            }))
        ));
    }
}

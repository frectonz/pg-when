use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult, Parser};

#[derive(Debug)]
pub enum DateKind {
    Week,
    Month,
    Year,
}

pub fn parse_date_kind(input: &str) -> IResult<&str, DateKind> {
    alt((
        map(tag("week"), |_| DateKind::Week),
        map(tag("month"), |_| DateKind::Month),
        map(tag("year"), |_| DateKind::Year),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::date_kind::{parse_date_kind, DateKind};

    #[test]
    fn parse_week() {
        let out = parse_date_kind("week");
        assert!(matches!(out, Ok(("", DateKind::Week))));
    }

    #[test]
    fn parse_month() {
        let out = parse_date_kind("month");
        assert!(matches!(out, Ok(("", DateKind::Month))));
    }

    #[test]
    fn parse_year() {
        let out = parse_date_kind("year");
        assert!(matches!(out, Ok(("", DateKind::Year))));
    }

    #[test]
    fn parse_unknown() {
        let out = parse_date_kind("unknown");

        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "unknown",
                code: nom::error::ErrorKind::Tag,
            }))
        ));
    }
}

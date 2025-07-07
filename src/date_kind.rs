use nom::{branch::alt, bytes::complete::tag, combinator::map, Parser};

use crate::NomResult;

#[derive(Debug)]
pub enum DateKind {
    Week,
    Month,
    Year,
}

impl DateKind {
    pub fn parse(input: &str) -> NomResult<&str, DateKind> {
        alt((
            map(tag("week"), |_| DateKind::Week),
            map(tag("month"), |_| DateKind::Month),
            map(tag("year"), |_| DateKind::Year),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::DateKind;

    #[test]
    fn parse_week() {
        let out = DateKind::parse("week");
        assert!(matches!(out, Ok(("", DateKind::Week))));
    }

    #[test]
    fn parse_month() {
        let out = DateKind::parse("month");
        assert!(matches!(out, Ok(("", DateKind::Month))));
    }

    #[test]
    fn parse_year() {
        let out = DateKind::parse("year");
        assert!(matches!(out, Ok(("", DateKind::Year))));
    }

    #[test]
    fn parse_unknown() {
        let out = DateKind::parse("unknown");
        dbg!(out);
        assert!(false);
    }
}

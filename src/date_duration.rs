use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{map, map_res},
    Parser,
};

use crate::NomResult;

#[derive(Debug)]
pub enum DateDuration {
    Days(u32),
    Weeks(u32),
    Months(u32),
    Years(u32),
}

impl DateDuration {
    pub fn parse(input: &str) -> NomResult<&str, DateDuration> {
        map(
            (
                map_res(digit1, |s: &str| s.parse::<u32>()),
                space0,
                alt((
                    tag("days"),
                    tag("day"),
                    tag("weeks"),
                    tag("week"),
                    tag("months"),
                    tag("month"),
                    tag("years"),
                    tag("year"),
                )),
            ),
            |(num, _, unit)| match unit {
                "days" | "day" => DateDuration::Days(num),
                "weeks" | "week" => DateDuration::Weeks(num),
                "months" | "month" => DateDuration::Months(num),
                "years" | "year" => DateDuration::Years(num),
                _ => unreachable!("all patterns have been matched"),
            },
        )
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use nom::Finish;
    use nom_language::error::convert_error;

    use crate::DateDuration;

    #[test]
    fn parse_days() {
        let out = DateDuration::parse("0 days");
        assert!(matches!(out, Ok(("", DateDuration::Days(0)))));

        let out = DateDuration::parse("1 day");
        assert!(matches!(out, Ok(("", DateDuration::Days(1)))));

        let out = DateDuration::parse("100 days");
        assert!(matches!(out, Ok(("", DateDuration::Days(100)))));

        let out = DateDuration::parse("200 day");
        assert!(matches!(out, Ok(("", DateDuration::Days(200)))));

        let out = DateDuration::parse("300days");
        assert!(matches!(out, Ok(("", DateDuration::Days(300)))));
    }

    #[test]
    fn parse_weeks() {
        let out = DateDuration::parse("0 weeks");
        assert!(matches!(out, Ok(("", DateDuration::Weeks(0)))));

        let out = DateDuration::parse("1 week");
        assert!(matches!(out, Ok(("", DateDuration::Weeks(1)))));

        let out = DateDuration::parse("100 weeks");
        assert!(matches!(out, Ok(("", DateDuration::Weeks(100)))));

        let out = DateDuration::parse("200 week");
        assert!(matches!(out, Ok(("", DateDuration::Weeks(200)))));

        let out = DateDuration::parse("300weeks");
        assert!(matches!(out, Ok(("", DateDuration::Weeks(300)))));
    }

    #[test]
    fn parse_months() {
        let out = DateDuration::parse("0 months");
        assert!(matches!(out, Ok(("", DateDuration::Months(0)))));

        let out = DateDuration::parse("1 month");
        assert!(matches!(out, Ok(("", DateDuration::Months(1)))));

        let out = DateDuration::parse("100 months");
        assert!(matches!(out, Ok(("", DateDuration::Months(100)))));

        let out = DateDuration::parse("200 month");
        assert!(matches!(out, Ok(("", DateDuration::Months(200)))));

        let out = DateDuration::parse("300months");
        assert!(matches!(out, Ok(("", DateDuration::Months(300)))));
    }

    #[test]
    fn parse_years() {
        let out = DateDuration::parse("0 years");
        assert!(matches!(out, Ok(("", DateDuration::Years(0)))));

        let out = DateDuration::parse("1 year");
        assert!(matches!(out, Ok(("", DateDuration::Years(1)))));

        let out = DateDuration::parse("100 years");
        assert!(matches!(out, Ok(("", DateDuration::Years(100)))));

        let out = DateDuration::parse("200 year");
        assert!(matches!(out, Ok(("", DateDuration::Years(200)))));

        let out = DateDuration::parse("300years");
        assert!(matches!(out, Ok(("", DateDuration::Years(300)))));
    }

    #[test]
    fn parse_unknown() {
        let input = "unknown";
        let err = DateDuration::parse(input).finish().unwrap_err();
        let err = convert_error(input, err);
        assert_snapshot!(err);
    }
}

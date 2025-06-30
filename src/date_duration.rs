use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{map, map_res},
    IResult, Parser,
};

#[derive(Debug)]
pub enum DateDuration {
    Days(u32),
    Weeks(u32),
    Months(u32),
}

pub fn parse_date_duration(input: &str) -> IResult<&str, DateDuration> {
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
            )),
        ),
        |(num, _, unit)| match unit {
            "days" | "day" => DateDuration::Days(num),
            "weeks" | "week" => DateDuration::Weeks(num),
            "months" | "month" => DateDuration::Months(num),
            _ => unreachable!("all patterns have been matched"),
        },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::date_duration::{parse_date_duration, DateDuration};

    #[test]
    fn parse_days() {
        let out = parse_date_duration("0 days");
        assert!(matches!(out, Ok(("", DateDuration::Days(0)))));

        let out = parse_date_duration("1 day");
        assert!(matches!(out, Ok(("", DateDuration::Days(1)))));

        let out = parse_date_duration("100 days");
        assert!(matches!(out, Ok(("", DateDuration::Days(100)))));

        let out = parse_date_duration("200 day");
        assert!(matches!(out, Ok(("", DateDuration::Days(200)))));

        let out = parse_date_duration("300days");
        assert!(matches!(out, Ok(("", DateDuration::Days(300)))));
    }

    #[test]
    fn parse_weeks() {
        let out = parse_date_duration("0 weeks");
        assert!(matches!(out, Ok(("", DateDuration::Weeks(0)))));

        let out = parse_date_duration("1 week");
        assert!(matches!(out, Ok(("", DateDuration::Weeks(1)))));

        let out = parse_date_duration("100 weeks");
        assert!(matches!(out, Ok(("", DateDuration::Weeks(100)))));

        let out = parse_date_duration("200 week");
        assert!(matches!(out, Ok(("", DateDuration::Weeks(200)))));

        let out = parse_date_duration("300weeks");
        assert!(matches!(out, Ok(("", DateDuration::Weeks(300)))));
    }

    #[test]
    fn parse_months() {
        let out = parse_date_duration("0 months");
        assert!(matches!(out, Ok(("", DateDuration::Months(0)))));

        let out = parse_date_duration("1 month");
        assert!(matches!(out, Ok(("", DateDuration::Months(1)))));

        let out = parse_date_duration("100 months");
        assert!(matches!(out, Ok(("", DateDuration::Months(100)))));

        let out = parse_date_duration("200 month");
        assert!(matches!(out, Ok(("", DateDuration::Months(200)))));

        let out = parse_date_duration("300months");
        assert!(matches!(out, Ok(("", DateDuration::Months(300)))));
    }
}

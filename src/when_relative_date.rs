use nom::{
    branch::alt, bytes::complete::tag, character::complete::space1, combinator::map,
    sequence::separated_pair, IResult, Parser,
};

use crate::{
    date_duration::{parse_date_duration, DateDuration},
    date_kind::{parse_date_kind, DateKind},
    weekday::{parse_weekday, Weekday},
};

#[derive(Debug)]
pub enum WhenRelativeDate {
    Yesterday,
    Tomorrow,

    LastDay(Weekday),
    NextDay(Weekday),
    ThisDay(Weekday),

    LastKind(DateKind),
    NextKind(DateKind),
    ThisKind(DateKind),

    Ago(DateDuration),
    In(DateDuration),
}

pub fn parse_when_relative_date(input: &str) -> IResult<&str, WhenRelativeDate> {
    alt((
        map(tag("yesterday"), |_| WhenRelativeDate::Yesterday),
        map(tag("tomorrow"), |_| WhenRelativeDate::Tomorrow),
        map(
            separated_pair(tag("last"), space1, parse_weekday),
            |(_, w)| WhenRelativeDate::LastDay(w),
        ),
        map(
            separated_pair(tag("next"), space1, parse_weekday),
            |(_, w)| WhenRelativeDate::NextDay(w),
        ),
        map(
            separated_pair(tag("this"), space1, parse_weekday),
            |(_, w)| WhenRelativeDate::ThisDay(w),
        ),
        map(
            separated_pair(tag("last"), space1, parse_date_kind),
            |(_, k)| WhenRelativeDate::LastKind(k),
        ),
        map(
            separated_pair(tag("next"), space1, parse_date_kind),
            |(_, k)| WhenRelativeDate::NextKind(k),
        ),
        map(
            separated_pair(tag("this"), space1, parse_date_kind),
            |(_, k)| WhenRelativeDate::ThisKind(k),
        ),
        map(
            separated_pair(parse_date_duration, space1, tag("ago")),
            |(d, _)| WhenRelativeDate::Ago(d),
        ),
        map(
            separated_pair(tag("in"), space1, parse_date_duration),
            |(_, d)| WhenRelativeDate::In(d),
        ),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::{
        date_duration::DateDuration,
        date_kind::DateKind,
        weekday::Weekday,
        when_relative_date::{parse_when_relative_date, WhenRelativeDate},
    };

    #[test]
    fn parse_yesterday() {
        let out = parse_when_relative_date("yesterday");
        assert!(matches!(out, Ok(("", WhenRelativeDate::Yesterday))));
    }

    #[test]
    fn parse_tomorrow() {
        let out = parse_when_relative_date("tomorrow");
        assert!(matches!(out, Ok(("", WhenRelativeDate::Tomorrow))));
    }

    #[test]
    fn parse_last_weekday() {
        let out = parse_when_relative_date("last monday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::LastDay(Weekday::Monday)))
        ));

        let out = parse_when_relative_date("last     tuesday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::LastDay(Weekday::Tuesday)))
        ));

        let out = parse_when_relative_date("last sunday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::LastDay(Weekday::Sunday)))
        ));
    }

    #[test]
    fn parse_next_weekday() {
        let out = parse_when_relative_date("next monday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::NextDay(Weekday::Monday)))
        ));

        let out = parse_when_relative_date("next     tuesday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::NextDay(Weekday::Tuesday)))
        ));

        let out = parse_when_relative_date("next sunday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::NextDay(Weekday::Sunday)))
        ));
    }

    #[test]
    fn parse_this_weekday() {
        let out = parse_when_relative_date("this monday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::ThisDay(Weekday::Monday)))
        ));

        let out = parse_when_relative_date("this     tuesday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::ThisDay(Weekday::Tuesday)))
        ));

        let out = parse_when_relative_date("this sunday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::ThisDay(Weekday::Sunday)))
        ));
    }

    #[test]
    fn parse_last_kind() {
        let out = parse_when_relative_date("last week");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::LastKind(DateKind::Week)))
        ));

        let out = parse_when_relative_date("last     month");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::LastKind(DateKind::Month)))
        ));

        let out = parse_when_relative_date("last year");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::LastKind(DateKind::Year)))
        ));
    }

    #[test]
    fn parse_next_kind() {
        let out = parse_when_relative_date("next week");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::NextKind(DateKind::Week)))
        ));

        let out = parse_when_relative_date("next     month");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::NextKind(DateKind::Month)))
        ));

        let out = parse_when_relative_date("next year");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::NextKind(DateKind::Year)))
        ));
    }

    #[test]
    fn parse_this_kind() {
        let out = parse_when_relative_date("this week");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::ThisKind(DateKind::Week)))
        ));

        let out = parse_when_relative_date("this     month");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::ThisKind(DateKind::Month)))
        ));

        let out = parse_when_relative_date("this year");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::ThisKind(DateKind::Year)))
        ));
    }

    #[test]
    fn parse_ago() {
        let out = parse_when_relative_date("10 days ago");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::Ago(DateDuration::Days(10))))
        ));

        let out = parse_when_relative_date("10 weeks ago");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::Ago(DateDuration::Weeks(10))))
        ));

        let out = parse_when_relative_date("10 months ago");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::Ago(DateDuration::Months(10))))
        ));
    }

    #[test]
    fn parse_in() {
        let out = parse_when_relative_date("in 10 days");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::In(DateDuration::Days(10))))
        ));

        let out = parse_when_relative_date("in 10 weeks");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::In(DateDuration::Weeks(10))))
        ));

        let out = parse_when_relative_date("in 10 months");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::In(DateDuration::Months(10))))
        ));
    }

    #[test]
    fn parse_unknown() {
        let out = parse_when_relative_date("unkown");

        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "unkown",
                code: nom::error::ErrorKind::Tag,
            }))
        ));
    }
}

use nom::{
    branch::alt, bytes::complete::tag, character::complete::space1, combinator::map,
    sequence::separated_pair, IResult, Parser,
};

use crate::{date_duration::DateDuration, date_kind::DateKind, weekday::Weekday};

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

impl WhenRelativeDate {
    pub fn parse(input: &str) -> IResult<&str, WhenRelativeDate> {
        alt((
            map(tag("yesterday"), |_| WhenRelativeDate::Yesterday),
            map(tag("tomorrow"), |_| WhenRelativeDate::Tomorrow),
            map(
                separated_pair(tag("last"), space1, Weekday::parse),
                |(_, w)| WhenRelativeDate::LastDay(w),
            ),
            map(
                separated_pair(tag("next"), space1, Weekday::parse),
                |(_, w)| WhenRelativeDate::NextDay(w),
            ),
            map(
                separated_pair(tag("this"), space1, Weekday::parse),
                |(_, w)| WhenRelativeDate::ThisDay(w),
            ),
            map(
                separated_pair(tag("last"), space1, DateKind::parse),
                |(_, k)| WhenRelativeDate::LastKind(k),
            ),
            map(
                separated_pair(tag("next"), space1, DateKind::parse),
                |(_, k)| WhenRelativeDate::NextKind(k),
            ),
            map(
                separated_pair(tag("this"), space1, DateKind::parse),
                |(_, k)| WhenRelativeDate::ThisKind(k),
            ),
            map(
                separated_pair(DateDuration::parse, space1, tag("ago")),
                |(d, _)| WhenRelativeDate::Ago(d),
            ),
            map(
                separated_pair(tag("in"), space1, DateDuration::parse),
                |(_, d)| WhenRelativeDate::In(d),
            ),
        ))
        .parse(input)
    }

    pub fn to_timestamp(&self, timezone: jiff::tz::TimeZone) -> Result<jiff::Zoned, jiff::Error> {
        use jiff::ToSpan;

        let now = jiff::Zoned::new(jiff::Timestamp::now(), timezone);

        match self {
            WhenRelativeDate::Yesterday => now.yesterday(),
            WhenRelativeDate::Tomorrow => now.tomorrow(),
            WhenRelativeDate::LastDay(weekday) => {
                let today_weekday = now.weekday().to_monday_one_offset();
                let target_weekday = weekday.number_from_monday();

                let diff_days = if today_weekday > target_weekday {
                    today_weekday - target_weekday
                } else {
                    7 - (target_weekday - today_weekday)
                };

                now.checked_sub(diff_days.day())
            }
            WhenRelativeDate::NextDay(weekday) => {
                let today_weekday = now.weekday().to_monday_one_offset();
                let target_weekday = weekday.number_from_monday();

                let diff = if target_weekday > today_weekday {
                    target_weekday - today_weekday
                } else {
                    7 - (today_weekday - target_weekday)
                };

                now.checked_add(diff.days())
            }
            WhenRelativeDate::ThisDay(weekday) => {
                let today_weekday = now.weekday().to_monday_one_offset();
                let target_weekday = weekday.number_from_monday();

                let diff = target_weekday - today_weekday;

                now.checked_add(diff.days())
            }
            WhenRelativeDate::LastKind(date_kind) => match date_kind {
                DateKind::Week => now.checked_sub(1.week()),
                DateKind::Month => now.checked_sub(1.month()),
                DateKind::Year => now.checked_sub(1.year()),
            },
            WhenRelativeDate::NextKind(date_kind) => match date_kind {
                DateKind::Week => now.checked_add(1.week()),
                DateKind::Month => now.checked_add(1.month()),
                DateKind::Year => now.checked_add(1.year()),
            },
            WhenRelativeDate::ThisKind(date_kind) => match date_kind {
                DateKind::Week => {
                    let offset = now.weekday().to_monday_zero_offset();
                    now.checked_sub(offset.days())
                }
                DateKind::Month => {
                    let offset = now.day();
                    now.checked_sub(offset.days())
                }
                DateKind::Year => {
                    let offset = now.day_of_year();
                    now.checked_sub(offset.days())
                }
            },
            WhenRelativeDate::Ago(date_duration) => match date_duration {
                DateDuration::Days(days) => now.checked_sub((*days as i32).day()),
                DateDuration::Weeks(weeks) => now.checked_sub((*weeks as i32).week()),
                DateDuration::Months(months) => now.checked_sub((*months as i32).week()),
            },
            WhenRelativeDate::In(date_duration) => match date_duration {
                DateDuration::Days(days) => now.checked_add((*days as i32).day()),
                DateDuration::Weeks(weeks) => now.checked_add((*weeks as i32).week()),
                DateDuration::Months(months) => now.checked_add((*months as i32).week()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        date_duration::DateDuration, date_kind::DateKind, weekday::Weekday,
        when_relative_date::WhenRelativeDate,
    };

    #[test]
    fn parse_yesterday() {
        let out = WhenRelativeDate::parse("yesterday");
        assert!(matches!(out, Ok(("", WhenRelativeDate::Yesterday))));
    }

    #[test]
    fn parse_tomorrow() {
        let out = WhenRelativeDate::parse("tomorrow");
        assert!(matches!(out, Ok(("", WhenRelativeDate::Tomorrow))));
    }

    #[test]
    fn parse_last_weekday() {
        let out = WhenRelativeDate::parse("last monday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::LastDay(Weekday::Monday)))
        ));

        let out = WhenRelativeDate::parse("last     tuesday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::LastDay(Weekday::Tuesday)))
        ));

        let out = WhenRelativeDate::parse("last sunday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::LastDay(Weekday::Sunday)))
        ));
    }

    #[test]
    fn parse_next_weekday() {
        let out = WhenRelativeDate::parse("next monday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::NextDay(Weekday::Monday)))
        ));

        let out = WhenRelativeDate::parse("next     tuesday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::NextDay(Weekday::Tuesday)))
        ));

        let out = WhenRelativeDate::parse("next sunday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::NextDay(Weekday::Sunday)))
        ));
    }

    #[test]
    fn parse_this_weekday() {
        let out = WhenRelativeDate::parse("this monday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::ThisDay(Weekday::Monday)))
        ));

        let out = WhenRelativeDate::parse("this     tuesday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::ThisDay(Weekday::Tuesday)))
        ));

        let out = WhenRelativeDate::parse("this sunday");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::ThisDay(Weekday::Sunday)))
        ));
    }

    #[test]
    fn parse_last_kind() {
        let out = WhenRelativeDate::parse("last week");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::LastKind(DateKind::Week)))
        ));

        let out = WhenRelativeDate::parse("last     month");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::LastKind(DateKind::Month)))
        ));

        let out = WhenRelativeDate::parse("last year");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::LastKind(DateKind::Year)))
        ));
    }

    #[test]
    fn parse_next_kind() {
        let out = WhenRelativeDate::parse("next week");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::NextKind(DateKind::Week)))
        ));

        let out = WhenRelativeDate::parse("next     month");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::NextKind(DateKind::Month)))
        ));

        let out = WhenRelativeDate::parse("next year");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::NextKind(DateKind::Year)))
        ));
    }

    #[test]
    fn parse_this_kind() {
        let out = WhenRelativeDate::parse("this week");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::ThisKind(DateKind::Week)))
        ));

        let out = WhenRelativeDate::parse("this     month");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::ThisKind(DateKind::Month)))
        ));

        let out = WhenRelativeDate::parse("this year");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::ThisKind(DateKind::Year)))
        ));
    }

    #[test]
    fn parse_ago() {
        let out = WhenRelativeDate::parse("10 days ago");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::Ago(DateDuration::Days(10))))
        ));

        let out = WhenRelativeDate::parse("10 weeks ago");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::Ago(DateDuration::Weeks(10))))
        ));

        let out = WhenRelativeDate::parse("10 months ago");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::Ago(DateDuration::Months(10))))
        ));
    }

    #[test]
    fn parse_in() {
        let out = WhenRelativeDate::parse("in 10 days");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::In(DateDuration::Days(10))))
        ));

        let out = WhenRelativeDate::parse("in 10 weeks");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::In(DateDuration::Weeks(10))))
        ));

        let out = WhenRelativeDate::parse("in 10 months");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeDate::In(DateDuration::Months(10))))
        ));
    }

    #[test]
    fn parse_unknown() {
        let out = WhenRelativeDate::parse("unknown");

        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "unknown",
                code: nom::error::ErrorKind::Tag,
            }))
        ));
    }

    #[test]
    fn parse_yesterday_timestamp() {
        let (_, out) = WhenRelativeDate::parse("yesterday").unwrap();
        let timestamp = out.to_timestamp(jiff::tz::TimeZone::UTC);
        assert!(timestamp.is_ok());
    }
}

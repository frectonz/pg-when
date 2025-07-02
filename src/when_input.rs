use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    combinator::{eof, map},
    IResult, Parser,
};

use crate::{
    when_date::{parse_when_date, WhenDate},
    when_time::{parse_when_time, WhenTime},
    when_timezone::{parse_when_timezone, WhenTimezone},
};

#[derive(Debug)]
pub struct WhenInput {
    pub time: WhenInputTime,
    pub timezone: Option<WhenTimezone>,
}

#[derive(Debug)]
pub enum WhenInputTime {
    OnlyDate(WhenDate),
    OnlyTime(WhenTime),
    DateAndTime { date: WhenDate, time: WhenTime },
}

impl WhenInputTime {
    pub fn parse(input: &str) -> IResult<&str, WhenInputTime> {
        alt((
            map(
                (parse_when_date, space1, tag("at"), space1, parse_when_time),
                |(date, _, _, _, time)| WhenInputTime::DateAndTime { date, time },
            ),
            map(parse_when_date, |date| WhenInputTime::OnlyDate(date)),
            map(parse_when_time, |time| WhenInputTime::OnlyTime(time)),
        ))
        .parse(input)
    }
}

impl WhenInput {
    fn parse(input: &str) -> IResult<&str, WhenInput> {
        alt((
            map((WhenInputTime::parse, eof), |(time, _)| WhenInput {
                time,
                timezone: None,
            }),
            map(
                (
                    WhenInputTime::parse,
                    space1,
                    tag("in"),
                    space1,
                    parse_when_timezone,
                ),
                |(time, _, _, _, timezone)| WhenInput {
                    time,
                    timezone: Some(timezone),
                },
            ),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        date_duration::DateDuration,
        time_duration::TimeDuration,
        time_kind::TimeKind,
        when_date::WhenDate,
        when_input::{WhenInput, WhenInputTime},
        when_named_timezone::WhenNamedTimezone,
        when_relative_date::WhenRelativeDate,
        when_relative_time::WhenRelativeTime,
        when_time::WhenTime,
        when_timezone::WhenTimezone,
    };

    #[test]
    fn parse_date_only() {
        let out = WhenInput::parse("in 10 days");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenInput {
                    time: WhenInputTime::OnlyDate(WhenDate::Relative(WhenRelativeDate::In(
                        DateDuration::Days(10)
                    ))),
                    timezone: None
                }
            ))
        ));
    }

    #[test]
    fn parse_time_only() {
        let out = WhenInput::parse("previous 10 hours");
        dbg!(&out);
        assert!(matches!(
            out,
            Ok((
                "",
                WhenInput {
                    time: WhenInputTime::OnlyTime(WhenTime::Relative(
                        WhenRelativeTime::PreviousDuration(TimeDuration::Hours(10))
                    )),
                    timezone: None
                }
            ))
        ));
    }

    #[test]
    fn parse_date_and_time() {
        let out = WhenInput::parse("10 days ago at the next 10 hours");
        dbg!(&out);
        assert!(matches!(
            out,
            Ok((
                "",
                WhenInput {
                    time: WhenInputTime::DateAndTime {
                        date: WhenDate::Relative(WhenRelativeDate::Ago(DateDuration::Days(10))),
                        time: WhenTime::Relative(WhenRelativeTime::NextDuration(
                            TimeDuration::Hours(10)
                        )),
                    },
                    timezone: None,
                },
            ),)
        ));
    }

    #[test]
    fn parse_date_time_timezone() {
        let out = WhenInput::parse("10 days ago at the previous hour in Africa/Addis_Ababa");

        let region: Box<str> = "Africa".into();
        let city: Box<str> = "Addis_Ababa".into();

        assert!(matches!(
            out,
            Ok((
                "",
                WhenInput {
                    time: WhenInputTime::DateAndTime {
                        date: WhenDate::Relative(WhenRelativeDate::Ago(DateDuration::Days(10))),
                        time: WhenTime::Relative(WhenRelativeTime::PreviousKind(TimeKind::Hour)),
                    },
                    timezone: Some(WhenTimezone::Named(WhenNamedTimezone { region, city })),
                },
            ),)
        ));
    }
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    combinator::{eof, map},
    IResult, Parser,
};

use crate::{WhenDate, WhenTime, WhenTimezone};

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
                (WhenDate::parse, space1, tag("at"), space1, WhenTime::parse),
                |(date, _, _, _, time)| WhenInputTime::DateAndTime { date, time },
            ),
            map(WhenDate::parse, WhenInputTime::OnlyDate),
            map(WhenTime::parse, WhenInputTime::OnlyTime),
        ))
        .parse(input)
    }

    pub fn to_timestamp(&self, timezone: jiff::tz::TimeZone) -> Result<jiff::Zoned, jiff::Error> {
        match self {
            WhenInputTime::OnlyDate(when_date) => when_date.to_timestamp(timezone),
            WhenInputTime::OnlyTime(when_time) => when_time.to_timestamp(timezone),
            WhenInputTime::DateAndTime { date, time } => {
                let zoned = date.to_timestamp(timezone)?;
                time.with_zoned(zoned)
            }
        }
    }
}

impl WhenInput {
    pub fn parse(input: &str) -> IResult<&str, WhenInput> {
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
                    WhenTimezone::parse,
                ),
                |(time, _, _, _, timezone)| WhenInput {
                    time,
                    timezone: Some(timezone),
                },
            ),
        ))
        .parse(input)
    }

    pub fn to_timestamp(&self) -> Result<jiff::Zoned, jiff::Error> {
        let timezone = match self.timezone {
            Some(ref timezone) => timezone.to_timezone()?,
            None => jiff::tz::TimeZone::UTC,
        };

        let zoned = self.time.to_timestamp(timezone)?;
        Ok(zoned)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        DateDuration, TimeDuration, TimeKind, WhenDate, WhenNamedTimezone, WhenRelativeDate,
        WhenRelativeTime, WhenTime, WhenTimezone, {WhenInput, WhenInputTime},
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
    #[allow(unused_variables)]
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

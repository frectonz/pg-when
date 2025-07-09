use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    combinator::{eof, map},
    Parser,
};

use crate::{NomResult, WhenDate, WhenTime, WhenTimezone};

#[derive(Debug)]
pub struct WhenInput {
    pub time: WhenInputTime,
    pub timezone: Option<WhenTimezone>,
}

#[derive(Debug)]
pub enum WhenInputTime {
    Now,
    OnlyDate(WhenDate),
    OnlyTime(WhenTime),
    DateAndTime { date: WhenDate, time: WhenTime },
}

impl WhenInputTime {
    pub fn parse(input: &str) -> NomResult<&str, WhenInputTime> {
        alt((
            map(tag("now"), |_| WhenInputTime::Now),
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
            WhenInputTime::Now => Ok(jiff::Zoned::now().with_time_zone(timezone)),
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
    pub fn parse(input: &str) -> NomResult<&str, WhenInput> {
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
        AmPm, AmPmTime, DateDuration, TimeDuration, TimeKind, WhenDate, WhenExactTime, WhenInput,
        WhenInputTime, WhenNamedTimezone, WhenRelativeDate, WhenRelativeTime, WhenTime,
        WhenTimezone, WhenUtcOffset, WhenUtcOffsetSign,
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

        let name: Box<str> = "Africa/Addis_Ababa".into();

        assert!(matches!(
            out,
            Ok((
                "",
                WhenInput {
                    time: WhenInputTime::DateAndTime {
                        date: WhenDate::Relative(WhenRelativeDate::Ago(DateDuration::Days(10))),
                        time: WhenTime::Relative(WhenRelativeTime::PreviousKind(TimeKind::Hour)),
                    },
                    timezone: Some(WhenTimezone::Named(WhenNamedTimezone { name })),
                },
            ),)
        ));
    }

    #[test]
    fn parse_sth() {
        let out = WhenInput::parse("in 2 months at 8:30 PM in UTC-8");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenInput {
                    time: WhenInputTime::DateAndTime {
                        date: WhenDate::Relative(WhenRelativeDate::In(DateDuration::Months(2))),
                        time: WhenTime::Exact(WhenExactTime::AmPm(AmPmTime {
                            hour: 8,
                            minute: 30,
                            second: 0,
                            period: AmPm::Pm,
                        })),
                    },
                    timezone: Some(WhenTimezone::UtcOffset(WhenUtcOffset {
                        sign: WhenUtcOffsetSign::Minus,
                        hour: 8,
                        minute: 0,
                        second: 0,
                    })),
                },
            ),)
        ));
    }
}

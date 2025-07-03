use nom::{
    branch::alt, bytes::complete::tag, character::complete::space1, combinator::map,
    sequence::separated_pair, IResult, Parser,
};

use crate::{time_duration::TimeDuration, time_kind::TimeKind};

#[derive(Debug)]
pub enum WhenRelativeTime {
    Noon,
    Morning,
    Evening,
    Midnight,

    NextKind(TimeKind),
    PreviousKind(TimeKind),
    ThisKind(TimeKind),

    NextDuration(TimeDuration),
    PreviousDuration(TimeDuration),
}

impl WhenRelativeTime {
    pub fn parse(input: &str) -> IResult<&str, WhenRelativeTime> {
        alt((
            map(tag("noon"), |_| WhenRelativeTime::Noon),
            map(tag("morning"), |_| WhenRelativeTime::Morning),
            map(tag("evening"), |_| WhenRelativeTime::Evening),
            map(tag("midnight"), |_| WhenRelativeTime::Midnight),
            map(
                separated_pair(
                    alt((
                        tag("next"),
                        map(separated_pair(tag("the"), space1, tag("next")), |_| "next"),
                    )),
                    space1,
                    TimeKind::parse,
                ),
                |(_, w)| WhenRelativeTime::NextKind(w),
            ),
            map(
                separated_pair(
                    alt((
                        tag("previous"),
                        map(separated_pair(tag("the"), space1, tag("previous")), |_| {
                            "previous"
                        }),
                    )),
                    space1,
                    TimeKind::parse,
                ),
                |(_, w)| WhenRelativeTime::PreviousKind(w),
            ),
            map(
                separated_pair(tag("this"), space1, TimeKind::parse),
                |(_, w)| WhenRelativeTime::ThisKind(w),
            ),
            map(
                separated_pair(
                    alt((
                        tag("next"),
                        map(separated_pair(tag("the"), space1, tag("next")), |_| "next"),
                    )),
                    space1,
                    TimeDuration::parse,
                ),
                |(_, w)| WhenRelativeTime::NextDuration(w),
            ),
            map(
                separated_pair(
                    alt((
                        tag("previous"),
                        map(separated_pair(tag("the"), space1, tag("previous")), |_| {
                            "previous"
                        }),
                    )),
                    space1,
                    TimeDuration::parse,
                ),
                |(_, w)| WhenRelativeTime::PreviousDuration(w),
            ),
        ))
        .parse(input)
    }

    pub fn with_zoned(&self, zoned: jiff::Zoned) -> Result<jiff::Zoned, jiff::Error> {
        use jiff::ToSpan;
        match self {
            WhenRelativeTime::Noon => zoned.with().time(jiff::civil::time(12, 0, 0, 0)).build(),
            WhenRelativeTime::Morning => zoned.with().time(jiff::civil::time(9, 0, 0, 0)).build(),
            WhenRelativeTime::Evening => zoned.with().time(jiff::civil::time(18, 0, 0, 0)).build(),
            WhenRelativeTime::Midnight => zoned.with().time(jiff::civil::time(0, 0, 0, 0)).build(),
            WhenRelativeTime::NextKind(time_kind) => match time_kind {
                TimeKind::Hour => zoned.checked_add(1.hour()),
                TimeKind::Minute => zoned.checked_add(1.minute()),
                TimeKind::Second => zoned.checked_add(1.second()),
            },
            WhenRelativeTime::PreviousKind(time_kind) => match time_kind {
                TimeKind::Hour => zoned.checked_sub(1.hour()),
                TimeKind::Minute => zoned.checked_sub(1.minute()),
                TimeKind::Second => zoned.checked_sub(1.second()),
            },
            WhenRelativeTime::ThisKind(time_kind) => {
                let now = jiff::Zoned::new(jiff::Timestamp::now(), zoned.time_zone().to_owned());

                match time_kind {
                    TimeKind::Hour => zoned.with().hour(now.hour()).build(),
                    TimeKind::Minute => zoned.with().hour(now.hour()).minute(now.minute()).build(),
                    TimeKind::Second => zoned
                        .with()
                        .hour(now.hour())
                        .minute(now.minute())
                        .second(now.second())
                        .build(),
                }
            }
            WhenRelativeTime::NextDuration(time_duration) => match time_duration {
                TimeDuration::Seconds(secs) => zoned.checked_add((*secs as i32).seconds()),
                TimeDuration::Minutes(mins) => zoned.checked_add((*mins as i32).minutes()),
                TimeDuration::Hours(hrs) => zoned.checked_add((*hrs as i32).hours()),
            },
            WhenRelativeTime::PreviousDuration(time_duration) => match time_duration {
                TimeDuration::Seconds(secs) => zoned.checked_sub((*secs as i32).seconds()),
                TimeDuration::Minutes(mins) => zoned.checked_sub((*mins as i32).minutes()),
                TimeDuration::Hours(hrs) => zoned.checked_sub((*hrs as i32).hours()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        time_duration::TimeDuration, time_kind::TimeKind, when_relative_time::WhenRelativeTime,
    };

    #[test]
    fn parse_noon() {
        let out = WhenRelativeTime::parse("noon");
        assert!(matches!(out, Ok(("", WhenRelativeTime::Noon))));
    }

    #[test]
    fn parse_morning() {
        let out = WhenRelativeTime::parse("morning");
        assert!(matches!(out, Ok(("", WhenRelativeTime::Morning))));
    }

    #[test]
    fn parse_evening() {
        let out = WhenRelativeTime::parse("evening");
        assert!(matches!(out, Ok(("", WhenRelativeTime::Evening))));
    }

    #[test]
    fn parse_midnight() {
        let out = WhenRelativeTime::parse("midnight");
        assert!(matches!(out, Ok(("", WhenRelativeTime::Midnight))));
    }

    #[test]
    fn parse_next_kind() {
        let out = WhenRelativeTime::parse("next hour");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::NextKind(TimeKind::Hour)))
        ));

        let out = WhenRelativeTime::parse("the next hour");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::NextKind(TimeKind::Hour)))
        ));

        let out = WhenRelativeTime::parse("next     minute");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::NextKind(TimeKind::Minute)))
        ));

        let out = WhenRelativeTime::parse("the next     minute");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::NextKind(TimeKind::Minute)))
        ));

        let out = WhenRelativeTime::parse("next second");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::NextKind(TimeKind::Second)))
        ));

        let out = WhenRelativeTime::parse("the next second");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::NextKind(TimeKind::Second)))
        ));
    }

    #[test]
    fn parse_previous_kind() {
        let out = WhenRelativeTime::parse("previous hour");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::PreviousKind(TimeKind::Hour)))
        ));

        let out = WhenRelativeTime::parse("the previous hour");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::PreviousKind(TimeKind::Hour)))
        ));

        let out = WhenRelativeTime::parse("previous     minute");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::PreviousKind(TimeKind::Minute)))
        ));

        let out = WhenRelativeTime::parse("the previous     minute");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::PreviousKind(TimeKind::Minute)))
        ));

        let out = WhenRelativeTime::parse("previous second");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::PreviousKind(TimeKind::Second)))
        ));

        let out = WhenRelativeTime::parse("the previous second");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::PreviousKind(TimeKind::Second)))
        ));
    }

    #[test]
    fn parse_this_kind() {
        let out = WhenRelativeTime::parse("this hour");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::ThisKind(TimeKind::Hour)))
        ));

        let out = WhenRelativeTime::parse("this minute");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::ThisKind(TimeKind::Minute)))
        ));

        let out = WhenRelativeTime::parse("this second");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::ThisKind(TimeKind::Second)))
        ));
    }

    #[test]
    fn parse_next_duration() {
        let out = WhenRelativeTime::parse("next 10 hours");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::NextDuration(TimeDuration::Hours(10))))
        ));

        let out = WhenRelativeTime::parse("the next 10 hours");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::NextDuration(TimeDuration::Hours(10))))
        ));

        let out = WhenRelativeTime::parse("next 10 minutes");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::NextDuration(TimeDuration::Minutes(10))
            ))
        ));

        let out = WhenRelativeTime::parse("the next 10 minutes");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::NextDuration(TimeDuration::Minutes(10))
            ))
        ));

        let out = WhenRelativeTime::parse("next 10 seconds");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::NextDuration(TimeDuration::Seconds(10))
            ))
        ));

        let out = WhenRelativeTime::parse("the next 10 seconds");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::NextDuration(TimeDuration::Seconds(10))
            ))
        ));
    }

    #[test]
    fn parse_previous_duration() {
        let out = WhenRelativeTime::parse("previous 10 hours");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::PreviousDuration(TimeDuration::Hours(10))
            ))
        ));

        let out = WhenRelativeTime::parse("the previous 10 hours");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::PreviousDuration(TimeDuration::Hours(10))
            ))
        ));

        let out = WhenRelativeTime::parse("previous 10 minutes");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::PreviousDuration(TimeDuration::Minutes(10))
            ))
        ));

        let out = WhenRelativeTime::parse("the previous 10 minutes");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::PreviousDuration(TimeDuration::Minutes(10))
            ))
        ));

        let out = WhenRelativeTime::parse("previous 10 seconds");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::PreviousDuration(TimeDuration::Seconds(10))
            ))
        ));

        let out = WhenRelativeTime::parse("the previous 10 seconds");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::PreviousDuration(TimeDuration::Seconds(10))
            ))
        ));
    }

    #[test]
    fn parse_unknown() {
        let out = WhenRelativeTime::parse("unknown");

        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "unknown",
                code: nom::error::ErrorKind::Tag,
            }))
        ));
    }
}

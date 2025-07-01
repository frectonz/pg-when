use nom::{
    branch::alt, bytes::complete::tag, character::complete::space1, combinator::map,
    sequence::separated_pair, IResult, Parser,
};

use crate::{
    time_duration::{parse_time_duration, TimeDuration},
    time_kind::{parse_time_kind, TimeKind},
};

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

pub fn parse_when_relative_time(input: &str) -> IResult<&str, WhenRelativeTime> {
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
                parse_time_kind,
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
                parse_time_kind,
            ),
            |(_, w)| WhenRelativeTime::PreviousKind(w),
        ),
        map(
            separated_pair(tag("this"), space1, parse_time_kind),
            |(_, w)| WhenRelativeTime::ThisKind(w),
        ),
        map(
            separated_pair(
                alt((
                    tag("next"),
                    map(separated_pair(tag("the"), space1, tag("next")), |_| "next"),
                )),
                space1,
                parse_time_duration,
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
                parse_time_duration,
            ),
            |(_, w)| WhenRelativeTime::PreviousDuration(w),
        ),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::{
        time_duration::TimeDuration,
        time_kind::TimeKind,
        when_relative_time::{parse_when_relative_time, WhenRelativeTime},
    };

    #[test]
    fn parse_noon() {
        let out = parse_when_relative_time("noon");
        assert!(matches!(out, Ok(("", WhenRelativeTime::Noon))));
    }

    #[test]
    fn parse_morning() {
        let out = parse_when_relative_time("morning");
        assert!(matches!(out, Ok(("", WhenRelativeTime::Morning))));
    }

    #[test]
    fn parse_evening() {
        let out = parse_when_relative_time("evening");
        assert!(matches!(out, Ok(("", WhenRelativeTime::Evening))));
    }

    #[test]
    fn parse_midnight() {
        let out = parse_when_relative_time("midnight");
        assert!(matches!(out, Ok(("", WhenRelativeTime::Midnight))));
    }

    #[test]
    fn parse_next_kind() {
        let out = parse_when_relative_time("next hour");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::NextKind(TimeKind::Hour)))
        ));

        let out = parse_when_relative_time("the next hour");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::NextKind(TimeKind::Hour)))
        ));

        let out = parse_when_relative_time("next     minute");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::NextKind(TimeKind::Minute)))
        ));

        let out = parse_when_relative_time("the next     minute");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::NextKind(TimeKind::Minute)))
        ));

        let out = parse_when_relative_time("next second");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::NextKind(TimeKind::Second)))
        ));

        let out = parse_when_relative_time("the next second");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::NextKind(TimeKind::Second)))
        ));
    }

    #[test]
    fn parse_previous_kind() {
        let out = parse_when_relative_time("previous hour");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::PreviousKind(TimeKind::Hour)))
        ));

        let out = parse_when_relative_time("the previous hour");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::PreviousKind(TimeKind::Hour)))
        ));

        let out = parse_when_relative_time("previous     minute");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::PreviousKind(TimeKind::Minute)))
        ));

        let out = parse_when_relative_time("the previous     minute");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::PreviousKind(TimeKind::Minute)))
        ));

        let out = parse_when_relative_time("previous second");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::PreviousKind(TimeKind::Second)))
        ));

        let out = parse_when_relative_time("the previous second");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::PreviousKind(TimeKind::Second)))
        ));
    }

    #[test]
    fn parse_this_kind() {
        let out = parse_when_relative_time("this hour");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::ThisKind(TimeKind::Hour)))
        ));

        let out = parse_when_relative_time("this minute");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::ThisKind(TimeKind::Minute)))
        ));

        let out = parse_when_relative_time("this second");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::ThisKind(TimeKind::Second)))
        ));
    }

    #[test]
    fn parse_next_duration() {
        let out = parse_when_relative_time("next 10 hours");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::NextDuration(TimeDuration::Hours(10))))
        ));

        let out = parse_when_relative_time("the next 10 hours");
        assert!(matches!(
            out,
            Ok(("", WhenRelativeTime::NextDuration(TimeDuration::Hours(10))))
        ));

        let out = parse_when_relative_time("next 10 minutes");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::NextDuration(TimeDuration::Minutes(10))
            ))
        ));

        let out = parse_when_relative_time("the next 10 minutes");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::NextDuration(TimeDuration::Minutes(10))
            ))
        ));

        let out = parse_when_relative_time("next 10 seconds");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::NextDuration(TimeDuration::Seconds(10))
            ))
        ));

        let out = parse_when_relative_time("the next 10 seconds");
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
        let out = parse_when_relative_time("previous 10 hours");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::PreviousDuration(TimeDuration::Hours(10))
            ))
        ));

        let out = parse_when_relative_time("the previous 10 hours");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::PreviousDuration(TimeDuration::Hours(10))
            ))
        ));

        let out = parse_when_relative_time("previous 10 minutes");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::PreviousDuration(TimeDuration::Minutes(10))
            ))
        ));

        let out = parse_when_relative_time("the previous 10 minutes");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::PreviousDuration(TimeDuration::Minutes(10))
            ))
        ));

        let out = parse_when_relative_time("previous 10 seconds");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenRelativeTime::PreviousDuration(TimeDuration::Seconds(10))
            ))
        ));

        let out = parse_when_relative_time("the previous 10 seconds");
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
        let out = parse_when_relative_time("unknown");

        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "unknown",
                code: nom::error::ErrorKind::Tag,
            }))
        ));
    }
}

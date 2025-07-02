use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{map, map_res},
    IResult, Parser,
};

#[derive(Debug)]
pub enum TimeDuration {
    Seconds(u32),
    Minutes(u32),
    Hours(u32),
}

impl TimeDuration {
    pub fn parse(input: &str) -> IResult<&str, TimeDuration> {
        map(
            (
                map_res(digit1, |s: &str| s.parse::<u32>()),
                space0,
                alt((
                    // seconds
                    tag("seconds"),
                    tag("second"),
                    tag("secs"),
                    tag("sec"),
                    tag("s"),
                    // minutes
                    tag("minutes"),
                    tag("minute"),
                    tag("mins"),
                    tag("min"),
                    tag("m"),
                    // hours
                    tag("hours"),
                    tag("hour"),
                    tag("hrs"),
                    tag("hr"),
                    tag("h"),
                )),
            ),
            |(num, _, unit)| match unit {
                "seconds" | "second" | "secs" | "sec" | "s" => TimeDuration::Seconds(num),
                "minutes" | "minute" | "mins" | "min" | "m" => TimeDuration::Minutes(num),
                "hours" | "hour" | "hrs" | "hr" | "h" => TimeDuration::Hours(num),
                _ => unreachable!("all patterns have been matched"),
            },
        )
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::time_duration::TimeDuration;

    #[test]
    fn parse_seconds() {
        let out = TimeDuration::parse("0 seconds");
        assert!(matches!(out, Ok(("", TimeDuration::Seconds(0)))));

        let out = TimeDuration::parse("1 second");
        assert!(matches!(out, Ok(("", TimeDuration::Seconds(1)))));

        let out = TimeDuration::parse("100 secs");
        assert!(matches!(out, Ok(("", TimeDuration::Seconds(100)))));

        let out = TimeDuration::parse("200 sec");
        assert!(matches!(out, Ok(("", TimeDuration::Seconds(200)))));

        let out = TimeDuration::parse("300s");
        assert!(matches!(out, Ok(("", TimeDuration::Seconds(300)))));
    }

    #[test]
    fn parse_minutes() {
        let out = TimeDuration::parse("0 minutes");
        assert!(matches!(out, Ok(("", TimeDuration::Minutes(0)))));

        let out = TimeDuration::parse("1 minute");
        assert!(matches!(out, Ok(("", TimeDuration::Minutes(1)))));

        let out = TimeDuration::parse("100 mins");
        assert!(matches!(out, Ok(("", TimeDuration::Minutes(100)))));

        let out = TimeDuration::parse("200 min");
        assert!(matches!(out, Ok(("", TimeDuration::Minutes(200)))));

        let out = TimeDuration::parse("300m");
        assert!(matches!(out, Ok(("", TimeDuration::Minutes(300)))));
    }

    #[test]
    fn parse_hours() {
        let out = TimeDuration::parse("0 hours");
        assert!(matches!(out, Ok(("", TimeDuration::Hours(0)))));

        let out = TimeDuration::parse("1 hour");
        assert!(matches!(out, Ok(("", TimeDuration::Hours(1)))));

        let out = TimeDuration::parse("100 hrs");
        assert!(matches!(out, Ok(("", TimeDuration::Hours(100)))));

        let out = TimeDuration::parse("200 hr");
        assert!(matches!(out, Ok(("", TimeDuration::Hours(200)))));

        let out = TimeDuration::parse("300h");
        assert!(matches!(out, Ok(("", TimeDuration::Hours(300)))));
    }

    #[test]
    fn parse_unknown() {
        let out = TimeDuration::parse("unknown");

        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "unknown",
                code: nom::error::ErrorKind::Digit,
            }))
        ));
    }
}

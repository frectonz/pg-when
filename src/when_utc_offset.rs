use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    Parser,
};

use crate::{
    parse_hms::{parse_hms, HmsFormat},
    NomResult,
};

#[derive(Debug)]
pub struct WhenUtcOffset {
    pub sign: WhenUtcOffsetSign,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

#[derive(Debug)]
pub enum WhenUtcOffsetSign {
    Plus,
    Minus,
}

impl WhenUtcOffset {
    pub fn to_seconds(&self) -> i32 {
        let total_seconds =
            (self.hour as i32 * 3600) + (self.minute as i32 * 60) + (self.second as i32);

        match self.sign {
            WhenUtcOffsetSign::Plus => total_seconds,
            WhenUtcOffsetSign::Minus => -total_seconds,
        }
    }

    pub fn to_timezone(&self) -> Result<jiff::tz::TimeZone, jiff::Error> {
        let offset = jiff::tz::Offset::from_seconds(self.to_seconds())?;
        Ok(jiff::tz::TimeZone::fixed(offset))
    }
}

fn sign(input: &str) -> NomResult<&str, WhenUtcOffsetSign> {
    alt((
        map(tag("+"), |_| WhenUtcOffsetSign::Plus),
        map(tag("-"), |_| WhenUtcOffsetSign::Minus),
    ))
    .parse(input)
}

fn utc(input: &str) -> NomResult<&str, &str> {
    tag("UTC").parse(input)
}

impl WhenUtcOffset {
    pub fn parse(input: &str) -> NomResult<&str, WhenUtcOffset> {
        all_consuming(map(
            (utc, sign, parse_hms(HmsFormat::H24)),
            |(_, sign, (hour, minute, second))| WhenUtcOffset {
                sign,
                hour,
                minute,
                second,
            },
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;
    use nom::Finish;

    use crate::when_utc_offset::{WhenUtcOffset, WhenUtcOffsetSign};

    #[test]
    fn parse_hour() {
        let out = WhenUtcOffset::parse("UTC+1");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenUtcOffset {
                    sign: WhenUtcOffsetSign::Plus,
                    hour: 1,
                    minute: 0,
                    second: 0
                }
            ))
        ));
    }

    #[test]
    fn parse_hour_and_minute() {
        let out = WhenUtcOffset::parse("UTC-1:30");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenUtcOffset {
                    sign: WhenUtcOffsetSign::Minus,
                    hour: 1,
                    minute: 30,
                    second: 0
                }
            ))
        ));
    }

    #[test]
    fn parse_all() {
        let out = WhenUtcOffset::parse("UTC+1:30:24");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenUtcOffset {
                    sign: WhenUtcOffsetSign::Plus,
                    hour: 1,
                    minute: 30,
                    second: 24
                }
            ))
        ));
    }

    #[test]
    fn parse_invalid_hour() {
        let input = "UTC+24";
        let err = WhenUtcOffset::parse(input).finish().unwrap_err();
        assert_debug_snapshot!(err);
    }

    #[test]
    fn parse_invalid_minute() {
        let input = "UTC+12:60";
        let err = WhenUtcOffset::parse(input).finish().unwrap_err();
        assert_debug_snapshot!(err);
    }

    #[test]
    fn parse_invalid_second() {
        let input = "UTC+12:58:60";
        let err = WhenUtcOffset::parse(input).finish().unwrap_err();
        assert_debug_snapshot!(err);
    }

    #[test]
    fn parse_hour_offset() {
        let (_, out) = WhenUtcOffset::parse("UTC+1").unwrap();

        let actual = out.to_timezone().unwrap();
        let expected = jiff::tz::Offset::from_hours(1).unwrap();

        assert_eq!(actual.to_fixed_offset().unwrap(), expected);
    }
}

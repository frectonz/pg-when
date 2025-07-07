use nom::{branch::alt, combinator::map, Parser};

use crate::{when_named_timezone::WhenNamedTimezone, when_utc_offset::WhenUtcOffset, NomResult};

#[derive(Debug)]
pub enum WhenTimezone {
    UtcOffset(WhenUtcOffset),
    Named(WhenNamedTimezone),
}

impl WhenTimezone {
    pub fn parse(input: &str) -> NomResult<&str, WhenTimezone> {
        alt((
            map(WhenUtcOffset::parse, WhenTimezone::UtcOffset),
            map(WhenNamedTimezone::parse, WhenTimezone::Named),
        ))
        .parse(input)
    }

    pub fn to_timezone(&self) -> Result<jiff::tz::TimeZone, jiff::Error> {
        match self {
            WhenTimezone::UtcOffset(when_utc_offset) => when_utc_offset.to_timezone(),
            WhenTimezone::Named(when_named_timezone) => when_named_timezone.to_timezone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        when_named_timezone::WhenNamedTimezone,
        when_timezone::WhenTimezone,
        when_utc_offset::{WhenUtcOffset, WhenUtcOffsetSign},
    };

    #[test]
    fn parse_offset() {
        let out = WhenTimezone::parse("UTC+3");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenTimezone::UtcOffset(WhenUtcOffset {
                    sign: WhenUtcOffsetSign::Plus,
                    hour: 3,
                    minute: 0,
                    second: 0
                })
            ))
        ));
    }

    #[test]
    #[allow(unused_variables)]
    fn parse_named() {
        let out = WhenTimezone::parse("Africa/Addis_Ababa");

        let name: Box<str> = "Africa/Addis_Ababa".into();

        assert!(matches!(
            out,
            Ok(("", WhenTimezone::Named(WhenNamedTimezone { name })))
        ));
    }

    #[test]
    #[allow(unused_variables)]
    fn parse_unknown() {
        let out = WhenTimezone::parse("unknown");

        let name: Box<str> = "unknown".into();

        assert!(matches!(
            out,
            Ok(("", WhenTimezone::Named(WhenNamedTimezone { name })))
        ));
    }

    #[test]
    fn compare_timezone() {
        let (_, named) = WhenTimezone::parse("Africa/Addis_Ababa").unwrap();
        let (_, offset) = WhenTimezone::parse("UTC+03:00").unwrap();

        let dt = jiff::civil::date(2025, 7, 3)
            .at(2, 22, 0, 0)
            .to_zoned(jiff::tz::TimeZone::UTC)
            .unwrap();

        let named = named.to_timezone().unwrap();
        let offset = offset.to_timezone().unwrap();

        let dt_named = dt.with_time_zone(named);
        let dt_offset = dt.with_time_zone(offset);

        assert_eq!(dt_named, dt_offset);
    }
}

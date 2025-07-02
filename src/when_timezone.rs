use nom::{branch::alt, combinator::map, IResult, Parser};

use crate::{
    when_named_timezone::{parse_when_named_timezone, WhenNamedTimezone},
    when_utc_offset::{parse_when_utc_offset, WhenUtcOffset},
};

#[derive(Debug)]
pub enum WhenTimezone {
    UtcOffset(WhenUtcOffset),
    Named(WhenNamedTimezone),
}

impl WhenTimezone {
    pub fn parse(input: &str) -> IResult<&str, WhenTimezone> {
        alt((
            map(parse_when_utc_offset, |offset| {
                WhenTimezone::UtcOffset(offset)
            }),
            map(parse_when_named_timezone, |named| {
                WhenTimezone::Named(named)
            }),
        ))
        .parse(input)
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
    fn parse_named() {
        let out = WhenTimezone::parse("Africa/Addis_Ababa");

        let region: Box<str> = "Africa".into();
        let city: Box<str> = "Addis_Ababa".into();

        assert!(matches!(
            out,
            Ok(("", WhenTimezone::Named(WhenNamedTimezone { region, city })))
        ));
    }

    #[test]
    fn parse_unknown() {
        let out = WhenTimezone::parse("unknown");
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "",
                code: nom::error::ErrorKind::Tag,
            }))
        ));
    }
}

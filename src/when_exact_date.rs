use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{map, map_res, verify},
    Parser,
};

use crate::{Month, NomResult};

#[derive(Debug)]
pub struct WhenExactDate {
    pub year: u32,
    pub month: u8,
    pub day: u8,
}

impl WhenExactDate {
    pub fn parse(input: &str) -> NomResult<&str, WhenExactDate> {
        alt((
            parse_with_dashes_dd_mm_yyyy,
            parse_with_slashes_dd_mm_yyyy,
            parse_mmm_dd_yyyy,
            parse_with_dashes_yyyy_mm_dd,
            parse_with_slashes_yyyy_mm_dd,
            parse_dd_mmm_yyyy,
        ))
        .parse(input)
    }

    pub fn to_timestamp(&self, timezone: jiff::tz::TimeZone) -> Result<jiff::Zoned, jiff::Error> {
        let date = jiff::civil::date(self.year as i16, self.month as i8, self.day as i8);
        date.to_zoned(timezone)
    }
}

fn parse_day(input: &str) -> NomResult<&str, u8> {
    verify(map_res(digit1, |s: &str| s.parse::<u8>()), |&day| {
        (1..=31).contains(&day)
    })
    .parse(input)
}

fn parse_month(input: &str) -> NomResult<&str, u8> {
    verify(map_res(digit1, |s: &str| s.parse::<u8>()), |&month| {
        (1..=12).contains(&month)
    })
    .parse(input)
}

fn parse_year(input: &str) -> NomResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>()).parse(input)
}

fn parse_with_dashes_dd_mm_yyyy(input: &str) -> NomResult<&str, WhenExactDate> {
    map(
        (parse_day, tag("-"), parse_month, tag("-"), parse_year),
        |(day, _, month, _, year)| WhenExactDate { year, month, day },
    )
    .parse(input)
}

fn parse_with_slashes_dd_mm_yyyy(input: &str) -> NomResult<&str, WhenExactDate> {
    map(
        (parse_day, tag("/"), parse_month, tag("/"), parse_year),
        |(day, _, month, _, year)| WhenExactDate { year, month, day },
    )
    .parse(input)
}

fn parse_with_dashes_yyyy_mm_dd(input: &str) -> NomResult<&str, WhenExactDate> {
    map(
        (parse_year, tag("-"), parse_month, tag("-"), parse_day),
        |(year, _, month, _, day)| WhenExactDate { year, month, day },
    )
    .parse(input)
}

fn parse_with_slashes_yyyy_mm_dd(input: &str) -> NomResult<&str, WhenExactDate> {
    map(
        (parse_year, tag("/"), parse_month, tag("/"), parse_day),
        |(year, _, month, _, day)| WhenExactDate { year, month, day },
    )
    .parse(input)
}

fn parse_mmm_dd_yyyy(input: &str) -> NomResult<&str, WhenExactDate> {
    map(
        (
            Month::parse,
            space1,
            parse_day,
            tag(","),
            space1,
            parse_year,
        ),
        |(month, _, day, _, _, year)| WhenExactDate {
            year,
            month: month.number_from_january(),
            day,
        },
    )
    .parse(input)
}

fn parse_dd_mmm_yyyy(input: &str) -> NomResult<&str, WhenExactDate> {
    map(
        (parse_day, space1, Month::parse, space1, parse_year),
        |(day, _, month, _, year)| WhenExactDate {
            year,
            month: month.number_from_january(),
            day,
        },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use nom::Finish;

    use crate::WhenExactDate;

    #[test]
    fn parse_dashes() {
        let out = WhenExactDate::parse("01-01-2004");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenExactDate {
                    year: 2004,
                    month: 1,
                    day: 1
                }
            ))
        ));
    }

    #[test]
    fn parse_slashes() {
        let out = WhenExactDate::parse("01/01/2004");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenExactDate {
                    year: 2004,
                    month: 1,
                    day: 1
                }
            ))
        ));
    }

    #[test]
    fn parse_mmm_dd_yyyy() {
        let out = WhenExactDate::parse("January 1, 2004");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenExactDate {
                    year: 2004,
                    month: 1,
                    day: 1
                }
            ))
        ));
    }

    #[test]
    fn parse_yyyy_mm_dd() {
        let out = WhenExactDate::parse("2004/01/01");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenExactDate {
                    year: 2004,
                    month: 1,
                    day: 1
                }
            ))
        ));
    }

    #[test]
    fn parse_invalid_month() {
        let input = "01/13/2004";
        let err = WhenExactDate::parse(input).finish().unwrap_err();
        assert_debug_snapshot!(err);

        let input = "01/00/2004";
        let err = WhenExactDate::parse(input).finish().unwrap_err();
        assert_debug_snapshot!(err);
    }

    #[test]
    fn parse_invalid_day() {
        let input = "32/12/2004";
        let err = WhenExactDate::parse(input).finish().unwrap_err();
        assert_debug_snapshot!(err);

        let input = "00/01/2004";
        let err = WhenExactDate::parse(input).finish().unwrap_err();
        assert_debug_snapshot!(err);
    }

    #[test]
    fn parse_unknown() {
        let input = "unknown";
        let err = WhenExactDate::parse(input).finish().unwrap_err();
        assert_debug_snapshot!(err);
    }

    #[test]
    fn parse_exact_date_timestamp() {
        let (_, out) = WhenExactDate::parse("01/01/2004").unwrap();
        let timestamp = out.to_timestamp(jiff::tz::TimeZone::UTC).unwrap();

        assert_eq!(timestamp.year(), 2004);
        assert_eq!(timestamp.month(), 1);
        assert_eq!(timestamp.day(), 1);
    }
}

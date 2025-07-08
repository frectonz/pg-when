use nom::{
    bytes::complete::tag,
    character::complete::space1,
    combinator::{all_consuming, map},
    Parser,
};

use crate::{parse_hms, HmsFormat, NomResult};

#[derive(Debug)]
pub struct GmtTime {
    pub hour: u8, // 0-23
    pub minute: u8,
    pub second: u8,
}

fn gmt(input: &str) -> NomResult<&str, &str> {
    tag("GMT").parse(input)
}

impl GmtTime {
    pub fn parse(input: &str) -> NomResult<&str, GmtTime> {
        all_consuming(map(
            (parse_hms(HmsFormat::H24), space1, gmt),
            |((hour, minute, second), _, _)| GmtTime {
                hour,
                minute,
                second,
            },
        ))
        .parse(input)
    }

    pub fn with_zoned(&self, zoned: jiff::Zoned) -> Result<jiff::Zoned, jiff::Error> {
        let t = jiff::civil::time(self.hour as i8, self.minute as i8, self.second as i8, 0);
        zoned.with().time(t).build()
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use nom::Finish;
    use nom_language::error::convert_error;

    use crate::GmtTime;

    #[test]
    fn parse_hour() {
        let out = GmtTime::parse("1 GMT");
        assert!(matches!(
            out,
            Ok((
                "",
                GmtTime {
                    hour: 1,
                    minute: 0,
                    second: 0
                }
            ))
        ));
    }

    #[test]
    fn parse_hour_and_minute() {
        let out = GmtTime::parse("1:30 GMT");
        assert!(matches!(
            out,
            Ok((
                "",
                GmtTime {
                    hour: 1,
                    minute: 30,
                    second: 0
                }
            ))
        ));
    }

    #[test]
    fn parse_all() {
        let out = GmtTime::parse("1:30:24 GMT");
        assert!(matches!(
            out,
            Ok((
                "",
                GmtTime {
                    hour: 1,
                    minute: 30,
                    second: 24
                }
            ))
        ));
    }

    #[test]
    fn parse_invalid_hour() {
        let input = "24 GMT";
        let err = GmtTime::parse(input).finish().unwrap_err();
        let err = convert_error(input, err);
        assert_snapshot!(err);
    }

    #[test]
    fn parse_invalid_minute() {
        let input = "12:60 GMT";
        let err = GmtTime::parse(input).finish().unwrap_err();
        let err = convert_error(input, err);
        assert_snapshot!(err);
    }

    #[test]
    fn parse_invalid_second() {
        let input = "12:58:60 GMT";
        let err = GmtTime::parse(input).finish().unwrap_err();
        let err = convert_error(input, err);
        assert_snapshot!(err);
    }
}

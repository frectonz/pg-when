use nom::{
    character::complete::space1,
    combinator::{all_consuming, map},
    Parser,
};

use crate::{parse_hms, AmPm, HmsFormat, NomResult};

#[derive(Debug)]
pub struct AmPmTime {
    pub hour: u8, // 1-12
    pub minute: u8,
    pub second: u8,
    pub period: AmPm,
}

impl AmPmTime {
    pub fn parse(input: &str) -> NomResult<&str, AmPmTime> {
        all_consuming(map(
            (parse_hms(HmsFormat::H12), space1, AmPm::parse),
            |((hour, minute, second), _, period)| AmPmTime {
                hour,
                minute,
                second,
                period,
            },
        ))
        .parse(input)
    }

    pub fn with_zoned(&self, zoned: jiff::Zoned) -> Result<jiff::Zoned, jiff::Error> {
        let hour24 = match (self.hour, &self.period) {
            (12, AmPm::Am) => 0,  // 12 AM -> midnight
            (12, AmPm::Pm) => 12, // 12 PM -> noon
            (1..=11, AmPm::Pm) => self.hour + 12,
            (1..=11, AmPm::Am) => self.hour, // 1 AM–11 AM -> 1–11
            _ => {
                return Err(jiff::Error::from_args(format_args!(
                    "invalid 12 hour format date"
                )))
            }
        };

        let t = jiff::civil::time(hour24 as i8, self.minute as i8, self.second as i8, 0);
        zoned.with().time(t).build()
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use nom::Finish;

    use crate::{AmPm, AmPmTime};

    #[test]
    fn parse_hour() {
        let out = AmPmTime::parse("1 pm");
        assert!(matches!(
            out,
            Ok((
                "",
                AmPmTime {
                    hour: 1,
                    minute: 0,
                    second: 0,
                    period: AmPm::Pm
                }
            ))
        ));
    }

    #[test]
    fn parse_hour_and_minute() {
        let out = AmPmTime::parse("1:30 pm");
        assert!(matches!(
            out,
            Ok((
                "",
                AmPmTime {
                    hour: 1,
                    minute: 30,
                    second: 0,
                    period: AmPm::Pm
                }
            ))
        ));
    }

    #[test]
    fn parse_all() {
        let out = AmPmTime::parse("1:30:24 pm");
        assert!(matches!(
            out,
            Ok((
                "",
                AmPmTime {
                    hour: 1,
                    minute: 30,
                    second: 24,
                    period: AmPm::Pm
                }
            ))
        ));
    }

    #[test]
    fn parse_invalid_hour() {
        let input = "13 am";
        let err = AmPmTime::parse(input).finish().unwrap_err();
        assert_debug_snapshot!(err);
    }

    #[test]
    fn parse_invalid_minute() {
        let input = "12:60 am";
        let err = AmPmTime::parse(input).finish().unwrap_err();
        assert_debug_snapshot!(err);
    }

    #[test]
    fn parse_invalid_second() {
        let input = "12:58:60 am";
        let err = AmPmTime::parse(input).finish().unwrap_err();
        assert_debug_snapshot!(err);
    }
}

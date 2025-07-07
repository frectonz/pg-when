use nom::{bytes::complete::take_while1, combinator::map, IResult, Parser};

#[derive(Debug)]
pub struct WhenNamedTimezone {
    pub name: Box<str>,
}

fn name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphabetic() || c == '_' || c == '/').parse(input)
}

impl WhenNamedTimezone {
    pub fn parse(input: &str) -> IResult<&str, WhenNamedTimezone> {
        map(name, |name| WhenNamedTimezone { name: name.into() }).parse(input)
    }

    pub fn to_timezone(&self) -> Result<jiff::tz::TimeZone, jiff::Error> {
        jiff::tz::TimeZone::get(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::when_named_timezone::WhenNamedTimezone;

    #[test]
    #[allow(unused_variables)]
    fn parse_addis() {
        let out = WhenNamedTimezone::parse("Africa/Addis_Ababa");

        let timezone = WhenNamedTimezone {
            name: "Africa/Addis_Ababa".into(),
        };

        assert!(matches!(out, Ok(("", timezone))));
    }

    #[test]
    #[allow(unused_variables)]
    fn parse_london() {
        let out = WhenNamedTimezone::parse("Europe/London");

        let timezone = WhenNamedTimezone {
            name: "Europe/London".into(),
        };

        assert!(matches!(out, Ok(("", timezone))));
    }

    #[test]
    #[allow(unused_variables)]
    fn parse_new_york() {
        let out = WhenNamedTimezone::parse("America/New_York");

        let timezone = WhenNamedTimezone {
            name: "America/New_York".into(),
        };

        assert!(matches!(out, Ok(("", timezone))));
    }

    #[test]
    fn parse_addis_timezone() {
        let (_, out) = WhenNamedTimezone::parse("Africa/Addis_Ababa").unwrap();
        let tz = out.to_timezone().unwrap();
        assert_eq!(tz.iana_name(), Some("Africa/Addis_Ababa"));
    }
}

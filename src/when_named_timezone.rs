use nom::{
    bytes::complete::{tag, take_while1},
    combinator::map,
    IResult, Parser,
};

#[derive(Debug)]
pub struct WhenNamedTimezone {
    pub region: Box<str>,
    pub city: Box<str>,
}

fn name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphabetic() || c == '_').parse(input)
}

impl WhenNamedTimezone {
    pub fn parse(input: &str) -> IResult<&str, WhenNamedTimezone> {
        map((name, tag("/"), name), |(region, _, city)| {
            WhenNamedTimezone {
                region: region.into(),
                city: city.into(),
            }
        })
        .parse(input)
    }

    pub fn to_timezone(&self) -> Result<jiff::tz::TimeZone, jiff::Error> {
        let timezone = format!("{}/{}", self.region, self.city);
        jiff::tz::TimeZone::get(&timezone)
    }
}

#[cfg(test)]
mod tests {
    use crate::when_named_timezone::WhenNamedTimezone;

    #[test]
    fn parse_addis() {
        let out = WhenNamedTimezone::parse("Africa/Addis_Ababa");

        let timezone = WhenNamedTimezone {
            region: "Africa".into(),
            city: "Addis_Ababa".into(),
        };

        assert!(matches!(out, Ok(("", timezone))));
    }

    #[test]
    fn parse_london() {
        let out = WhenNamedTimezone::parse("Europe/London");

        let timezone = WhenNamedTimezone {
            region: "Europe".into(),
            city: "London".into(),
        };

        assert!(matches!(out, Ok(("", timezone))));
    }

    #[test]
    fn parse_new_york() {
        let out = WhenNamedTimezone::parse("America/New_York");

        let timezone = WhenNamedTimezone {
            region: "America".into(),
            city: "New_York".into(),
        };

        assert!(matches!(out, Ok(("", timezone))));
    }

    #[test]
    fn parse_unknown() {
        let out = WhenNamedTimezone::parse("unknown");
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "",
                code: nom::error::ErrorKind::Tag,
            }))
        ));
    }

    #[test]
    fn parse_addis_timezone() {
        let (_, out) = WhenNamedTimezone::parse("Africa/Addis_Ababa").unwrap();
        let tz = out.to_timezone().unwrap();
        assert_eq!(tz.iana_name(), Some("Africa/Addis_Ababa"));
    }
}

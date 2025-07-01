use nom::{
    bytes::complete::{tag, take_while1},
    combinator::map,
    IResult, Input, Parser,
};

#[derive(Debug)]
pub struct WhenNamedTimezone {
    pub region: Box<str>,
    pub city: Box<str>,
}

fn name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphabetic() || c == '_').parse(input)
}

pub fn parse_when_named_timezone(input: &str) -> IResult<&str, WhenNamedTimezone> {
    map((name, tag("/"), name), |(region, _, city)| {
        WhenNamedTimezone {
            region: region.into(),
            city: city.into(),
        }
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::when_named_timezone::{parse_when_named_timezone, WhenNamedTimezone};

    #[test]
    fn parse_addis() {
        let out = parse_when_named_timezone("Africa/Addis_Ababa");

        let timezone = WhenNamedTimezone {
            region: "Africa".into(),
            city: "Addis_Ababa".into(),
        };

        assert!(matches!(out, Ok(("", timezone))));
    }

    #[test]
    fn parse_london() {
        let out = parse_when_named_timezone("Europe/London");

        let timezone = WhenNamedTimezone {
            region: "Europe".into(),
            city: "London".into(),
        };

        assert!(matches!(out, Ok(("", timezone))));
    }

    #[test]
    fn parse_new_york() {
        let out = parse_when_named_timezone("America/New_York");

        let timezone = WhenNamedTimezone {
            region: "America".into(),
            city: "New_York".into(),
        };

        assert!(matches!(out, Ok(("", timezone))));
    }

    #[test]
    fn parse_unknown() {
        let out = parse_when_named_timezone("unknown");
        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "",
                code: nom::error::ErrorKind::Tag,
            }))
        ));
    }
}

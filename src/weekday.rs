use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult, Parser};

#[derive(Debug)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    pub fn parse(input: &str) -> IResult<&str, Weekday> {
        alt((
            map(tag("monday"), |_| Weekday::Monday),
            map(tag("tuesday"), |_| Weekday::Tuesday),
            map(tag("wednesday"), |_| Weekday::Wednesday),
            map(tag("thursday"), |_| Weekday::Thursday),
            map(tag("friday"), |_| Weekday::Friday),
            map(tag("saturday"), |_| Weekday::Saturday),
            map(tag("sunday"), |_| Weekday::Sunday),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::weekday::Weekday;

    #[test]
    fn parse_monday() {
        let out = Weekday::parse("monday");
        assert!(matches!(out, Ok(("", Weekday::Monday))));
    }

    #[test]
    fn parse_tuesday() {
        let out = Weekday::parse("tuesday");
        assert!(matches!(out, Ok(("", Weekday::Tuesday))));
    }

    #[test]
    fn parse_wednesday() {
        let out = Weekday::parse("wednesday");
        assert!(matches!(out, Ok(("", Weekday::Wednesday))));
    }

    #[test]
    fn parse_thursday() {
        let out = Weekday::parse("thursday");
        assert!(matches!(out, Ok(("", Weekday::Thursday))));
    }

    #[test]
    fn parse_friday() {
        let out = Weekday::parse("friday");
        assert!(matches!(out, Ok(("", Weekday::Friday))));
    }

    #[test]
    fn parse_saturday() {
        let out = Weekday::parse("saturday");
        assert!(matches!(out, Ok(("", Weekday::Saturday))));
    }

    #[test]
    fn parse_unknown() {
        let out = Weekday::parse("unknown");

        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "unknown",
                code: nom::error::ErrorKind::Tag,
            }))
        ));
    }
}

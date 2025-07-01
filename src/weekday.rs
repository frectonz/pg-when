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

pub fn parse_weekday(input: &str) -> IResult<&str, Weekday> {
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

#[cfg(test)]
mod tests {
    use crate::weekday::{parse_weekday, Weekday};

    #[test]
    fn parse_monday() {
        let out = parse_weekday("monday");
        assert!(matches!(out, Ok(("", Weekday::Monday))));
    }

    #[test]
    fn parse_tuesday() {
        let out = parse_weekday("tuesday");
        assert!(matches!(out, Ok(("", Weekday::Tuesday))));
    }

    #[test]
    fn parse_wednesday() {
        let out = parse_weekday("wednesday");
        assert!(matches!(out, Ok(("", Weekday::Wednesday))));
    }

    #[test]
    fn parse_thursday() {
        let out = parse_weekday("thursday");
        assert!(matches!(out, Ok(("", Weekday::Thursday))));
    }

    #[test]
    fn parse_friday() {
        let out = parse_weekday("friday");
        assert!(matches!(out, Ok(("", Weekday::Friday))));
    }

    #[test]
    fn parse_saturday() {
        let out = parse_weekday("saturday");
        assert!(matches!(out, Ok(("", Weekday::Saturday))));
    }

    #[test]
    fn parse_unknown() {
        let out = parse_weekday("unknown");

        assert!(matches!(
            out,
            Err(nom::Err::Error(nom::error::Error {
                input: "unknown",
                code: nom::error::ErrorKind::Tag,
            }))
        ));
    }
}

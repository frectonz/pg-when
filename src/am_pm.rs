use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult, Parser};

#[derive(Debug)]
pub enum AmPm {
    Am,
    Pm,
}

impl AmPm {
    pub fn parse(input: &str) -> IResult<&str, AmPm> {
        alt((
            map(alt((tag("am"), tag("AM"))), |_| AmPm::Am),
            map(alt((tag("pm"), tag("PM"))), |_| AmPm::Pm),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::AmPm;

    #[test]
    fn parse_am() {
        let out = AmPm::parse("am");
        assert!(matches!(out, Ok(("", AmPm::Am))));

        let out = AmPm::parse("AM");
        assert!(matches!(out, Ok(("", AmPm::Am))));
    }

    #[test]
    fn parse_pm() {
        let out = AmPm::parse("pm");
        assert!(matches!(out, Ok(("", AmPm::Pm))));

        let out = AmPm::parse("PM");
        assert!(matches!(out, Ok(("", AmPm::Pm))));
    }
}

use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult, Parser};

#[derive(Debug)]
pub enum AmPm {
    Am,
    Pm,
}

pub fn parse_am_pm(input: &str) -> IResult<&str, AmPm> {
    alt((
        map(alt((tag("am"), tag("AM"))), |_| AmPm::Am),
        map(alt((tag("pm"), tag("PM"))), |_| AmPm::Pm),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::am_pm::{parse_am_pm, AmPm};

    #[test]
    fn parse_am() {
        let out = parse_am_pm("am");
        assert!(matches!(out, Ok(("", AmPm::Am))));

        let out = parse_am_pm("AM");
        assert!(matches!(out, Ok(("", AmPm::Am))));
    }

    #[test]
    fn parse_pm() {
        let out = parse_am_pm("pm");
        assert!(matches!(out, Ok(("", AmPm::Pm))));

        let out = parse_am_pm("PM");
        assert!(matches!(out, Ok(("", AmPm::Pm))));
    }
}

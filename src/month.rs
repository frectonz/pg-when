use nom::{branch::alt, bytes::complete::tag, combinator::map, Parser};

use crate::NomResult;

#[derive(Debug)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn parse(input: &str) -> NomResult<&str, Month> {
        alt((
            map(
                alt((tag("Jan"), tag("jan"), tag("January"), tag("january"))),
                |_| Month::January,
            ),
            map(
                alt((tag("Feb"), tag("feb"), tag("February"), tag("february"))),
                |_| Month::February,
            ),
            map(
                alt((tag("Mar"), tag("mar"), tag("March"), tag("march"))),
                |_| Month::March,
            ),
            map(
                alt((tag("Apr"), tag("apr"), tag("April"), tag("april"))),
                |_| Month::April,
            ),
            map(alt((tag("May"), tag("may"))), |_| Month::May),
            map(
                alt((tag("Jun"), tag("jun"), tag("June"), tag("june"))),
                |_| Month::June,
            ),
            map(
                alt((tag("Jul"), tag("jul"), tag("July"), tag("july"))),
                |_| Month::July,
            ),
            map(
                alt((tag("Aug"), tag("aug"), tag("August"), tag("august"))),
                |_| Month::August,
            ),
            map(
                alt((tag("Sep"), tag("sep"), tag("September"), tag("september"))),
                |_| Month::September,
            ),
            map(
                alt((tag("Oct"), tag("oct"), tag("October"), tag("october"))),
                |_| Month::October,
            ),
            map(
                alt((tag("Nov"), tag("nov"), tag("November"), tag("november"))),
                |_| Month::November,
            ),
            map(
                alt((tag("Dec"), tag("dec"), tag("December"), tag("december"))),
                |_| Month::December,
            ),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use nom::Finish;
    use nom_language::error::convert_error;

    use crate::Month;

    #[test]
    fn parse_january() {
        let (_, month) = Month::parse("January").unwrap();
        assert!(matches!(month, Month::January));

        let (_, month) = Month::parse("january").unwrap();
        assert!(matches!(month, Month::January));

        let (_, month) = Month::parse("Jan").unwrap();
        assert!(matches!(month, Month::January));

        let (_, month) = Month::parse("jan").unwrap();
        assert!(matches!(month, Month::January));
    }

    #[test]
    fn parse_february() {
        let (_, month) = Month::parse("February").unwrap();
        assert!(matches!(month, Month::February));

        let (_, month) = Month::parse("february").unwrap();
        assert!(matches!(month, Month::February));

        let (_, month) = Month::parse("Feb").unwrap();
        assert!(matches!(month, Month::February));

        let (_, month) = Month::parse("feb").unwrap();
        assert!(matches!(month, Month::February));
    }

    #[test]
    fn parse_march() {
        let (_, month) = Month::parse("March").unwrap();
        assert!(matches!(month, Month::March));

        let (_, month) = Month::parse("march").unwrap();
        assert!(matches!(month, Month::March));

        let (_, month) = Month::parse("Mar").unwrap();
        assert!(matches!(month, Month::March));

        let (_, month) = Month::parse("mar").unwrap();
        assert!(matches!(month, Month::March));
    }

    #[test]
    fn parse_april() {
        let (_, month) = Month::parse("April").unwrap();
        assert!(matches!(month, Month::April));

        let (_, month) = Month::parse("april").unwrap();
        assert!(matches!(month, Month::April));

        let (_, month) = Month::parse("Apr").unwrap();
        assert!(matches!(month, Month::April));

        let (_, month) = Month::parse("apr").unwrap();
        assert!(matches!(month, Month::April));
    }

    #[test]
    fn parse_may() {
        let (_, month) = Month::parse("May").unwrap();
        assert!(matches!(month, Month::May));

        let (_, month) = Month::parse("may").unwrap();
        assert!(matches!(month, Month::May));
    }

    #[test]
    fn parse_june() {
        let (_, month) = Month::parse("June").unwrap();
        assert!(matches!(month, Month::June));

        let (_, month) = Month::parse("june").unwrap();
        assert!(matches!(month, Month::June));

        let (_, month) = Month::parse("Jun").unwrap();
        assert!(matches!(month, Month::June));

        let (_, month) = Month::parse("jun").unwrap();
        assert!(matches!(month, Month::June));
    }

    #[test]
    fn parse_july() {
        let (_, month) = Month::parse("July").unwrap();
        assert!(matches!(month, Month::July));

        let (_, month) = Month::parse("july").unwrap();
        assert!(matches!(month, Month::July));

        let (_, month) = Month::parse("Jul").unwrap();
        assert!(matches!(month, Month::July));

        let (_, month) = Month::parse("jul").unwrap();
        assert!(matches!(month, Month::July));
    }

    #[test]
    fn parse_august() {
        let (_, month) = Month::parse("August").unwrap();
        assert!(matches!(month, Month::August));

        let (_, month) = Month::parse("august").unwrap();
        assert!(matches!(month, Month::August));

        let (_, month) = Month::parse("Aug").unwrap();
        assert!(matches!(month, Month::August));

        let (_, month) = Month::parse("aug").unwrap();
        assert!(matches!(month, Month::August));
    }

    #[test]
    fn parse_september() {
        let (_, month) = Month::parse("September").unwrap();
        assert!(matches!(month, Month::September));

        let (_, month) = Month::parse("september").unwrap();
        assert!(matches!(month, Month::September));

        let (_, month) = Month::parse("Sep").unwrap();
        assert!(matches!(month, Month::September));

        let (_, month) = Month::parse("sep").unwrap();
        assert!(matches!(month, Month::September));
    }

    #[test]
    fn parse_october() {
        let (_, month) = Month::parse("October").unwrap();
        assert!(matches!(month, Month::October));

        let (_, month) = Month::parse("october").unwrap();
        assert!(matches!(month, Month::October));

        let (_, month) = Month::parse("Oct").unwrap();
        assert!(matches!(month, Month::October));

        let (_, month) = Month::parse("oct").unwrap();
        assert!(matches!(month, Month::October));
    }

    #[test]
    fn parse_november() {
        let (_, month) = Month::parse("November").unwrap();
        assert!(matches!(month, Month::November));

        let (_, month) = Month::parse("november").unwrap();
        assert!(matches!(month, Month::November));

        let (_, month) = Month::parse("Nov").unwrap();
        assert!(matches!(month, Month::November));

        let (_, month) = Month::parse("nov").unwrap();
        assert!(matches!(month, Month::November));
    }

    #[test]
    fn parse_december() {
        let (_, month) = Month::parse("December").unwrap();
        assert!(matches!(month, Month::December));

        let (_, month) = Month::parse("december").unwrap();
        assert!(matches!(month, Month::December));

        let (_, month) = Month::parse("Dec").unwrap();
        assert!(matches!(month, Month::December));

        let (_, month) = Month::parse("dec").unwrap();
        assert!(matches!(month, Month::December));
    }

    #[test]
    fn parse_unknown() {
        let input = "unknown";
        let err = Month::parse(input).finish().unwrap_err();
        let err = convert_error(input, err);
        assert_snapshot!(err);
    }
}

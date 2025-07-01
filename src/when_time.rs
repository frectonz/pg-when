use nom::{branch::alt, combinator::map, IResult, Parser};

use crate::{
    when_exact_time::{parse_when_exact_time, WhenExactTime},
    when_relative_time::{parse_when_relative_time, WhenRelativeTime},
};

#[derive(Debug)]
pub enum WhenTime {
    Relative(WhenRelativeTime),
    Exact(WhenExactTime),
}

pub fn parse_when_time(input: &str) -> IResult<&str, WhenTime> {
    alt((
        map(parse_when_relative_time, |r| WhenTime::Relative(r)),
        map(parse_when_exact_time, |e| WhenTime::Exact(e)),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::{
        gmt_time::GmtTime,
        time_duration::TimeDuration,
        when_exact_time::WhenExactTime,
        when_relative_time::WhenRelativeTime,
        when_time::{parse_when_time, WhenTime},
    };

    #[test]
    fn parse_relative() {
        let out = parse_when_time("the previous 10 mins");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenTime::Relative(WhenRelativeTime::PreviousDuration(TimeDuration::Minutes(
                    10
                )))
            ))
        ));
    }

    #[test]
    fn parse_exact() {
        let out = parse_when_time("10:01:30 GMT");
        assert!(matches!(
            out,
            Ok((
                "",
                WhenTime::Exact(WhenExactTime::Gmt(GmtTime {
                    hour: 10,
                    minute: 1,
                    second: 30
                }))
            ))
        ));
    }
}

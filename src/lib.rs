use pgrx::prelude::*;

::pgrx::pg_module_magic!();

#[derive(Debug)]
struct WhenInput {
    on: Option<WhenDate>,
    in_: Option<WhenTimezone>,
    at: Option<WhenTime>,
}

#[derive(Debug)]
enum WhenDate {
    Relative(WhenRelativeDate),
    Exact(WhenExactDate),
}

#[derive(Debug)]
enum WhenRelativeDate {
    Yesterday,
    Tomorrow,

    LastDay(Weekday),
    NextDay(Weekday),
    ThisDay(Weekday),

    LastKind(DateKind),
    NextKind(DateKind),

    Ago(DateDuration),
    In(DateDuration),
}

#[derive(Debug)]
enum DateKind {
    Week,
    Month,
    Year,
}

#[derive(Debug)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug)]
enum DateDuration {
    Days(u32),
    Weeks(u32),
    Months(u32),
}

#[derive(Debug)]
struct WhenExactDate {
    year: u32,
    month: u32,
    day: u32,
}

#[derive(Debug)]
enum WhenTimezone {
    UtcOffset(i32), // negative and positive offsets in seconds or minutes
    Named(String),  // e.g. "Asia/Addis_Ababa"
}

#[derive(Debug)]
enum WhenTime {
    Relative(WhenRelativeTime),
    Exact(WhenExactTime),
}

#[derive(Debug)]
enum WhenRelativeTime {
    Noon,
    Morning,
    Evening,
    Midnight,

    NextKind(TimeKind),
    PreviousKind(TimeKind),

    Ago(TimeDuration),
    In(TimeDuration),
}

#[derive(Debug)]
enum TimeKind {
    Hour,
    Minute,
    Second,
}

#[derive(Debug)]
enum TimeDuration {
    Seconds(u32),
    Minutes(u32),
    Hours(u32),
}

#[derive(Debug)]
enum WhenExactTime {
    AmPm(AmPmTime),
    Gmt(GmtTime),
}

#[derive(Debug)]
struct AmPmTime {
    hour: u8, // 1-12
    minute: u8,
    second: u8,
    period: AmPm,
}

#[derive(Debug)]
enum AmPm {
    Am,
    Pm,
}

#[derive(Debug)]
struct GmtTime {
    hour: u8, // 0-23
    minute: u8,
    second: u8,
}

#[pg_extern]
fn hello_pg_when() -> &'static str {
    "Hello, pg_when"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_pg_when() {
        assert_eq!("Hello, pg_when", crate::hello_pg_when());
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}

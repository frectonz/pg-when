use pgrx::prelude::*;

::pgrx::pg_module_magic!();

mod am_pm;
mod am_pm_time;
mod date_duration;
mod date_kind;
mod gmt_time;
mod time_duration;
mod time_kind;
mod weekday;
mod when_date;
mod when_exact_date;
mod when_relative_date;
mod when_relative_time;

use am_pm::AmPm;
use am_pm_time::AmPmTime;
use date_duration::DateDuration;
use date_kind::DateKind;
use gmt_time::GmtTime;
use time_duration::TimeDuration;
use time_kind::TimeKind;
use weekday::Weekday;
use when_date::WhenDate;
use when_exact_date::WhenExactDate;
use when_relative_date::WhenRelativeDate;
use when_relative_time::WhenRelativeTime;

#[derive(Debug)]
struct WhenInput {
    on: Option<WhenDate>,
    in_: Option<WhenTimezone>,
    at: Option<WhenTime>,
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
enum WhenExactTime {
    AmPm(AmPmTime),
    Gmt(GmtTime),
}

#[pg_extern]
fn hello_pg_when() -> &'static str {
    "Hello, pg_when"
}

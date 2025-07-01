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
mod when_exact_time;
mod when_named_timezone;
mod when_relative_date;
mod when_relative_time;
mod when_time;
mod when_timezone;
mod when_utc_offset;

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
use when_exact_time::WhenExactTime;
use when_named_timezone::WhenNamedTimezone;
use when_relative_date::WhenRelativeDate;
use when_relative_time::WhenRelativeTime;
use when_time::WhenTime;
use when_timezone::WhenTimezone;
use when_utc_offset::WhenUtcOffset;

#[derive(Debug)]
struct WhenInput {
    on: Option<WhenDate>,
    in_: Option<WhenTimezone>,
    at: Option<WhenTime>,
}

#[pg_extern]
fn hello_pg_when() -> &'static str {
    "Hello, pg_when"
}

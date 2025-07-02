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
mod when_input;
mod when_named_timezone;
mod when_relative_date;
mod when_relative_time;
mod when_time;
mod when_timezone;
mod when_utc_offset;

use when_input::WhenInput;

#[pg_extern]
fn when(input: &str) -> String {
    let (_, input) = WhenInput::parse(input).unwrap();
    format!("{input:?}")
}

use nom::Finish;
use pgrx::{pg_sys::panic::ErrorReportable, prelude::*};

::pgrx::pg_module_magic!();

use crate::WhenInput;

fn parse_input(input: &str) -> WhenInput {
    let result = WhenInput::parse(input).finish();

    match result {
        Ok((_, input)) => input,
        Err(_) => {
            error!("parsing '{input}' failed")
        }
    }
}

#[pg_extern(strict, immutable, parallel_safe)]
fn when_is(input: &str) -> pgrx::datum::TimestampWithTimeZone {
    let input = parse_input(input);

    let zoned = input.to_timestamp().unwrap_or_report();
    let zoned = zoned.with_time_zone(jiff::tz::TimeZone::UTC);

    pgrx::datum::TimestampWithTimeZone::with_timezone(
        zoned.year() as i32,
        zoned.month() as u8,
        zoned.day() as u8,
        zoned.hour() as u8,
        zoned.minute() as u8,
        zoned.second() as f64,
        "UTC",
    )
    .unwrap_or_report()
}

#[pg_extern(strict, immutable, parallel_safe)]
fn seconds_at(input: &str) -> i64 {
    let input = parse_input(input);
    let zoned = input.to_timestamp().unwrap_or_report();
    zoned.timestamp().as_second()
}

#[pg_extern(strict, immutable, parallel_safe)]
fn millis_at(input: &str) -> i64 {
    let input = parse_input(input);
    let zoned = input.to_timestamp().unwrap_or_report();
    zoned.timestamp().as_millisecond()
}

#[pg_extern(strict, immutable, parallel_safe)]
fn micros_at(input: &str) -> i64 {
    let input = parse_input(input);
    let zoned = input.to_timestamp().unwrap_or_report();
    zoned.timestamp().as_microsecond()
}

#[pg_extern(strict, immutable, parallel_safe)]
fn nanos_at(input: &str) -> i64 {
    let input = parse_input(input);
    let zoned = input.to_timestamp().unwrap_or_report();
    match zoned.timestamp().as_nanosecond().try_into() {
        Ok(nanos) => nanos,
        Err(_) => error!("nanosecond can not be represented as a bigint"),
    }
}

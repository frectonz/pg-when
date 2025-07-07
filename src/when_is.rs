use pgrx::prelude::*;

::pgrx::pg_module_magic!();

use crate::WhenInput;

#[pg_extern(strict, immutable, parallel_safe)]
fn when_is(input: &str) -> pgrx::datum::TimestampWithTimeZone {
    let (_, input) = WhenInput::parse(input).unwrap();
    let zoned = input.to_timestamp().unwrap();
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
    .unwrap()
}

#[pg_extern(strict, immutable, parallel_safe)]
fn seconds_at(input: &str) -> i64 {
    let (_, input) = WhenInput::parse(input).unwrap();
    let zoned = input.to_timestamp().unwrap();
    zoned.timestamp().as_second()
}

#[pg_extern(strict, immutable, parallel_safe)]
fn millis_at(input: &str) -> i64 {
    let (_, input) = WhenInput::parse(input).unwrap();
    let zoned = input.to_timestamp().unwrap();
    zoned.timestamp().as_millisecond()
}

#[pg_extern(strict, immutable, parallel_safe)]
fn micros_at(input: &str) -> i64 {
    let (_, input) = WhenInput::parse(input).unwrap();
    let zoned = input.to_timestamp().unwrap();
    zoned.timestamp().as_microsecond()
}

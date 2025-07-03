use pgrx::prelude::*;

::pgrx::pg_module_magic!();

use crate::WhenInput;

#[pg_extern(strict, immutable, parallel_safe)]
fn when_is(input: &str) -> i64 {
    let (_, input) = WhenInput::parse(input).unwrap();
    let zoned = input.to_timestamp().unwrap();
    zoned.timestamp().as_second()
}

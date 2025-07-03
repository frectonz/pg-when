use pgrx::prelude::*;

::pgrx::pg_module_magic!();

use crate::WhenInput;

#[pg_extern]
fn when_is(input: &str) -> String {
    let (_, input) = WhenInput::parse(input).unwrap();
    let zoned = input.to_timestamp().unwrap();
    format!("{zoned}")
}

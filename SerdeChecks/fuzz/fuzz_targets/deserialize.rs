#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

extern crate serde_checks;

use serde_checks::parse_sequence;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let _ = parse_sequence(data);
});

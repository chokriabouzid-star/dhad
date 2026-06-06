//! Fuzz target: feed arbitrary bytes into process_mode_a.
//! Goal: no panic, no UB — every input must return Ok or Err cleanly.
#![no_main]

use dhad::modes::process_mode_a;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = process_mode_a(data);
});

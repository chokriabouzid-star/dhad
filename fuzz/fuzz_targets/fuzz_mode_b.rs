//! Fuzz target: feed arbitrary bytes into parse_frame (Mode B).
//! Goal: no panic on any malformed binary frame.
#![no_main]

use libfuzzer_sys::fuzz_target;
use dhad::mode_b::parse_frame;

fuzz_target!(|data: &[u8]| {
    // Raw bytes — could be anything.
    // parse_frame must return Ok or Err, never panic.
    let _ = parse_frame(data);
});

//! Fuzz target: verify determinism — same input => same output, always.
#![no_main]

use dhad::modes::process_mode_a;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let r1 = process_mode_a(data);
    let r2 = process_mode_a(data);

    match (r1, r2) {
        (Ok(a), Ok(b)) => {
            assert_eq!(a.core_hash, b.core_hash, "core_hash mismatch");
            assert_eq!(a.phonetic_hash, b.phonetic_hash, "phonetic_hash mismatch");
        }
        (Err(e1), Err(e2)) => {
            assert_eq!(
                format!("{:?}", e1),
                format!("{:?}", e2),
                "error determinism mismatch"
            );
        }
        _ => panic!("determinism violation: same input produced different result classes"),
    }
});

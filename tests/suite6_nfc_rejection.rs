//! Suite 6 — NFC Rejection Contract
//!
//! These tests pin the v1.x input contract:
//! Dhad v1.x processes a strict, NFC-precomposed Arabic profile.
//!
//! The following combining marks are intentionally NOT mapped in v1.x:
//!   - U+0653 ARABIC MADDAH ABOVE
//!   - U+0654 ARABIC HAMZA ABOVE
//!   - U+0655 ARABIC HAMZA BELOW
//!
//! Any input that contains these codepoints — whether bare or attached
//! to a base letter — MUST be rejected deterministically with
//! `ErrorKind::UnmappedCodepoint`. The error must also report the
//! exact offending codepoint and its byte/codepoint position so the
//! caller can correct or normalize the input.
//!
//! These tests guard the README's "Known Limitations" section as a
//! real contract, not only documentation. If support for these
//! combining forms is added in a future version, these tests will
//! fail by design and force a conscious migration.

use dhad::model::ErrorKind;
use dhad::modes::process_mode_a;

fn assert_unmapped(input: &str, expected_cp: u32, expected_pos: usize, label: &str) {
    let r = process_mode_a(input.as_bytes());
    match r {
        Err(ErrorKind::UnmappedCodepoint { codepoint, position }) => {
            assert_eq!(
                codepoint, expected_cp,
                "{label}: wrong codepoint in error (got U+{codepoint:04X}, want U+{expected_cp:04X})"
            );
            assert_eq!(
                position, expected_pos,
                "{label}: wrong position in error (got {position}, want {expected_pos})"
            );
        }
        other => panic!(
            "{label}: expected Err(UnmappedCodepoint {{ codepoint: U+{expected_cp:04X}, position: {expected_pos} }}), got {:?}",
            other
        ),
    }
}

// ──────────────────────────────────────────────────────────────
// Bare combining marks — must be rejected on their own
// ──────────────────────────────────────────────────────────────

#[test]
fn nfc_bare_u0653_maddah_above_rejected() {
    assert_unmapped("\u{0653}", 0x0653, 0, "bare U+0653");
}

#[test]
fn nfc_bare_u0654_hamza_above_rejected() {
    assert_unmapped("\u{0654}", 0x0654, 0, "bare U+0654");
}

#[test]
fn nfc_bare_u0655_hamza_below_rejected() {
    assert_unmapped("\u{0655}", 0x0655, 0, "bare U+0655");
}

// ──────────────────────────────────────────────────────────────
// Decomposed (NFD-style) sequences — must be rejected even when
// they are canonically equivalent to a precomposed form.
// ──────────────────────────────────────────────────────────────

#[test]
fn nfc_alef_plus_u0653_decomposed_rejected() {
    // canonical equivalent of U+0622 (ALEF WITH MADDA ABOVE)
    assert_unmapped("\u{0627}\u{0653}", 0x0653, 1, "ALEF + U+0653");
}

#[test]
fn nfc_alef_plus_u0654_decomposed_rejected() {
    // canonical equivalent of U+0623 (ALEF WITH HAMZA ABOVE)
    assert_unmapped("\u{0627}\u{0654}", 0x0654, 1, "ALEF + U+0654");
}

#[test]
fn nfc_alef_plus_u0655_decomposed_rejected() {
    // canonical equivalent of U+0625 (ALEF WITH HAMZA BELOW)
    assert_unmapped("\u{0627}\u{0655}", 0x0655, 1, "ALEF + U+0655");
}

#[test]
fn nfc_waw_plus_u0654_decomposed_rejected() {
    // canonical equivalent of U+0624 (WAW WITH HAMZA ABOVE)
    assert_unmapped("\u{0648}\u{0654}", 0x0654, 1, "WAW + U+0654");
}

#[test]
fn nfc_yeh_plus_u0654_decomposed_rejected() {
    // canonical equivalent of U+0626 (YEH WITH HAMZA ABOVE)
    assert_unmapped("\u{064A}\u{0654}", 0x0654, 1, "YEH + U+0654");
}

// ──────────────────────────────────────────────────────────────
// Precomposed forms remain accepted (sanity guard, so that future
// changes to base_map cannot silently regress the precomposed path
// while the rejection contract above is being adjusted).
// ──────────────────────────────────────────────────────────────

#[test]
fn nfc_precomposed_alef_madda_still_accepted() {
    // U+0622 = ALEF WITH MADDA ABOVE
    let r =
        process_mode_a("\u{0622}".as_bytes()).expect("precomposed U+0622 must be accepted in v1.x");
    assert_eq!(r.stream.len(), 1, "U+0622 must produce exactly 1 atom");
}

#[test]
fn nfc_precomposed_alef_hamza_above_still_accepted() {
    // U+0623 = ALEF WITH HAMZA ABOVE
    let r =
        process_mode_a("\u{0623}".as_bytes()).expect("precomposed U+0623 must be accepted in v1.x");
    assert_eq!(r.stream.len(), 1, "U+0623 must produce exactly 1 atom");
}

#[test]
fn nfc_precomposed_alef_hamza_below_still_accepted() {
    // U+0625 = ALEF WITH HAMZA BELOW
    let r =
        process_mode_a("\u{0625}".as_bytes()).expect("precomposed U+0625 must be accepted in v1.x");
    assert_eq!(r.stream.len(), 1, "U+0625 must produce exactly 1 atom");
}

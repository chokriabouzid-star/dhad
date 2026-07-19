//! Suite 3 — Adversarial Tests
//! Source of truth: Dhad-Spec-v1.0 adversarial test vectors
//! AT-030..AT-034 و AT-035/038/039 تُختبر عبر invariants::validate_atom مباشرةً
//! (Mode B parser غير موجود بعد — ينتقل المسار الكامل لـ v3.0)

use dhad::invariants;
use dhad::model::{DhadAtom, ErrorKind};
use dhad::modes::process_mode_a;
use dhad::registry::base;

macro_rules! must_fail {
    ($name:ident, $input:expr, $pat:pat) => {
        #[test]
        fn $name() {
            match process_mode_a($input) {
                Err(e) => assert!(
                    matches!(e, $pat),
                    "wrong error: got {:?}, expected {}",
                    e,
                    stringify!($pat)
                ),
                Ok(_) => panic!("expected error {}, got Ok", stringify!($pat)),
            }
        }
    };
}

// ═══════════════════════════════════════════════════════════════════
// A: ERR_MALFORMED_UTF8 (AT-001 إلى AT-010)
// ═══════════════════════════════════════════════════════════════════
must_fail!(
    at_001_overlong_2byte,
    &[0xC1, 0x41],
    ErrorKind::MalformedUtf8 { .. }
);
must_fail!(
    at_002_overlong_3byte,
    &[0xE0, 0x81, 0x41],
    ErrorKind::MalformedUtf8 { .. }
);
must_fail!(
    at_003_isolated_cont,
    &[0x80],
    ErrorKind::MalformedUtf8 { .. }
);
must_fail!(
    at_004_truncated_2byte,
    &[0xD8],
    ErrorKind::MalformedUtf8 { .. }
);
must_fail!(
    at_005_truncated_3byte,
    &[0xE2, 0x80],
    ErrorKind::MalformedUtf8 { .. }
);
must_fail!(
    at_006_invalid_lead_fe,
    &[0xFE],
    ErrorKind::MalformedUtf8 { .. }
);
must_fail!(
    at_007_invalid_lead_ff,
    &[0xFF],
    ErrorKind::MalformedUtf8 { .. }
);
must_fail!(
    at_008_surrogate_d800,
    &[0xED, 0xA0, 0x80],
    ErrorKind::MalformedUtf8 { .. }
);
must_fail!(
    at_009_surrogate_dfff,
    &[0xED, 0xBF, 0xBF],
    ErrorKind::MalformedUtf8 { .. }
);
must_fail!(
    at_010_above_10ffff,
    &[0xF4, 0x90, 0x80, 0x80],
    ErrorKind::MalformedUtf8 { .. }
);

// ═══════════════════════════════════════════════════════════════════
// B: ERR_UNMAPPED_CODEPOINT (AT-011 إلى AT-016)
// ═══════════════════════════════════════════════════════════════════
must_fail!(
    at_011_latin_a_in_context,
    &[0xD8, 0xA8, 0x41, 0xD8, 0xA8],
    ErrorKind::UnmappedCodepoint { .. }
);
must_fail!(
    at_012_greek_alpha,
    &[0xCE, 0xB1],
    ErrorKind::UnmappedCodepoint { .. }
);
must_fail!(
    at_013_cjk,
    &[0xE4, 0xB8, 0xAD],
    ErrorKind::UnmappedCodepoint { .. }
);
must_fail!(
    at_014_pua,
    &[0xEE, 0x80, 0x80],
    ErrorKind::UnmappedCodepoint { .. }
);
must_fail!(
    at_015_arabic_ext_a,
    &[0xE2, 0xA2, 0xA0],
    ErrorKind::UnmappedCodepoint { .. }
);
must_fail!(
    at_016_farsi_yeh,
    &[0xDB, 0x8C],
    ErrorKind::UnmappedCodepoint { .. }
);

// ═══════════════════════════════════════════════════════════════════
// C: ERR_ORPHAN_DIACRITIC (AT-017 إلى AT-022)
// ═══════════════════════════════════════════════════════════════════
must_fail!(
    at_017_orphan_fatha,
    &[0xD9, 0x8E],
    ErrorKind::OrphanDiacritic { .. }
);
must_fail!(
    at_018_orphan_shadda,
    &[0xD9, 0x91],
    ErrorKind::OrphanDiacritic { .. }
);
must_fail!(
    at_019_orphan_tanween_fath,
    &[0xD9, 0x8B],
    ErrorKind::OrphanDiacritic { .. }
);
must_fail!(
    at_020_orphan_super_alef,
    &[0xD9, 0xB0],
    ErrorKind::OrphanDiacritic { .. }
);
must_fail!(
    at_021_double_fatha_orphan,
    &[0xD9, 0x8E, 0xD9, 0x8E],
    ErrorKind::OrphanDiacritic { .. }
);
must_fail!(
    at_022_bom_then_fatha,
    &[0xEF, 0xBB, 0xBF, 0xD9, 0x8E],
    ErrorKind::OrphanDiacritic { .. }
);

// ═══════════════════════════════════════════════════════════════════
// D: ERR_INVALID_MARK_COMBO (AT-023 إلى AT-029)
// ═══════════════════════════════════════════════════════════════════
must_fail!(
    at_023_fatha_damma,
    &[0xD8, 0xA8, 0xD9, 0x8E, 0xD9, 0x8F],
    ErrorKind::InvalidMarkCombo { .. }
);
must_fail!(
    at_024_kasra_sukun,
    &[0xD8, 0xA8, 0xD9, 0x90, 0xD9, 0x92],
    ErrorKind::InvalidMarkCombo { .. }
);
must_fail!(
    at_025_shadda_sukun,
    &[0xD8, 0xA8, 0xD9, 0x91, 0xD9, 0x92],
    ErrorKind::InvalidMarkCombo { .. }
);
must_fail!(
    at_026_fatha_kasra,
    &[0xD8, 0xA8, 0xD9, 0x8E, 0xD9, 0x90],
    ErrorKind::InvalidMarkCombo { .. }
);
must_fail!(
    at_027_three_diacritics,
    &[0xD8, 0xA8, 0xD9, 0x8E, 0xD9, 0x8F, 0xD9, 0x90],
    ErrorKind::InvalidMarkCombo { .. }
);
must_fail!(
    at_028_damma_on_space,
    &[0x20, 0xD9, 0x8F],
    ErrorKind::InvalidMarkCombo { .. }
);
must_fail!(
    at_029_fatha_on_digit,
    &[0x31, 0xD9, 0x8E],
    ErrorKind::InvalidMarkCombo { .. }
);

// ═══════════════════════════════════════════════════════════════════
// E: ERR_INVALID_FLAG_COMBO (AT-030 إلى AT-034)
// synthesised atoms — تُختبر عبر invariants::validate_atom مباشرةً
// ═══════════════════════════════════════════════════════════════════
#[test]
fn at_030_hamza_above_and_below() {
    let atom = DhadAtom {
        base: base::ALEF,
        marks: 0,
        flags: 0x03,
        prosody: 0,
        reserved: 0,
    };
    assert!(
        matches!(
            invariants::validate_atom(&atom, 0),
            Err(ErrorKind::InvalidFlagCombo { .. })
        ),
        "H_ABOVE|H_BELOW must be rejected"
    );
}

#[test]
fn at_031_madda_and_hamza_above() {
    let atom = DhadAtom {
        base: base::ALEF,
        marks: 0,
        flags: 0x05,
        prosody: 0,
        reserved: 0,
    };
    assert!(
        matches!(
            invariants::validate_atom(&atom, 0),
            Err(ErrorKind::InvalidFlagCombo { .. })
        ),
        "MADDA|H_ABOVE must be rejected"
    );
}

#[test]
fn at_032_hamza_above_on_meem() {
    let atom = DhadAtom {
        base: base::MEEM,
        marks: 0,
        flags: 0x01,
        prosody: 0,
        reserved: 0,
    };
    assert!(
        matches!(
            invariants::validate_atom(&atom, 0),
            Err(ErrorKind::InvalidFlagCombo { .. })
        ),
        "H_ABOVE on MEEM must be rejected (invalid seat)"
    );
}

#[test]
fn at_033_hamza_below_on_waw() {
    let atom = DhadAtom {
        base: base::WAW,
        marks: 0,
        flags: 0x02,
        prosody: 0,
        reserved: 0,
    };
    assert!(
        matches!(
            invariants::validate_atom(&atom, 0),
            Err(ErrorKind::InvalidFlagCombo { .. })
        ),
        "H_BELOW on WAW must be rejected (invalid seat)"
    );
}

#[test]
fn at_034_madda_on_noon() {
    let atom = DhadAtom {
        base: base::NOON,
        marks: 0,
        flags: 0x04,
        prosody: 0,
        reserved: 0,
    };
    assert!(
        matches!(
            invariants::validate_atom(&atom, 0),
            Err(ErrorKind::InvalidFlagCombo { .. })
        ),
        "MADDA on NOON must be rejected (invalid seat)"
    );
}

// ═══════════════════════════════════════════════════════════════════
// F: ERR_INVALID_PROSODY (AT-035 إلى AT-040)
// AT-035/038/039 = synthesised; AT-036/037/040 = UTF-8 input
// ═══════════════════════════════════════════════════════════════════
#[test]
fn at_035_tanween_fath_and_damm_synthesised() {
    // TANWEEN_FATH | TANWEEN_DAMM على نفس الذرة — مستحيل من Mode A
    let atom = DhadAtom {
        base: base::NOON,
        marks: 0,
        flags: 0,
        prosody: 0x03,
        reserved: 0,
    };
    assert!(
        matches!(
            invariants::validate_atom(&atom, 0),
            Err(ErrorKind::InvalidProsody { .. })
        ),
        "TW_FATH|TW_DAMM must be rejected"
    );
}

must_fail!(
    at_036_tanween_fath_plus_fatha,
    &[0xD9, 0x86, 0xD9, 0x8E, 0xD9, 0x8B],
    ErrorKind::InvalidProsody { .. }
);

must_fail!(
    at_037_sukun_plus_tanween,
    &[0xD9, 0x86, 0xD9, 0x92, 0xD9, 0x8B],
    ErrorKind::InvalidProsody { .. }
);

#[test]
fn at_038_madd_normal_and_extended_synthesised() {
    let atom = DhadAtom {
        base: base::ALEF,
        marks: 0,
        flags: 0,
        prosody: 0x18,
        reserved: 0,
    }; // MADD_N | MADD_X
    assert!(
        matches!(
            invariants::validate_atom(&atom, 0),
            Err(ErrorKind::InvalidProsody { .. })
        ),
        "MADD_N|MADD_X must be rejected"
    );
}

#[test]
fn at_039_madd_normal_on_beh_synthesised() {
    let atom = DhadAtom {
        base: base::BEH,
        marks: 0,
        flags: 0,
        prosody: 0x08,
        reserved: 0,
    }; // MADD_N on non-long-vowel
    assert!(
        matches!(
            invariants::validate_atom(&atom, 0),
            Err(ErrorKind::InvalidProsody { .. })
        ),
        "MADD_N on BEH must be rejected (not LONG_VOWEL_CLASS)"
    );
}

must_fail!(
    at_040_tanween_fath_on_space,
    &[0x20, 0xD9, 0x8B],
    ErrorKind::InvalidProsody { .. }
);

#[test]
fn at_new_duplicate_tanween_fath_same_atom() {
    let input = &[0xD9, 0x86, 0xD9, 0x8B, 0xD9, 0x8B]; // NOON + TW_FATH + TW_FATH
    let result = process_mode_a(input);
    assert!(
        matches!(result, Err(ErrorKind::InvalidProsody { .. })),
        "duplicate TANWEEN_FATH on same atom must be rejected"
    );
}

#[test]
fn at_new_duplicate_superscript_alef_same_atom() {
    let input = &[0xD8, 0xA7, 0xD9, 0xB0, 0xD9, 0xB0]; // ALEF + SUPERSCRIPT_ALEF + SUPERSCRIPT_ALEF
    let result = process_mode_a(input);
    assert!(
        matches!(result, Err(ErrorKind::InvalidProsody { .. })),
        "duplicate SUPERSCRIPT_ALEF on same atom must be rejected"
    );
}

// ═══════════════════════════════════════════════════════════════════
// I16 (§8): AT-041 — U+0670 بعد ذرة PROSODY_INERT
// هذا الاختبار إلزامي بشكل خاص (I16, §8)
// ═══════════════════════════════════════════════════════════════════
must_fail!(
    at_041_superscript_alef_after_space,
    &[0x20, 0xD9, 0xB0],
    ErrorKind::InvalidProsody { .. }
);

// ═══════════════════════════════════════════════════════════════════
// G: Attack Vectors / Crash Resistance (AT-042 إلى AT-046)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn at_042_oversized_input() {
    // 4MiB + 1 byte → ERR_INPUT_TOO_LARGE قبل أي تخصيص ذاكرة
    let big = vec![0xD8u8; 4_194_305];
    assert!(
        matches!(process_mode_a(&big), Err(ErrorKind::InputTooLarge(_))),
        "must reject > MAX_INPUT_BYTES before allocation"
    );
}

#[test]
fn at_043_diacritic_flood() {
    // BEH + 100 × FATHA: يجب أن يُوقف عند الثانية ولا يتعطل
    let mut input = vec![0xD8u8, 0xA8]; // BEH
    for _ in 0..100 {
        input.extend_from_slice(&[0xD9, 0x8E]); // FATHA
    }
    let result = process_mode_a(&input);
    assert!(result.is_err(), "second FATHA must be rejected");
    // يجب أن لا يتعطل أو يستهلك ذاكرة غير محدودة
}

#[test]
fn at_044_all_lam_alef_variants() {
    // كل أشكال لام-ألف الثمانية → stream صالح من 16 ذرة
    let input = &[
        0xEF, 0xBB, 0xBB, // FEFB
        0xEF, 0xBB, 0xBC, // FEFC
        0xEF, 0xBB, 0xB5, // FEF5
        0xEF, 0xBB, 0xB6, // FEF6
        0xEF, 0xBB, 0xB7, // FEF7
        0xEF, 0xBB, 0xB8, // FEF8
        0xEF, 0xBB, 0xB9, // FEF9
        0xEF, 0xBB, 0xBA, // FEFA
    ];
    let result = process_mode_a(input).expect("all Lam-Alef variants must succeed");
    assert_eq!(
        result.stream.len(),
        16,
        "8 ligatures × 2 atoms each = 16 atoms"
    );
    assert_eq!(
        result.stream.to_bytes().len(),
        128,
        "16 atoms × 8 bytes = 128 bytes"
    );
}

#[test]
fn at_045_zwj_injection_bismi() {
    // ZWJ بين كل حرفين في "بسم" → 3 ذرات فقط
    let input = &[
        0xD8, 0xA8, 0xE2, 0x80, 0x8D, // BEH + ZWJ
        0xD8, 0xB3, 0xE2, 0x80, 0x8D, // SEEN + ZWJ
        0xD9, 0x85, // MEEM
    ];
    let result = process_mode_a(input).expect("ZWJ must be filtered silently");
    assert_eq!(result.stream.len(), 3, "ZWJ removed: 3 atoms expected");
}

#[test]
fn at_046_bidi_rlo_attack() {
    // RLO (U+202E) + نص عربي → RLO يُزال، الحروف تبقى
    let input = &[
        0xE2, 0x80, 0xAE, // U+202E RLO
        0xD8, 0xA8, // BEH
        0xD8, 0xA7, // ALEF
        0xD8, 0xA8, // BEH
    ];
    let result = process_mode_a(input).expect("RLO must be filtered silently");
    assert_eq!(result.stream.len(), 3, "[BEH][ALEF][BEH] expected");
}

// ═══════════════════════════════════════════════════════════════════
// اختبارات Reserved Base IDs (I01/I02, §8) — synthesised atoms
// ═══════════════════════════════════════════════════════════════════
#[test]
fn at_reserved_base_001d() {
    let atom = DhadAtom {
        base: 0x001D,
        marks: 0,
        flags: 0,
        prosody: 0,
        reserved: 0,
    };
    assert!(
        matches!(
            invariants::validate_atom(&atom, 0),
            Err(ErrorKind::UnmappedCodepoint { .. })
        ),
        "base 0x001D must be explicitly rejected (I02, §8)"
    );
}

#[test]
fn at_reserved_base_001e() {
    let atom = DhadAtom {
        base: 0x001E,
        marks: 0,
        flags: 0,
        prosody: 0,
        reserved: 0,
    };
    assert!(matches!(
        invariants::validate_atom(&atom, 0),
        Err(ErrorKind::UnmappedCodepoint { .. })
    ));
}

#[test]
fn at_reserved_base_001f() {
    let atom = DhadAtom {
        base: 0x001F,
        marks: 0,
        flags: 0,
        prosody: 0,
        reserved: 0,
    };
    assert!(matches!(
        invariants::validate_atom(&atom, 0),
        Err(ErrorKind::UnmappedCodepoint { .. })
    ));
}

// Reserved field غير صفري (I22)
#[test]
fn at_reserved_field_nonzero() {
    let atom = DhadAtom {
        base: base::ALEF,
        marks: 0,
        flags: 0,
        prosody: 0,
        reserved: 0x0001,
    };
    assert!(
        matches!(
            invariants::validate_atom(&atom, 0),
            Err(ErrorKind::ReservedFieldNonZero { .. })
        ),
        "reserved != 0 must be rejected (I22, §8)"
    );
}

// ═══════════════════════════════════════════════════════════════════
// P10: Crash Resistance (proptest)
// ═══════════════════════════════════════════════════════════════════
use proptest::prelude::*;
proptest! {
    #[test]
    fn p10_crash_resistance(
        input in proptest::collection::vec(any::<u8>(), 0..4096)
    ) {
        // يجب أن لا يتعطل بغض النظر عن المدخل
        let _ = process_mode_a(&input);
    }
}

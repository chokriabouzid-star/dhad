/// Declarative Mode A adversarial vectors extracted from suite3_adversarial.rs.
///
/// This file contains only protocol-level cases exercised through
/// `process_mode_a()`.
///
/// Intentionally omitted from this manifest:
/// - AT-030..AT-034  (internal invariant tests; promoted to tagged_cases.rs)
/// - AT-035          (internal invariant test; promoted to tagged_cases.rs)
/// - AT-038..AT-039  (internal invariant tests; promoted to tagged_cases.rs)
/// - reserved base / reserved field invariant tests
/// - p10_crash_resistance (property test, not a single vector)

#[derive(Debug, Clone, Copy)]
pub enum AdversarialInput {
    Bytes(&'static [u8]),
    Generated(fn() -> Vec<u8>),
}

#[derive(Debug, Clone, Copy)]
pub enum AdversarialExpected {
    Ok {
        stream_hex: &'static str,
        core_hash: &'static str,
        phonetic_hash: &'static str,
    },
    Err {
        error_kind: &'static str,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct AdversarialCase {
    pub name: &'static str,
    pub input: AdversarialInput,
    pub expected: AdversarialExpected,
}

fn input_at_042_oversized() -> Vec<u8> {
    vec![0xD8u8; 4_194_305]
}

fn input_at_043_diacritic_flood() -> Vec<u8> {
    let mut input = vec![0xD8u8, 0xA8]; // BEH
    for _ in 0..100 {
        input.extend_from_slice(&[0xD9, 0x8E]); // FATHA
    }
    input
}

// ─────────────────────────────────────────────────────────────────────────────
// Static Mode A byte inputs
// ─────────────────────────────────────────────────────────────────────────────

// A: ERR_MALFORMED_UTF8 (AT-001..AT-010)
const AT_001_OVERLONG_2BYTE: &[u8] = &[0xC1, 0x41];
const AT_002_OVERLONG_3BYTE: &[u8] = &[0xE0, 0x81, 0x41];
const AT_003_ISOLATED_CONT: &[u8] = &[0x80];
const AT_004_TRUNCATED_2BYTE: &[u8] = &[0xD8];
const AT_005_TRUNCATED_3BYTE: &[u8] = &[0xE2, 0x80];
const AT_006_INVALID_LEAD_FE: &[u8] = &[0xFE];
const AT_007_INVALID_LEAD_FF: &[u8] = &[0xFF];
const AT_008_SURROGATE_D800: &[u8] = &[0xED, 0xA0, 0x80];
const AT_009_SURROGATE_DFFF: &[u8] = &[0xED, 0xBF, 0xBF];
const AT_010_ABOVE_10FFFF: &[u8] = &[0xF4, 0x90, 0x80, 0x80];

// B: ERR_UNMAPPED_CODEPOINT (AT-011..AT-016)
const AT_011_LATIN_A_IN_CONTEXT: &[u8] = &[0xD8, 0xA8, 0x41, 0xD8, 0xA8];
const AT_012_GREEK_ALPHA: &[u8] = &[0xCE, 0xB1];
const AT_013_CJK: &[u8] = &[0xE4, 0xB8, 0xAD];
const AT_014_PUA: &[u8] = &[0xEE, 0x80, 0x80];
const AT_015_ARABIC_EXT_A: &[u8] = &[0xE2, 0xA2, 0xA0];
const AT_016_FARSI_YEH: &[u8] = &[0xDB, 0x8C];

// C: ERR_ORPHAN_DIACRITIC (AT-017..AT-022)
const AT_017_ORPHAN_FATHA: &[u8] = &[0xD9, 0x8E];
const AT_018_ORPHAN_SHADDA: &[u8] = &[0xD9, 0x91];
const AT_019_ORPHAN_TANWEEN_FATH: &[u8] = &[0xD9, 0x8B];
const AT_020_ORPHAN_SUPER_ALEF: &[u8] = &[0xD9, 0xB0];
const AT_021_DOUBLE_FATHA_ORPHAN: &[u8] = &[0xD9, 0x8E, 0xD9, 0x8E];
const AT_022_BOM_THEN_FATHA: &[u8] = &[0xEF, 0xBB, 0xBF, 0xD9, 0x8E];

// D: ERR_INVALID_MARK_COMBO (AT-023..AT-029)
const AT_023_FATHA_DAMMA: &[u8] = &[0xD8, 0xA8, 0xD9, 0x8E, 0xD9, 0x8F];
const AT_024_KASRA_SUKUN: &[u8] = &[0xD8, 0xA8, 0xD9, 0x90, 0xD9, 0x92];
const AT_025_SHADDA_SUKUN: &[u8] = &[0xD8, 0xA8, 0xD9, 0x91, 0xD9, 0x92];
const AT_026_FATHA_KASRA: &[u8] = &[0xD8, 0xA8, 0xD9, 0x8E, 0xD9, 0x90];
const AT_027_THREE_DIACRITICS: &[u8] = &[0xD8, 0xA8, 0xD9, 0x8E, 0xD9, 0x8F, 0xD9, 0x90];
const AT_028_DAMMA_ON_SPACE: &[u8] = &[0x20, 0xD9, 0x8F];
const AT_029_FATHA_ON_DIGIT: &[u8] = &[0x31, 0xD9, 0x8E];

// F: ERR_INVALID_PROSODY protocol-level only
const AT_036_TANWEEN_FATH_PLUS_FATHA: &[u8] = &[0xD9, 0x86, 0xD9, 0x8E, 0xD9, 0x8B];
const AT_040_TANWEEN_FATH_ON_SPACE: &[u8] = &[0x20, 0xD9, 0x8B];
const AT_NEW_DUPLICATE_TANWEEN_FATH_SAME_ATOM: &[u8] = &[0xD9, 0x86, 0xD9, 0x8B, 0xD9, 0x8B];
const AT_NEW_DUPLICATE_SUPERSCRIPT_ALEF_SAME_ATOM: &[u8] = &[0xD8, 0xA7, 0xD9, 0xB0, 0xD9, 0xB0];
const AT_041_SUPERSCRIPT_ALEF_AFTER_SPACE: &[u8] = &[0x20, 0xD9, 0xB0];

// G: Attack vectors / robustness
const AT_044_ALL_LAM_ALEF_VARIANTS: &[u8] = &[
    0xEF, 0xBB, 0xBB, // FEFB
    0xEF, 0xBB, 0xBC, // FEFC
    0xEF, 0xBB, 0xB5, // FEF5
    0xEF, 0xBB, 0xB6, // FEF6
    0xEF, 0xBB, 0xB7, // FEF7
    0xEF, 0xBB, 0xB8, // FEF8
    0xEF, 0xBB, 0xB9, // FEF9
    0xEF, 0xBB, 0xBA, // FEFA
];

const AT_045_ZWJ_INJECTION_BISMI: &[u8] = &[
    0xD8, 0xA8, 0xE2, 0x80, 0x8D, // BEH + ZWJ
    0xD8, 0xB3, 0xE2, 0x80, 0x8D, // SEEN + ZWJ
    0xD9, 0x85, // MEEM
];

const AT_046_BIDI_RLO_ATTACK: &[u8] = &[
    0xE2, 0x80, 0xAE, // RLO
    0xD8, 0xA8, // BEH
    0xD8, 0xA7, // ALEF
    0xD8, 0xA8, // BEH
];

// ─────────────────────────────────────────────────────────────────────────────
// Canonical adversarial vector set
// ─────────────────────────────────────────────────────────────────────────────

pub const ADVERSARIAL_CASES: &[AdversarialCase] = &[
    // A: ERR_MALFORMED_UTF8
    AdversarialCase {
        name: "at_001_overlong_2byte",
        input: AdversarialInput::Bytes(AT_001_OVERLONG_2BYTE),
        expected: AdversarialExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    AdversarialCase {
        name: "at_002_overlong_3byte",
        input: AdversarialInput::Bytes(AT_002_OVERLONG_3BYTE),
        expected: AdversarialExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    AdversarialCase {
        name: "at_003_isolated_cont",
        input: AdversarialInput::Bytes(AT_003_ISOLATED_CONT),
        expected: AdversarialExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    AdversarialCase {
        name: "at_004_truncated_2byte",
        input: AdversarialInput::Bytes(AT_004_TRUNCATED_2BYTE),
        expected: AdversarialExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    AdversarialCase {
        name: "at_005_truncated_3byte",
        input: AdversarialInput::Bytes(AT_005_TRUNCATED_3BYTE),
        expected: AdversarialExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    AdversarialCase {
        name: "at_006_invalid_lead_fe",
        input: AdversarialInput::Bytes(AT_006_INVALID_LEAD_FE),
        expected: AdversarialExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    AdversarialCase {
        name: "at_007_invalid_lead_ff",
        input: AdversarialInput::Bytes(AT_007_INVALID_LEAD_FF),
        expected: AdversarialExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    AdversarialCase {
        name: "at_008_surrogate_d800",
        input: AdversarialInput::Bytes(AT_008_SURROGATE_D800),
        expected: AdversarialExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    AdversarialCase {
        name: "at_009_surrogate_dfff",
        input: AdversarialInput::Bytes(AT_009_SURROGATE_DFFF),
        expected: AdversarialExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    AdversarialCase {
        name: "at_010_above_10ffff",
        input: AdversarialInput::Bytes(AT_010_ABOVE_10FFFF),
        expected: AdversarialExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },

    // B: ERR_UNMAPPED_CODEPOINT
    AdversarialCase {
        name: "at_011_latin_a_in_context",
        input: AdversarialInput::Bytes(AT_011_LATIN_A_IN_CONTEXT),
        expected: AdversarialExpected::Err {
            error_kind: "UnmappedCodepoint",
        },
    },
    AdversarialCase {
        name: "at_012_greek_alpha",
        input: AdversarialInput::Bytes(AT_012_GREEK_ALPHA),
        expected: AdversarialExpected::Err {
            error_kind: "UnmappedCodepoint",
        },
    },
    AdversarialCase {
        name: "at_013_cjk",
        input: AdversarialInput::Bytes(AT_013_CJK),
        expected: AdversarialExpected::Err {
            error_kind: "UnmappedCodepoint",
        },
    },
    AdversarialCase {
        name: "at_014_pua",
        input: AdversarialInput::Bytes(AT_014_PUA),
        expected: AdversarialExpected::Err {
            error_kind: "UnmappedCodepoint",
        },
    },
    AdversarialCase {
        name: "at_015_arabic_ext_a",
        input: AdversarialInput::Bytes(AT_015_ARABIC_EXT_A),
        expected: AdversarialExpected::Err {
            error_kind: "UnmappedCodepoint",
        },
    },
    AdversarialCase {
        name: "at_016_farsi_yeh",
        input: AdversarialInput::Bytes(AT_016_FARSI_YEH),
        expected: AdversarialExpected::Err {
            error_kind: "UnmappedCodepoint",
        },
    },

    // C: ERR_ORPHAN_DIACRITIC
    AdversarialCase {
        name: "at_017_orphan_fatha",
        input: AdversarialInput::Bytes(AT_017_ORPHAN_FATHA),
        expected: AdversarialExpected::Err {
            error_kind: "OrphanDiacritic",
        },
    },
    AdversarialCase {
        name: "at_018_orphan_shadda",
        input: AdversarialInput::Bytes(AT_018_ORPHAN_SHADDA),
        expected: AdversarialExpected::Err {
            error_kind: "OrphanDiacritic",
        },
    },
    AdversarialCase {
        name: "at_019_orphan_tanween_fath",
        input: AdversarialInput::Bytes(AT_019_ORPHAN_TANWEEN_FATH),
        expected: AdversarialExpected::Err {
            error_kind: "OrphanDiacritic",
        },
    },
    AdversarialCase {
        name: "at_020_orphan_super_alef",
        input: AdversarialInput::Bytes(AT_020_ORPHAN_SUPER_ALEF),
        expected: AdversarialExpected::Err {
            error_kind: "OrphanDiacritic",
        },
    },
    AdversarialCase {
        name: "at_021_double_fatha_orphan",
        input: AdversarialInput::Bytes(AT_021_DOUBLE_FATHA_ORPHAN),
        expected: AdversarialExpected::Err {
            error_kind: "OrphanDiacritic",
        },
    },
    AdversarialCase {
        name: "at_022_bom_then_fatha",
        input: AdversarialInput::Bytes(AT_022_BOM_THEN_FATHA),
        expected: AdversarialExpected::Err {
            error_kind: "OrphanDiacritic",
        },
    },

    // D: ERR_INVALID_MARK_COMBO
    AdversarialCase {
        name: "at_023_fatha_damma",
        input: AdversarialInput::Bytes(AT_023_FATHA_DAMMA),
        expected: AdversarialExpected::Err {
            error_kind: "InvalidMarkCombo",
        },
    },
    AdversarialCase {
        name: "at_024_kasra_sukun",
        input: AdversarialInput::Bytes(AT_024_KASRA_SUKUN),
        expected: AdversarialExpected::Err {
            error_kind: "InvalidMarkCombo",
        },
    },
    AdversarialCase {
        name: "at_025_shadda_sukun",
        input: AdversarialInput::Bytes(AT_025_SHADDA_SUKUN),
        expected: AdversarialExpected::Err {
            error_kind: "InvalidMarkCombo",
        },
    },
    AdversarialCase {
        name: "at_026_fatha_kasra",
        input: AdversarialInput::Bytes(AT_026_FATHA_KASRA),
        expected: AdversarialExpected::Err {
            error_kind: "InvalidMarkCombo",
        },
    },
    AdversarialCase {
        name: "at_027_three_diacritics",
        input: AdversarialInput::Bytes(AT_027_THREE_DIACRITICS),
        expected: AdversarialExpected::Err {
            error_kind: "InvalidMarkCombo",
        },
    },
    AdversarialCase {
        name: "at_028_damma_on_space",
        input: AdversarialInput::Bytes(AT_028_DAMMA_ON_SPACE),
        expected: AdversarialExpected::Err {
            error_kind: "InvalidMarkCombo",
        },
    },
    AdversarialCase {
        name: "at_029_fatha_on_digit",
        input: AdversarialInput::Bytes(AT_029_FATHA_ON_DIGIT),
        expected: AdversarialExpected::Err {
            error_kind: "InvalidMarkCombo",
        },
    },

    // F: ERR_INVALID_PROSODY (protocol-level only)
    AdversarialCase {
        name: "at_036_tanween_fath_plus_fatha",
        input: AdversarialInput::Bytes(AT_036_TANWEEN_FATH_PLUS_FATHA),
        expected: AdversarialExpected::Err {
            error_kind: "InvalidProsody",
        },
    },
    AdversarialCase {
        name: "at_040_tanween_fath_on_space",
        input: AdversarialInput::Bytes(AT_040_TANWEEN_FATH_ON_SPACE),
        expected: AdversarialExpected::Err {
            error_kind: "InvalidProsody",
        },
    },
    AdversarialCase {
        name: "at_new_duplicate_tanween_fath_same_atom",
        input: AdversarialInput::Bytes(AT_NEW_DUPLICATE_TANWEEN_FATH_SAME_ATOM),
        expected: AdversarialExpected::Err {
            error_kind: "InvalidProsody",
        },
    },
    AdversarialCase {
        name: "at_new_duplicate_superscript_alef_same_atom",
        input: AdversarialInput::Bytes(AT_NEW_DUPLICATE_SUPERSCRIPT_ALEF_SAME_ATOM),
        expected: AdversarialExpected::Err {
            error_kind: "InvalidProsody",
        },
    },
    AdversarialCase {
        name: "at_041_superscript_alef_after_space",
        input: AdversarialInput::Bytes(AT_041_SUPERSCRIPT_ALEF_AFTER_SPACE),
        expected: AdversarialExpected::Err {
            error_kind: "InvalidProsody",
        },
    },

    // G: Attack vectors / robustness
    AdversarialCase {
        name: "at_042_oversized_input",
        input: AdversarialInput::Generated(input_at_042_oversized),
        expected: AdversarialExpected::Err {
            error_kind: "InputTooLarge",
        },
    },
    AdversarialCase {
        name: "at_043_diacritic_flood",
        input: AdversarialInput::Generated(input_at_043_diacritic_flood),
        expected: AdversarialExpected::Err {
            error_kind: "InvalidMarkCombo",
        },
    },
    AdversarialCase {
        name: "at_044_all_lam_alef_variants",
        input: AdversarialInput::Bytes(AT_044_ALL_LAM_ALEF_VARIANTS),
        expected: AdversarialExpected::Ok {
            stream_hex: "1700000000000000010000000000000017000000000000000100000000000000170000000000000001000000040000001700000000000000010000000400000017000000000000000100000001000000170000000000000001000000010000001700000000000000010000000200000017000000000000000100000002000000",
            core_hash: "970a22c9c61a1f9baae342a4707a75ae6657657723623ed42db8035db20821a8",
            phonetic_hash: "9bfd01f45834be69a9527d9b70882a5ba6e6ab3f8ce1a3c6b259a5b05d3d94bd",
        },
    },
    AdversarialCase {
        name: "at_045_zwj_injection_bismi",
        input: AdversarialInput::Bytes(AT_045_ZWJ_INJECTION_BISMI),
        expected: AdversarialExpected::Ok {
            stream_hex: "02000000000000000c000000000000001800000000000000",
            core_hash: "0fb2277838219bbb6fa949b0ddecb22bf25c69161168022a801b032b41f23ac3",
            phonetic_hash: "12a5a9738a06de8b2abd74637ae4efa3c52b29e3aba8b702bb26715fe6df7cf4",
        },
    },
    AdversarialCase {
        name: "at_046_bidi_rlo_attack",
        input: AdversarialInput::Bytes(AT_046_BIDI_RLO_ATTACK),
        expected: AdversarialExpected::Ok {
            stream_hex: "020000000000000001000000000000000200000000000000",
            core_hash: "a94b75e97b7b88f1cabe3547784d1d6c65558263dab844225c14d8583f2dee40",
            phonetic_hash: "495c2f30739a11b02392c39ef60eb863a0a227604632312bac0c06d2e96c3094",
        },
    },
];

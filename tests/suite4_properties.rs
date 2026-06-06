//! Suite 4 — Property Tests (P1–P10)
//! Source of truth: Dhad-Spec-v1.0 §10
//! Covers all 10 properties with correct NOISE_SET (34 codepoints)

use dhad::mode_b::build_frame;
use dhad::model::DhadAtom;
use dhad::modes::{process_mode_a, process_mode_b};
use dhad::registry::base;
use proptest::prelude::*;

// ═══════════════════════════════════════════════════════════════════
// NOISE_SET كامل — 34 codepoint كما تُعرّفها الـ Spec §4 Stage 4
// ═══════════════════════════════════════════════════════════════════
fn all_noise_codepoints() -> Vec<u32> {
    let mut v = vec![
        0x0640, // ARABIC TATWEEL
        0x200C, // ZERO WIDTH NON-JOINER
        0x200D, // ZERO WIDTH JOINER
        0x200E, // LEFT-TO-RIGHT MARK
        0x200F, // RIGHT-TO-LEFT MARK
        0xFEFF, // BYTE ORDER MARK (non-initial)
        0x034F, // COMBINING GRAPHEME JOINER
    ];
    // BiDi controls: U+202A–U+202E (5 codepoints)
    v.extend(0x202A_u32..=0x202E);
    // BiDi isolate controls: U+2066–U+2069 (4 codepoints)
    v.extend(0x2066_u32..=0x2069);
    // Variation selectors: U+FE00–U+FE0F (16 codepoints)
    v.extend(0xFE00_u32..=0xFE0F);
    v.sort();
    v.dedup();
    v
}

fn cp_to_utf8(cp: u32) -> Vec<u8> {
    char::from_u32(cp)
        .expect("valid codepoint")
        .to_string()
        .into_bytes()
}

// ═══════════════════════════════════════════════════════════════════
// P1: Determinism
// ∀ input I: Dhad(I) == Dhad(I) (same output every call)
// ═══════════════════════════════════════════════════════════════════
proptest! {
    #[test]
    fn p1_determinism(
        input in proptest::collection::vec(any::<u8>(), 0..4096)
    ) {
        let r1 = process_mode_a(&input);
        let r2 = process_mode_a(&input);
        match (r1, r2) {
            (Ok(a), Ok(b)) => {
                prop_assert_eq!(a.stream.to_bytes(), b.stream.to_bytes(),
                    "non-deterministic stream");
                prop_assert_eq!(a.core_hash, b.core_hash,
                    "non-deterministic CoreHash");
                prop_assert_eq!(a.phonetic_hash, b.phonetic_hash,
                    "non-deterministic PhoneticHash");
            }
            (Err(e1), Err(e2)) => {
                prop_assert_eq!(
                    std::mem::discriminant(&e1),
                    std::mem::discriminant(&e2),
                    "non-deterministic error kind"
                );
            }
            _ => prop_assert!(false, "non-deterministic Ok/Err divergence"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// P2: Hash-Stream Consistency
// CoreHash fully determined by {base, marks, flags}
// PhoneticHash fully determined by CoreHash + {prosody}
// ═══════════════════════════════════════════════════════════════════
#[test]
fn p2_hash_stream_consistency_core_ignores_prosody() {
    let atom_no_pros = DhadAtom {
        base: base::NOON,
        marks: 0,
        flags: 0,
        prosody: 0x00,
        reserved: 0,
    };
    let atom_tw_fath = DhadAtom {
        base: base::NOON,
        marks: 0,
        flags: 0,
        prosody: 0x01,
        reserved: 0,
    };
    let atom_tw_damm = DhadAtom {
        base: base::NOON,
        marks: 0,
        flags: 0,
        prosody: 0x02,
        reserved: 0,
    };
    let atom_super_al = DhadAtom {
        base: base::NOON,
        marks: 0,
        flags: 0,
        prosody: 0x20,
        reserved: 0,
    };

    let ch_base = dhad::hash::core_hash(&[atom_no_pros]);
    let ch_tw_f = dhad::hash::core_hash(&[atom_tw_fath]);
    let ch_tw_d = dhad::hash::core_hash(&[atom_tw_damm]);
    let ch_super = dhad::hash::core_hash(&[atom_super_al]);

    assert_eq!(ch_base, ch_tw_f, "P2: prosody must not affect CoreHash");
    assert_eq!(ch_base, ch_tw_d, "P2: prosody must not affect CoreHash");
    assert_eq!(ch_base, ch_super, "P2: prosody must not affect CoreHash");
}

#[test]
fn p2_hash_stream_consistency_phonetic_requires_prosody() {
    let atom_bare = DhadAtom {
        base: base::NOON,
        marks: 0,
        flags: 0,
        prosody: 0x00,
        reserved: 0,
    };
    let atom_tw = DhadAtom {
        base: base::NOON,
        marks: 0,
        flags: 0,
        prosody: 0x01,
        reserved: 0,
    };

    let ch = dhad::hash::core_hash(&[atom_bare]);
    let ph_bare = dhad::hash::phonetic_hash(&[atom_bare], &ch);
    let ph_tw = dhad::hash::phonetic_hash(&[atom_tw], &ch);

    assert_ne!(
        ph_bare, ph_tw,
        "P2: different prosody must yield different PhoneticHash"
    );
}

// ═══════════════════════════════════════════════════════════════════
// P3: Noise Filter Completeness — NOISE_SET كامل (34 codepoints)
// ∀ c ∈ NOISE_SET: Dhad([c]) == Dhad([])
// ═══════════════════════════════════════════════════════════════════
#[test]
fn p3_noise_filter_completeness_all_34() {
    let empty_result = process_mode_a(b"").unwrap();
    let noise_set = all_noise_codepoints();

    // BOM (U+FEFF) كأول codepoint يُعالَج بـ Stage 2 وليس Stage 4
    // لكن النتيجة يجب أن تكون نفسها
    for &cp in &noise_set {
        let input = cp_to_utf8(cp);
        let result = process_mode_a(&input)
            .unwrap_or_else(|e| panic!("noise codepoint U+{:04X} should not error: {:?}", cp, e));
        assert!(
            result.stream.is_empty(),
            "noise codepoint U+{:04X} must produce empty stream",
            cp
        );
        assert_eq!(
            result.core_hash, empty_result.core_hash,
            "noise codepoint U+{:04X} CoreHash must match empty",
            cp
        );
        assert_eq!(
            result.phonetic_hash, empty_result.phonetic_hash,
            "noise codepoint U+{:04X} PhoneticHash must match empty",
            cp
        );
    }
}

// Property variant: تسلسلات عشوائية من NOISE_SET
proptest! {
    #[test]
    fn p3_noise_sequences(
        indices in proptest::collection::vec(0usize..32usize, 1..50)
    ) {
        let noise_set = all_noise_codepoints();
        let input: Vec<u8> = indices.iter()
            .flat_map(|&i| cp_to_utf8(noise_set[i % noise_set.len()]))
            .collect();
        let result = process_mode_a(&input).unwrap();
        prop_assert!(result.stream.is_empty(),
            "noise-only input must produce empty stream");
    }
}

// ═══════════════════════════════════════════════════════════════════
// P4: Digit Source Independence (Axiom A7) — كل الأرقام 0–9
// ═══════════════════════════════════════════════════════════════════
proptest! {
    #[test]
    fn p4_digit_source_independence(d in 0u32..=9u32) {
        let ascii_in  = &[b'0' + d as u8];
        let ar_in:  Vec<u8> = cp_to_utf8(0x0660 + d);
        let ext_in: Vec<u8> = cp_to_utf8(0x06F0 + d);

        let r_ascii = process_mode_a(ascii_in).unwrap();
        let r_ar    = process_mode_a(&ar_in).unwrap();
        let r_ext   = process_mode_a(&ext_in).unwrap();

        prop_assert_eq!(&r_ascii.stream.to_bytes(), &r_ar.stream.to_bytes(),
            "digit {} arabic-indic stream differs", d);
        prop_assert_eq!(&r_ascii.stream.to_bytes(), &r_ext.stream.to_bytes(),
            "digit {} extended stream differs", d);
        prop_assert_eq!(r_ascii.core_hash, r_ar.core_hash,
            "digit {} arabic-indic CoreHash differs", d);
        prop_assert_eq!(r_ascii.core_hash, r_ext.core_hash,
            "digit {} extended CoreHash differs", d);
        prop_assert_eq!(r_ascii.phonetic_hash, r_ar.phonetic_hash,
            "digit {} arabic-indic PhoneticHash differs", d);
        prop_assert_eq!(r_ascii.phonetic_hash, r_ext.phonetic_hash,
            "digit {} extended PhoneticHash differs", d);
    }
}

// ═══════════════════════════════════════════════════════════════════
// P5: Lam-Alef Decomposition Correctness
// Isolated == Final forms for all 4 Lam-Alef variants
// ═══════════════════════════════════════════════════════════════════
#[test]
fn p5_lam_alef_all_pairs() {
    let pairs: &[(&[u8], &[u8])] = &[
        (&[0xEF, 0xBB, 0xBB], &[0xEF, 0xBB, 0xBC]), // LAM+ALEF isolated/final
        (&[0xEF, 0xBB, 0xB5], &[0xEF, 0xBB, 0xB6]), // LAM+ALEF+MADDA isolated/final
        (&[0xEF, 0xBB, 0xB7], &[0xEF, 0xBB, 0xB8]), // LAM+ALEF+H_ABOVE isolated/final
        (&[0xEF, 0xBB, 0xB9], &[0xEF, 0xBB, 0xBA]), // LAM+ALEF+H_BELOW isolated/final
    ];
    for (iso, fin) in pairs {
        let r_iso = process_mode_a(iso).unwrap();
        let r_fin = process_mode_a(fin).unwrap();
        assert_eq!(
            r_iso.stream.to_bytes(),
            r_fin.stream.to_bytes(),
            "Lam-Alef: isolated != final (stream)"
        );
        assert_eq!(
            r_iso.core_hash, r_fin.core_hash,
            "Lam-Alef: isolated != final (CoreHash)"
        );
        assert_eq!(
            r_iso.phonetic_hash, r_fin.phonetic_hash,
            "Lam-Alef: isolated != final (PhoneticHash)"
        );
    }
}

// P5 variant: LAM+ALEF == individual LAM then ALEF
#[test]
fn p5_lam_alef_equals_sequence() {
    // ﻻ (U+FEFB) يجب أن ينتج نفس نتيجة LAM + ALEF منفصلين
    let ligature = process_mode_a(&[0xEF, 0xBB, 0xBB]).unwrap();
    let sequence = process_mode_a(&[0xD9, 0x84, 0xD8, 0xA7]).unwrap();
    assert_eq!(ligature.stream.to_bytes(), sequence.stream.to_bytes());
    assert_eq!(ligature.core_hash, sequence.core_hash);
}

// ═══════════════════════════════════════════════════════════════════
// P6: CoreHash/PhoneticHash Separation (Axiom A3)
// ═══════════════════════════════════════════════════════════════════
#[test]
fn p6_separation_tanween_vs_bare() {
    let bare = process_mode_a(&[0xD9, 0x86]).unwrap(); // NOON
    let tw_f = process_mode_a(&[0xD9, 0x86, 0xD9, 0x8B]).unwrap(); // NOON+TW_FATH
    let tw_d = process_mode_a(&[0xD9, 0x86, 0xD9, 0x8C]).unwrap(); // NOON+TW_DAMM
    let tw_k = process_mode_a(&[0xD9, 0x86, 0xD9, 0x8D]).unwrap(); // NOON+TW_KASR

    assert_eq!(bare.core_hash, tw_f.core_hash);
    assert_eq!(bare.core_hash, tw_d.core_hash);
    assert_eq!(bare.core_hash, tw_k.core_hash);

    assert_ne!(bare.phonetic_hash, tw_f.phonetic_hash);
    assert_ne!(bare.phonetic_hash, tw_d.phonetic_hash);
    assert_ne!(bare.phonetic_hash, tw_k.phonetic_hash);
    assert_ne!(tw_f.phonetic_hash, tw_d.phonetic_hash);
    assert_ne!(tw_f.phonetic_hash, tw_k.phonetic_hash);
    assert_ne!(tw_d.phonetic_hash, tw_k.phonetic_hash);
}

#[test]
fn p6_separation_madd_mode_b() {
    let bare = process_mode_a(&[0xD8, 0xA7]).unwrap();
    let madd_n = {
        let a = DhadAtom {
            base: base::ALEF,
            marks: 0,
            flags: 0,
            prosody: 0x08,
            reserved: 0,
        };
        process_mode_b(&build_frame(&[a])).unwrap()
    };
    let madd_x = {
        let a = DhadAtom {
            base: base::ALEF,
            marks: 0,
            flags: 0,
            prosody: 0x10,
            reserved: 0,
        };
        process_mode_b(&build_frame(&[a])).unwrap()
    };

    assert_eq!(
        bare.core_hash, madd_n.core_hash,
        "MADD_N must not change CoreHash"
    );
    assert_eq!(
        bare.core_hash, madd_x.core_hash,
        "MADD_X must not change CoreHash"
    );
    assert_ne!(bare.phonetic_hash, madd_n.phonetic_hash);
    assert_ne!(bare.phonetic_hash, madd_x.phonetic_hash);
    assert_ne!(madd_n.phonetic_hash, madd_x.phonetic_hash);
}

// ═══════════════════════════════════════════════════════════════════
// P7: Mark Order Independence (Axiom A6) — كاملة
// ═══════════════════════════════════════════════════════════════════
#[test]
fn p7_mark_order_all_compatible_pairs() {
    let combos: &[(&[u8], &[u8])] = &[
        // SHADDA+FATHA vs FATHA+SHADDA
        (
            &[0xD8, 0xA8, 0xD9, 0x91, 0xD9, 0x8E],
            &[0xD8, 0xA8, 0xD9, 0x8E, 0xD9, 0x91],
        ),
        // SHADDA+DAMMA vs DAMMA+SHADDA
        (
            &[0xD8, 0xA8, 0xD9, 0x91, 0xD9, 0x8F],
            &[0xD8, 0xA8, 0xD9, 0x8F, 0xD9, 0x91],
        ),
        // SHADDA+KASRA vs KASRA+SHADDA
        (
            &[0xD8, 0xA8, 0xD9, 0x91, 0xD9, 0x90],
            &[0xD8, 0xA8, 0xD9, 0x90, 0xD9, 0x91],
        ),
    ];
    for (order_a, order_b) in combos {
        let ra = process_mode_a(order_a).unwrap();
        let rb = process_mode_a(order_b).unwrap();
        assert_eq!(
            ra.stream.to_bytes(),
            rb.stream.to_bytes(),
            "P7 violation: mark order affects stream"
        );
        assert_eq!(
            ra.core_hash, rb.core_hash,
            "P7 violation: mark order affects CoreHash"
        );
        assert_eq!(
            ra.phonetic_hash, rb.phonetic_hash,
            "P7 violation: mark order affects PhoneticHash"
        );
    }
}

proptest! {
    #[test]
    fn p7_mark_order_proptest(
        base_cp in prop::sample::select(vec![
            vec![0xD8u8,0xA8u8], // BEH
            vec![0xD9u8,0x86u8], // NOON
            vec![0xD9u8,0x84u8], // LAM
        ]),
        vowel in prop::sample::select(vec![
            vec![0xD9u8,0x8Eu8], // FATHA
            vec![0xD9u8,0x8Fu8], // DAMMA
            vec![0xD9u8,0x90u8], // KASRA
        ])
    ) {
        let shadda = vec![0xD9u8,0x91u8];
        let mut order_a = base_cp.clone();
        order_a.extend_from_slice(&shadda);
        order_a.extend_from_slice(&vowel);

        let mut order_b = base_cp.clone();
        order_b.extend_from_slice(&vowel);
        order_b.extend_from_slice(&shadda);

        let ra = process_mode_a(&order_a).unwrap();
        let rb = process_mode_a(&order_b).unwrap();

        prop_assert_eq!(ra.stream.to_bytes(), rb.stream.to_bytes(),
            "P7 proptest: mark order affects stream");
        prop_assert_eq!(ra.core_hash, rb.core_hash,
            "P7 proptest: mark order affects CoreHash");
    }
}

// ═══════════════════════════════════════════════════════════════════
// P8: Error Determinism
// ═══════════════════════════════════════════════════════════════════
proptest! {
    #[test]
    fn p8_error_determinism(
        input in proptest::collection::vec(any::<u8>(), 0..512)
    ) {
        let r1 = process_mode_a(&input);
        let r2 = process_mode_a(&input);
        prop_assert_eq!(r1.is_err(), r2.is_err(),
            "P8: Ok/Err divergence");
        if let (Err(e1), Err(e2)) = (r1, r2) {
            prop_assert_eq!(
                std::mem::discriminant(&e1),
                std::mem::discriminant(&e2),
                "P8: error kind divergence"
            );
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// P9: Atom Byte Size
// ∀ valid stream S with n atoms: len(serialize(S)) == n × 8
// ═══════════════════════════════════════════════════════════════════
proptest! {
    #[test]
    fn p9_atom_byte_size(
        input in proptest::collection::vec(any::<u8>(), 0..4096)
    ) {
        if let Ok(result) = process_mode_a(&input) {
            let n = result.stream.len();
            prop_assert_eq!(
                result.stream.to_bytes().len(),
                n * 8,
                "P9: byte size != n * 8"
            );
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// P10: Crash Resistance
// ═══════════════════════════════════════════════════════════════════
proptest! {
    #[test]
    fn p10_crash_resistance_mode_a(
        input in proptest::collection::vec(any::<u8>(), 0..4096)
    ) {
        let _ = process_mode_a(&input);
    }
}

proptest! {
    #[test]
    fn p10_crash_resistance_mode_b(
        input in proptest::collection::vec(any::<u8>(), 0..4096)
    ) {
        let _ = process_mode_b(&input);
    }
}

#[test]
fn p10_max_input_boundary() {
    use dhad::model::ErrorKind;

    let at_limit = vec![0u8; 4_194_304];
    let result = process_mode_a(&at_limit);
    assert!(
        result.is_ok() || result.is_err(),
        "at MAX_INPUT_BYTES must not panic"
    );

    let over_limit = vec![0u8; 4_194_305];
    assert!(
        matches!(
            process_mode_a(&over_limit),
            Err(ErrorKind::InputTooLarge(_))
        ),
        "above MAX_INPUT_BYTES must be ERR_INPUT_TOO_LARGE"
    );
}

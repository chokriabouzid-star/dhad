//! Suite 2 — Mode B Tagged Binary Tests
//! Source of truth: Dhad-Spec-v1.0 §5.2, GT-T01–GT-T04
//! GT-092/093/094/095 from JSON (previously misclassified as Mode A)
//! are re-integrated here as Mode B tests via build_frame().

use dhad::mode_b::build_frame;
use dhad::model::{DhadAtom, ErrorKind};
use dhad::modes::process_mode_b;
use dhad::registry::base;

// ═══════════════════════════════════════════════════════════════════
// Helper: build a single-atom frame and process it
// ═══════════════════════════════════════════════════════════════════
fn single_atom_result(base_id: u16, marks: u16, flags: u8, prosody: u8) -> dhad::model::DhadResult {
    let atom = DhadAtom {
        base: base_id,
        marks,
        flags,
        prosody,
        reserved: 0,
    };
    let frame = build_frame(&[atom]);
    process_mode_b(&frame).unwrap_or_else(|e| panic!("unexpected error: {:?}", e))
}

// ═══════════════════════════════════════════════════════════════════
// GT-T01: ALEF + MADD_NORMAL
// CoreHash == GT-001 (ALEF bare) — MADD affects only PhoneticHash
// ═══════════════════════════════════════════════════════════════════
#[test]
fn gt_t01_alef_madd_normal() {
    let r = single_atom_result(base::ALEF, 0x0000, 0x00, 0x08);

    assert_eq!(
        r.stream.to_bytes(),
        &[0x01, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00],
        "stream mismatch"
    );
    assert_eq!(
        hex::encode(r.core_hash),
        "68d32b955388e186a3ad963008c4aed8f9d957d9fe72ad0e29ad5012d57e140d",
        "CoreHash must match GT-001 (ALEF bare)"
    );
    assert_eq!(
        hex::encode(r.phonetic_hash),
        "81c01948a1bde7141ecbd8aef66b1914544cd8b969351e8112259c53a826d6a1"
    );
}

// ═══════════════════════════════════════════════════════════════════
// GT-T02: WAW + MADD_EXTENDED
// CoreHash == GT-027 (WAW bare)
// ═══════════════════════════════════════════════════════════════════
#[test]
fn gt_t02_waw_madd_extended() {
    let r = single_atom_result(base::WAW, 0x0000, 0x00, 0x10);

    assert_eq!(
        r.stream.to_bytes(),
        &[0x1B, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00]
    );
    assert_eq!(
        hex::encode(r.core_hash),
        "34161eaaa10194d217c8726f773fd5e21e9abd0890c8b4e760d7b90b0f64a42a",
        "CoreHash must match GT-027 (WAW bare)"
    );
    assert_eq!(
        hex::encode(r.phonetic_hash),
        "1f8a62d3ff11167e2db0d0b317808dbd89502cf3f1e66f4d56943e5c905f2ca8"
    );
}

// ═══════════════════════════════════════════════════════════════════
// GT-T03: YEH + MADD_NORMAL
// CoreHash == GT-028 (YEH bare)
// ═══════════════════════════════════════════════════════════════════
#[test]
fn gt_t03_yeh_madd_normal() {
    let r = single_atom_result(base::YEH, 0x0000, 0x00, 0x08);

    assert_eq!(
        r.stream.to_bytes(),
        &[0x1C, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00]
    );
    assert_eq!(
        hex::encode(r.core_hash),
        "8aac943540c928674e5b2e38ef9f89c08b80117c051921f7cd83ef933d5d62f8",
        "CoreHash must match GT-028 (YEH bare)"
    );
    assert_eq!(
        hex::encode(r.phonetic_hash),
        "6770112f294a8e8b6c149712a2e3f17da5b3a199cda8f0fbc321a01075636c80"
    );
}

// ═══════════════════════════════════════════════════════════════════
// GT-T04: ALEF_MAQSURA + MADD_NORMAL
// CoreHash == GT-031 (ALEF_MAQSURA bare)
// ═══════════════════════════════════════════════════════════════════
#[test]
fn gt_t04_alef_maqsura_madd_normal() {
    let r = single_atom_result(base::ALEF_MAQSURA, 0x0000, 0x00, 0x08);

    assert_eq!(
        r.stream.to_bytes(),
        &[0x22, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00]
    );
    assert_eq!(
        hex::encode(r.core_hash),
        "cbd7e522708e96087330790a3e2beb03115e545e8788400853c4056450c411cb",
        "CoreHash must match GT-031 (ALEF_MAQSURA bare)"
    );
    assert_eq!(
        hex::encode(r.phonetic_hash),
        "681d1c467f63196ce78f1eedfe9ea8296ea00798df94d184f24a9b404da58ccb"
    );
}

// ═══════════════════════════════════════════════════════════════════
// GT-T05/T06/T07/T08: JSON GT-092–095 re-integrated as Mode B
// These were misclassified in JSON as Mode A (had input_utf8_hex)
// but expect MADD bits — impossible in Mode A per Spec §5.1
// ═══════════════════════════════════════════════════════════════════
#[test]
fn gt_t05_json_gt092_alef_madd_normal() {
    // JSON GT-092: "ALEF + MADD_NORMAL — pre-annotated"
    // Same as GT-T01 but validates JSON hash values match
    let r = single_atom_result(base::ALEF, 0x0000, 0x00, 0x08);
    assert_eq!(
        hex::encode(r.core_hash),
        "68d32b955388e186a3ad963008c4aed8f9d957d9fe72ad0e29ad5012d57e140d"
    );
    assert_eq!(
        hex::encode(r.phonetic_hash),
        "81c01948a1bde7141ecbd8aef66b1914544cd8b969351e8112259c53a826d6a1"
    );
}

#[test]
fn gt_t06_json_gt093_waw_madd_extended() {
    // JSON GT-093: "WAW + MADD_EXTENDED — pre-annotated"
    let r = single_atom_result(base::WAW, 0x0000, 0x00, 0x10);
    assert_eq!(
        hex::encode(r.core_hash),
        "34161eaaa10194d217c8726f773fd5e21e9abd0890c8b4e760d7b90b0f64a42a"
    );
    assert_eq!(
        hex::encode(r.phonetic_hash),
        "1f8a62d3ff11167e2db0d0b317808dbd89502cf3f1e66f4d56943e5c905f2ca8"
    );
}

#[test]
fn gt_t07_json_gt094_yeh_madd_normal() {
    let r = single_atom_result(base::YEH, 0x0000, 0x00, 0x08);
    assert_eq!(
        hex::encode(r.phonetic_hash),
        "6770112f294a8e8b6c149712a2e3f17da5b3a199cda8f0fbc321a01075636c80"
    );
}

#[test]
fn gt_t08_json_gt095_alef_maqsura_madd_normal() {
    let r = single_atom_result(base::ALEF_MAQSURA, 0x0000, 0x00, 0x08);
    assert_eq!(
        hex::encode(r.phonetic_hash),
        "681d1c467f63196ce78f1eedfe9ea8296ea00798df94d184f24a9b404da58ccb"
    );
}

// ═══════════════════════════════════════════════════════════════════
// A3: CoreHash/PhoneticHash Separation — Mode B specific
// MADD must not affect CoreHash
// ═══════════════════════════════════════════════════════════════════
#[test]
fn gt_t_a3_madd_does_not_affect_core_hash() {
    // ALEF bare (Mode A)
    let mode_a = dhad::modes::process_mode_a(&[0xD8, 0xA7]).unwrap();

    // ALEF + MADD_NORMAL (Mode B)
    let madd_n = single_atom_result(base::ALEF, 0x0000, 0x00, 0x08);
    // ALEF + MADD_EXTENDED (Mode B)
    let madd_x = single_atom_result(base::ALEF, 0x0000, 0x00, 0x10);

    assert_eq!(
        mode_a.core_hash, madd_n.core_hash,
        "MADD_NORMAL must not affect CoreHash (A3)"
    );
    assert_eq!(
        mode_a.core_hash, madd_x.core_hash,
        "MADD_EXTENDED must not affect CoreHash (A3)"
    );
    assert_ne!(
        mode_a.phonetic_hash, madd_n.phonetic_hash,
        "MADD_NORMAL must affect PhoneticHash (A3)"
    );
    assert_ne!(
        mode_a.phonetic_hash, madd_x.phonetic_hash,
        "MADD_EXTENDED must affect PhoneticHash (A3)"
    );
    assert_ne!(
        madd_n.phonetic_hash, madd_x.phonetic_hash,
        "MADD_NORMAL and MADD_EXTENDED must produce different PhoneticHashes"
    );
}

// ═══════════════════════════════════════════════════════════════════
// Frame Structure Error Paths
// ═══════════════════════════════════════════════════════════════════

#[test]
fn frame_err_too_short() {
    // أقل من 14 bytes
    let frame = &[0x44, 0x48, 0x41, 0x44, 0x01, 0x42, 0x00];
    assert!(
        process_mode_b(frame).is_err(),
        "short frame must be rejected"
    );
}

#[test]
fn frame_err_wrong_magic() {
    let atom = DhadAtom {
        base: base::ALEF,
        marks: 0,
        flags: 0,
        prosody: 0,
        reserved: 0,
    };
    let mut frame = build_frame(&[atom]);
    frame[0] = 0xFF; // corrupt magic
    assert!(
        process_mode_b(&frame).is_err(),
        "wrong magic must be rejected"
    );
}

#[test]
fn frame_err_wrong_version() {
    let atom = DhadAtom {
        base: base::ALEF,
        marks: 0,
        flags: 0,
        prosody: 0,
        reserved: 0,
    };
    let mut frame = build_frame(&[atom]);
    frame[4] = 0x02; // wrong version
    assert!(
        process_mode_b(&frame).is_err(),
        "wrong version must be rejected"
    );
}

#[test]
fn frame_err_wrong_mode() {
    let atom = DhadAtom {
        base: base::ALEF,
        marks: 0,
        flags: 0,
        prosody: 0,
        reserved: 0,
    };
    let mut frame = build_frame(&[atom]);
    frame[5] = 0x41; // 'A' instead of 'B'
    assert!(
        process_mode_b(&frame).is_err(),
        "wrong mode must be rejected"
    );
}

#[test]
fn frame_err_wrong_crc() {
    let atom = DhadAtom {
        base: base::ALEF,
        marks: 0,
        flags: 0,
        prosody: 0,
        reserved: 0,
    };
    let mut frame = build_frame(&[atom]);
    let last = frame.len() - 1;
    frame[last] ^= 0xFF; // corrupt CRC
    assert!(
        process_mode_b(&frame).is_err(),
        "CRC mismatch must be rejected"
    );
}

#[test]
fn frame_err_n_atoms_overflow() {
    // n_atoms يُشير لـ 1000 ذرة لكن الـ frame قصير
    let mut frame = vec![
        0x44, 0x48, 0x41, 0x44, // DHAD
        0x01, 0x42, // version, mode
        0xE8, 0x03, 0x00, 0x00, // n_atoms = 1000 LE
    ];
    // أضف CRC مزيف (4 bytes)
    frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    assert!(
        process_mode_b(&frame).is_err(),
        "n_atoms overflow must be rejected before atom parsing"
    );
}

#[test]
fn frame_err_n_atoms_usize_overflow_like_value() {
    // قيمة كبيرة جداً في n_atoms (0xFFFF_FFFF) يجب أن تُرفض بلا panic
    let mut frame = vec![
        0x44, 0x48, 0x41, 0x44, // DHAD
        0x01, 0x42, // version, mode
        0xFF, 0xFF, 0xFF, 0xFF, // n_atoms = u32::MAX
    ];
    frame.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // fake CRC
    assert!(
        process_mode_b(&frame).is_err(),
        "huge n_atoms must be rejected deterministically without panic"
    );
}

// ═══════════════════════════════════════════════════════════════════
// CR-07: reserved field non-zero
// ═══════════════════════════════════════════════════════════════════
#[test]
fn frame_err_reserved_nonzero() {
    // بناء frame يدوياً مع reserved = 0x0001
    let mut frame = vec![
        0x44, 0x48, 0x41, 0x44, // magic
        0x01, 0x42, // version, mode
        0x01, 0x00, 0x00, 0x00, // n_atoms = 1
    ];
    // ذرة: base=ALEF, marks=0, flags=0, prosody=0, reserved=0x0001
    frame.extend_from_slice(&[
        0x01, 0x00, // base = ALEF
        0x00, 0x00, // marks = 0
        0x00, // flags = 0
        0x00, // prosody = 0
        0x01, 0x00, // reserved = 0x0001 ← CR-07 violation
    ]);
    // CRC صحيح
    let crc = crc32fast::hash(&frame);
    frame.extend_from_slice(&crc.to_le_bytes());

    let result = process_mode_b(&frame);
    assert!(
        matches!(result, Err(ErrorKind::ReservedFieldNonZero { .. })),
        "reserved != 0 must produce ERR_RESERVED_FIELD_NONZERO"
    );
}

// ═══════════════════════════════════════════════════════════════════
// CR-01: Reserved Base IDs في Mode B
// ═══════════════════════════════════════════════════════════════════
#[test]
fn frame_err_reserved_base_001d() {
    let atom = DhadAtom {
        base: 0x001D,
        marks: 0,
        flags: 0,
        prosody: 0,
        reserved: 0,
    };
    let frame = build_frame(&[atom]);
    assert!(
        matches!(
            process_mode_b(&frame),
            Err(ErrorKind::UnmappedCodepoint { .. })
        ),
        "base 0x001D must be rejected (CR-01)"
    );
}

#[test]
fn frame_err_reserved_base_001e() {
    let atom = DhadAtom {
        base: 0x001E,
        marks: 0,
        flags: 0,
        prosody: 0,
        reserved: 0,
    };
    let frame = build_frame(&[atom]);
    assert!(matches!(
        process_mode_b(&frame),
        Err(ErrorKind::UnmappedCodepoint { .. })
    ));
}

#[test]
fn frame_err_reserved_base_001f() {
    let atom = DhadAtom {
        base: 0x001F,
        marks: 0,
        flags: 0,
        prosody: 0,
        reserved: 0,
    };
    let frame = build_frame(&[atom]);
    assert!(matches!(
        process_mode_b(&frame),
        Err(ErrorKind::UnmappedCodepoint { .. })
    ));
}

// ═══════════════════════════════════════════════════════════════════
// CR-02: MADD bits مقبولة فقط على LONG_VOWEL_CLASS
// ═══════════════════════════════════════════════════════════════════
#[test]
fn frame_err_madd_on_beh() {
    // BEH ليس LONG_VOWEL_CLASS → MADD يجب أن يُرفض
    let atom = DhadAtom {
        base: base::BEH,
        marks: 0,
        flags: 0,
        prosody: 0x08,
        reserved: 0,
    };
    let frame = build_frame(&[atom]);
    assert!(
        matches!(
            process_mode_b(&frame),
            Err(ErrorKind::InvalidProsody { .. })
        ),
        "MADD_NORMAL on BEH must be rejected (I15)"
    );
}

#[test]
fn frame_err_madd_normal_and_extended() {
    // MADD_N | MADD_X معاً — I14
    let atom = DhadAtom {
        base: base::ALEF,
        marks: 0,
        flags: 0,
        prosody: 0x18,
        reserved: 0,
    };
    let frame = build_frame(&[atom]);
    assert!(
        matches!(
            process_mode_b(&frame),
            Err(ErrorKind::InvalidProsody { .. })
        ),
        "MADD_N|MADD_X must be rejected (I14)"
    );
}

// ═══════════════════════════════════════════════════════════════════
// CR-04: MAX_INPUT_BYTES يُطبَّق على Mode B
// ═══════════════════════════════════════════════════════════════════
#[test]
fn frame_err_oversized() {
    let big = vec![0u8; 4_194_305];
    assert!(
        matches!(process_mode_b(&big), Err(ErrorKind::InputTooLarge(_))),
        "Mode B must enforce MAX_INPUT_BYTES (CR-04)"
    );
}

// ═══════════════════════════════════════════════════════════════════
// Empty frame (n_atoms = 0) → empty AtomStream
// ═══════════════════════════════════════════════════════════════════
#[test]
fn gt_t_empty_frame() {
    let frame = build_frame(&[]);
    let r = process_mode_b(&frame).unwrap();
    assert!(
        r.stream.is_empty(),
        "empty frame must produce empty AtomStream"
    );
    assert_eq!(
        hex::encode(r.core_hash),
        "8dd837c60eff174e0f40e75636deedec4bf020751f97cfe10dd1cf7117b16be0",
        "empty stream CoreHash must match GT-E00 anchor"
    );
    assert_eq!(
        hex::encode(r.phonetic_hash),
        "c5f62e920f5b06c74a02f25341f63499f1132da19084eb38e7b806c4a60a03f7",
        "empty stream PhoneticHash must match GT-E00 anchor"
    );
}

// ═══════════════════════════════════════════════════════════════════
// Round-trip: Mode A → bytes → Mode B re-validation
// ═══════════════════════════════════════════════════════════════════
#[test]
fn round_trip_mode_a_to_mode_b() {
    // معالجة "بِسْمِ" بـ Mode A
    let input = &[
        0xD8, 0xA8, 0xD9, 0x90, 0xD8, 0xB3, 0xD9, 0x92, 0xD9, 0x85, 0xD9, 0x90,
    ];
    let mode_a = dhad::modes::process_mode_a(input).unwrap();

    // بناء Mode B frame من الذرات
    let atoms: Vec<DhadAtom> = mode_a.stream.atoms().to_vec();
    let frame = build_frame(&atoms);

    // إعادة المعالجة بـ Mode B
    let mode_b = process_mode_b(&frame).unwrap();

    // النتائج يجب أن تكون متطابقة
    assert_eq!(
        mode_a.stream.to_bytes(),
        mode_b.stream.to_bytes(),
        "round-trip stream mismatch"
    );
    assert_eq!(
        mode_a.core_hash, mode_b.core_hash,
        "round-trip CoreHash mismatch"
    );
    assert_eq!(
        mode_a.phonetic_hash, mode_b.phonetic_hash,
        "round-trip PhoneticHash mismatch"
    );
}

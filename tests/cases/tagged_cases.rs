use dhad::mode_b::build_frame;
use dhad::model::DhadAtom;
use dhad::registry::base;

/// Minimal declarative atom for Mode B vector cases.
///
/// `reserved` is intentionally omitted from the normal atom path because
/// `build_frame()` serializes atoms through `DhadAtom::to_bytes()`, which
/// always writes reserved = 0x0000 by design.
///
/// Cases that must exercise malformed binary structure or non-zero reserved
/// fields must use `TaggedInput::RawFrame(...)` or `TaggedInput::GeneratedFrame(...)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaggedAtom {
    pub base: u16,
    pub marks: u16,
    pub flags: u8,
    pub prosody: u8,
}

/// Input source for a Mode B vector.
///
/// - `Atoms(...)`: normal declarative path, later serialized via `build_frame()`.
/// - `RawFrame(...)`: byte-exact literal frame for fixed malformed/boundary inputs.
/// - `GeneratedFrame(...)`: deterministic runtime frame builder for cases that
///   cannot be expressed conveniently as a static literal, such as CRC-correct
///   malformed frames, oversized frames, or reserved != 0.
#[derive(Debug, Clone, Copy)]
pub enum TaggedInput {
    Atoms(&'static [TaggedAtom]),
    RawFrame(&'static [u8]),
    GeneratedFrame(fn() -> Vec<u8>),
}

#[derive(Debug, Clone, Copy)]
pub enum TaggedExpected {
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
pub struct TaggedCase {
    pub name: &'static str,
    pub input: TaggedInput,
    pub expected: TaggedExpected,
}

const fn atom(base: u16, marks: u16, flags: u8, prosody: u8) -> TaggedAtom {
    TaggedAtom {
        base,
        marks,
        flags,
        prosody,
    }
}

fn to_dhad_atom(a: TaggedAtom) -> DhadAtom {
    DhadAtom {
        base: a.base,
        marks: a.marks,
        flags: a.flags,
        prosody: a.prosody,
        reserved: 0,
    }
}

fn build_atoms_frame(atoms: &[TaggedAtom]) -> Vec<u8> {
    let atoms: Vec<DhadAtom> = atoms.iter().copied().map(to_dhad_atom).collect();
    build_frame(&atoms)
}

fn frame_wrong_magic() -> Vec<u8> {
    let mut frame = build_atoms_frame(&[atom(base::ALEF, 0, 0, 0)]);
    frame[0] = 0xFF;
    frame
}

fn frame_wrong_version() -> Vec<u8> {
    let mut frame = build_atoms_frame(&[atom(base::ALEF, 0, 0, 0)]);
    frame[4] = 0x02;
    frame
}

fn frame_wrong_mode() -> Vec<u8> {
    let mut frame = build_atoms_frame(&[atom(base::ALEF, 0, 0, 0)]);
    frame[5] = 0x41; // 'A' instead of 'B'
    frame
}

fn frame_wrong_crc() -> Vec<u8> {
    let mut frame = build_atoms_frame(&[atom(base::ALEF, 0, 0, 0)]);
    let last = frame.len() - 1;
    frame[last] ^= 0xFF;
    frame
}

fn frame_reserved_nonzero() -> Vec<u8> {
    let mut frame = vec![
        0x44, 0x48, 0x41, 0x44, // "DHAD"
        0x01, // version
        0x42, // mode
        0x01, 0x00, 0x00, 0x00, // n_atoms = 1
        0x01, 0x00, // base = ALEF
        0x00, 0x00, // marks = 0
        0x00, // flags = 0
        0x00, // prosody = 0
        0x01, 0x00, // reserved = 0x0001 (intentionally invalid)
    ];
    let crc = crc32fast::hash(&frame);
    frame.extend_from_slice(&crc.to_le_bytes());
    frame
}

fn frame_oversized() -> Vec<u8> {
    vec![0u8; 4_194_305]
}

// ─────────────────────────────────────────────────────────────────────────────
// Static atom sets
// ─────────────────────────────────────────────────────────────────────────────

const ATOMS_GT_T01: &[TaggedAtom] = &[atom(base::ALEF, 0x0000, 0x00, 0x08)];
const ATOMS_GT_T02: &[TaggedAtom] = &[atom(base::WAW, 0x0000, 0x00, 0x10)];
const ATOMS_GT_T03: &[TaggedAtom] = &[atom(base::YEH, 0x0000, 0x00, 0x08)];
const ATOMS_GT_T04: &[TaggedAtom] = &[atom(base::ALEF_MAQSURA, 0x0000, 0x00, 0x08)];

const ATOMS_E01_HAMZA_ABOVE_AND_BELOW: &[TaggedAtom] = &[atom(base::ALEF, 0, 0x03, 0)];
const ATOMS_E02_MADDA_AND_HAMZA_ABOVE: &[TaggedAtom] = &[atom(base::ALEF, 0, 0x05, 0)];
const ATOMS_E03_HAMZA_ABOVE_ON_MEEM: &[TaggedAtom] = &[atom(base::MEEM, 0, 0x01, 0)];
const ATOMS_E04_HAMZA_BELOW_ON_WAW: &[TaggedAtom] = &[atom(base::WAW, 0, 0x02, 0)];
const ATOMS_E05_MADDA_ON_NOON: &[TaggedAtom] = &[atom(base::NOON, 0, 0x04, 0)];

const ATOMS_F01_TW_FATH_AND_DAMM: &[TaggedAtom] = &[atom(base::NOON, 0, 0x00, 0x03)];

const ATOMS_RESERVED_BASE_001D: &[TaggedAtom] = &[atom(0x001D, 0, 0, 0)];
const ATOMS_RESERVED_BASE_001E: &[TaggedAtom] = &[atom(0x001E, 0, 0, 0)];
const ATOMS_RESERVED_BASE_001F: &[TaggedAtom] = &[atom(0x001F, 0, 0, 0)];

const ATOMS_MADD_ON_BEH: &[TaggedAtom] = &[atom(base::BEH, 0, 0x00, 0x08)];
const ATOMS_MADD_NORMAL_AND_EXTENDED: &[TaggedAtom] = &[atom(base::ALEF, 0, 0x00, 0x18)];

// ─────────────────────────────────────────────────────────────────────────────
// Static raw frames
// ─────────────────────────────────────────────────────────────────────────────

const FRAME_EMPTY: &[u8] = &[];

const FRAME_TOO_SHORT: &[u8] = &[0x44, 0x48, 0x41, 0x44, 0x01, 0x42, 0x00];

const FRAME_N_ATOMS_1000_BUT_TRUNCATED: &[u8] = &[
    0x44, 0x48, 0x41, 0x44, // DHAD
    0x01, 0x42, // version, mode
    0xE8, 0x03, 0x00, 0x00, // n_atoms = 1000
    0x00, 0x00, 0x00, 0x00, // fake CRC
];

const FRAME_N_ATOMS_U32_MAX_BUT_TRUNCATED: &[u8] = &[
    0x44, 0x48, 0x41, 0x44, // DHAD
    0x01, 0x42, // version, mode
    0xFF, 0xFF, 0xFF, 0xFF, // n_atoms = u32::MAX
    0x00, 0x00, 0x00, 0x00, // fake CRC
];

// ─────────────────────────────────────────────────────────────────────────────
// Canonical tagged vector set
//
// Included:
// - Mode B success vectors
// - Malformed frame protocol vectors
// - Internal invariant tests that are representable through process_mode_b()
//
// Intentionally omitted:
// - gt_t_a3_madd_does_not_affect_core_hash (relationship/property test)
// - round_trip_mode_a_to_mode_b         (relationship/property test)
// - suite5 mode_b_bad_magic/version/mode (coverage duplicates of suite2 frame_err_*)
// ─────────────────────────────────────────────────────────────────────────────

pub const TAGGED_CASES: &[TaggedCase] = &[
    // ── GT-T01..GT-T04 ──────────────────────────────────────────────────────
    TaggedCase {
        name: "gt_t01_alef_madd_normal",
        input: TaggedInput::Atoms(ATOMS_GT_T01),
        expected: TaggedExpected::Ok {
            stream_hex: "0100000000080000",
            core_hash: "68d32b955388e186a3ad963008c4aed8f9d957d9fe72ad0e29ad5012d57e140d",
            phonetic_hash: "81c01948a1bde7141ecbd8aef66b1914544cd8b969351e8112259c53a826d6a1",
        },
    },
    TaggedCase {
        name: "gt_t02_waw_madd_extended",
        input: TaggedInput::Atoms(ATOMS_GT_T02),
        expected: TaggedExpected::Ok {
            stream_hex: "1b00000000100000",
            core_hash: "34161eaaa10194d217c8726f773fd5e21e9abd0890c8b4e760d7b90b0f64a42a",
            phonetic_hash: "1f8a62d3ff11167e2db0d0b317808dbd89502cf3f1e66f4d56943e5c905f2ca8",
        },
    },
    TaggedCase {
        name: "gt_t03_yeh_madd_normal",
        input: TaggedInput::Atoms(ATOMS_GT_T03),
        expected: TaggedExpected::Ok {
            stream_hex: "1c00000000080000",
            core_hash: "8aac943540c928674e5b2e38ef9f89c08b80117c051921f7cd83ef933d5d62f8",
            phonetic_hash: "6770112f294a8e8b6c149712a2e3f17da5b3a199cda8f0fbc321a01075636c80",
        },
    },
    TaggedCase {
        name: "gt_t04_alef_maqsura_madd_normal",
        input: TaggedInput::Atoms(ATOMS_GT_T04),
        expected: TaggedExpected::Ok {
            stream_hex: "2200000000080000",
            core_hash: "cbd7e522708e96087330790a3e2beb03115e545e8788400853c4056450c411cb",
            phonetic_hash: "681d1c467f63196ce78f1eedfe9ea8296ea00798df94d184f24a9b404da58ccb",
        },
    },
    // ── GT-T05..GT-T08 ──────────────────────────────────────────────────────
    TaggedCase {
        name: "gt_t05_json_gt092_alef_madd_normal",
        input: TaggedInput::Atoms(ATOMS_GT_T01),
        expected: TaggedExpected::Ok {
            stream_hex: "0100000000080000",
            core_hash: "68d32b955388e186a3ad963008c4aed8f9d957d9fe72ad0e29ad5012d57e140d",
            phonetic_hash: "81c01948a1bde7141ecbd8aef66b1914544cd8b969351e8112259c53a826d6a1",
        },
    },
    TaggedCase {
        name: "gt_t06_json_gt093_waw_madd_extended",
        input: TaggedInput::Atoms(ATOMS_GT_T02),
        expected: TaggedExpected::Ok {
            stream_hex: "1b00000000100000",
            core_hash: "34161eaaa10194d217c8726f773fd5e21e9abd0890c8b4e760d7b90b0f64a42a",
            phonetic_hash: "1f8a62d3ff11167e2db0d0b317808dbd89502cf3f1e66f4d56943e5c905f2ca8",
        },
    },
    TaggedCase {
        name: "gt_t07_json_gt094_yeh_madd_normal",
        input: TaggedInput::Atoms(ATOMS_GT_T03),
        expected: TaggedExpected::Ok {
            stream_hex: "1c00000000080000",
            core_hash: "8aac943540c928674e5b2e38ef9f89c08b80117c051921f7cd83ef933d5d62f8",
            phonetic_hash: "6770112f294a8e8b6c149712a2e3f17da5b3a199cda8f0fbc321a01075636c80",
        },
    },
    TaggedCase {
        name: "gt_t08_json_gt095_alef_maqsura_madd_normal",
        input: TaggedInput::Atoms(ATOMS_GT_T04),
        expected: TaggedExpected::Ok {
            stream_hex: "2200000000080000",
            core_hash: "cbd7e522708e96087330790a3e2beb03115e545e8788400853c4056450c411cb",
            phonetic_hash: "681d1c467f63196ce78f1eedfe9ea8296ea00798df94d184f24a9b404da58ccb",
        },
    },
    // ── Empty valid frame ────────────────────────────────────────────────────
    TaggedCase {
        name: "gt_t_empty_frame",
        input: TaggedInput::Atoms(&[]),
        expected: TaggedExpected::Ok {
            stream_hex: "",
            core_hash: "8dd837c60eff174e0f40e75636deedec4bf020751f97cfe10dd1cf7117b16be0",
            phonetic_hash: "c5f62e920f5b06c74a02f25341f63499f1132da19084eb38e7b806c4a60a03f7",
        },
    },
    // ── Malformed frame boundaries / structure ───────────────────────────────
    TaggedCase {
        name: "mode_b_empty",
        input: TaggedInput::RawFrame(FRAME_EMPTY),
        expected: TaggedExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    TaggedCase {
        name: "frame_err_too_short",
        input: TaggedInput::RawFrame(FRAME_TOO_SHORT),
        expected: TaggedExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    TaggedCase {
        name: "frame_err_wrong_magic",
        input: TaggedInput::GeneratedFrame(frame_wrong_magic),
        expected: TaggedExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    TaggedCase {
        name: "frame_err_wrong_version",
        input: TaggedInput::GeneratedFrame(frame_wrong_version),
        expected: TaggedExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    TaggedCase {
        name: "frame_err_wrong_mode",
        input: TaggedInput::GeneratedFrame(frame_wrong_mode),
        expected: TaggedExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    TaggedCase {
        name: "frame_err_wrong_crc",
        input: TaggedInput::GeneratedFrame(frame_wrong_crc),
        expected: TaggedExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    TaggedCase {
        name: "frame_err_n_atoms_overflow",
        input: TaggedInput::RawFrame(FRAME_N_ATOMS_1000_BUT_TRUNCATED),
        expected: TaggedExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    TaggedCase {
        name: "frame_err_n_atoms_usize_overflow_like_value",
        input: TaggedInput::RawFrame(FRAME_N_ATOMS_U32_MAX_BUT_TRUNCATED),
        expected: TaggedExpected::Err {
            error_kind: "MalformedUtf8",
        },
    },
    TaggedCase {
        name: "frame_err_oversized",
        input: TaggedInput::GeneratedFrame(frame_oversized),
        expected: TaggedExpected::Err {
            error_kind: "InputTooLarge",
        },
    },
    // ── Reserved field / reserved bases ─────────────────────────────────────
    TaggedCase {
        name: "frame_err_reserved_nonzero",
        input: TaggedInput::GeneratedFrame(frame_reserved_nonzero),
        expected: TaggedExpected::Err {
            error_kind: "ReservedFieldNonZero",
        },
    },
    TaggedCase {
        name: "frame_err_reserved_base_001d",
        input: TaggedInput::Atoms(ATOMS_RESERVED_BASE_001D),
        expected: TaggedExpected::Err {
            error_kind: "UnmappedCodepoint",
        },
    },
    TaggedCase {
        name: "frame_err_reserved_base_001e",
        input: TaggedInput::Atoms(ATOMS_RESERVED_BASE_001E),
        expected: TaggedExpected::Err {
            error_kind: "UnmappedCodepoint",
        },
    },
    TaggedCase {
        name: "frame_err_reserved_base_001f",
        input: TaggedInput::Atoms(ATOMS_RESERVED_BASE_001F),
        expected: TaggedExpected::Err {
            error_kind: "UnmappedCodepoint",
        },
    },
    // ── Mode B prosody violations already present in suite2 ─────────────────
    TaggedCase {
        name: "frame_err_madd_on_beh",
        input: TaggedInput::Atoms(ATOMS_MADD_ON_BEH),
        expected: TaggedExpected::Err {
            error_kind: "InvalidProsody",
        },
    },
    TaggedCase {
        name: "frame_err_madd_normal_and_extended",
        input: TaggedInput::Atoms(ATOMS_MADD_NORMAL_AND_EXTENDED),
        expected: TaggedExpected::Err {
            error_kind: "InvalidProsody",
        },
    },
    // ── Promoted internal invariant tests: invalid flags ────────────────────
    TaggedCase {
        name: "at_e01_hamza_above_and_below",
        input: TaggedInput::Atoms(ATOMS_E01_HAMZA_ABOVE_AND_BELOW),
        expected: TaggedExpected::Err {
            error_kind: "InvalidFlagCombo",
        },
    },
    TaggedCase {
        name: "at_e02_madda_and_hamza_above",
        input: TaggedInput::Atoms(ATOMS_E02_MADDA_AND_HAMZA_ABOVE),
        expected: TaggedExpected::Err {
            error_kind: "InvalidFlagCombo",
        },
    },
    TaggedCase {
        name: "at_e03_hamza_above_on_meem",
        input: TaggedInput::Atoms(ATOMS_E03_HAMZA_ABOVE_ON_MEEM),
        expected: TaggedExpected::Err {
            error_kind: "InvalidFlagCombo",
        },
    },
    TaggedCase {
        name: "at_e04_hamza_below_on_waw",
        input: TaggedInput::Atoms(ATOMS_E04_HAMZA_BELOW_ON_WAW),
        expected: TaggedExpected::Err {
            error_kind: "InvalidFlagCombo",
        },
    },
    TaggedCase {
        name: "at_e05_madda_on_noon",
        input: TaggedInput::Atoms(ATOMS_E05_MADDA_ON_NOON),
        expected: TaggedExpected::Err {
            error_kind: "InvalidFlagCombo",
        },
    },
    // ── Promoted internal invariant test: tanween bit collision ─────────────
    TaggedCase {
        name: "at_f01_tanween_fath_and_damm_mode_b",
        input: TaggedInput::Atoms(ATOMS_F01_TW_FATH_AND_DAMM),
        expected: TaggedExpected::Err {
            error_kind: "InvalidProsody",
        },
    },
];

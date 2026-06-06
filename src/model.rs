/// One canonical unit of Arabic text.
///
/// Each `DhadAtom` represents a single grapheme after full canonicalization.
/// Size is exactly 8 bytes in wire format (see [`DhadAtom::to_bytes`]).
///
/// All fields satisfy invariants I01–I23 after Stage 10 validation.
///
/// # Field Layout (wire format, little-endian)
///
/// ```text
/// Bytes 0–1: base     (u16 LE) — Base ID from registry §3
/// Bytes 2–3: marks    (u16 LE) — diacritic bitmask §2.2
/// Byte  4:   flags    (u8)     — structural modifier §2.3
/// Byte  5:   prosody  (u8)     — prosodic annotation §2.4
/// Bytes 6–7: reserved (u16 LE) — MUST be 0x0000
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DhadAtom {
    pub base: u16,     // little-endian, Base ID registry §3
    pub marks: u16,    // little-endian, diacritic bitmask §2.2
    pub flags: u8,     // structural modifier bitmask §2.3
    pub prosody: u8,   // prosodic annotation bitmask §2.4
    pub reserved: u16, // MUST be 0x0000 — enforced by I22
}

impl DhadAtom {
    /// Serialize this atom to exactly 8 bytes in little-endian wire format.
    ///
    /// The reserved field is always written as `0x0000` regardless of
    /// the struct field value (which must be 0 after validation).
    pub fn to_bytes(self) -> [u8; 8] {
        let mut out = [0u8; 8];
        out[0..2].copy_from_slice(&self.base.to_le_bytes());
        out[2..4].copy_from_slice(&self.marks.to_le_bytes());
        out[4] = self.flags;
        out[5] = self.prosody;
        out[6..8].copy_from_slice(&0u16.to_le_bytes());
        out
    }
}

pub mod marks {
    pub const FATHA: u16 = 0x0001;
    pub const DAMMA: u16 = 0x0002;
    pub const KASRA: u16 = 0x0004;
    pub const SUKUN: u16 = 0x0008;
    pub const SHADDA: u16 = 0x0010;

    /// All combinations that pass I03. Any other value → ERR_INVALID_MARK_COMBO.
    pub const VALID: &[u16] = &[
        0x0000,
        FATHA,
        DAMMA,
        KASRA,
        SUKUN,
        SHADDA,
        SHADDA | FATHA,
        SHADDA | DAMMA,
        SHADDA | KASRA,
    ];
}

pub mod flags {
    pub const HAMZA_ABOVE: u8 = 0x01;
    pub const HAMZA_BELOW: u8 = 0x02;
    pub const MADDA: u8 = 0x04;
    pub const VALID: &[u8] = &[0x00, 0x01, 0x02, 0x04];
}

pub mod prosody {
    pub const TANWEEN_FATH: u8 = 0x01;
    pub const TANWEEN_DAMM: u8 = 0x02;
    pub const TANWEEN_KASR: u8 = 0x04;
    pub const MADD_NORMAL: u8 = 0x08; // Mode B only
    pub const MADD_EXTENDED: u8 = 0x10; // Mode B only
    pub const SUPERSCRIPT_ALEF: u8 = 0x20;
}

/// All error conditions the pipeline can produce.
///
/// Every error includes location information (`byte_offset` or `atom_index`)
/// to support precise error reporting. No silent correction is ever performed
/// (Axiom A2).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ErrorKind {
    #[error("input exceeds MAX_INPUT_BYTES ({0} bytes)")]
    InputTooLarge(usize),

    #[error("malformed UTF-8 at byte offset {byte_offset}")]
    MalformedUtf8 { byte_offset: usize },

    #[error("unmapped codepoint U+{codepoint:04X} at stream position {position}")]
    UnmappedCodepoint { codepoint: u32, position: usize },

    #[error(
        "orphan diacritic U+{codepoint:04X} at stream position {position}: no preceding base atom"
    )]
    OrphanDiacritic { codepoint: u32, position: usize },

    #[error("invalid mark combination 0x{marks:04X} on atom at index {atom_index}")]
    InvalidMarkCombo { marks: u16, atom_index: usize },

    #[error("invalid flag combination 0x{flags:02X} on atom at index {atom_index}")]
    InvalidFlagCombo { flags: u8, atom_index: usize },

    #[error("invalid prosody 0x{prosody:02X} on atom at index {atom_index}: {reason}")]
    InvalidProsody {
        prosody: u8,
        atom_index: usize,
        reason: &'static str,
    },

    #[error("reserved field non-zero (0x{reserved:04X}) on atom at index {atom_index}")]
    ReservedFieldNonZero { reserved: u16, atom_index: usize },
}

/// A canonicalized, validated sequence of [`DhadAtom`]s.
///
/// An empty `AtomStream` is valid and represents the canonical identity
/// of noise-only or empty input.
///
/// # Byte Size
///
/// `stream.to_bytes().len() == stream.len() * 8` (Property P9)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomStream {
    atoms: Vec<DhadAtom>,
}

impl AtomStream {
    pub fn new(atoms: Vec<DhadAtom>) -> Self {
        Self { atoms }
    }
    pub fn atoms(&self) -> &[DhadAtom] {
        &self.atoms
    }
    pub fn len(&self) -> usize {
        self.atoms.len()
    }
    pub fn is_empty(&self) -> bool {
        self.atoms.is_empty()
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.atoms.iter().flat_map(|a| a.to_bytes()).collect()
    }
}

/// The complete result of Dhad processing.
///
/// # Hash Relationship
///
/// - `core_hash` captures orthographic identity: `{base, marks, flags}`
/// - `phonetic_hash` commits to `core_hash` + `{prosody}` layer
/// - Two texts with identical `core_hash` may have different `phonetic_hash`
/// - Two texts with identical `phonetic_hash` necessarily have identical `core_hash`
#[derive(Debug)]
pub struct DhadResult {
    pub stream: AtomStream,
    pub core_hash: [u8; 32],
    pub phonetic_hash: [u8; 32],
}

use crate::model::{AtomStream, DhadResult, ErrorKind};
use crate::pipeline::*;
use crate::{hash, invariants, mode_b};

/// Process a UTF-8 byte slice through the full 12-stage Dhad pipeline (Mode A).
///
/// This is the primary entry point for standard Arabic text processing.
///
/// # Errors
///
/// Returns [`ErrorKind`] on any invalid input. All errors are deterministic:
/// the same invalid input always produces the same error (Property P8).
///
/// | Error | Cause |
/// |-------|-------|
/// | [`ErrorKind::InputTooLarge`] | input > 4 MiB |
/// | [`ErrorKind::MalformedUtf8`] | invalid byte sequence |
/// | [`ErrorKind::UnmappedCodepoint`] | codepoint not in Dhad v1.0 |
/// | [`ErrorKind::OrphanDiacritic`] | diacritic with no preceding base |
/// | [`ErrorKind::InvalidMarkCombo`] | incompatible diacritics |
/// | [`ErrorKind::InvalidFlagCombo`] | incompatible flags |
/// | [`ErrorKind::InvalidProsody`] | prosody violation |
///
/// # Examples
///
/// ```rust
/// use dhad::modes::process_mode_a;
///
/// // Valid input
/// let r = process_mode_a("بِسْمِ".as_bytes()).unwrap();
/// assert_eq!(r.stream.len(), 3);
///
/// // Empty input is valid (returns empty stream)
/// let r = process_mode_a(b"").unwrap();
/// assert!(r.stream.is_empty());
///
/// // Invalid input returns error
/// assert!(process_mode_a(&[0xC1, 0x41]).is_err()); // overlong UTF-8
/// ```
pub fn process_mode_a(input: &[u8]) -> Result<DhadResult, ErrorKind> {
    pre_stage_size_check(input)?;
    let codepoints = stage1_utf8::decode(input)?;
    let codepoints = stage2_bom::remove_bom(codepoints);
    let codepoints = stage3_faps::decompose(codepoints)?;
    let codepoints = stage4_noise::filter(codepoints);
    let tokens = stage5_classify::classify(&codepoints)?;
    let atoms = stage6_atoms::build(tokens)?;
    let atoms = stage7_flags::resolve(atoms); // flags already set in base_map
    let atoms = stage8_digits::normalize(atoms); // already normalized in base_map
    let atoms = stage9_prosody::resolve(atoms)?;
    stage10_crf::validate(&atoms)?;
    let stream = AtomStream::new(atoms);
    let ch = hash::core_hash(stream.atoms());
    let ph = hash::phonetic_hash(stream.atoms(), &ch);
    Ok(DhadResult {
        stream,
        core_hash: ch,
        phonetic_hash: ph,
    })
}

/// Process a Mode B tagged binary frame.
///
/// Mode B allows pre-annotated input with MADD bits (`0x08`, `0x10`)
/// that cannot be produced by Mode A (no Unicode source codepoints exist).
///
/// # Frame Format
///
/// ```text
/// magic   : 4 bytes  = b"DHAD"
/// version : 1 byte   = 0x01
/// mode    : 1 byte   = 0x42 ('B')
/// n_atoms : 4 bytes  = u32 LE
/// atoms   : n_atoms × 8 bytes
/// checksum: 4 bytes  = CRC-32 of all preceding bytes
/// ```
///
/// # Errors
///
/// In addition to the errors from Mode A validation, Mode B also produces:
///
/// | Error | Cause |
/// |-------|-------|
/// | [`ErrorKind::ReservedFieldNonZero`] | atom.reserved != 0 (CR-07) |
///
/// # Examples
///
/// ```rust
/// use dhad::modes::process_mode_b;
/// use dhad::mode_b::build_frame;
/// use dhad::model::DhadAtom;
/// use dhad::registry::base;
///
/// // ALEF with MADD_NORMAL annotation
/// let atom = DhadAtom { base: base::ALEF, marks: 0, flags: 0,
///                       prosody: 0x08, reserved: 0 };
/// let frame = build_frame(&[atom]);
/// let r = process_mode_b(&frame).unwrap();
/// assert_eq!(r.stream.len(), 1);
/// ```
pub fn process_mode_b(frame: &[u8]) -> Result<DhadResult, ErrorKind> {
    // Pre-stage: size check
    if frame.len() > MAX_INPUT_BYTES {
        return Err(ErrorKind::InputTooLarge(frame.len()));
    }

    // Parse frame → raw atoms (validates magic, version, CRC, reserved)
    let atoms = mode_b::parse_frame(frame)?;

    // Validate each atom against all 23 invariants
    for (idx, atom) in atoms.iter().enumerate() {
        invariants::validate_atom(atom, idx)?;
    }

    let stream = AtomStream::new(atoms);
    let ch = hash::core_hash(stream.atoms());
    let ph = hash::phonetic_hash(stream.atoms(), &ch);

    Ok(DhadResult {
        stream,
        core_hash: ch,
        phonetic_hash: ph,
    })
}

const MAX_INPUT_BYTES: usize = 4_194_304;

fn pre_stage_size_check(input: &[u8]) -> Result<(), ErrorKind> {
    if input.len() > MAX_INPUT_BYTES {
        Err(ErrorKind::InputTooLarge(input.len()))
    } else {
        Ok(())
    }
}

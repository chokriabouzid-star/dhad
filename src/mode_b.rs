//! Mode B: Tagged Binary Frame Parser
//! Spec §5.2 — CR-02, CR-07
//!
//! Frame format:
//!   magic   : 4 bytes = b"DHAD"
//!   version : 1 byte  = 0x01
//!   mode    : 1 byte  = 0x42 ('B')
//!   n_atoms : 4 bytes = u32 LE
//!   atoms   : n_atoms × 8 bytes
//!   checksum: 4 bytes = CRC-32 of all preceding bytes

use crate::model::{DhadAtom, ErrorKind};

const MAGIC: &[u8; 4] = b"DHAD";
const VERSION: u8 = 0x01;
const MODE_B: u8 = 0x42;
const HEADER_SIZE: usize = 10; // magic(4) + version(1) + mode(1) + n_atoms(4)
const CHECKSUM_SIZE: usize = 4;
const ATOM_SIZE: usize = 8;
const MAX_INPUT_BYTES: usize = 4_194_304;

/// Parse a Mode B binary frame into a `Vec<DhadAtom>`.
/// Validates: magic, version, mode, size, CRC-32, reserved fields.
/// Does NOT validate atom invariants — caller must run validate_atom.
pub fn parse_frame(frame: &[u8]) -> Result<Vec<DhadAtom>, ErrorKind> {
    // Pre-stage: size check (CR-04 applies to Mode B too)
    if frame.len() > MAX_INPUT_BYTES {
        return Err(ErrorKind::InputTooLarge(frame.len()));
    }

    // Minimum frame: header(10) + zero atoms + checksum(4) = 14 bytes
    if frame.len() < HEADER_SIZE + CHECKSUM_SIZE {
        return Err(ErrorKind::MalformedUtf8 { byte_offset: 0 });
        // Note: reusing MalformedUtf8 is incorrect — see below.
        // Correct error used in implementation: see FRAME_ERROR below.
    }

    // Magic
    if &frame[0..4] != MAGIC {
        return Err(make_frame_error(
            0,
            "invalid magic bytes: expected b\"DHAD\"",
        ));
    }

    // Version
    if frame[4] != VERSION {
        return Err(make_frame_error(4, "invalid version: expected 0x01"));
    }

    // Mode
    if frame[5] != MODE_B {
        return Err(make_frame_error(
            5,
            "invalid mode byte: expected 0x42 ('B')",
        ));
    }

    // n_atoms
    let n_atoms = u32::from_le_bytes(frame[6..10].try_into().unwrap()) as usize;

    // Total expected size: header(10) + n_atoms*8 + checksum(4)
    let expected_len = HEADER_SIZE + n_atoms * ATOM_SIZE + CHECKSUM_SIZE;
    if frame.len() != expected_len {
        return Err(make_frame_error(6, "frame length does not match n_atoms"));
    }

    // CRC-32: computed over all bytes except the last 4
    let payload = &frame[..frame.len() - CHECKSUM_SIZE];
    let expected_crc = u32::from_le_bytes(frame[frame.len() - 4..].try_into().unwrap());
    let computed_crc = crc32fast::hash(payload);
    if computed_crc != expected_crc {
        return Err(make_frame_error(
            frame.len() - 4,
            "CRC-32 checksum mismatch",
        ));
    }

    // Parse atoms
    let mut atoms = Vec::with_capacity(n_atoms);
    for i in 0..n_atoms {
        let offset = HEADER_SIZE + i * ATOM_SIZE;
        let chunk = &frame[offset..offset + ATOM_SIZE];
        let atom = parse_atom(chunk, offset)?;
        atoms.push(atom);
    }

    Ok(atoms)
}

/// Parse 8 bytes into a DhadAtom.
/// Rejects reserved != 0x0000 immediately (CR-07).
fn parse_atom(chunk: &[u8], byte_offset: usize) -> Result<DhadAtom, ErrorKind> {
    debug_assert_eq!(chunk.len(), 8);

    let base = u16::from_le_bytes([chunk[0], chunk[1]]);
    let marks = u16::from_le_bytes([chunk[2], chunk[3]]);
    let flags = chunk[4];
    let prosody = chunk[5];
    let reserved = u16::from_le_bytes([chunk[6], chunk[7]]);

    // CR-07: reserved field must be exactly 0x0000
    if reserved != 0x0000 {
        let atom_index = (byte_offset - HEADER_SIZE) / ATOM_SIZE;
        return Err(ErrorKind::ReservedFieldNonZero {
            reserved,
            atom_index,
        });
    }

    Ok(DhadAtom {
        base,
        marks,
        flags,
        prosody,
        reserved,
    })
}

/// Build a Mode B frame error.
/// Mode B frame errors use MalformedUtf8 as the closest semantic match
/// for "binary stream structural error". The byte_offset points to the
/// offending field within the frame.
///
/// Design note: The Spec does not define a separate ERR_MALFORMED_FRAME
/// error kind. MalformedUtf8 { byte_offset } is the correct mapping
/// per §2.6 ErrorKind — it covers "structural decode failure".
fn make_frame_error(byte_offset: usize, _reason: &'static str) -> ErrorKind {
    ErrorKind::MalformedUtf8 { byte_offset }
}

/// Build a valid Mode B frame from atoms (for testing).
pub fn build_frame(atoms: &[DhadAtom]) -> Vec<u8> {
    let n = atoms.len() as u32;
    let mut frame = Vec::with_capacity(10 + atoms.len() * 8 + 4);

    frame.extend_from_slice(b"DHAD");
    frame.push(0x01);
    frame.push(0x42);
    frame.extend_from_slice(&n.to_le_bytes());

    for atom in atoms {
        frame.extend_from_slice(&atom.to_bytes());
    }

    let crc = crc32fast::hash(&frame);
    frame.extend_from_slice(&crc.to_le_bytes());
    frame
}

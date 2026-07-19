use crate::model::DhadAtom;
use sha2::{Digest, Sha256};

/// Compute the CoreHash for an atom slice.
///
/// CoreHash captures **orthographic identity**: base character, diacritics,
/// and structural flags. It is independent of prosodic annotation.
///
/// # Domain Prefix
///
/// `"DHAD-CORE-V1"` (12 ASCII bytes) — fixed, length-prefixed domain
/// separator that guarantees CoreHash cannot collide with PhoneticHash.
///
/// # Input to SHA-256
///
/// ```text
/// "DHAD-CORE-V1"      (12 bytes)
/// LE_u32(n_atoms)     (4 bytes)
/// for each atom:
///   LE_u16(base)      (2 bytes)
///   LE_u16(marks)     (2 bytes)
///   flags             (1 byte)
/// ```
/// Total: 16 + 5n bytes
///
/// # Anchor
///
/// Empty slice → `8dd837c60eff174e0f40e75636deedec4bf020751f97cfe10dd1cf7117b16be0`
pub fn core_hash(atoms: &[DhadAtom]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"DHAD-CORE-V1");
    h.update((atoms.len() as u32).to_le_bytes());
    for atom in atoms {
        h.update(atom.base.to_le_bytes());
        h.update(atom.marks.to_le_bytes());
        h.update([atom.flags]);
    }
    h.finalize().into()
}

/// Compute the PhoneticHash for an atom slice.
///
/// PhoneticHash captures **prosodic identity**: it commits to both the
/// CoreHash and the prosody layer. This design (A4) ensures:
///
/// - `CoreHash(a) == CoreHash(b)` does NOT imply `PhoneticHash(a) == PhoneticHash(b)`
/// - `PhoneticHash(a) == PhoneticHash(b)` IMPLIES `CoreHash(a) == CoreHash(b)`
///
/// # Domain Prefix
///
/// `"DHAD-PROSODY-V1"` (15 ASCII bytes)
///
/// # Input to SHA-256
///
/// ```text
/// "DHAD-PROSODY-V1"   (15 bytes)
/// core_hash           (32 bytes — raw digest, NOT hex)
/// LE_u32(n_atoms)     (4 bytes)
/// for each atom:
///   prosody           (1 byte)
/// ```
/// Total: 51 + n bytes
///
/// # Anchor
///
/// Empty slice → `c5f62e920f5b06c74a02f25341f63499f1132da19084eb38e7b806c4a60a03f7`
pub fn phonetic_hash(atoms: &[DhadAtom], ch: &[u8; 32]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"DHAD-PROSODY-V1");
    h.update(ch);
    h.update((atoms.len() as u32).to_le_bytes());
    for atom in atoms {
        h.update([atom.prosody]);
    }
    h.finalize().into()
}

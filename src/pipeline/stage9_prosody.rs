use crate::model::prosody;
use crate::model::{DhadAtom, ErrorKind};
use crate::registry::base;

pub fn apply_prosody(
    atom: &mut DhadAtom,
    prosody_bit: u8,
    atom_index: usize,
) -> Result<(), ErrorKind> {
    if !base::is_prosody_active(atom.base) {
        if prosody_bit == prosody::SUPERSCRIPT_ALEF {
            return Err(ErrorKind::InvalidProsody {
                prosody: prosody_bit,
                atom_index,
                reason: "U+0670 SUPERSCRIPT_ALEF cannot attach to a structural or digit atom",
            });
        }
        return Err(ErrorKind::InvalidProsody {
            prosody: prosody_bit,
            atom_index,
            reason: "Prosody cannot attach to PROSODY_INERT_CLASS",
        });
    }

    atom.prosody |= prosody_bit;
    Ok(())
}

pub fn resolve(atoms: Vec<DhadAtom>) -> Result<Vec<DhadAtom>, ErrorKind> {
    Ok(atoms)
}

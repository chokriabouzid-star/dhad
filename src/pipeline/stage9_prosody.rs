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

    if atom.prosody & prosody_bit != 0 {
        return Err(ErrorKind::InvalidProsody {
            prosody: atom.prosody | prosody_bit,
            atom_index,
            reason: "duplicate prosody mark on same atom",
        });
    }

    atom.prosody |= prosody_bit;
    Ok(())
}

pub fn resolve(atoms: Vec<DhadAtom>) -> Result<Vec<DhadAtom>, ErrorKind> {
    Ok(atoms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inert_atom_rejects_superscript_alef_with_specific_reason() {
        let mut atom = DhadAtom {
            base: base::SPACE,
            marks: 0,
            flags: 0,
            prosody: 0,
            reserved: 0,
        };

        let err = apply_prosody(&mut atom, prosody::SUPERSCRIPT_ALEF, 7).unwrap_err();

        match err {
            ErrorKind::InvalidProsody {
                prosody,
                atom_index,
                reason,
            } => {
                assert_eq!(prosody, crate::model::prosody::SUPERSCRIPT_ALEF);
                assert_eq!(atom_index, 7);
                assert_eq!(
                    reason,
                    "U+0670 SUPERSCRIPT_ALEF cannot attach to a structural or digit atom"
                );
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn inert_atom_rejects_generic_prosody() {
        let mut atom = DhadAtom {
            base: base::SPACE,
            marks: 0,
            flags: 0,
            prosody: 0,
            reserved: 0,
        };

        let err = apply_prosody(&mut atom, prosody::TANWEEN_FATH, 3).unwrap_err();

        match err {
            ErrorKind::InvalidProsody {
                prosody,
                atom_index,
                reason,
            } => {
                assert_eq!(prosody, crate::model::prosody::TANWEEN_FATH);
                assert_eq!(atom_index, 3);
                assert_eq!(reason, "Prosody cannot attach to PROSODY_INERT_CLASS");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn active_atom_accepts_prosody_and_resolve_is_passthrough() {
        let mut atom = DhadAtom {
            base: base::WAW,
            marks: 0,
            flags: 0,
            prosody: 0,
            reserved: 0,
        };

        apply_prosody(&mut atom, prosody::SUPERSCRIPT_ALEF, 1).unwrap();
        assert_eq!(atom.prosody, crate::model::prosody::SUPERSCRIPT_ALEF);

        let out = resolve(vec![atom]).unwrap();
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].prosody, crate::model::prosody::SUPERSCRIPT_ALEF);
    }
}

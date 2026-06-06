use crate::model::{marks, DhadAtom, ErrorKind};
use crate::pipeline::stage5_classify::Token;
use crate::pipeline::stage9_prosody;
use crate::registry::base;

pub fn build(tokens: Vec<Token>) -> Result<Vec<DhadAtom>, ErrorKind> {
    let mut atoms = Vec::new();
    for token in tokens {
        match token {
            Token::Base { base_id, flags, .. } => {
                atoms.push(DhadAtom {
                    base: base_id,
                    marks: 0,
                    flags,
                    prosody: 0,
                    reserved: 0,
                });
            }
            Token::Diacritic {
                cp,
                mark_bit,
                position,
            } => {
                if atoms.is_empty() {
                    return Err(ErrorKind::OrphanDiacritic {
                        codepoint: cp,
                        position,
                    });
                }
                let last_idx = atoms.len() - 1;
                let atom = &mut atoms[last_idx];

                if !base::is_prosody_active(atom.base) {
                    return Err(ErrorKind::InvalidMarkCombo {
                        marks: mark_bit,
                        atom_index: last_idx,
                    });
                }

                let new_marks = atom.marks | mark_bit;
                if atom.marks & mark_bit != 0 || !marks::VALID.contains(&new_marks) {
                    return Err(ErrorKind::InvalidMarkCombo {
                        marks: new_marks,
                        atom_index: last_idx,
                    });
                }
                atom.marks = new_marks;
            }
            Token::Prosodic {
                cp,
                prosody_bit,
                position,
            } => {
                if atoms.is_empty() {
                    return Err(ErrorKind::OrphanDiacritic {
                        codepoint: cp,
                        position,
                    });
                }
                let last_idx = atoms.len() - 1;
                stage9_prosody::apply_prosody(&mut atoms[last_idx], prosody_bit, last_idx)?;
            }
        }
    }
    Ok(atoms)
}

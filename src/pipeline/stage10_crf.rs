use crate::invariants;
use crate::model::{DhadAtom, ErrorKind};

pub fn validate(atoms: &[DhadAtom]) -> Result<(), ErrorKind> {
    for (idx, atom) in atoms.iter().enumerate() {
        invariants::validate_atom(atom, idx)?;
    }
    Ok(())
}

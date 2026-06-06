use crate::model::DhadAtom;

#[allow(dead_code)]
pub fn serialize(atoms: &[DhadAtom]) -> Vec<u8> {
    atoms.iter().flat_map(|a| a.to_bytes()).collect()
}

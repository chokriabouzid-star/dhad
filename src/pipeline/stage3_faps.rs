use crate::faps::{faps_decompose, FapsResult};
use crate::model::ErrorKind;

pub fn decompose(cps: Vec<u32>) -> Result<Vec<u32>, ErrorKind> {
    let mut out = Vec::with_capacity(cps.len());
    for (i, &cp) in cps.iter().enumerate() {
        match faps_decompose(cp) {
            FapsResult::PassThrough => out.push(cp),
            FapsResult::One(a) => out.push(a),
            FapsResult::Two(a, b) => {
                out.push(a);
                out.push(b);
            }
            FapsResult::Unmapped => {
                return Err(ErrorKind::UnmappedCodepoint {
                    codepoint: cp,
                    position: i,
                });
            }
        }
    }
    Ok(out)
}

use crate::faps::{faps_decompose, FapsResult};
use crate::model::ErrorKind;
use crate::noise::is_noise;

pub fn decompose(cps: Vec<u32>) -> Result<Vec<(u32, usize)>, ErrorKind> {
    let mut out = Vec::with_capacity(cps.len());
    let mut filtered_idx = 0;

    for &cp in cps.iter() {
        let is_input_noise = is_noise(cp);
        let current_pos = filtered_idx;

        if !is_input_noise {
            filtered_idx += 1;
        }

        match faps_decompose(cp) {
            FapsResult::PassThrough => out.push((cp, current_pos)),
            FapsResult::One(a) => out.push((a, current_pos)),
            FapsResult::Two(a, b) => {
                out.push((a, current_pos));
                out.push((b, current_pos));
            }
            FapsResult::Unmapped => {
                return Err(ErrorKind::UnmappedCodepoint {
                    codepoint: cp,
                    position: current_pos,
                });
            }
        }
    }
    Ok(out)
}

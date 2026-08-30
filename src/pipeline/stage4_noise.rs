use crate::noise::is_noise;

pub fn filter(cps: Vec<(u32, usize)>) -> Vec<(u32, usize)> {
    cps.into_iter().filter(|&(cp, _)| !is_noise(cp)).collect()
}

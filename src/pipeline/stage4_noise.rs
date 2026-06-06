use crate::noise::is_noise;

pub fn filter(cps: Vec<u32>) -> Vec<u32> {
    cps.into_iter().filter(|&cp| !is_noise(cp)).collect()
}

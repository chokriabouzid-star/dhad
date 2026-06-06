pub fn remove_bom(mut cps: Vec<u32>) -> Vec<u32> {
    if !cps.is_empty() && cps[0] == 0xFEFF {
        cps.remove(0);
    }
    cps
}

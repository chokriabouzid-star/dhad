/// Returns true if the codepoint is a noise character
/// that must be silently removed (Stage 4).
#[inline]
pub fn is_noise(cp: u32) -> bool {
    matches!(cp,
        0x0640 |            // ARABIC TATWEEL / KASHIDA
        0x200C |            // ZERO WIDTH NON-JOINER
        0x200D |            // ZERO WIDTH JOINER
        0x200E |            // LEFT-TO-RIGHT MARK
        0x200F |            // RIGHT-TO-LEFT MARK
        0x202A..=0x202E |   // BiDi controls
        0x2066..=0x2069 |   // BiDi isolate controls
        0xFEFF |            // BYTE ORDER MARK (non-initial occurrences)
        0xFE00..=0xFE0F |   // VARIATION SELECTORS
        0x034F               // COMBINING GRAPHEME JOINER
    )
}

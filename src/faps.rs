/// FAPS decomposition result.
pub enum FapsResult {
    /// Single output codepoint
    One(u32),
    /// Two output codepoints (Lam-Alef only)
    Two(u32, u32),
    /// Not in presentation forms range — pass through
    PassThrough,
    /// In range but not mapped → ERR_UNMAPPED_CODEPOINT
    Unmapped,
}

/// Stage 3: Map Arabic Presentation Forms to canonical codepoints.
pub fn faps_decompose(cp: u32) -> FapsResult {
    match cp {
        // ── Harakat (FE70–FE7F) ──────────────────────────────────
        0xFE70 => FapsResult::One(0x064B),
        0xFE71 => FapsResult::Two(0x0640, 0x064B),
        0xFE72 => FapsResult::One(0x064C),
        0xFE73 => FapsResult::Unmapped,
        0xFE74 => FapsResult::One(0x064D),
        0xFE75 => FapsResult::Unmapped,
        0xFE76 => FapsResult::One(0x064E),
        0xFE77 => FapsResult::Two(0x0640, 0x064E),
        0xFE78 => FapsResult::One(0x064F),
        0xFE79 => FapsResult::Two(0x0640, 0x064F),
        0xFE7A => FapsResult::One(0x0650),
        0xFE7B => FapsResult::Two(0x0640, 0x0650),
        0xFE7C => FapsResult::One(0x0651),
        0xFE7D => FapsResult::Two(0x0640, 0x0651),
        0xFE7E => FapsResult::One(0x0652),
        0xFE7F => FapsResult::Two(0x0640, 0x0652),

        // ── Hamza/Alef variants (FE80–FE8C) ─────────────────────
        0xFE80 => FapsResult::One(0x0621),
        0xFE81 | 0xFE82 => FapsResult::One(0x0622),
        0xFE83 | 0xFE84 => FapsResult::One(0x0623),
        0xFE85 | 0xFE86 => FapsResult::One(0x0624),
        0xFE87 | 0xFE88 => FapsResult::One(0x0625),
        0xFE89..=0xFE8C => FapsResult::One(0x0626),

        // ── Core 28 letters (FE8D–FEF4) — positional forms ──────
        0xFE8D | 0xFE8E => FapsResult::One(0x0627),
        0xFE8F..=0xFE92 => FapsResult::One(0x0628),
        0xFE93 | 0xFE94 => FapsResult::One(0x0629),
        0xFE95..=0xFE98 => FapsResult::One(0x062A),
        0xFE99..=0xFE9C => FapsResult::One(0x062B),
        0xFE9D..=0xFEA0 => FapsResult::One(0x062C),
        0xFEA1..=0xFEA4 => FapsResult::One(0x062D),
        0xFEA5..=0xFEA8 => FapsResult::One(0x062E),
        0xFEA9 | 0xFEAA => FapsResult::One(0x062F),
        0xFEAB | 0xFEAC => FapsResult::One(0x0630),
        0xFEAD | 0xFEAE => FapsResult::One(0x0631),
        0xFEAF | 0xFEB0 => FapsResult::One(0x0632),
        0xFEB1..=0xFEB4 => FapsResult::One(0x0633),
        0xFEB5..=0xFEB8 => FapsResult::One(0x0634),
        0xFEB9..=0xFEBC => FapsResult::One(0x0635),
        0xFEBD..=0xFEC0 => FapsResult::One(0x0636),
        0xFEC1..=0xFEC4 => FapsResult::One(0x0637),
        0xFEC5..=0xFEC8 => FapsResult::One(0x0638),
        0xFEC9..=0xFECC => FapsResult::One(0x0639),
        0xFECD..=0xFED0 => FapsResult::One(0x063A),
        0xFED1..=0xFED4 => FapsResult::One(0x0641),
        0xFED5..=0xFED8 => FapsResult::One(0x0642),
        0xFED9..=0xFEDC => FapsResult::One(0x0643),
        0xFEDD..=0xFEE0 => FapsResult::One(0x0644),
        0xFEE1..=0xFEE4 => FapsResult::One(0x0645),
        0xFEE5..=0xFEE8 => FapsResult::One(0x0646),
        0xFEE9..=0xFEEC => FapsResult::One(0x0647),
        0xFEED | 0xFEEE => FapsResult::One(0x0648),
        0xFEEF | 0xFEF0 => FapsResult::One(0x0649),
        0xFEF1..=0xFEF4 => FapsResult::One(0x064A),

        // ── Lam-Alef ligatures (FEF5–FEFC) — TWO outputs ────────
        0xFEF5 | 0xFEF6 => FapsResult::Two(0x0644, 0x0622),
        0xFEF7 | 0xFEF8 => FapsResult::Two(0x0644, 0x0623),
        0xFEF9 | 0xFEFA => FapsResult::Two(0x0644, 0x0625),
        0xFEFB | 0xFEFC => FapsResult::Two(0x0644, 0x0627),

        // FE range remainders
        0xFEFD | 0xFEFE => FapsResult::Unmapped,
        // 0xFEFF handled upstream (BOM/noise) — never reaches here

        // ── Presentation Forms-A (FB50–FDFF) ────────────────────
        0xFB50 | 0xFB51 => FapsResult::One(0x0671),
        0xFB52..=0xFDFF => FapsResult::Unmapped,

        // Not in any Presentation Forms range
        _ => FapsResult::PassThrough,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn covers_harakat_forms_fe70_to_fe7f() {
        assert!(matches!(faps_decompose(0xFE70), FapsResult::One(0x064B)));
        assert!(matches!(faps_decompose(0xFE71), FapsResult::Two(0x0640, 0x064B)));
        assert!(matches!(faps_decompose(0xFE72), FapsResult::One(0x064C)));
        assert!(matches!(faps_decompose(0xFE73), FapsResult::Unmapped));
        assert!(matches!(faps_decompose(0xFE74), FapsResult::One(0x064D)));
        assert!(matches!(faps_decompose(0xFE75), FapsResult::Unmapped));
        assert!(matches!(faps_decompose(0xFE76), FapsResult::One(0x064E)));
        assert!(matches!(faps_decompose(0xFE77), FapsResult::Two(0x0640, 0x064E)));
        assert!(matches!(faps_decompose(0xFE78), FapsResult::One(0x064F)));
        assert!(matches!(faps_decompose(0xFE79), FapsResult::Two(0x0640, 0x064F)));
        assert!(matches!(faps_decompose(0xFE7A), FapsResult::One(0x0650)));
        assert!(matches!(faps_decompose(0xFE7B), FapsResult::Two(0x0640, 0x0650)));
        assert!(matches!(faps_decompose(0xFE7C), FapsResult::One(0x0651)));
        assert!(matches!(faps_decompose(0xFE7D), FapsResult::Two(0x0640, 0x0651)));
        assert!(matches!(faps_decompose(0xFE7E), FapsResult::One(0x0652)));
        assert!(matches!(faps_decompose(0xFE7F), FapsResult::Two(0x0640, 0x0652)));
    }

    #[test]
    fn covers_forms_a_and_edge_unmapped_ranges() {
        assert!(matches!(faps_decompose(0xFEFD), FapsResult::Unmapped));
        assert!(matches!(faps_decompose(0xFEFE), FapsResult::Unmapped));

        assert!(matches!(faps_decompose(0xFB50), FapsResult::One(0x0671)));
        assert!(matches!(faps_decompose(0xFB51), FapsResult::One(0x0671)));
        assert!(matches!(faps_decompose(0xFB52), FapsResult::Unmapped));
        assert!(matches!(faps_decompose(0xFDFF), FapsResult::Unmapped));
    }

    #[test]
    fn covers_passthrough_branch() {
        assert!(matches!(faps_decompose(0x0628), FapsResult::PassThrough));
    }
}

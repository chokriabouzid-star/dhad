use crate::registry::base;

/// Maps a Unicode codepoint to a base ID.
/// Returns None if unmapped (→ ERR_UNMAPPED_CODEPOINT at Stage 5).
/// This is a pure function with no side effects.
pub fn unicode_to_base(cp: u32) -> Option<(u16, u8)> {
    // Returns (base_id, flags)
    // flags: 0x00=none, 0x01=HAMZA_ABOVE, 0x02=HAMZA_BELOW, 0x04=MADDA
    match cp {
        0x0621 => Some((base::HAMZA, 0x00)),
        0x0622 => Some((base::ALEF, 0x04)), // آ → ALEF + MADDA
        0x0623 => Some((base::ALEF, 0x01)), // أ → ALEF + HAMZA_ABOVE
        0x0624 => Some((base::WAW, 0x01)),  // ؤ → WAW  + HAMZA_ABOVE
        0x0625 => Some((base::ALEF, 0x02)), // إ → ALEF + HAMZA_BELOW
        0x0626 => Some((base::YEH, 0x01)),  // ئ → YEH  + HAMZA_ABOVE
        0x0627 => Some((base::ALEF, 0x00)),
        0x0628 => Some((base::BEH, 0x00)),
        0x0629 => Some((base::TEH_MARBUTA, 0x00)),
        0x062A => Some((base::TEH, 0x00)),
        0x062B => Some((base::THEH, 0x00)),
        0x062C => Some((base::JEEM, 0x00)),
        0x062D => Some((base::HAH, 0x00)),
        0x062E => Some((base::KHAH, 0x00)),
        0x062F => Some((base::DAL, 0x00)),
        0x0630 => Some((base::THAL, 0x00)),
        0x0631 => Some((base::REH, 0x00)),
        0x0632 => Some((base::ZAIN, 0x00)),
        0x0633 => Some((base::SEEN, 0x00)),
        0x0634 => Some((base::SHEEN, 0x00)),
        0x0635 => Some((base::SAD, 0x00)),
        0x0636 => Some((base::DAD, 0x00)),
        0x0637 => Some((base::TAH, 0x00)),
        0x0638 => Some((base::ZAH, 0x00)),
        0x0639 => Some((base::AIN, 0x00)),
        0x063A => Some((base::GHAIN, 0x00)),
        0x0641 => Some((base::FEH, 0x00)),
        0x0642 => Some((base::QAF, 0x00)),
        0x0643 => Some((base::KAF, 0x00)),
        0x0644 => Some((base::LAM, 0x00)),
        0x0645 => Some((base::MEEM, 0x00)),
        0x0646 => Some((base::NOON, 0x00)),
        0x0647 => Some((base::HEH, 0x00)),
        0x0648 => Some((base::WAW, 0x00)),
        0x0649 => Some((base::ALEF_MAQSURA, 0x00)),
        0x064A => Some((base::YEH, 0x00)),
        0x0671 => Some((base::ALEF_WASLA, 0x00)),
        // Structural
        0x0020 => Some((base::SPACE, 0x00)),
        0x002E => Some((base::FULL_STOP, 0x00)),
        0x003A => Some((base::COLON, 0x00)),
        0x060C => Some((base::AR_COMMA, 0x00)),
        0x061B => Some((base::AR_SEMICOLON, 0x00)),
        0x061F => Some((base::AR_QUESTION, 0x00)),
        // Digits (all three forms)
        0x0030..=0x0039 => Some((base::DIGIT_0 + (cp - 0x0030) as u16, 0x00)),
        0x0660..=0x0669 => Some((base::DIGIT_0 + (cp - 0x0660) as u16, 0x00)),
        0x06F0..=0x06F9 => Some((base::DIGIT_0 + (cp - 0x06F0) as u16, 0x00)),
        _ => None,
    }
}

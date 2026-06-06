pub mod base {
    // Core 28 letters
    pub const ALEF: u16 = 0x0001;
    pub const BEH: u16 = 0x0002;
    pub const TEH: u16 = 0x0003;
    pub const THEH: u16 = 0x0004;
    pub const JEEM: u16 = 0x0005;
    pub const HAH: u16 = 0x0006;
    pub const KHAH: u16 = 0x0007;
    pub const DAL: u16 = 0x0008;
    pub const THAL: u16 = 0x0009;
    pub const REH: u16 = 0x000A;
    pub const ZAIN: u16 = 0x000B;
    pub const SEEN: u16 = 0x000C;
    pub const SHEEN: u16 = 0x000D;
    pub const SAD: u16 = 0x000E;
    pub const DAD: u16 = 0x000F;
    pub const TAH: u16 = 0x0010;
    pub const ZAH: u16 = 0x0011;
    pub const AIN: u16 = 0x0012;
    pub const GHAIN: u16 = 0x0013;
    pub const FEH: u16 = 0x0014;
    pub const QAF: u16 = 0x0015;
    pub const KAF: u16 = 0x0016;
    pub const LAM: u16 = 0x0017;
    pub const MEEM: u16 = 0x0018;
    pub const NOON: u16 = 0x0019;
    pub const HEH: u16 = 0x001A;
    pub const WAW: u16 = 0x001B;
    pub const YEH: u16 = 0x001C;

    // Orthographic units
    pub const HAMZA: u16 = 0x0020;
    pub const TEH_MARBUTA: u16 = 0x0021;
    pub const ALEF_MAQSURA: u16 = 0x0022;
    pub const ALEF_WASLA: u16 = 0x0023;

    // Structural
    pub const SPACE: u16 = 0x0040;
    pub const AR_COMMA: u16 = 0x0041;
    pub const AR_SEMICOLON: u16 = 0x0042;
    pub const AR_QUESTION: u16 = 0x0043;
    pub const FULL_STOP: u16 = 0x0044;
    pub const COLON: u16 = 0x0045;

    // Digits
    pub const DIGIT_0: u16 = 0x0100;
    // DIGIT_N = DIGIT_0 + N  for N in 0..=9

    /// Base IDs that are PROSODY_ACTIVE_CLASS (may carry marks and prosody)
    pub fn is_prosody_active(base: u16) -> bool {
        matches!(base, 0x0001..=0x001C | 0x0020..=0x0023)
    }

    /// LONG_VOWEL_CLASS — may carry MADD bits
    pub fn is_long_vowel(base: u16) -> bool {
        matches!(base, ALEF | WAW | YEH | ALEF_MAQSURA)
    }

    /// Valid HAMZA_ABOVE seats
    pub fn is_hamza_above_seat(base: u16) -> bool {
        matches!(base, ALEF | WAW | YEH)
    }

    /// Valid HAMZA_BELOW seats
    pub fn is_hamza_below_seat(base: u16) -> bool {
        base == ALEF
    }

    /// Valid MADDA seats
    pub fn is_madda_seat(base: u16) -> bool {
        base == ALEF
    }
}

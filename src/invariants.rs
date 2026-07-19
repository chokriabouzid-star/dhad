use crate::model::{flags, marks, prosody, DhadAtom, ErrorKind};
use crate::registry::base;

/// Check all 24 invariants on a single atom.
/// Returns Ok(()) if all pass, Err(ErrorKind) on first violation.
/// atom_index is the 0-based position in the stream (for error messages).
pub fn validate_atom(atom: &DhadAtom, atom_index: usize) -> Result<(), ErrorKind> {
    i01_valid_base(atom, atom_index)?;
    i02_reserved_base(atom, atom_index)?;
    i03_valid_marks(atom, atom_index)?;
    i04_valid_flags(atom, atom_index)?;
    i05_hamza_above_seat(atom, atom_index)?;
    i06_hamza_below_seat(atom, atom_index)?;
    i07_madda_seat(atom, atom_index)?;
    i08_no_dual_hamza(atom, atom_index)?;
    i09_no_madda_with_hamza(atom, atom_index)?;
    i10_tanween_mutual_exclusion_fd(atom, atom_index)?;
    i11_tanween_fath_kasr(atom, atom_index)?;
    i12_tanween_damm_kasr(atom, atom_index)?;
    i13_madd_not_with_tanween(atom, atom_index)?;
    i14_madd_mutual_exclusion(atom, atom_index)?;
    i15_madd_long_vowel_only(atom, atom_index)?;
    i16_inert_prosody_zero(atom, atom_index)?;
    i17_inert_marks_zero(atom, atom_index)?;
    i18_tanween_fath_not_fatha(atom, atom_index)?;
    i19_tanween_damm_not_damma(atom, atom_index)?;
    i20_tanween_kasr_not_kasra(atom, atom_index)?;
    i21_superscript_not_tanween(atom, atom_index)?;
    i22_reserved_zero(atom, atom_index)?;
    i23_marks_reserved_bits(atom, atom_index)?;
    i24_no_sukun_with_tanween(atom, atom_index)?;
    Ok(())
}

fn i01_valid_base(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    let valid = matches!(a.base,
        0x0001..=0x001C | 0x0020..=0x0023 | 0x0040..=0x0045 | 0x0100..=0x0109
    );
    if !valid {
        Err(ErrorKind::UnmappedCodepoint {
            codepoint: a.base as u32,
            position: idx,
        })
    } else {
        Ok(())
    }
}

fn i02_reserved_base(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if matches!(a.base, 0x001D..=0x001F) {
        Err(ErrorKind::UnmappedCodepoint {
            codepoint: a.base as u32,
            position: idx,
        })
    } else {
        Ok(())
    }
}

fn i03_valid_marks(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if !marks::VALID.contains(&a.marks) {
        Err(ErrorKind::InvalidMarkCombo {
            marks: a.marks,
            atom_index: idx,
        })
    } else {
        Ok(())
    }
}

fn i04_valid_flags(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if !flags::VALID.contains(&a.flags) {
        Err(ErrorKind::InvalidFlagCombo {
            flags: a.flags,
            atom_index: idx,
        })
    } else {
        Ok(())
    }
}

fn i05_hamza_above_seat(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.flags & flags::HAMZA_ABOVE != 0 && !base::is_hamza_above_seat(a.base) {
        Err(ErrorKind::InvalidFlagCombo {
            flags: a.flags,
            atom_index: idx,
        })
    } else {
        Ok(())
    }
}

fn i06_hamza_below_seat(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.flags & flags::HAMZA_BELOW != 0 && !base::is_hamza_below_seat(a.base) {
        Err(ErrorKind::InvalidFlagCombo {
            flags: a.flags,
            atom_index: idx,
        })
    } else {
        Ok(())
    }
}

fn i07_madda_seat(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.flags & flags::MADDA != 0 && !base::is_madda_seat(a.base) {
        Err(ErrorKind::InvalidFlagCombo {
            flags: a.flags,
            atom_index: idx,
        })
    } else {
        Ok(())
    }
}

fn i08_no_dual_hamza(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.flags & flags::HAMZA_ABOVE != 0 && a.flags & flags::HAMZA_BELOW != 0 {
        Err(ErrorKind::InvalidFlagCombo {
            flags: a.flags,
            atom_index: idx,
        })
    } else {
        Ok(())
    }
}

fn i09_no_madda_with_hamza(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.flags & flags::MADDA != 0 && a.flags & (flags::HAMZA_ABOVE | flags::HAMZA_BELOW) != 0 {
        Err(ErrorKind::InvalidFlagCombo {
            flags: a.flags,
            atom_index: idx,
        })
    } else {
        Ok(())
    }
}

fn i10_tanween_mutual_exclusion_fd(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.prosody & prosody::TANWEEN_FATH != 0 && a.prosody & prosody::TANWEEN_DAMM != 0 {
        Err(ErrorKind::InvalidProsody {
            prosody: a.prosody,
            atom_index: idx,
            reason: "TANWEEN_FATH and TANWEEN_DAMM are mutually exclusive",
        })
    } else {
        Ok(())
    }
}

fn i11_tanween_fath_kasr(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.prosody & prosody::TANWEEN_FATH != 0 && a.prosody & prosody::TANWEEN_KASR != 0 {
        Err(ErrorKind::InvalidProsody {
            prosody: a.prosody,
            atom_index: idx,
            reason: "TANWEEN_FATH and TANWEEN_KASR are mutually exclusive",
        })
    } else {
        Ok(())
    }
}

fn i12_tanween_damm_kasr(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.prosody & prosody::TANWEEN_DAMM != 0 && a.prosody & prosody::TANWEEN_KASR != 0 {
        Err(ErrorKind::InvalidProsody {
            prosody: a.prosody,
            atom_index: idx,
            reason: "TANWEEN_DAMM and TANWEEN_KASR are mutually exclusive",
        })
    } else {
        Ok(())
    }
}

fn i13_madd_not_with_tanween(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.prosody & (prosody::MADD_NORMAL | prosody::MADD_EXTENDED) != 0 && a.prosody & 0x07 != 0 {
        Err(ErrorKind::InvalidProsody {
            prosody: a.prosody,
            atom_index: idx,
            reason: "MADD bits are mutually exclusive with TANWEEN bits",
        })
    } else {
        Ok(())
    }
}

fn i14_madd_mutual_exclusion(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.prosody & prosody::MADD_NORMAL != 0 && a.prosody & prosody::MADD_EXTENDED != 0 {
        Err(ErrorKind::InvalidProsody {
            prosody: a.prosody,
            atom_index: idx,
            reason: "MADD_NORMAL and MADD_EXTENDED are mutually exclusive",
        })
    } else {
        Ok(())
    }
}

fn i15_madd_long_vowel_only(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.prosody & (prosody::MADD_NORMAL | prosody::MADD_EXTENDED) != 0
        && !base::is_long_vowel(a.base)
    {
        Err(ErrorKind::InvalidProsody {
            prosody: a.prosody,
            atom_index: idx,
            reason: "MADD bits only permitted on LONG_VOWEL_CLASS atoms",
        })
    } else {
        Ok(())
    }
}

fn i16_inert_prosody_zero(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if !base::is_prosody_active(a.base) && a.prosody != 0x00 {
        Err(ErrorKind::InvalidProsody {
            prosody: a.prosody,
            atom_index: idx,
            reason: "PROSODY_INERT_CLASS atom must have prosody == 0x00",
        })
    } else {
        Ok(())
    }
}

fn i17_inert_marks_zero(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if !base::is_prosody_active(a.base) && a.marks != 0x0000 {
        Err(ErrorKind::InvalidMarkCombo {
            marks: a.marks,
            atom_index: idx,
        })
    } else {
        Ok(())
    }
}

fn i18_tanween_fath_not_fatha(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.prosody & prosody::TANWEEN_FATH != 0 && a.marks & marks::FATHA != 0 {
        Err(ErrorKind::InvalidProsody {
            prosody: a.prosody,
            atom_index: idx,
            reason: "TANWEEN_FATH and FATHA are contradictory",
        })
    } else {
        Ok(())
    }
}

fn i19_tanween_damm_not_damma(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.prosody & prosody::TANWEEN_DAMM != 0 && a.marks & marks::DAMMA != 0 {
        Err(ErrorKind::InvalidProsody {
            prosody: a.prosody,
            atom_index: idx,
            reason: "TANWEEN_DAMM and DAMMA are contradictory",
        })
    } else {
        Ok(())
    }
}

fn i20_tanween_kasr_not_kasra(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.prosody & prosody::TANWEEN_KASR != 0 && a.marks & marks::KASRA != 0 {
        Err(ErrorKind::InvalidProsody {
            prosody: a.prosody,
            atom_index: idx,
            reason: "TANWEEN_KASR and KASRA are contradictory",
        })
    } else {
        Ok(())
    }
}

fn i21_superscript_not_tanween(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.prosody & prosody::SUPERSCRIPT_ALEF != 0 && a.prosody & 0x07 != 0 {
        Err(ErrorKind::InvalidProsody {
            prosody: a.prosody,
            atom_index: idx,
            reason: "SUPERSCRIPT_ALEF and TANWEEN bits are contradictory",
        })
    } else {
        Ok(())
    }
}

fn i22_reserved_zero(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.reserved != 0x0000 {
        Err(ErrorKind::ReservedFieldNonZero {
            reserved: a.reserved,
            atom_index: idx,
        })
    } else {
        Ok(())
    }
}

fn i23_marks_reserved_bits(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    if a.marks & 0xFFE0 != 0 {
        Err(ErrorKind::InvalidMarkCombo {
            marks: a.marks,
            atom_index: idx,
        })
    } else {
        Ok(())
    }
}

fn i24_no_sukun_with_tanween(a: &DhadAtom, idx: usize) -> Result<(), ErrorKind> {
    let has_tanween =
        a.prosody & (prosody::TANWEEN_FATH | prosody::TANWEEN_DAMM | prosody::TANWEEN_KASR) != 0;
    if a.marks & marks::SUKUN != 0 && has_tanween {
        Err(ErrorKind::InvalidProsody {
            prosody: a.prosody,
            atom_index: idx,
            reason: "SUKUN and TANWEEN are mutually exclusive on the same atom",
        })
    } else {
        Ok(())
    }
}

//! Suite 5: Coverage gap tests for faps.rs and stage9_prosody.rs
//!
//! These tests target specific code paths that are not exercised
//! by suites 1–4, to push coverage above 95%.

use dhad::model::flags;
use dhad::model::prosody;
use dhad::model::ErrorKind;
use dhad::modes::process_mode_a;
use dhad::registry::base;

// ═══════════════════════════════════════════════════════════════
// FAPS: Harakat presentation forms (FE70–FE7F)
// ═══════════════════════════════════════════════════════════════

fn mode_a(input: &str) -> dhad::model::DhadResult {
    let bytes = input.as_bytes();
    process_mode_a(bytes).expect("should succeed")
}

/// FE70 = isolated fathatan → canonical 064B
#[test]
fn faps_fe70_fathatan_isolated() {
    let r = process_mode_a("\u{FE70}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::OrphanDiacritic { .. })),
        "U+FE70 must be an orphan diacritic, got {:?}",
        r
    );
}

/// FE71 = fathatan on tatweel → Two(0640, 064B)
/// The tatweel (0640) is noise and gets filtered,
/// so only the haraka remains or it becomes orphan.
#[test]
fn faps_fe71_fathatan_medial() {
    let r = process_mode_a("\u{FE71}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::OrphanDiacritic { .. })),
        "U+FE71 must be an orphan diacritic, got {:?}",
        r
    );
}

/// FE72 = isolated dammatan
#[test]
fn faps_fe72_dammatan_isolated() {
    let r = process_mode_a("\u{FE72}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::OrphanDiacritic { .. })),
        "U+FE72 must be an orphan diacritic, got {:?}",
        r
    );
}

/// FE73 = unmapped in harakat range
#[test]
fn faps_fe73_unmapped() {
    let r = process_mode_a("\u{FE73}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::UnmappedCodepoint { .. })),
        "U+FE73 must be UnmappedCodepoint, got {:?}",
        r
    );
}

/// FE74 = isolated kasratan
#[test]
fn faps_fe74_kasratan_isolated() {
    let r = process_mode_a("\u{FE74}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::OrphanDiacritic { .. })),
        "U+FE74 must be an orphan diacritic, got {:?}",
        r
    );
}

/// FE75 = unmapped
#[test]
fn faps_fe75_unmapped() {
    let r = process_mode_a("\u{FE75}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::UnmappedCodepoint { .. })),
        "U+FE75 must be UnmappedCodepoint, got {:?}",
        r
    );
}

/// FE76 = isolated fatha
#[test]
fn faps_fe76_fatha_isolated() {
    let r = process_mode_a("\u{FE76}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::OrphanDiacritic { .. })),
        "U+FE76 must be an orphan diacritic, got {:?}",
        r
    );
}

/// FE77 = fatha on tatweel → Two(0640, 064E)
#[test]
fn faps_fe77_fatha_medial() {
    let r = process_mode_a("\u{FE77}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::OrphanDiacritic { .. })),
        "U+FE77 must be an orphan diacritic, got {:?}",
        r
    );
}

/// FE78 = isolated damma
#[test]
fn faps_fe78_damma_isolated() {
    let r = process_mode_a("\u{FE78}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::OrphanDiacritic { .. })),
        "U+FE78 must be an orphan diacritic, got {:?}",
        r
    );
}

/// FE79 = damma on tatweel → Two(0640, 064F)
#[test]
fn faps_fe79_damma_medial() {
    let r = process_mode_a("\u{FE79}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::OrphanDiacritic { .. })),
        "U+FE79 must be an orphan diacritic, got {:?}",
        r
    );
}

/// FE7A = isolated kasra
#[test]
fn faps_fe7a_kasra_isolated() {
    let r = process_mode_a("\u{FE7A}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::OrphanDiacritic { .. })),
        "U+FE7A must be an orphan diacritic, got {:?}",
        r
    );
}

/// FE7B = kasra on tatweel → Two(0640, 0650)
#[test]
fn faps_fe7b_kasra_medial() {
    let r = process_mode_a("\u{FE7B}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::OrphanDiacritic { .. })),
        "U+FE7B must be an orphan diacritic, got {:?}",
        r
    );
}

/// FE7C = isolated shadda
#[test]
fn faps_fe7c_shadda_isolated() {
    let r = process_mode_a("\u{FE7C}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::OrphanDiacritic { .. })),
        "U+FE7C must be an orphan diacritic, got {:?}",
        r
    );
}

/// FE7D = shadda on tatweel → Two(0640, 0651)
#[test]
fn faps_fe7d_shadda_medial() {
    let r = process_mode_a("\u{FE7D}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::OrphanDiacritic { .. })),
        "U+FE7D must be an orphan diacritic, got {:?}",
        r
    );
}

/// FE7E = isolated sukun
#[test]
fn faps_fe7e_sukun_isolated() {
    let r = process_mode_a("\u{FE7E}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::OrphanDiacritic { .. })),
        "U+FE7E must be an orphan diacritic, got {:?}",
        r
    );
}

/// FE7F = sukun on tatweel → Two(0640, 0652)
#[test]
fn faps_fe7f_sukun_medial() {
    let r = process_mode_a("\u{FE7F}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::OrphanDiacritic { .. })),
        "U+FE7F must be an orphan diacritic, got {:?}",
        r
    );
}

// ═══════════════════════════════════════════════════════════════
// FAPS: Unmapped tails and Forms-A
// ═══════════════════════════════════════════════════════════════

/// FEFD = unmapped
#[test]
fn faps_fefd_unmapped() {
    let r = process_mode_a("\u{FEFD}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::UnmappedCodepoint { .. })),
        "U+FEFD must be UnmappedCodepoint, got {:?}",
        r
    );
}

/// FEFE = unmapped
#[test]
fn faps_fefe_unmapped() {
    let r = process_mode_a("\u{FEFE}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::UnmappedCodepoint { .. })),
        "U+FEFE must be UnmappedCodepoint, got {:?}",
        r
    );
}

/// FB50 = Alef Wasla presentation form
#[test]
fn faps_fb50_alef_wasla() {
    let r = process_mode_a("\u{FB50}".as_bytes()).expect("FB50 must decode");
    assert_eq!(r.stream.len(), 1, "FB50 must produce exactly 1 atom");
}

/// FB51 = Alef Wasla final form
#[test]
fn faps_fb51_alef_wasla_final() {
    let r = process_mode_a("\u{FB51}".as_bytes()).expect("FB51 must decode");
    assert_eq!(r.stream.len(), 1, "FB51 must produce exactly 1 atom");
}

/// FB52 = beginning of unmapped Forms-A range
#[test]
fn faps_fb52_unmapped_forms_a() {
    let r = process_mode_a("\u{FB52}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::UnmappedCodepoint { .. })),
        "U+FB52 must be UnmappedCodepoint, got {:?}",
        r
    );
}

/// FDFF = end of unmapped Forms-A range
#[test]
fn faps_fdff_unmapped_forms_a_end() {
    let r = process_mode_a("\u{FDFF}".as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::UnmappedCodepoint { .. })),
        "U+FDFF must be UnmappedCodepoint, got {:?}",
        r
    );
}

// ═══════════════════════════════════════════════════════════════
// FAPS: Harakat forms attached to a base letter
// (ensures the Two() path produces valid atoms when tatweel is
//  filtered and the haraka attaches to a preceding base)
// ═══════════════════════════════════════════════════════════════

/// Beh + FE77 (fatha on tatweel) → beh gets fatha
#[test]
fn faps_haraka_on_tatweel_after_base() {
    let input = "\u{0628}\u{FE77}"; // beh + presentation fatha-medial
    let r = process_mode_a(input.as_bytes())
        .expect("BEH + FE77 must decode (tatweel filtered, fatha attached)");
    assert_eq!(
        r.stream.len(),
        1,
        "BEH + FE77 must collapse to exactly 1 atom"
    );
}

/// Beh + FE7D (shadda on tatweel) → beh gets shadda
#[test]
fn faps_shadda_on_tatweel_after_base() {
    let input = "\u{0628}\u{FE7D}";
    let r = process_mode_a(input.as_bytes())
        .expect("BEH + FE7D must decode (tatweel filtered, shadda attached)");
    assert_eq!(
        r.stream.len(),
        1,
        "BEH + FE7D must collapse to exactly 1 atom"
    );
}

// ═══════════════════════════════════════════════════════════════
// Stage 9 Prosody: SUPERSCRIPT_ALEF on inert atom
// ═══════════════════════════════════════════════════════════════

/// Superscript alef (U+0670) after a space → should error
#[test]
fn prosody_superscript_alef_on_space() {
    let input = "\u{0020}\u{0670}";
    let r = process_mode_a(input.as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::InvalidProsody { .. })),
        "superscript alef on space must be InvalidProsody, got {:?}",
        r
    );
}

/// Superscript alef (U+0670) after a digit → should error
#[test]
fn prosody_superscript_alef_on_digit() {
    let input = "1\u{0670}";
    let r = process_mode_a(input.as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::InvalidProsody { .. })),
        "superscript alef on digit must be InvalidProsody, got {:?}",
        r
    );
}

// ═══════════════════════════════════════════════════════════════
// Stage 9 Prosody: resolve() path
// ═══════════════════════════════════════════════════════════════

/// Ensure resolve() is exercised via normal pipeline
/// (any valid input with prosody triggers it)
#[test]
fn prosody_resolve_via_tanween() {
    let input = "\u{0628}\u{064B}"; // beh + fathatan (tanween)
    let r = mode_a(input);
    assert_eq!(
        r.stream.len(),
        1,
        "BEH + tanween must produce exactly 1 atom"
    );
    let atom = r.stream.atoms()[0];
    assert_eq!(atom.base, base::BEH, "atom base must be BEH");
    assert_eq!(
        atom.prosody & prosody::TANWEEN_FATH,
        prosody::TANWEEN_FATH,
        "TANWEEN_FATH bit must be set"
    );
}

/// Ensure resolve() with superscript alef on valid base
#[test]
fn prosody_resolve_via_superscript_alef() {
    let input = "\u{0648}\u{0670}"; // waw + superscript alef
    let r = mode_a(input);
    assert_eq!(
        r.stream.len(),
        1,
        "WAW + superscript alef must produce exactly 1 atom"
    );
    let atom = r.stream.atoms()[0];
    assert_eq!(atom.base, base::WAW, "atom base must be WAW");
    assert_eq!(
        atom.prosody & prosody::SUPERSCRIPT_ALEF,
        prosody::SUPERSCRIPT_ALEF,
        "SUPERSCRIPT_ALEF bit must be set"
    );
}

// ═══════════════════════════════════════════════════════════════
// Remaining small gaps: mode_b, model, noise, stages 1-3
// ═══════════════════════════════════════════════════════════════

/// noise.rs: ensure a pure-noise string returns empty atoms
#[test]
fn noise_only_tatweel() {
    let input = "\u{0640}\u{0640}\u{0640}"; // all tatweel = noise
    let r = mode_a(input);
    assert!(
        r.stream.is_empty(),
        "pure-tatweel input must yield an empty stream"
    );
}

/// stage1: invalid UTF-8 sequence
#[test]
fn stage1_invalid_utf8() {
    let bad = &[0xFF, 0xFE, 0x00];
    let r = process_mode_a(bad);
    assert!(
        matches!(r, Err(ErrorKind::MalformedUtf8 { .. })),
        "invalid UTF-8 must be MalformedUtf8, got {:?}",
        r
    );
}

/// stage2: BOM in middle of text (should be filtered)
#[test]
fn stage2_bom_mid_text() {
    let input = "\u{0628}\u{FEFF}\u{0628}"; // beh + BOM + beh
    let r = mode_a(input);
    assert_eq!(
        r.stream.len(),
        2,
        "BEH + mid-text BOM + BEH must keep exactly 2 BEH atoms"
    );
    let atoms = r.stream.atoms();
    assert_eq!(atoms[0].base, base::BEH, "first atom must be BEH");
    assert_eq!(atoms[1].base, base::BEH, "second atom must be BEH");
}

/// stage3: FAPS PassThrough (normal codepoint not in FE/FB range)
#[test]
fn stage3_faps_passthrough() {
    let input = "\u{0628}"; // plain beh — not a presentation form
    let r = mode_a(input);
    assert_eq!(
        r.stream.len(),
        1,
        "passthrough codepoint must produce exactly 1 atom"
    );
    assert_eq!(r.stream.atoms()[0].base, base::BEH, "atom base must be BEH");
}

// ═══════════════════════════════════════════════════════════════
// Suite 5b: Additional coverage gap tests
// ═══════════════════════════════════════════════════════════════

// --- stage9_prosody: SUPERSCRIPT_ALEF branch (line 12-13) ---

/// Superscript alef on punctuation (arabic comma)
#[test]
fn prosody_superscript_alef_on_comma() {
    let input = "\u{060C}\u{0670}";
    let r = process_mode_a(input.as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::InvalidProsody { .. })),
        "superscript alef on comma must be InvalidProsody, got {:?}",
        r
    );
}

/// Generic prosody on inert atom (not SUPERSCRIPT_ALEF path)
/// tanween_fath on space → triggers the second Err branch
#[test]
fn prosody_tanween_on_space() {
    let input = "\u{0020}\u{064B}";
    let r = process_mode_a(input.as_bytes());
    assert!(
        matches!(r, Err(ErrorKind::InvalidProsody { .. })),
        "tanween on space must be InvalidProsody, got {:?}",
        r
    );
}

// --- noise.rs: remaining noise codepoints ---

/// ZWNJ (200C) is noise
#[test]
fn noise_zwnj() {
    let input = "\u{0628}\u{200C}\u{0628}";
    let r = mode_a(input);
    assert_eq!(
        r.stream.len(),
        2,
        "BEH + ZWNJ + BEH must keep exactly 2 BEH atoms"
    );
    let atoms = r.stream.atoms();
    assert_eq!(atoms[0].base, base::BEH, "first atom must be BEH");
    assert_eq!(atoms[1].base, base::BEH, "second atom must be BEH");
}

/// BiDi control 202A is noise
#[test]
fn noise_bidi_202a() {
    let input = "\u{0628}\u{202A}\u{0628}";
    let r = mode_a(input);
    assert_eq!(
        r.stream.len(),
        2,
        "BEH + BiDi-202A + BEH must keep exactly 2 BEH atoms"
    );
    let atoms = r.stream.atoms();
    assert_eq!(atoms[0].base, base::BEH, "first atom must be BEH");
    assert_eq!(atoms[1].base, base::BEH, "second atom must be BEH");
}

/// BiDi isolate 2066 is noise
#[test]
fn noise_bidi_isolate_2066() {
    let input = "\u{0628}\u{2066}\u{0628}";
    let r = mode_a(input);
    assert_eq!(
        r.stream.len(),
        2,
        "BEH + BiDi-2066 + BEH must keep exactly 2 BEH atoms"
    );
    let atoms = r.stream.atoms();
    assert_eq!(atoms[0].base, base::BEH, "first atom must be BEH");
    assert_eq!(atoms[1].base, base::BEH, "second atom must be BEH");
}

/// Combining grapheme joiner 034F is noise
#[test]
fn noise_cgj_034f() {
    let input = "\u{0628}\u{034F}\u{0628}";
    let r = mode_a(input);
    assert_eq!(
        r.stream.len(),
        2,
        "BEH + CGJ-034F + BEH must keep exactly 2 BEH atoms"
    );
    let atoms = r.stream.atoms();
    assert_eq!(atoms[0].base, base::BEH, "first atom must be BEH");
    assert_eq!(atoms[1].base, base::BEH, "second atom must be BEH");
}

// --- mode_b.rs: error paths ---

/// Mode B: empty frame (too short)
#[test]
fn mode_b_empty() {
    // v1.x contract: Mode B structural decode failures are reported as
    // ErrorKind::MalformedUtf8 { byte_offset } for API compatibility.
    // This will be replaced by MalformedFrame in v2.0.
    let r = dhad::mode_b::parse_frame(&[]);
    assert!(
        matches!(r, Err(ErrorKind::MalformedUtf8 { .. })),
        "empty Mode B frame must be MalformedUtf8 (v1.x contract), got {:?}",
        r
    );
}

/// Mode B: wrong magic bytes
#[test]
fn mode_b_bad_magic() {
    let r = dhad::mode_b::parse_frame(b"XXXX\x01\x42\x00\x00\x00\x00\x00\x00\x00\x00");
    assert!(
        matches!(r, Err(ErrorKind::MalformedUtf8 { .. })),
        "bad magic must be MalformedUtf8 (v1.x contract), got {:?}",
        r
    );
}

/// Mode B: wrong version
#[test]
fn mode_b_bad_version() {
    let r = dhad::mode_b::parse_frame(b"DHAD\xFF\x42\x00\x00\x00\x00\x00\x00\x00\x00");
    assert!(
        matches!(r, Err(ErrorKind::MalformedUtf8 { .. })),
        "bad version must be MalformedUtf8 (v1.x contract), got {:?}",
        r
    );
}

/// Mode B: wrong mode byte
#[test]
fn mode_b_bad_mode() {
    let r = dhad::mode_b::parse_frame(b"DHAD\x01\xFF\x00\x00\x00\x00\x00\x00\x00\x00");
    assert!(
        matches!(r, Err(ErrorKind::MalformedUtf8 { .. })),
        "bad mode byte must be MalformedUtf8 (v1.x contract), got {:?}",
        r
    );
}

// --- model.rs: DhadAtom::to_bytes ---

/// Verify atom serialization round-trip
#[test]
fn model_atom_to_bytes_reserved_zero() {
    let atom = dhad::model::DhadAtom {
        base: 0x0628,
        marks: 0x0001,
        flags: 0x01,
        prosody: 0x00,
        reserved: 0x0000,
    };
    let bytes = atom.to_bytes();
    assert_eq!(
        bytes,
        [0x28, 0x06, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00],
        "DhadAtom wire format must be exact little-endian and reserved must always be zero"
    );
}

// --- faps.rs: remaining unmapped branches ---

/// FE80 = standalone hamza presentation form
#[test]
fn faps_fe80_hamza() {
    let r = mode_a("\u{FE80}");
    assert_eq!(r.stream.len(), 1, "FE80 must produce 1 atom");
    let atom = r.stream.atoms()[0];
    assert_eq!(atom.base, base::HAMZA, "FE80 must decode to HAMZA");
}

/// FE89 = yeh hamza initial form
#[test]
fn faps_fe89_yeh_hamza() {
    let r = mode_a("\u{FE89}");
    assert_eq!(r.stream.len(), 1, "FE89 must produce 1 atom");
    let atom = r.stream.atoms()[0];
    assert_eq!(atom.base, base::YEH, "FE89 must decode to YEH base");
    assert_eq!(
        atom.flags & flags::HAMZA_ABOVE,
        flags::HAMZA_ABOVE,
        "FE89 must carry HAMZA_ABOVE flag"
    );
}

/// FE8D = alef isolated presentation form
#[test]
fn faps_fe8d_alef() {
    let r = mode_a("\u{FE8D}");
    assert_eq!(r.stream.len(), 1, "FE8D must produce 1 atom");
    assert_eq!(
        r.stream.atoms()[0].base,
        base::ALEF,
        "FE8D must decode to ALEF"
    );
}

/// FE93 = teh marbuta isolated
#[test]
fn faps_fe93_teh_marbuta() {
    let r = mode_a("\u{FE93}");
    assert_eq!(r.stream.len(), 1, "FE93 must produce 1 atom");
    assert_eq!(
        r.stream.atoms()[0].base,
        base::TEH_MARBUTA,
        "FE93 must decode to TEH_MARBUTA"
    );
}

/// FEF5 = lam-alef madda isolated
#[test]
fn faps_fef5_lam_alef_madda() {
    let r = mode_a("\u{FEF5}");
    assert_eq!(r.stream.len(), 2, "FEF5 must decompose to exactly 2 atoms");
    let atoms = r.stream.atoms();
    assert_eq!(atoms[0].base, base::LAM, "first atom must be LAM");
    assert_eq!(atoms[1].base, base::ALEF, "second atom must be ALEF");
    assert_eq!(
        atoms[1].flags & flags::MADDA,
        flags::MADDA,
        "second atom (ALEF) must carry MADDA flag"
    );
}

/// FEFB = lam-alef isolated
#[test]
fn faps_fefb_lam_alef() {
    let r = mode_a("\u{FEFB}");
    assert_eq!(r.stream.len(), 2, "FEFB must decompose to exactly 2 atoms");
    let atoms = r.stream.atoms();
    assert_eq!(atoms[0].base, base::LAM, "first atom must be LAM");
    assert_eq!(atoms[1].base, base::ALEF, "second atom must be ALEF");
}

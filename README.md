# Dhad (ضاد)

[![Crates.io](https://img.shields.io/crates/v/dhad.svg)](https://crates.io/crates/dhad)
[![Docs.rs](https://docs.rs/dhad/badge.svg)](https://docs.rs/dhad)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tests](https://img.shields.io/badge/tests-284%20verified-blue.svg)](#conformance)
[![Conformance](https://img.shields.io/badge/conformance-185%2F185-brightgreen.svg)](https://github.com/chokriabouzid-star/dhad-conformance-suite)
[![Proofs](https://img.shields.io/badge/proofs-10%20verified-blue.svg)](./PROOFS.md)

**An identity layer for digital Arabic text.**

> [اقرأ بالعربية](./README.ar.md)


## The 30-Second Pitch

You stored "محمد" in your database. A user searches for the same name copied
from a PDF: "ﻣﺤﻤﺪ". Your search returns nothing.

The two strings look identical. In Unicode, they are completely different
byte sequences:

    "محمد"  →  d985 d8ad d985 d8af           (8 bytes)
    "ﻣﺤﻤﺪ"  →  efbba3 efbaa4 efbba4 efbaaa   (12 bytes)

Dhad gives them one identity:

```rust
use dhad::modes::process_mode_a;

let a = process_mode_a("محمد".as_bytes())?;
let b = process_mode_a("ﻣﺤﻤﺪ".as_bytes())?;

assert_eq!(a.core_hash, b.core_hash);
// Both produce: 7661c73ca2970a2a7d6824c4e3a27560...
```

One Arabic word. One deterministic fingerprint. Forever.

This is empirically verified in [`PROOFS.md`](./PROOFS.md) (Proof 2).


## Why Dhad Exists

Arabic text in Unicode suffers from extreme representation ambiguity:

- A single letter has up to **5 positional forms** (isolated, initial, medial, final, canonical)
- Numbers have **3 representations** (ASCII, Arabic-Indic, Extended Arabic-Indic)
- Diacritic order is **swappable** (fatha+shadda visually equals shadda+fatha)
- **Invisible characters** (ZWJ, BOM, RTL marks, Tatweel) can silently contaminate text

This breaks:

- **Search** — `"محمد"` won't find `"ﻣﺤﻤﺪ"` in a database
- **Indexing** — the same word stored as 5+ different keys
- **Deduplication** — duplicate content slips through
- **Verification** — "is this Quranic verse identical to the master copy?" becomes unanswerable
- **NLP preprocessing** — language models see noise instead of structure

Dhad solves this with a **formal specification** (23 invariants, 7 corrections),
a **deterministic Rust implementation** (284 tests), and a **185-vector
conformance suite** verified across two independent implementations.

## Quick Start

Add Dhad to your project:

```bash
cargo add dhad
```

Process Arabic text (the examples below use the `hex` crate to display hashes — add it with `cargo add hex`):

```rust
use dhad::modes::process_mode_a;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = process_mode_a("بِسْمِ اللَّهِ".as_bytes())?;

    println!("atoms:    {}", result.stream.len());
    println!("core:     {}", hex::encode(result.core_hash));
    println!("phonetic: {}", hex::encode(result.phonetic_hash));

    Ok(())
}
```

Use the CLI (reads UTF-8 from stdin):

```bash
echo -n "بسم" | dhad-cli
# atoms:    3
# core:     0fb2277838219bbb6fa949b0ddecb22bf25c69161168022a801b032b41f23ac3
# phonetic: 12a5a9738a06de8b2abd74637ae4efa3c52b29e3aba8b702bb26715fe6df7cf4
```
---

## What You Get

Seven verified guarantees, each backed by an empirical proof:

| Guarantee | Description | Proof |
|-----------|-------------|-------|
| **Determinism** (A1) | Same input produces identical output, forever | All proofs |
| **No silent correction** (A2) | Invalid input returns a typed error, never a guess | Proof 9 (CR-07) |
| **Hash separation** (A3) | `CoreHash` and `PhoneticHash` capture independent layers | Proof 6 |
| **Glyph independence** (A5) | Positional forms (isolated/initial/medial/final) carry zero info | Proofs 1, 2 |
| **Mark order independence** (A6) | Diacritic ordering does not affect the atom | Proof 5 |
| **Digit source independence** (A7) | ASCII `1` = Arabic-Indic `١` = Extended `۱` | Proofs 3, 4 |
| **Noise filtering** | BOM, ZWJ, Tatweel silently removed | Proof 8 |

Every claim is reproducible from [`PROOFS.md`](./PROOFS.md).


## Two Hashes, Two Purposes

Dhad produces two independent SHA-256 fingerprints per text:

### CoreHash — Orthographic Identity

> "Are these written the same way?"

Captures letters, diacritics, and structural marks (hamza, madda).

Use for: search, deduplication, written-form comparison.

### PhoneticHash — Pronunciation Identity

> "Are these pronounced the same way?"

Captures CoreHash + prosodic information (tanween, madd, superscript alef).

Use for: phonetic search, prosodic analysis, recitation verification.

### Why Two Hashes?

The architectural insight: Arabic text carries two distinct identity layers.

A scribe and a reciter may agree on **what is written** while disagreeing
on **how it is recited**. Dhad captures this distinction computationally:

```rust
use dhad::modes::process_mode_a;

// Same letter, with and without tanween
let bare    = process_mode_a("ن".as_bytes())?;       // ن
let tanween = process_mode_a("نً".as_bytes())?;      // نً (with TANWEEN_FATH)

// CoreHash: identical (same orthography)
assert_eq!(bare.core_hash, tanween.core_hash);

// PhoneticHash: different (tanween changes pronunciation)
assert_ne!(bare.phonetic_hash, tanween.phonetic_hash);
```

Verified in [`PROOFS.md`](./PROOFS.md) (Proof 6).

---

## Empirically Verified Examples

Every example below is reproducible. Run them yourself.

### Example 1: Positional Forms (A5)

The same word in different Unicode encoding strategies:

```rust
use dhad::modes::process_mode_a;

// "Muhammad" in canonical form (basic Arabic block)
let canonical = process_mode_a("محمد".as_bytes())?;

// Same word in presentation forms (from PDF extraction)
let presentation = process_mode_a("ﻣﺤﻤﺪ".as_bytes())?;

assert_eq!(canonical.core_hash, presentation.core_hash);
// Both: 7661c73ca2970a2a7d6824c4e3a27560d9b5bec46be58b92e94e08ab90771365
```

Different byte sequences (8 vs 12 bytes), same canonical identity.

### Example 2: Digit Source Independence (A7)

Year "2025" in three Unicode digit systems:

```rust
let ascii    = process_mode_a("2025".as_bytes())?;    // U+0030..U+0039
let arabic   = process_mode_a("٢٠٢٥".as_bytes())?;    // U+0660..U+0669
let extended = process_mode_a("۲۰۲۵".as_bytes())?;    // U+06F0..U+06F9

assert_eq!(ascii.core_hash, arabic.core_hash);
assert_eq!(ascii.core_hash, extended.core_hash);
// All: bdcbec9491bc5bc56850fc9624a88792f24834c663ffc2b7fb11695e6ed7d14a
```

### Example 3: Invisible Character Filtering

Hostile or contaminated text becomes clean automatically:

```rust
let clean  = process_mode_a("بسم".as_bytes())?;

// Same text with BOM, ZWJ, and Tatweel injected
let dirty: &[u8] = &[
    0xef, 0xbb, 0xbf,                    // BOM
    0xd8, 0xa8,                          // ب
    0xe2, 0x80, 0x8d,                    // ZWJ
    0xd9, 0x80,                          // Tatweel
    0xd8, 0xb3,                          // س
    0xd9, 0x85,                          // م
];
let cleaned = process_mode_a(dirty)?;

assert_eq!(clean.core_hash, cleaned.core_hash);
// Both: 0fb2277838219bbb6fa949b0ddecb22bf25c69161168022a801b032b41f23ac3
```

See [`PROOFS.md`](./PROOFS.md) for full test results and reproduction steps.

---

## Modes

Dhad provides two entry points that produce identical results for the
same atom stream (verified in Proof 10).

### Mode A — UTF-8 Input (standard)

The primary path. Accepts UTF-8 byte sequences.

```rust
use dhad::modes::process_mode_a;
let result = process_mode_a("النص العربي".as_bytes())?;
```

### Mode B — Tagged Binary Frame

For content with prosodic annotations that have no Unicode source
(e.g., Quranic recitation data with madd duration markers).

```rust
use dhad::modes::process_mode_b;
use dhad::mode_b::build_frame;
use dhad::model::DhadAtom;

// ALEF with MADD_NORMAL (longest madd duration)
let atom = DhadAtom {
    base: 0x0001,      // ALEF
    marks: 0x0000,
    flags: 0x00,
    prosody: 0x08,     // MADD_NORMAL
    reserved: 0x0000,
};

let frame = build_frame(&[atom]);
let result = process_mode_b(&frame)?;

// CoreHash matches bare ALEF (madd is prosodic, not orthographic)
// PhoneticHash differs (madd affects pronunciation)
```

---

## Specification Highlights

### Atom Wire Format

Each atom is exactly **8 bytes**, little-endian:

| Bytes | Field | Type | Description |
|-------|-------|------|-------------|
| 0–1 | `base` | `u16` | Base letter ID |
| 2–3 | `marks` | `u16` | Diacritic bitmask |
| 4 | `flags` | `u8` | Hamza/Madda structural flags |
| 5 | `prosody` | `u8` | Tanween/MADD/Superscript Alef |
| 6–7 | `reserved` | `u16` | Always `0x0000` |

### Hash Construction

    CoreHash     = SHA-256("DHAD-CORE-V1"    || LE_u32(n) || atoms_compact)
    PhoneticHash = SHA-256("DHAD-PROSODY-V1" || CoreHash  || LE_u32(n) || prosody_bytes)

Where:
- `atoms_compact` = for each atom: `base` (LE u16) + `marks` (LE u16) + `flags` (u8)
- `prosody_bytes` = for each atom: `prosody` (u8)

### Anchor Constants (Empty Stream)

A conformant implementation **must** produce exactly these values for empty input:

    CoreHash:     8dd837c60eff174e0f40e75636deedec4bf020751f97cfe10dd1cf7117b16be0
    PhoneticHash: c5f62e920f5b06c74a02f25341f63499f1132da19084eb38e7b806c4a60a03f7

This is a mandatory self-test.

---

## Conformance

Dhad v1.2.0 is verified across **two independent implementations**:

| Implementation | Coverage | Status |
|----------------|----------|--------|
| Rust (this repo) | 284 unit + integration tests | ✅ |
| Python (independent reference) | 185/185 protocol vectors | ✅ |
| Anchor constants (cross-impl) | 4/4 | ✅ |
| Empirical behavioral proofs | 10/10 | ✅ |

### Conformance Vectors

| File | Mode | Vectors | Composition |
|------|------|---------|-------------|
| `golden.json` | A | 116 | All success cases |
| `adversarial.json` | A | 39 | 3 success + 36 typed errors |
| `tagged.json` | B | 30 | 9 success + 21 typed errors |
| **Total** | | **185** | |

Published in: [dhad-conformance-suite](https://github.com/chokriabouzid-star/dhad-conformance-suite)

### Verifying Locally

```bash
# Generate vectors from this implementation
cargo run --example export_vectors

# Verify with the dependency-free Python verifier
python3 ../dhad-conformance-suite/tools/verify_vectors.py

# Verify with the independent Python reference
python3 ../dhad-conformance-suite/python_ref/verify_golden_ref.py
python3 ../dhad-conformance-suite/python_ref/verify_tagged_ref.py
```

See [`CONFORMANCE.md`](./CONFORMANCE.md) for the full report.

---

## Error Handling

Dhad never guesses. Every error is typed and tells you exactly what went wrong:

| Error | Stage | Condition |
|-------|-------|-----------|
| `InputTooLarge` | Pre-1 | Input > 4 MiB |
| `MalformedUtf8` | 1 | Invalid byte sequence |
| `UnmappedCodepoint` | 3, 5 | Codepoint not in Dhad v1.0 registry |
| `OrphanDiacritic` | 6 | Diacritic with no preceding base |
| `InvalidMarkCombo` | 6, 10 | Incompatible diacritics |
| `InvalidFlagCombo` | 7, 10 | Incompatible flag bits |
| `InvalidProsody` | 9, 10 | Prosody violation |
| `ReservedFieldNonZero` | 10 | Atom `reserved` field != 0 (Mode B) |

Example:

```rust
use dhad::modes::process_mode_a;
use dhad::model::ErrorKind;

match process_mode_a(b"\xff") {
    Ok(_) => unreachable!(),
    Err(ErrorKind::MalformedUtf8 { byte_offset }) => {
        eprintln!("Invalid UTF-8 at byte {}", byte_offset);
    }
    Err(other) => eprintln!("Other error: {}", other),
}
```

---

## Known Limitations

Dhad v1.x is honest about what it does **not** do yet.

### CoreHash Includes Diacritics

The same Arabic word vocalized and unvocalized produces **different**
CoreHashes. This is by design (per spec §7.1):

```rust
let unvocalized = process_mode_a("محمد".as_bytes())?;
let vocalized   = process_mode_a("مُحَمَّد".as_bytes())?;

assert_ne!(unvocalized.core_hash, vocalized.core_hash);
```

This reflects the linguistic reality that diacritics carry semantic
information in Arabic (`عَلِمَ` "knew" vs `عُلِمَ` "was known").

**For fuzzy search across vocalized forms**, strip diacritics before
calling Dhad:

```rust
fn strip_diacritics(s: &str) -> String {
    s.chars()
        .filter(|c| {
            let cp = *c as u32;
            !(0x064B..=0x065F).contains(&cp) && *c != '\u{0670}'
        })
        .collect()
}

let normalized = strip_diacritics("مُحَمَّد");
let result = process_mode_a(normalized.as_bytes())?;
// Now matches CoreHash of "محمد"
```

A skeletal-only hash variant is under discussion via RFC-001 (see Roadmap).

### NFC Normalization Profile

Dhad expects Arabic text in **precomposed (NFC) form**. Decomposed
combining marks (`U+0653`, `U+0654`, `U+0655`) return `UnmappedCodepoint`.

If your source may contain decomposed text, normalize to NFC before
calling `process_mode_a`. Enforced by `tests/suite6_nfc_rejection.rs`.

### Quranic Recitation Marks

Extended recitation marks (`U+06D6`–`U+06ED`) return `UnmappedCodepoint`
in v1.x. A `QuranicRelaxed` profile is planned for v1.3.0.

### Mode B Frame Errors

In v1.x, malformed Mode B binary frames are reported as
`MalformedUtf8 { byte_offset }` for API compatibility. A dedicated
`MalformedFrame` error is planned for v2.0.

---

## Roadmap

| Version | Status | Highlights |
|---------|--------|------------|
| **v1.2.0** | Released | Conformance suite + Python reference parity + PROOFS.md |
| v1.3.0 | Planned | `QuranicRelaxed` profile, RFC-001 decision |
| v2.0.0 | Planned | `MalformedFrame` error, DhadIR (word-level representation) |
| Future | Vision | Wethaq (text authentication), search engine integrations |

### Open RFCs

- **RFC-001:** Should Dhad add a `SkeletalHash` (base-only fingerprint)?

  Community input wanted. See GitHub Discussions.

---

## Contributing

High-impact areas where help is most welcome:

- **Python binding** via PyO3 (`pip install dhad`)
- **WASM build** via wasm-pack (browser and Node.js)
- **PostgreSQL extension** for indexable Arabic search
- **Documentation translations** (especially Arabic, Persian, Urdu)
- **Additional conformance vectors** for edge cases

Before contributing changes to the core specification or hash computation,
please open an issue first — these affect the conformance contract.

The protected files list is enforced via `.github/CODEOWNERS`.

---

## Project Status

- **Version:** `1.2.0` (June 2025)
- **License:** MIT — free for all uses including commercial
- **Stability:** Production-ready for the documented scope
- **Maintenance:** Active, single maintainer
- **Empirical proofs:** 10 verified ([`PROOFS.md`](./PROOFS.md))
- **Stars:** Help us grow! ⭐ this repo if you find it useful.

---

## Dependencies

Minimal, audited dependencies:

```toml
sha2      = "0.10"   # SHA-256
crc32fast = "1.4"    # CRC-32 for Mode B frames
thiserror = "1.0"    # Error derive macros
hex       = "0.4"    # Hex encoding (re-exported for examples)
```

**No unsafe code. No nightly features. Stable Rust ≥ 1.75.0.**

---

## Fuzzing

Dhad includes `cargo-fuzz` / libFuzzer targets (requires nightly toolchain):

```bash
cargo +nightly fuzz run fuzz_mode_a -- -max_total_time=30
cargo +nightly fuzz run fuzz_mode_b -- -max_total_time=30
cargo +nightly fuzz run fuzz_determinism -- -max_total_time=30
```

---

## Resources

- **crates.io:** https://crates.io/crates/dhad
- **API Docs:** https://docs.rs/dhad
- **Conformance Suite:** https://github.com/chokriabouzid-star/dhad-conformance-suite
- **Conformance Report:** [`CONFORMANCE.md`](./CONFORMANCE.md)
- **Empirical Proofs:** [`PROOFS.md`](./PROOFS.md)

---

## License

MIT — see [LICENSE](./LICENSE) for full text.

---

## Contact

**Author:** Chokri Abouzid
**Email:** chokriabouzid@gmail.com
**GitHub:** [@chokriabouzid-star](https://github.com/chokriabouzid-star)

---

*Built with care for Arabic text. Designed as a protocol, not a library.*

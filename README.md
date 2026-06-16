# Dhad (ضاد)

[![Crates.io](https://img.shields.io/crates/v/dhad.svg)](https://crates.io/crates/dhad)
[![Docs.rs](https://docs.rs/dhad/badge.svg)](https://docs.rs/dhad)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tests](https://img.shields.io/badge/tests-284%20passing-brightgreen.svg)](#conformance)
[![Conformance](https://img.shields.io/badge/conformance-185%2F185-brightgreen.svg)](https://github.com/chokriabouzid-star/dhad-conformance-suite)

**An identity layer for digital Arabic text.**

> [اقرأ بالعربية](./README.ar.md)

---

## The 30-Second Pitch

These three strings look identical:

    "محمد"   "مُحَمَّد"   "ﻣﺤﻤﺪ"

In Unicode, they are **completely different byte sequences**. Your database search won't find them. Your deduplication breaks. Your text comparison fails.

Dhad gives them **one identity**:

```rust
use dhad::modes::process_mode_a;

let a = process_mode_a("محمد".as_bytes())?;
let b = process_mode_a("مُحَمَّد".as_bytes())?;
let c = process_mode_a("ﻣﺤﻤﺪ".as_bytes())?;

assert_eq!(a.core_hash, b.core_hash);
assert_eq!(a.core_hash, c.core_hash);
```

One Arabic word. One deterministic fingerprint. Forever.

---

## Why Dhad Exists

Arabic text in Unicode suffers from extreme representation ambiguity:

- A single letter has up to **5 positional forms**
- Numbers have **3 representations** (ASCII, Arabic-Indic, Extended)
- Diacritic order is **swappable** (fatha+shadda = shadda+fatha visually)
- **Invisible characters** (ZWJ, BOM, RTL marks) can be silently embedded

This breaks search, indexing, deduplication, verification, and NLP.

Dhad solves it with a **formal specification** (23 invariants, 7 corrections), a **deterministic implementation**, and a **185-vector conformance suite** verified across two independent implementations.

---


## Quick Start

Install:

```bash
cargo add dhad
```

Use:

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

CLI:

```bash
echo -n "بِسْمِ" | dhad-cli
```

---

## What You Get

| Feature | Guarantee |
|---------|-----------|
| **Determinism** (A1) | Same input → identical output, forever |
| **No silent correction** (A2) | Invalid input → typed error, never a guess |
| **Hash separation** (A3) | CoreHash and PhoneticHash are independent |
| **Glyph independence** (A5) | Positional forms carry zero information |
| **Mark order independence** (A6) | Diacritic ordering does not affect the atom |
| **Digit source independence** (A7) | 1 = ١ = ۱ |
| **Semantic integrity** (A8) | Contradictory combinations rejected |

---

## Two Hashes, Two Purposes

### CoreHash — Orthographic Identity

> "Are these written the same way?"

Captures letters, diacritics, and structural marks (hamza, madda). Use for: search, deduplication, written-form comparison.

### PhoneticHash — Pronunciation Identity

> "Are these pronounced the same way?"

Captures CoreHash + prosodic information (tanween, madd, superscript alef). Use for: phonetic search, prosodic analysis, recitation verification.

---

## Use Cases

| Use Case | Why Dhad |
|----------|----------|
| Search and indexing | One key per word, regardless of input variation |
| Deduplication | Detect duplicates across encoding differences |
| Quranic verification | Cryptographic-grade verse identity |
| Hadith authentication | Compare narrations across manuscripts |
| NLP preprocessing | Clean, deterministic tokenizer input |
| Legal document integrity | Detect any modification to Arabic text |
| Digital archive identity | Stable IDs for manuscript transcriptions |

---


## Modes

### Mode A — UTF-8 Input (standard)

```rust
use dhad::modes::process_mode_a;
let result = process_mode_a("النص العربي".as_bytes())?;
```

### Mode B — Tagged Binary

For content with prosodic annotations (e.g., Quranic recitation data):

```rust
use dhad::modes::process_mode_b;
use dhad::mode_b::build_frame;
use dhad::model::DhadAtom;
use dhad::registry::base;

let atom = DhadAtom {
    base: base::ALEF,
    marks: 0,
    flags: 0,
    prosody: 0x08,
    reserved: 0,
};
let frame = build_frame(&[atom]);
let result = process_mode_b(&frame)?;
```

---

## Atom Wire Format

Each atom is exactly **8 bytes**, little-endian:

| Bytes | Field | Type | Description |
|-------|-------|------|-------------|
| 0–1 | base | u16 | Base letter ID |
| 2–3 | marks | u16 | Diacritic bitmask |
| 4 | flags | u8 | Hamza/Madda structural flags |
| 5 | prosody | u8 | Tanween/MADD/Superscript Alef |
| 6–7 | reserved | u16 | Always 0x0000 |

---

## Hash Construction

    CoreHash     = SHA-256("DHAD-CORE-V1"    || LE_u32(n) || atoms)
    PhoneticHash = SHA-256("DHAD-PROSODY-V1" || CoreHash || LE_u32(n) || prosody)

### Anchor Constants (empty stream)

    CoreHash:     8dd837c60eff174e0f40e75636deedec4bf020751f97cfe10dd1cf7117b16be0
    PhoneticHash: c5f62e920f5b06c74a02f25341f63499f1132da19084eb38e7b806c4a60a03f7

---


## Conformance

Dhad v1.2.0 is verified across **two independent implementations**:

| Implementation | Coverage | Status |
|----------------|----------|--------|
| Rust (this repo) | 284 unit + integration tests | ✅ |
| Python (independent reference) | 185/185 protocol vectors | ✅ |
| Anchor constants (cross-impl) | 4/4 | ✅ |

### Conformance Vectors

| File | Mode | Vectors |
|------|------|---------|
| golden.json | A | 116 (success) |
| adversarial.json | A | 39 (3 ok + 36 errors) |
| tagged.json | B | 30 (9 ok + 21 errors) |
| **Total** | | **185** |

Published in: [dhad-conformance-suite](https://github.com/chokriabouzid-star/dhad-conformance-suite)

### Verifying Locally

```bash
cargo run --example export_vectors
python3 ../dhad-conformance-suite/tools/verify_vectors.py
python3 ../dhad-conformance-suite/python_ref/verify_golden_ref.py
python3 ../dhad-conformance-suite/python_ref/verify_tagged_ref.py
```

See [CONFORMANCE.md](./CONFORMANCE.md) for the full report.

---

## Error Handling

Dhad never guesses. Every error is typed:

| Error | Stage | Condition |
|-------|-------|-----------|
| InputTooLarge | Pre-1 | Input > 4 MiB |
| MalformedUtf8 | 1 | Invalid byte sequence |
| UnmappedCodepoint | 3, 5 | Codepoint not in Dhad v1.0 |
| OrphanDiacritic | 6 | Diacritic with no preceding base |
| InvalidMarkCombo | 6, 10 | Incompatible diacritics |
| InvalidFlagCombo | 7, 10 | Incompatible flag bits |
| InvalidProsody | 9, 10 | Prosody violation |
| ReservedFieldNonZero | 10 | reserved != 0 (Mode B) |

---

## Known Limitations

### NFC Normalization Profile

Dhad expects Arabic text in **precomposed (NFC) form**. Decomposed combining marks (U+0653, U+0654, U+0655) return UnmappedCodepoint. Enforced by `tests/suite6_nfc_rejection.rs`.

### Quranic Recitation Marks

Extended recitation marks (U+06D6–U+06ED) return UnmappedCodepoint in v1.x. A `QuranicRelaxed` profile is planned for v1.3.0.

### Mode B Frame Errors

In v1.x, malformed Mode B frames are reported as `MalformedUtf8 { byte_offset }` for API compatibility. A dedicated `MalformedFrame` error is planned for v2.0.

---


## Roadmap

| Version | Status | Highlights |
|---------|--------|------------|
| **v1.2.0** | Released | Conformance suite + Python reference parity |
| v1.3.0 | Planned | QuranicRelaxed profile |
| v2.0.0 | Planned | MalformedFrame, DhadIR (word-level) |
| Future | Vision | Wethaq (authentication), search integration |

---

## Contributing

High-impact areas:

- Python binding via PyO3 (`pip install dhad`)
- WASM build via wasm-pack (browser + Node.js)
- PostgreSQL extension for indexable Arabic search
- Documentation improvements and translations
- Additional conformance vectors for edge cases

Before contributing changes to the core specification or hash computation, please open an issue first — these affect the conformance contract.

---

## Dependencies

```toml
sha2      = "0.10"
crc32fast = "1.4"
thiserror = "1.0"
```

**No unsafe code. No nightly features. Stable Rust ≥ 1.75.0.**

---

## Fuzzing

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
- **Conformance Report:** [CONFORMANCE.md](./CONFORMANCE.md)

---

## License

MIT — free for all uses including commercial.

---

## Contact

**Author:** Chokri Abouzid
**Email:** chokriabouzid@gmail.com
**GitHub:** [@chokriabouzid-star](https://github.com/chokriabouzid-star)

---

*Built with care for Arabic text. Designed as a protocol, not a library.*

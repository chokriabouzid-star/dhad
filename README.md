# Dhad (ضاد)

A deterministic Arabic text canonicalization library in Rust.

Dhad converts Arabic Unicode text into a canonical atom stream and computes
two independent SHA-256 hashes: one for orthographic identity, one for
prosodic identity.

## Features

- **Deterministic** (A1): Same input → identical output on every conformant implementation
- **No silent correction** (A2): Invalid input → typed error, never a best guess
- **Hash separation** (A3): CoreHash (orthography) and PhoneticHash (prosody) are independent
- **Source independence** (A7): ASCII / Arabic-Indic / Extended Arabic-Indic digits are identical
- **Glyph independence** (A5): Positional forms (isolated/initial/medial/final) carry zero information
- **Mark order independence** (A6): Diacritic ordering does not affect the atom

## Usage

### Mode A: UTF-8 Input (standard)

```rust
use dhad::modes::process_mode_a;

let result = process_mode_a("بِسْمِ اللَّهِ".as_bytes())?;

println!("atoms:    {}", result.stream.len());
println!("core:     {}", hex::encode(result.core_hash));
println!("phonetic: {}", hex::encode(result.phonetic_hash));

// Serialize atom stream (n × 8 bytes, little-endian)
let bytes = result.stream.to_bytes();
```

### Mode B: Tagged Binary Input (MADD annotation)

```rust
use dhad::modes::process_mode_b;
use dhad::mode_b::build_frame;
use dhad::model::DhadAtom;
use dhad::registry::base;

// ALEF with MADD_NORMAL (longest madd) from recitation metadata
let atom = DhadAtom {
    base: base::ALEF, marks: 0, flags: 0,
    prosody: 0x08,  // MADD_NORMAL
    reserved: 0,
};
let frame = build_frame(&[atom]);
let result = process_mode_b(&frame)?;
```

### CLI

```bash
echo -n "بِسْمِ" | dhad-cli
# atoms:    3
# core:     ...
# phonetic: ...
```

## DhadAtom Wire Format

Each atom is exactly **8 bytes**, little-endian:

| Bytes | Field    | Type  | Description                    |
|-------|----------|-------|--------------------------------|
| 0–1   | base     | u16   | Base ID (letter identity)      |
| 2–3   | marks    | u16   | Diacritic bitmask              |
| 4     | flags    | u8    | Hamza/Madda structural flags   |
| 5     | prosody  | u8    | Tanween/MADD/Superscript Alef  |
| 6–7   | reserved | u16   | Always 0x0000                  |

## Hash Specification

### CoreHash

```
SHA-256("DHAD-CORE-V1" || LE_u32(n) || ∀ atom: LE_u16(base) || LE_u16(marks) || flags)
```

### PhoneticHash

```
SHA-256("DHAD-PROSODY-V1" || CoreHash || LE_u32(n) || ∀ atom: prosody)
```

### Anchor Constants (empty stream — mandatory self-test)

```
CoreHash:     8dd837c60eff174e0f40e75636deedec4bf020751f97cfe10dd1cf7117b16be0
PhoneticHash: c5f62e920f5b06c74a02f25341f63499f1132da19084eb38e7b806c4a60a03f7
```

## Error Types

| Error | Stage | Condition |
|-------|-------|-----------|
| `InputTooLarge` | Pre-1 | input > 4 MiB |
| `MalformedUtf8` | 1 | Invalid byte sequence |
| `UnmappedCodepoint` | 3, 5 | Codepoint not in Dhad v1.0 |
| `OrphanDiacritic` | 6 | Diacritic with no preceding base |
| `InvalidMarkCombo` | 6, 10 | Incompatible diacritics |
| `InvalidFlagCombo` | 7, 10 | Incompatible flag bits |
| `InvalidProsody` | 9, 10 | Prosody violation |
| `ReservedFieldNonZero` | 10 | reserved != 0 (Mode B) |

## Known Limitations

### Input normalization profile (NFC)

Dhad v1.x expects Arabic text in a **precomposed, NFC-oriented profile**.
The following decomposed combining marks are **not mapped** and will return
`UnmappedCodepoint`:

- U+0653 ARABIC MADDAH ABOVE
- U+0654 ARABIC HAMZA ABOVE
- U+0655 ARABIC HAMZA BELOW

NFC and NFD forms are **not** treated as equivalent in v1.x. If your source
may contain decomposed text, normalize it to NFC before calling `process_mode_a`.

This contract is enforced by `tests/suite6_nfc_rejection.rs`, which asserts
that bare and decomposed forms of these marks are rejected deterministically
with `ErrorKind::UnmappedCodepoint`.

### Quranic annotation marks

Extended Quranic recitation and pause marks (for example U+06D6–U+06ED and
related ranges) are **out of scope** for strict Mode A processing in v1.x and
will return `UnmappedCodepoint`.

### Mode B frame errors

In v1.x, malformed Mode B binary frames are reported as
`MalformedUtf8 { byte_offset }` for API compatibility. In this context,
`byte_offset` refers to the frame byte position, not UTF-8 text.
A dedicated `MalformedFrame` error is planned for a future major version.

### Identity pipeline stages

The pipeline is documented as 12 stages to match the specification.
In v1.x, some stages are identity stages because the relevant normalization
already happens earlier in the pipeline.

## Specification Conformance

Dhad v1.x is the **reference implementation** of the Dhad Implementation
Specification v1.0 (with corrections CR-01 through CR-07).

The vectors in this repository serve as a **self-consistency regression suite**.
An independent second implementation is planned to provide true
cross-implementation conformance.

See [CONFORMANCE.md](CONFORMANCE.md).

## Dependencies

```toml
sha2      = "0.10.8"   # SHA-256
crc32fast = "1.4.2"    # CRC-32 for Mode B frames
thiserror = "1.0.61"   # Error derive
```

No unsafe code. No nightly features. Stable Rust ≥ 1.75.0.

## Fuzzing

Dhad includes `cargo-fuzz` / libFuzzer targets for robustness testing.

The library itself remains stable-Rust compatible.
However, fuzzing requires a nightly toolchain because sanitizer-based fuzzing in Rust depends on nightly instrumentation.

### Fuzz targets

- `fuzz_mode_a`: feeds arbitrary bytes into `process_mode_a`
- `fuzz_mode_b`: feeds arbitrary bytes into `parse_frame`
- `fuzz_determinism`: checks that the same input always produces the same result class and hashes

### Run locally

```bash
rustup toolchain install nightly --profile minimal
cargo +nightly fuzz run fuzz_mode_a -- -max_total_time=30 -max_len=4096
cargo +nightly fuzz run fuzz_mode_b -- -max_total_time=30 -max_len=4096
cargo +nightly fuzz run fuzz_determinism -- -max_total_time=30 -max_len=4096
```

### Notes

- The fuzz harnesses are intentionally kept separate from the stable library build.
- Generated corpora, artifacts, and logs are ignored from git.
- Initial baseline fuzzing completed without crashes across all three targets.

## License

MIT — free for all uses including commercial.

For enterprise support or custom licensing, 
contact: CHOKRIABOUZID@GMAIL.COM

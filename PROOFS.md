# Dhad — Proven Behaviors

This document records empirical tests of Dhad's documented behaviors.
Each entry is a verified claim that may be referenced in README and HN posts.

**Implementation tested:** `dhad v1.2.0` from crates.io
**Test environment:** `~/test_dhad/`

---

## Proof 1: A5 — Glyph Independence (Letter BEH)

**Claim:** All five Unicode positional forms of letter BEH produce the
same CoreHash.

**Test inputs:**

| Form | Codepoint | Glyph |
|------|-----------|-------|
| Canonical | U+0628 | ب |
| Isolated (presentation) | U+FE8F | ﺏ |
| Initial | U+FE91 | ﺑ |
| Medial | U+FE92 | ﺒ |
| Final | U+FE90 | ﺐ |

**Result:**

All five inputs produce identical CoreHash:
`4cd5488d16f55023...`

This matches the normative test vector `GT-C02` in the specification.

**Verified:** ✅ A5 holds.

---


## Proof 2: A5 — Glyph Independence (Full Word "محمد")

**Claim:** The same Arabic word in canonical form vs presentation forms
produces identical CoreHash, PhoneticHash, and atom count.

**Test inputs:**

| Form | Bytes (hex) | Length |
|------|-------------|--------|
| Canonical `محمد` | `d985d8add985d8af` | 8 bytes |
| Presentation `ﻣﺤﻤﺪ` | `efbba3efbaa4efbba4efbaaa` | 12 bytes |

**Result:**

| Property | Value |
|----------|-------|
| CoreHash (both) | `7661c73ca2970a2a7d6824c4e3a27560d9b5bec46be58b92e94e08ab90771365` |
| PhoneticHash | Identical |
| Atom count | 4 = 4 |

**Significance:**

This is the strongest demonstration of A5. The same word, written via
different Unicode strategies (basic Arabic block vs Arabic Presentation
Forms-B), produces one canonical identity.

This is the exact scenario faced by:
- Text extracted from PDFs (often in presentation forms)
- OCR output
- Mixed-source text aggregation
- Cross-platform Arabic text

**Verified:** ✅ A5 holds at the word level.

---

## Proof 3: A7 — Digit Source Independence (Single Digit)

**Claim:** ASCII, Arabic-Indic, and Extended Arabic-Indic representations
of the same digit produce identical CoreHash.

**Test inputs:**

| Source | Codepoint | Bytes | Glyph |
|--------|-----------|-------|-------|
| ASCII | U+0031 | `31` | 1 |
| Arabic-Indic | U+0661 | `d9 a1` | ١ |
| Extended Arabic-Indic | U+06F1 | `db b1` | ۱ |

**Result:**

All three inputs produce identical CoreHash:
`1bb193bf83dbfcd2df724b5be0221c0e...`

Matches normative test vector `GT-D01` from the specification.

**Verified:** ✅ A7 holds for single digits.

---

## Proof 4: A7 — Digit Independence (Multi-Digit Year "2025")

**Claim:** A multi-digit number written in three different Unicode digit
systems produces identical CoreHash.

**Test inputs:**

| Source | Bytes (hex) | Byte length | Glyph |
|--------|-------------|-------------|-------|
| ASCII | `32303235` | 4 | 2025 |
| Arabic-Indic | `d9a2d9a0d9a2d9a5` | 8 | ٢٠٢٥ |
| Extended Arabic-Indic | `dbb2dbb0dbb2dbb5` | 8 | ۲۰۲۵ |

**Result:**

| Property | Value |
|----------|-------|
| CoreHash (all three) | `bdcbec9491bc5bc56850fc9624a88792f24834c663ffc2b7fb11695e6ed7d14a` |
| Atom count | 4 = 4 = 4 |

**Significance:**

Real-world impact: dates, financial figures, and statistics aggregated
from mixed Arab-region sources will deduplicate correctly without manual
normalization.

**Verified:** ✅ A7 holds at multi-digit scale.

---

## Proof 5: A6 — Mark Order Independence

**Claim:** When multiple diacritics are applied to the same letter, the
order in which they appear in the byte stream does not affect the CoreHash.

**Test inputs:** Letter BEH (ب) with both SHADDA (ّ) and FATHA (َ).

| Order | Bytes (hex) |
|-------|-------------|
| SHADDA then FATHA | `d8a8 d991 d98e` |
| FATHA then SHADDA | `d8a8 d98e d991` |

**Result:**

| Property | Value |
|----------|-------|
| CoreHash (both) | `564c8f3c08e292d38da6768c7c627321ae2606f7465c6088103575d9973c9f52` |
| PhoneticHash (both) | Identical |
| Atom count | 1 (single atom carrying both marks) |

Matches normative test vector `GT-M06`.

**Significance:**

Real-world impact: different keyboards, text editors, and operating
systems may emit diacritics in different orders. Without A6, visually
identical text would produce different fingerprints across platforms.

**Verified:** ✅ A6 holds.

---

## Proof 6: A3 — Hash Separation (Prosody Independence)

**Claim:** Tanween (prosodic annotation) affects PhoneticHash but NOT CoreHash.
This demonstrates that the two hashes capture genuinely different layers
of identity.

**Test inputs:**

| Form | Bytes (hex) | Meaning |
|------|-------------|---------|
| ن (bare NOON) | `d986` | Letter without prosodic annotation |
| نً (NOON + TANWEEN_FATH) | `d986 d98b` | Same letter with tanween |

**Result:**

| Hash | Input 1 (bare) | Input 2 (tanween) | Equal? |
|------|----------------|-------------------|--------|
| CoreHash | `4163ae2243aed7a7...` | `4163ae2243aed7a7...` | ✅ Yes |
| PhoneticHash | `5dc0e3712d074f51...` | `e6f46a21a00fb496...` | ❌ No |

Matches normative test vectors `GT-C25` (bare NOON) and `GT-R01` (NOON+TANWEEN).

**Significance:**

This is the architectural signature of Dhad. The same orthographic word
can carry different recitation annotations, and Dhad captures this
distinction in a computable, verifiable way:

- **For search and indexing:** use CoreHash (matches regardless of tanween)
- **For recitation verification:** use PhoneticHash (catches tanween differences)

No other Arabic text library provides this layered identity model.

**Verified:** ✅ A3 holds. Hash separation works as designed.

---

## Proof 7: Documented Boundary — Diacritics Affect CoreHash

**Claim:** Per specification §6.1, CoreHash is computed over `base + marks + flags`.
Therefore, the same Arabic word with and without diacritics produces
DIFFERENT CoreHashes. This is a deliberate design decision, not a bug.

**Test inputs:**

| Form | Bytes (hex) | Atoms |
|------|-------------|-------|
| محمد (unvocalized) | `d985 d8ad d985 d8af` | 4 |
| مُحَمَّد (vocalized) | `d985 d98f d8ad d98e d985 d98e d991 d8af` | 4 |

**Result:**

| Hash | Unvocalized | Vocalized | Equal? |
|------|-------------|-----------|--------|
| CoreHash | `7661c73ca2970a2a...` | `de1389a54044b6d4...` | ❌ No |
| PhoneticHash | derived from above | derived from above | ❌ No |
| Atom count | 4 | 4 | ✅ Same structural shape |

**Why this matters:**

Diacritics carry real semantic information in Arabic:
- عَلِمَ (he knew, past active)
- عُلِمَ (was known, past passive)
- عَالِم (scholar, noun)
- عِلْم (knowledge, noun)

All share the same letter skeleton but are different words. Treating
them as equal would be semantically incorrect.

**Implication for users:**

If your application needs to match vocalized and unvocalized forms
(common in fuzzy search), strip diacritics before passing text to Dhad:

```rust
fn strip_diacritics(s: &str) -> String {
    s.chars()
        .filter(|c| {
            let cp = *c as u32;
            !(0x064B..=0x065F).contains(&cp)
                && *c != '\u{0670}'
        })
        .collect()
}

let normalized = strip_diacritics("مُحَمَّد");
let result = process_mode_a(normalized.as_bytes())?;
// Now matches CoreHash of "محمد"
Status: A skeletal-only hash variant is under discussion via RFC-001.
Until then, diacritic stripping is the user's responsibility.

Verified: ✅ Boundary documented and reproducible.


## Proof 8: Invisible Character Filtering

**Claim:** Dhad silently filters invisible Unicode characters that
commonly contaminate Arabic text from various sources (editors, web
pages, OCR, copy-paste). All produce identical CoreHash.

**Test inputs:**

| Form | Bytes (hex) | Length |
|------|-------------|--------|
| Clean بسم | `d8a8 d8b3 d985` | 6 bytes |
| With BOM (U+FEFF) | `efbbbf d8a8 d8b3 d985` | 9 bytes |
| With ZWJ (U+200D × 2) | `d8a8 e2808d d8b3 e2808d d985` | 12 bytes |
| With Tatweel (U+0640 × 3) | `d8a8 d980 d980 d8b3 d980 d985` | 12 bytes |

**Result:**

| Property | Value |
|----------|-------|
| CoreHash (all four) | `0fb2277838219bbb6fa949b0ddecb22b...` |
| Atom count | 3 = 3 = 3 = 3 |

**Significance:**

These four sources commonly contaminate Arabic text without user awareness:

- **BOM (U+FEFF):** Auto-inserted by Windows editors, some HTTP responses
- **ZWJ (U+200D):** Embedded by certain websites and copy-paste operations
- **Tatweel (U+0640):** Used for visual justification in formal documents

Without Dhad, byte-level comparison would treat these as different strings.
With Dhad, they correctly resolve to one identity.

**Verified:** ✅ Silent noise filtering works for all major invisible sources.

---

## Summary: All Eight Proofs Verified

| # | Proof | Axiom | Status |
|---|-------|-------|--------|
| 1 | Single letter BEH — 5 glyph forms equal | A5 | ✅ |
| 2 | Full word محمد — canonical vs presentation | A5 | ✅ |
| 3 | Single digit — 3 source systems equal | A7 | ✅ |
| 4 | Multi-digit year 2025 — 3 systems equal | A7 | ✅ |
| 5 | Mark order — shadda+fatha = fatha+shadda | A6 | ✅ |
| 6 | Hash separation — tanween affects only PhoneticHash | A3 | ✅ |
| 7 | Boundary documented — diacritics affect CoreHash | (spec) | ✅ |
| 8 | Invisible filtering — BOM, ZWJ, Tatweel removed | (Stage 4) | ✅ |

**All eight tests reproducible from `~/test_dhad/` using `dhad v1.2.0`.**

---

## Proof 9: Mode B — Round-trip and Validation

**Claim:** Mode B (`build_frame` / `process_mode_b` / `parse_frame`) works
correctly and enforces all documented invariants including CR-07.

### 9.1: Round-trip with build_frame + process_mode_b

**Test:** ALEF + MADD_NORMAL atom (matches normative test vector GT-T01).

| Property | Value |
|----------|-------|
| Frame magic | `44484144` ("DHAD") |
| Frame version | `0x01` |
| Frame mode | `0x42` ("B") |
| Atom bytes | `01 00 00 00 00 08 00 00` |
| CoreHash | `68d32b955388e186...` (identical to bare ALEF) |
| PhoneticHash | `81c01948a1bde714...` (different, MADD affects prosody) |

Matches GT-T01 exactly. **Verified:** ✅

### 9.2: Round-trip with parse_frame

`parse_frame(build_frame([atom]))` returns the original atom unchanged.
**Verified:** ✅

### 9.3: CR-07 Enforcement (Reserved Field Non-Zero)

**Method:** Hand-crafted frame with `reserved = 0x0001` in atom bytes,
valid CRC-32 computed over the complete frame.

**Frame:** `444841440142010000000100000000000100128333a3`

**Result:**
Error: reserved field non-zero (0x0001) on atom at index 0

text


`process_mode_b` correctly rejects with `ReservedFieldNonZero` error.
**Verified:** ✅

### 9.4: CRC-32 Corruption Detection

**Method:** Flip one bit in CRC bytes of a valid frame.

**Result:** Frame rejected with error at byte offset 18.

Note: In v1.x, frame structural errors are reported as `MalformedUtf8`
for API compatibility (documented limitation, see README).
A dedicated `MalformedFrame` error is planned for v2.0.

**Verified:** ✅

### Mode B Architectural Insight

The two-layer design is intentional and elegant:

- `build_frame` is a **safe constructor** — it accepts a `DhadAtom` struct
  (Rust type system guarantees structural validity) and writes bytes.
  It does not validate semantic content because Rust's type system makes
  malformed atoms impossible at compile time.

- `process_mode_b` is a **strict validator** — it accepts arbitrary bytes
  from any source (network, file, hostile input) and validates every
  invariant including CR-07.

This separation means:
- Internal Rust code uses `build_frame` safely
- External/untrusted input goes through `process_mode_b` validation
- The wire format is always safe to consume

**Verified:** ✅ Two-layer design works as intended.

---

## Proof 10: AtomStream Serialization and Mode A ↔ Mode B Equivalence

**Claim:** `AtomStream::to_bytes()` produces exactly `n × 8` bytes per
the wire format specification. Atoms produced by Mode A, when fed back
through Mode B, produce identical hashes.

### 10.1: AtomStream.to_bytes() format

**Test:** Process `"بسم"` (3 letters) and inspect serialized bytes.

**Result:**

| Property | Value |
|----------|-------|
| Atoms produced | 3 |
| Serialized length | 24 bytes |
| Expected length | 24 bytes (3 × 8) |

**Byte-level breakdown:**

| Atom | Bytes (hex) | Decoded |
|------|-------------|---------|
| 0 (ب) | `02 00 00 00 00 00 00 00` | base=0x0002, marks=0, flags=0, prosody=0, reserved=0 |
| 1 (س) | `0c 00 00 00 00 00 00 00` | base=0x000c, marks=0, flags=0, prosody=0, reserved=0 |
| 2 (م) | `18 00 00 00 00 00 00 00` | base=0x0018, marks=0, flags=0, prosody=0, reserved=0 |

Each atom is exactly 8 bytes, little-endian, as specified. **Verified:** ✅

### 10.2: Mode A → Mode B Round-trip Equivalence

**Test:** Take atoms produced by Mode A, rebuild a Mode B frame from them,
process that frame through Mode B, and compare results.

**Method:**

```rust
let r_a = process_mode_a("بسم".as_bytes())?;
let atoms = r_a.stream.atoms().to_vec();
let frame = build_frame(&atoms);
let r_b = process_mode_b(&frame)?;

assert_eq!(r_a.core_hash, r_b.core_hash);
assert_eq!(r_a.phonetic_hash, r_b.phonetic_hash);
Result:

Hash	Mode A	Mode B	Equal?
CoreHash	0fb2277838219bbb...	0fb2277838219bbb...	✅
PhoneticHash	12a5a9738a06de8b...	12a5a9738a06de8b...	✅
Atom count	3	3	✅
Significance:

This proves that the two processing modes are semantically equivalent
for the same atom stream. Mode A is the UTF-8 entry point; Mode B is the
binary entry point. Both converge to the same canonical identity.

This also confirms the integrity of:

AtomStream::to_bytes() serialization
DhadAtom::to_bytes() byte layout
build_frame() framing
process_mode_b() parsing and revalidation
The wire format specification itself
Verified: ✅ Full round-trip equivalence between Mode A and Mode B.

Final Summary: 10 Verified Proofs
#	Proof	Category	Status
1	Single letter (BEH × 5 forms)	A5 — Glyph Independence	✅
2	Full word (محمد × 2 forms)	A5 — Glyph Independence	✅
3	Single digit (1/١/۱)	A7 — Digit Independence	✅
4	Multi-digit year (2025)	A7 — Digit Independence	✅
5	Mark order (shadda+fatha)	A6 — Order Independence	✅
6	Tanween affects PhoneticHash only	A3 — Hash Separation	✅
7	Diacritics affect CoreHash	Design Boundary	✅
8	BOM/ZWJ/Tatweel filtering	Stage 4 — Noise Filter	✅
9	Mode B round-trip + CR-07	Mode B Integrity	✅
10	Mode A ↔ Mode B equivalence	Wire Format	✅
All proofs reproducible from ~/test_dhad/ using dhad v1.2.0.

This document is the empirical foundation for all README claims.

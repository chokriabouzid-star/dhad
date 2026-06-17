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

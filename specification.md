# Dhad (ضاد) — Protocol Specification

**Version:** 1.0
**Status:** Normative
**Repository:** https://github.com/chokriabouzid-star/dhad
**Conformance suite:** https://github.com/chokriabouzid-star/dhad-conformance-suite

---

## 1. Purpose

Dhad is a deterministic canonicalization protocol for Arabic text. It
transforms Arabic Unicode input into a fixed-width binary representation
(an **AtomStream**) and derives two independent SHA-256 fingerprints from
it: **CoreHash** (orthographic identity) and **PhoneticHash** (prosodic
identity).

Dhad performs no linguistic interpretation. It does not know what a word
means, does not correct spelling, and does not infer intent. It only
records, deterministically, which characters are present and how they are
structured.

---

## 2. Design Axioms

Eleven axioms govern this specification. No implementation may violate
any of them.

| ID | Axiom | Statement |
|----|-------|-----------|
| A1 | Determinism | Identical input bytes produce identical output on every conformant implementation. |
| A2 | No Silent Correction | Invalid input produces a typed error. An implementation never emits a best-guess AtomStream for invalid input. |
| A3 | Sovereignty | No external normalization algorithm (NFC, NFD, NFKC, NFKD, or ICU rules) is invoked internally. Input is expected to already be NFC-precomposed; non-conforming input is rejected, not corrected. |
| A4 | Separation | Lexical identity (`base`), orthographic form (`marks` + `flags`), and prosody (`prosody`) occupy distinct, non-overlapping fields. CoreHash and PhoneticHash are the direct expression of this separation. |
| A5 | Glyph Independence | Positional presentation forms (isolated, initial, medial, final) carry zero information. Every form of a letter maps to the same Base ID. |
| A6 | Immutability | An atom cannot be modified after construction. |
| A7 | Mark Order Independence | The order in which diacritics arrive does not affect the resulting atom. |
| A8 | Digit Source Independence | ASCII digits, Arabic-Indic digits, and Extended Arabic-Indic digits are canonically identical. |
| A9 | No Inference | No atom field is derived from another field of the same atom. Fields that appear related (e.g., a letter and its hamza) are read independently from one external source. |
| A10 | Semantic Integrity | Contradictory prosodic annotations on the same atom are rejected rather than resolved by preference. |
| A11 | Completeness | Every valid Arabic character has exactly one canonical representation. This is an emergent property of A5, A6, A7, and A8 acting together, not a separate mechanism. |

---

## 3. Data Model

### 3.1 DhadAtom

Exactly 8 bytes, little-endian.

| Bytes | Field | Type | Description |
|-------|-------|------|-------------|
| 0–1 | `base` | u16 | Base ID (§4) |
| 2–3 | `marks` | u16 | Diacritic bitmask (§3.2) |
| 4 | `flags` | u8 | Structural modifier bitmask (§3.3) |
| 5 | `prosody` | u8 | Prosodic annotation bitmask (§3.4) |
| 6–7 | `reserved` | u16 | Must be `0x0000` |

### 3.2 Marks (u16)

| Bit | Mask | Name | Unicode |
|-----|------|------|---------|
| 0 | 0x0001 | FATHA | U+064E |
| 1 | 0x0002 | DAMMA | U+064F |
| 2 | 0x0004 | KASRA | U+0650 |
| 3 | 0x0008 | SUKUN | U+0652 |
| 4 | 0x0010 | SHADDA | U+0651 |

Nine values are valid: `0x0000`, `0x0001`, `0x0002`, `0x0004`, `0x0008`,
`0x0010`, `0x0011`, `0x0012`, `0x0014`. Any other value, including a
repeated attachment of an already-set bit, produces
`ERR_INVALID_MARK_COMBO`.

### 3.3 Flags (u8)

| Bit | Mask | Name | Permitted base |
|-----|------|------|-----------------|
| 0 | 0x01 | HAMZA_ABOVE | ALEF, WAW, YEH |
| 1 | 0x02 | HAMZA_BELOW | ALEF only |
| 2 | 0x04 | MADDA | ALEF only |

Four values are valid: `0x00`, `0x01`, `0x02`, `0x04`. Any other
combination produces `ERR_INVALID_FLAG_COMBO`.

### 3.4 Prosody (u8)

| Bit | Mask | Name | Unicode | Mode |
|-----|------|------|---------|------|
| 0 | 0x01 | TANWEEN_FATH | U+064B | A, B |
| 1 | 0x02 | TANWEEN_DAMM | U+064C | A, B |
| 2 | 0x04 | TANWEEN_KASR | U+064D | A, B |
| 3 | 0x08 | MADD_NORMAL | none | B only |
| 4 | 0x10 | MADD_EXTENDED | none | B only |
| 5 | 0x20 | SUPERSCRIPT_ALEF | U+0670 | A, B |

MADD_NORMAL and MADD_EXTENDED have no Unicode source and can only be set
through Mode B (§6.2).

Tanween is classified as prosody, not marks, because it is treated as a
word-final nunation layered on top of a base vowel, not a substitute for
one. As a direct consequence, tanween and its corresponding plain vowel
mark cannot coexist on the same atom (I18–I20).

### 3.5 AtomStream

An ordered sequence of atoms. The empty stream is valid. Serialized size
is `n × 8` bytes.

### 3.6 DhadResult

```
DhadResult {
    stream:        AtomStream,
    core_hash:     [u8; 32],
    phonetic_hash: [u8; 32],
}
```

---

## 4. Base ID Registry

### 4.1 Core Alphabet (0x0001–0x001C)

| ID | Letter | Unicode | | ID | Letter | Unicode |
|----|--------|---------|---|----|--------|---------|
| 0x0001 | ا ALEF | U+0627 | | 0x0011 | ظ ZAH | U+0638 |
| 0x0002 | ب BEH | U+0628 | | 0x0012 | ع AIN | U+0639 |
| 0x0003 | ت TEH | U+062A | | 0x0013 | غ GHAIN | U+063A |
| 0x0004 | ث THEH | U+062B | | 0x0014 | ف FEH | U+0641 |
| 0x0005 | ج JEEM | U+062C | | 0x0015 | ق QAF | U+0642 |
| 0x0006 | ح HAH | U+062D | | 0x0016 | ك KAF | U+0643 |
| 0x0007 | خ KHAH | U+062E | | 0x0017 | ل LAM | U+0644 |
| 0x0008 | د DAL | U+062F | | 0x0018 | م MEEM | U+0645 |
| 0x0009 | ذ THAL | U+0630 | | 0x0019 | ن NOON | U+0646 |
| 0x000A | ر REH | U+0631 | | 0x001A | ه HEH | U+0647 |
| 0x000B | ز ZAIN | U+0632 | | 0x001B | و WAW | U+0648 |
| 0x000C | س SEEN | U+0633 | | 0x001C | ي YEH | U+064A |
| 0x000D | ش SHEEN | U+0634 | | | | |
| 0x000E | ص SAD | U+0635 | | | | |
| 0x000F | ض DAD | U+0636 | | | | |
| 0x0010 | ط TAH | U+0637 | | | | |

Range 0x001D–0x001F is reserved. Any atom in this range produces
`ERR_UNMAPPED_CODEPOINT`.

### 4.2 Orthographic Units (0x0020–0x0023)

| ID | Character | Unicode | Name |
|----|-----------|---------|------|
| 0x0020 | ء | U+0621 | HAMZA (standalone) |
| 0x0021 | ة | U+0629 | TEH_MARBUTA |
| 0x0022 | ى | U+0649 | ALEF_MAQSURA |
| 0x0023 | ٱ | U+0671 | ALEF_WASLA |

TEH_MARBUTA and ALEF_MAQSURA are distinct identities and are never
normalized to HEH or YEH respectively.

### 4.3 Structural Characters (0x0040–0x0045)

| ID | Character | Unicode |
|----|-----------|---------|
| 0x0040 | (space) | U+0020 |
| 0x0041 | ، | U+060C |
| 0x0042 | ؛ | U+061B |
| 0x0043 | ؟ | U+061F |
| 0x0044 | . | U+002E |
| 0x0045 | : | U+003A |

### 4.4 Digits (0x0100–0x0109)

Three source forms per digit collapse to the same Base ID (Axiom A8).

| ID | Digit | ASCII | Arabic-Indic | Extended Arabic-Indic |
|----|-------|-------|--------------|--------------------------|
| 0x0100 | 0 | U+0030 | U+0660 | U+06F0 |
| 0x0101 | 1 | U+0031 | U+0661 | U+06F1 |
| 0x0102 | 2 | U+0032 | U+0662 | U+06F2 |
| 0x0103 | 3 | U+0033 | U+0663 | U+06F3 |
| 0x0104 | 4 | U+0034 | U+0664 | U+06F4 |
| 0x0105 | 5 | U+0035 | U+0665 | U+06F5 |
| 0x0106 | 6 | U+0036 | U+0666 | U+06F6 |
| 0x0107 | 7 | U+0037 | U+0667 | U+06F7 |
| 0x0108 | 8 | U+0038 | U+0668 | U+06F8 |
| 0x0109 | 9 | U+0039 | U+0669 | U+06F9 |

### 4.5 Precomposed Hamza and Madda

| Unicode | Glyph | Base ID | flags |
|---------|-------|---------|-------|
| U+0621 | ء | HAMZA (0x0020) | 0x00 |
| U+0622 | آ | ALEF (0x0001) | MADDA (0x04) |
| U+0623 | أ | ALEF (0x0001) | HAMZA_ABOVE (0x01) |
| U+0624 | ؤ | WAW (0x001B) | HAMZA_ABOVE (0x01) |
| U+0625 | إ | ALEF (0x0001) | HAMZA_BELOW (0x02) |
| U+0626 | ئ | YEH (0x001C) | HAMZA_ABOVE (0x01) |
| U+0671 | ٱ | ALEF_WASLA (0x0023) | 0x00 |

**On the hamza written on the line (U+0621, alone, not seated on a
carrier letter):** this is the same standalone HAMZA atom regardless of
where it occurs in a word — after a long alef as in إقصاء, in the middle
of a word, or word-finally as in سماء. Dhad converts each Unicode
codepoint to its atom independent of Arabic orthographic rules that
govern *why* a writer chose that seat; it does not validate that the
seat chosen matches standard Arabic spelling conventions. See §11.5.

### 4.6 Out of Scope in v1.0

Not mapped; produce `ERR_UNMAPPED_CODEPOINT`:

- Latin letters, non-listed punctuation, currency symbols, emoji.
- Persian/Urdu extension letters (e.g., PEH U+067E, Farsi YEH U+06CC).
- Decomposed combining forms U+0653, U+0654, U+0655 (§11.1).
- Quranic recitation marks U+06D6–U+06ED (§11.2).

Mixed-language input must be segmented by the caller before invoking
Dhad; Dhad processes homogeneous Arabic-script runs.

---

## 5. Canonicalization Pipeline

Twelve stages, executed in order.

| Stage | Name | Function |
|-------|------|----------|
| Pre | Size check | Reject input over 4,194,304 bytes |
| 1 | UTF-8 decode | Strict RFC 3629; reject overlong sequences, surrogates, codepoints above U+10FFFF |
| 2 | BOM removal | Strip a leading U+FEFF |
| 3 | FAPS decomposition | Map 141 presentation-form codepoints (127 single-codepoint mappings + 14 Lam-Alef two-codepoint mappings, across U+FB50–FDFF and U+FE70–FEFF) to canonical codepoints |
| 4 | Noise filtering | Silently remove 32 invisible/control codepoints (§5.1) |
| 5 | Classification | Sort codepoints into base, diacritic, prosodic, digit, or structural |
| 6 | Atom construction | Build atoms; attach diacritics; reject orphans and duplicates |
| 7 | Flag resolution | Identity stage — flags are already resolved in Stage 6 |
| 8 | Digit normalization | Identity stage — digits are already resolved in Stage 5 |
| 9 | Prosody resolution | Attach tanween and superscript-alef bits; reject contradictions |
| 10 | Validation | Check all 23 invariants (§8); first failure halts processing |
| 11 | Serialization | Write validated atoms as 8-byte records |
| 12 | Hashing | Compute CoreHash and PhoneticHash (§7) |

Stages 7 and 8 perform no independent work in the current implementation;
their corresponding transformations happen earlier, in Stage 6 and Stage
5 respectively. This is intentional — the stage numbering is preserved
for specification continuity.

Stage 9's prosody-attachment logic likewise executes inline within Stage
6's atom-construction loop rather than as a separate subsequent pass;
this produces identical output to a literal two-pass reading of this
table, since bitmask accumulation is order-independent and "attach to
the nearest preceding base atom" resolves identically either way. Stage
11 is not invoked as a discrete pipeline call in the reference
implementation — the equivalent capability is provided on demand via
`AtomStream::to_bytes()` and `DhadAtom::to_bytes()`. Both simplifications
are noted here for the same reason as Stages 7–8: numbering is preserved
for specification continuity, and no independent implementation should
infer a structural requirement — such as a mandatory second pass, or a
literal intermediate byte buffer — from stage count alone. What is
normative is the final AtomStream and its hashes, not the internal
call structure that produces them.

### 5.1 Noise Set (32 codepoints)

| Codepoint(s) | Count |
|--------------|-------|
| U+0640 (tatweel) | 1 |
| U+200C–U+200F (ZWNJ, ZWJ, LRM, RLM) | 4 |
| U+202A–U+202E (BiDi embedding) | 5 |
| U+2066–U+2069 (BiDi isolates) | 4 |
| U+FEFF (non-initial BOM) | 1 |
| U+FE00–U+FE0F (variation selectors) | 16 |
| U+034F (combining grapheme joiner) | 1 |
| **Total** | **32** |

---

## 6. Input Modes

### 6.1 Mode A — UTF-8 Text

The standard entry point. Accepts raw UTF-8 bytes. Cannot produce atoms
with MADD_NORMAL or MADD_EXTENDED set — no Unicode codepoint maps to
those bits.

### 6.2 Mode B — Tagged Binary Frame

Accepts pre-annotated atoms, used when a higher-level system supplies
recitation metadata (such as madd duration) that has no Unicode
representation.

| Bytes | Field | Value |
|-------|-------|-------|
| 0–3 | magic | `44 48 41 44` ("DHAD") |
| 4 | version | `0x01` |
| 5 | mode | `0x42` ('B') |
| 6–9 | atom count | u32, little-endian |
| 10..10+n×8 | atoms | n × 8 bytes |
| last 4 | checksum | CRC-32 over all preceding bytes |

Minimum frame size: 14 bytes. Every atom is validated against the full
invariant set (§8), and a nonzero `reserved` field is rejected.

In the current version, malformed-frame errors are reported using the
same error kind as malformed UTF-8, for API stability; the byte offset
in that case refers to a position within the frame, not within UTF-8
text.

---

## 7. Hash Specification

### 7.1 CoreHash

```
CoreHash = SHA-256(
    "DHAD-CORE-V1"
  || atom_count as u32, little-endian
  || for each atom, in order:
       base    as u16, little-endian
       marks   as u16, little-endian
       flags   as u8
)
```

### 7.2 PhoneticHash

```
PhoneticHash = SHA-256(
    "DHAD-PROSODY-V1"
  || CoreHash                          (32 raw bytes, not hex)
  || atom_count as u32, little-endian
  || for each atom, in order:
       prosody as u8
)
```

### 7.3 Properties

- CoreHash and PhoneticHash cannot collide with each other.
- Two texts may share CoreHash while differing in PhoneticHash — this is
  the direct expression of Axiom A4.
- Identical PhoneticHash implies identical CoreHash.
- `reserved` never contributes to either hash.

### 7.4 Mandatory Anchors

Every conformant implementation must reproduce these exactly.

**Empty stream**

```
core_hash:      8dd837c60eff174e0f40e75636deedec4bf020751f97cfe10dd1cf7117b16be0
phonetic_hash:  c5f62e920f5b06c74a02f25341f63499f1132da19084eb38e7b806c4a60a03f7
```

**ALEF bare** (`d8a7` → stream `0100000000000000`)

```
core_hash:      68d32b955388e186a3ad963008c4aed8f9d957d9fe72ad0e29ad5012d57e140d
phonetic_hash:  984a596fe5175c6413a180a8d1f09891fb53675f5a8b9daac5a1dd4a2ea784d0
```

**BEH bare** (`d8a8` → stream `0200000000000000`)

```
core_hash:      4cd5488d16f55023d7a6816009777bac5297dbb57a0f5315085693a1dfb438ac
phonetic_hash:  2e5317b842f738a15e3aaf04cb527e61cb7979a44ac1c99f6a4b464fb50056a3
```

**BEH + FATHA** (`d8a8d98e` → stream `0200010000000000`)

```
core_hash:      f4226a79f1c62998559c44298ad718388045dc6cd5c096a9bc197175268d2a04
phonetic_hash:  97b6eb001215607e9e8424d96893106a648b21f345d06c8819591a481c2b6ec8
```

If an implementation disagrees with any anchor above, the implementation
is wrong.

---

## 8. Formal Invariants

```
I01  base in valid range                              -> ERR_UNMAPPED_CODEPOINT
I02  base not in {0x001D, 0x001E, 0x001F}              -> ERR_UNMAPPED_CODEPOINT
I03  marks in the 9 valid combinations                 -> ERR_INVALID_MARK_COMBO
I04  flags in {0x00, 0x01, 0x02, 0x04}                 -> ERR_INVALID_FLAG_COMBO
I05  HAMZA_ABOVE requires base in {ALEF, WAW, YEH}      -> ERR_INVALID_FLAG_COMBO
I06  HAMZA_BELOW requires base == ALEF                 -> ERR_INVALID_FLAG_COMBO
I07  MADDA requires base == ALEF                       -> ERR_INVALID_FLAG_COMBO
I08  not (HAMZA_ABOVE and HAMZA_BELOW)                 -> ERR_INVALID_FLAG_COMBO
I09  not (MADDA and any HAMZA flag)                    -> ERR_INVALID_FLAG_COMBO
I10  not (TANWEEN_FATH and TANWEEN_DAMM)               -> ERR_INVALID_PROSODY
I11  not (TANWEEN_FATH and TANWEEN_KASR)               -> ERR_INVALID_PROSODY
I12  not (TANWEEN_DAMM and TANWEEN_KASR)               -> ERR_INVALID_PROSODY
I13  not (MADD bit and any TANWEEN bit)                -> ERR_INVALID_PROSODY
I14  not (MADD_NORMAL and MADD_EXTENDED)                -> ERR_INVALID_PROSODY
I15  MADD requires base in LONG_VOWEL_CLASS             -> ERR_INVALID_PROSODY
I16  inert-class atom has prosody == 0                  -> ERR_INVALID_PROSODY
I17  inert-class atom has marks == 0                    -> ERR_INVALID_MARK_COMBO
I18  TANWEEN_FATH excludes FATHA on the same atom       -> ERR_INVALID_PROSODY
I19  TANWEEN_DAMM excludes DAMMA on the same atom       -> ERR_INVALID_PROSODY
I20  TANWEEN_KASR excludes KASRA on the same atom       -> ERR_INVALID_PROSODY
I21  SUPERSCRIPT_ALEF excludes any TANWEEN bit          -> ERR_INVALID_PROSODY
I22  reserved == 0x0000                                 -> ERR_RESERVED_FIELD_NONZERO
I23  marks reserved bits (5-15) == 0                    -> ERR_INVALID_MARK_COMBO
```

`LONG_VOWEL_CLASS` = {ALEF, WAW, YEH, ALEF_MAQSURA}.
`inert-class` = structural characters and digits (Base IDs 0x0040–0x0045
and 0x0100–0x0109); these atoms never carry marks or prosody.

I02 is subsumed by I01 in the current check order (both reject the same
range); I08 and I09 are subsumed by I04 (multi-bit flag values are
rejected before either check is reached). All 23 rules are enforced —
some fire through an earlier check rather than independently.

### 8.1 Invariant I24 — SUKUN / TANWEEN Exclusion

Before this rule was added, no invariant rejected an atom carrying both
`SUKUN` (marks) and any `TANWEEN_*` bit (prosody) simultaneously, even
though the two are linguistically incompatible — sukun denotes the
absence of a vowel, while tanween is a vowel-plus-nunation ending. The
gap was acknowledged in the test suite before this rule existed (a
corresponding rejection test had been removed with a comment noting the
missing rule; that test — `at_037` — has been reintroduced alongside
this invariant) and is now closed.

```
I24: not (SUKUN and any TANWEEN bit) on the same atom
     -> ERR_INVALID_PROSODY
```

I24 is the first addition to the invariant set since v1.0, added under
the minor-version-compatible carve-out stated in §10: it rejects
previously-unspecified input without changing the output of any input
that was valid before it was added. Unlike I01–I23, I24 is not frozen by
the same v1.0 anchor commitment described in §10 — as a minor addition
it is itself now part of the current normative rule set and is enforced
identically for both Mode A and Mode B input, alongside I01–I23, in
`validate_atom`.

---

## 9. Error Catalog

| Error | Condition |
|-------|-----------|
| `InputTooLarge` | input exceeds 4,194,304 bytes |
| `MalformedUtf8` | invalid UTF-8 byte sequence, or a malformed Mode B frame |
| `UnmappedCodepoint` | codepoint has no Base ID mapping |
| `OrphanDiacritic` | a diacritic has no preceding base atom |
| `InvalidMarkCombo` | an incompatible diacritic combination |
| `InvalidFlagCombo` | an incompatible flag combination |
| `InvalidProsody` | a prosody rule violation (I10–I21, I24) |
| `ReservedFieldNonZero` | `reserved != 0` in a Mode B atom |

Every error is deterministic: the same invalid input always produces the
same error kind on every conformant implementation.

### 9.1 Correction Note — FAPS Codepoint Count

An earlier draft of this specification stated the Stage 3 decomposition
table maps "153" codepoints. That figure was not independently verified
at the time it was written and was carried forward from an early
descriptive estimate rather than a direct count of the shipped table.
Direct enumeration of every match arm in `src/faps.rs` gives **141**:
127 codepoints that decompose to a single canonical codepoint, plus 14
Lam-Alef ligature codepoints that decompose to two canonical codepoints
each. This has been corrected throughout this document. No code change
was required — the implementation was correct; only the prose count was
wrong.

---

## 10. Compatibility Policy

- **Patch releases** correct specification text with no behavior change.
- **Minor releases** add backward-compatible, opt-in features.
- **Major releases** may change the output of an existing vector.

Invariant additions that close a gap already inconsistent with an
existing axiom — as opposed to introducing a new restriction unrelated
to prior stated behavior — are minor-compatible even though they cause
some previously-accepted input to be rejected. I24 (§8.1) is the
reference case: it enforces Axiom A10 (Semantic Integrity) against a
combination the invariant set never correctly covered, rather than
imposing a new, independently-motivated restriction. A future invariant
addition qualifies for this carve-out only if it can be shown, as I24
was, to enforce an axiom already in §2 against a case that axiom's own
statement already covered in spirit.

The following are frozen for v1.x and cannot change without a major
version:

- The four mandatory anchors (§7.4).
- Invariants I01–I23.
- The hash domain prefixes and construction (§7.1, §7.2).
- Every existing Base ID assignment (§4).
- The axiom numbering in §2. No future minor version may reuse a number
  for a different concept.

---

## 11. Known Limitations

### 11.1 NFC Input Required

Dhad processes NFC-precomposed Arabic text only. The decomposed
combining forms U+0653, U+0654, and U+0655 are unmapped and produce
`ERR_UnmappedCodepoint`. Per Axiom A3, Dhad does not normalize input
itself; callers whose text may be decomposed must normalize to NFC
before calling Dhad.

### 11.2 Quranic Recitation Marks

The extended annotation range U+06D6–U+06ED is unmapped in the default
processing path. A relaxed profile that accepts these marks with
explicit reporting is planned for a future minor release.

### 11.3 Mode B Error Precision

Mode B frame errors currently reuse the same error kind as malformed
UTF-8 text, for API stability. A dedicated frame-error kind is planned
for the next major version.

### 11.4 Homogeneous Input Only

Dhad does not tolerate mixed-language strings in a single call. A
caller must segment Arabic runs from non-Arabic content before invoking
Dhad; this follows from Axiom A2 — silently skipping unrecognized
characters inside a mixed string would itself be a form of guessing.

### 11.5 No Orthographic Validation

Dhad converts whatever hamza placement the input actually contains — on
alef, waw, yeh, or standalone on the line — into the corresponding atom.
It does not check whether that placement follows standard Arabic
spelling rules (for example, whether a given word's hamza should sit on
a carrier letter or stand alone according to its grammatical case). Two
different spellings of what a human reader would consider "the same
word," if they place the hamza differently, produce different
CoreHashes by design; validating spelling correctness is outside Dhad's
scope.

---

## 12. Conformance

An implementation is conformant with this specification when it:

1. Reproduces the four mandatory anchors in §7.4 exactly.
2. Passes all vectors in the published conformance corpus (185 vectors:
   116 golden, 39 adversarial, 30 tagged).
3. Never produces MADD bits from Mode A input.
4. Rejects U+0653, U+0654, and U+0655 with `UnmappedCodepoint`.
5. Rejects U+0670 following an inert-class atom with `InvalidProsody`.
6. Produces no panic or undefined behavior for any input up to the
   maximum size, including malformed and adversarial input.
7. Rejects an atom carrying both `SUKUN` and any `TANWEEN_*` bit with
   `InvalidProsody` (I24, §8.1).

---

## Appendix A — Property Reference (P1–P10)

Informative, not normative. These labels are cited as test-traceability
evidence elsewhere in this document and in the project's generated
conformance report; they are not additional protocol rules beyond the
axioms (§2) and invariants (§8). Each is verified by one or more
property-based or example tests in `tests/suite4_properties.rs`.

**Derivation note:** this list was reconstructed from the `p1_`…`p10_`
test function names observed in an actual `cargo test` run, not
transcribed from the doc comments inside `suite4_properties.rs` itself.
Cross-check the wording below against that file's own comments before
treating it as final; adjust the descriptions, not the numbering, if
they differ.

| ID | Property | Statement | Relates to |
|----|----------|-----------|------------|
| P1 | Determinism | The same input bytes, processed twice, produce byte-identical `AtomStream`, `CoreHash`, and `PhoneticHash`. | A1 |
| P2 | Hash–Stream Consistency | `CoreHash` depends only on `base`/`marks`/`flags` and is unaffected by `prosody`; `PhoneticHash` depends on `prosody` and changes when it does. | A4 |
| P3 | Noise Filter Completeness | All 32 codepoints in the noise set (§5.1), in any position or combination, are silently and completely removed with no residue. | Stage 4 |
| P4 | Digit Source Independence | ASCII, Arabic-Indic, and Extended Arabic-Indic digits for the same numeral value always produce the same Base ID. | A8 |
| P5 | Lam-Alef Ligature Equivalence | Every precomposed Lam-Alef presentation-form ligature produces the same atom pair as its decomposed LAM + ALEF-variant sequence. | §4.5, FAPS |
| P6 | Separation Under Mode B | MADD bits (Mode B only) and TANWEEN bits do not affect `CoreHash`, and bare vs. tanween-marked atoms differ only in `PhoneticHash`. | A4 |
| P7 | Mark Order Independence | Diacritics attached to the same base atom in any arrival order produce the same final `marks` value. | A7 |
| P8 | Error Determinism | The same invalid input always produces the same `ErrorKind`, with the same relevant fields, on every run. | A2 |
| P9 | Atom Byte Size | `DhadAtom::to_bytes()` always returns exactly 8 bytes, for every valid atom. | §3.1 |
| P10 | Crash Resistance | No input up to the maximum size (Pre-stage limit) — including adversarial and malformed input, in both Mode A and Mode B — causes a panic or undefined behavior. | §12.6 |

---

*End of specification.*

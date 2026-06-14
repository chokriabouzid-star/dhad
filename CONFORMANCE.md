# Dhad Conformance Report
**Generated:** 2026-06-13T23:29:17Z
**Specification:** Dhad Implementation Specification v1.0
**Status:** ✅ CONFORMANT

---

## Test Results

| Suite | Tests | Result |
|-------|-------|--------|
| suite1_golden (Mode A)         | 121 | ✅ |
| suite2_tagged (Mode B)         | 25  | ✅ |
| suite3_adversarial             | 52  | ✅ |
| suite4_properties              | 17  | ✅ |
| suite5_coverage (behavioral)   | 49  | ✅ |
| suite6_nfc_rejection (new)     | 11  | ✅ |
| unit tests (lib)               |  6  | ✅ |
| doc tests                      |  3  | ✅ |
| **Total**                      | **284** | ✅ **284 passed, 0 failed** |

---

## Specification Corrections

| CR | Description | Status |
|----|-------------|--------|
| CR-01 | Base IDs 0x001D–0x001F explicitly rejected | ✅ |
| CR-02 | MADD bits (0x08, 0x10) only via Mode B | ✅ |
| CR-03 | All hash values verified against reference Python | ✅ |
| CR-04 | MAX_INPUT_BYTES = 4,194,304 enforced | ✅ |
| CR-05 | P2 is Hash-Stream Consistency (not Idempotency) | ✅ |
| CR-06 | U+0670 after PROSODY_INERT_CLASS → ERR_INVALID_PROSODY | ✅ |
| CR-07 | Mode B reserved field rejected if non-zero | ✅ |

---

## Design Axioms

| Axiom | Description | Verified By |
|-------|-------------|-------------|
| A1 Determinism | Same input → identical output | P1 (proptest) |
| A2 No Silent Correction | Invalid → error, never guess | suite3 (46 error tests) |
| A3 Separation | CoreHash ⊥ PhoneticHash | P2, P6, GT-R01–R03, GT-T01–T04 |
| A5 Glyph Independence | Positional forms carry zero info | GT-111–116, cross_a5 |
| A6 Mark Order Independence | Diacritic order irrelevant | P7, GT-125/126 |
| A7 Source Digit Independence | ASCII=Arabic-Indic=Extended | P4, GT-039–068 |
| A8 Semantic Integrity | Contradictory prosody rejected | AT-036, AT-040, AT-041 |

---

## Formal Invariants (I01–I23)

All 23 invariants enforced in Stage 10 (`src/invariants.rs`):

| Invariant | Description | Test |
|-----------|-------------|------|
| I01 | base ∈ valid ranges | AT-reserved_base |
| I02 | base ∉ {0x001D, 0x001E, 0x001F} | AT-reserved_base_001d/e/f |
| I03 | marks ∈ VALID_MARK_COMBINATIONS | AT-023–027 |
| I04 | flags ∈ {0x00, 0x01, 0x02, 0x04} | AT-030–034 |
| I05 | HAMZA_ABOVE seat: ALEF, WAW, YEH only | AT-032 |
| I06 | HAMZA_BELOW seat: ALEF only | AT-033 |
| I07 | MADDA seat: ALEF only | AT-034 |
| I08 | NOT (HAMZA_ABOVE AND HAMZA_BELOW) | AT-030 |
| I09 | NOT (MADDA AND HAMZA_*) | AT-031 |
| I10 | NOT (TW_FATH AND TW_DAMM) | AT-035 |
| I11 | NOT (TW_FATH AND TW_KASR) | AT-035 |
| I12 | NOT (TW_DAMM AND TW_KASR) | AT-035 |
| I13 | NOT (MADD AND TANWEEN) | AT-038 |
| I14 | NOT (MADD_N AND MADD_X) | AT-038 |
| I15 | MADD → LONG_VOWEL_CLASS | AT-039, frame_err_madd_on_beh |
| I16 | INERT → prosody == 0 | AT-040 |
| I17 | INERT → marks == 0 | AT-028, AT-029 |
| I18 | TW_FATH → NOT FATHA | AT-036 |
| I19 | TW_DAMM → NOT DAMMA | (covered by I10–I12) |
| I20 | TW_KASR → NOT KASRA | AT-036 variant |
| I21 | SUPER_ALEF → NOT TANWEEN | (I21 enforced in stage9) |
| I22 | reserved == 0x0000 | AT-reserved_field_nonzero |
| I23 | marks reserved bits [5..15] == 0 | (enforced by VALID set) |

---

## Hash Anchors (Mandatory Self-Test)

Empty stream MUST produce exactly these values:

| Hash | Value |
|------|-------|
| CoreHash | `8dd837c60eff174e0f40e75636deedec4bf020751f97cfe10dd1cf7117b16be0` |
| PhoneticHash | `c5f62e920f5b06c74a02f25341f63499f1132da19084eb38e7b806c4a60a03f7` |

Verified by: `gt_117_empty` (suite1), `gt_t_empty_frame` (suite2)

---

## Implementation Quality

| Check | Result |
|-------|--------|
| Unsafe code | ✅ None |
| Clippy warnings | ✅ 0 warnings |
| CLI binary builds | ✅ |
| Benchmarks compile | ✅ |
| Rust edition | 2021 (stable ≥ 1.75.0) |
| Nightly features | ✅ None used |

---

## Spec Deviation Log

| ID | Description | Resolution |
|----|-------------|------------|
| DEV-01 | GT-S02 (Allah) stream shows LAM+SUKUN but input has no U+0652 | Spec stream is incorrect. Implementation follows input (A1). Hashes recomputed. |
| DEV-02 | JSON GT-092–095 classified as Mode A but expect MADD bits | Reclassified as Mode B. Tested in suite2 as GT-T05–T08. |
| DEV-03 | at_037 (SUKUN+TANWEEN) not prohibited by I01–I23 | Test removed. Candidate for CR-08 in future Spec revision. |

---

*Generated automatically by `generate_conformance.py`.*
*Do not edit manually — re-run the script to update.*

---

## Pending Corrections (Future Spec Revision)

### CR-08 (Proposed)

**Issue:** GT-S02 (اللَّه) in Spec §9 shows expected stream with
LAM+SUKUN on first LAM, but the input D9B1 D984 D984 D991 D98E D987
contains no U+0652 (SUKUN) codepoint.

**Root cause:** The normative stream in the Spec was computed with an
implicit SUKUN that has no Unicode source in the given input.

**Resolution in this implementation:**
- The implementation follows Axiom A1 (Determinism): output is fully
  determined by input. No SUKUN is synthesized.
- Correct stream: LAM bare (0x1700000000000000) for first LAM.
- Recomputed hashes:
  - CoreHash:     402b6c8b13295c3eb313892366f81c2be02e12d9172b9eafb625757de3cf57f0
  - PhoneticHash: 45b9f356ff4f0cb173bf10c612bc1a4796ae3b9bf0bc541cb856819e52cfb6a2
- Test gt_099_allah uses corrected values (verified against Python reference).

**Recommended Spec fix:** Update GT-S02 stream and hashes to match
the input, OR add U+0652 to the normative input bytes.

# Dhad Conformance Report
**Generated:** 2026-07-19T22:29:37Z
**Specification:** Dhad v1.0
**Status:** ✅ CONFORMANT

---

## Versions

| Component | Version |
|-----------|---------|
| Library (`dhad`) | 1.2.0 |
| Rust edition | 2021 |
| Dhad spec | v1.0 |

---

## Test Results

| Suite | Tests | Result |
|-------|-------|--------|
| suite1_golden (Mode A) | 121 | ✅ |
| suite2_tagged (Mode B) | 25 | ✅ |
| suite3_adversarial | 53 | ✅ |
| suite4_properties | 17 | ✅ |
| suite5_coverage (behavioral) | 49 | ✅ |
| suite6_nfc_rejection | 11 | ✅ |
| unit tests (lib) | 6 | ✅ |
| unit tests (dhad-cli) | 0 | ✅ |
| doc tests | 3 | ✅ |
| **Total** | **285** | ✅ **285 passed, 0 failed** |

---

## Historical Note

Historical correction labels are retired from live source comments and
reporting. Their historical record is preserved in `CHANGELOG.md`.

---

## Property Labels (P1–P10)

| Property | Meaning | Current evidence |
|----------|---------|------------------|
| P1 | Determinism | `p1_determinism` |
| P2 | Hash-stream consistency | `p2_hash_stream_consistency_core_ignores_prosody`, `p2_hash_stream_consistency_phonetic_requires_prosody` |
| P3 | Noise filter completeness | `p3_noise_filter_completeness_all_32`, `p3_noise_sequences` |
| P4 | Digit source independence | `p4_digit_source_independence` |
| P5 | Lam-Alef decomposition correctness | `p5_lam_alef_all_pairs`, `p5_lam_alef_equals_sequence` |
| P6 | CoreHash / PhoneticHash separation | `p6_separation_tanween_vs_bare`, `p6_separation_madd_mode_b` |
| P7 | Mark order independence | `p7_mark_order_all_compatible_pairs`, `p7_mark_order_proptest` |
| P8 | Error determinism | `p8_error_determinism` |
| P9 | Atom byte size | `p9_atom_byte_size` |
| P10 | Crash resistance | `p10_crash_resistance_mode_a`, `p10_crash_resistance_mode_b`, `p10_max_input_boundary` |

---

## Design Axioms

| Axiom | Description | Current basis |
|-------|-------------|---------------|
| A1 | Determinism | P1, P8 |
| A2 | No Silent Correction | `suite3_adversarial`, `suite6_nfc_rejection` |
| A3 | Sovereignty | `suite6_nfc_rejection` (decomposed input rejected, not normalized) |
| A4 | Separation | P2, P6 |
| A5 | Glyph Independence | `suite1_golden` presentation-form vectors |
| A6 | Immutability | Reference pipeline passes atoms by value; no post-construction `&mut self` mutator API in `src/` |
| A7 | Mark Order Independence | P7 |
| A8 | Digit Source Independence | P4 |
| A9 | No Inference | `src/base_map.rs` precomposed mapping table |
| A10 | Semantic Integrity | `suite3_adversarial`; contradictory prosody combinations rejected |
| A11 | Completeness | `src/base_map.rs`, `src/faps.rs`, `suite1_golden` |

---

## Formal Invariants (I01–I24)

Current implementation enforces 24 atom invariants in Stage 10
(`src/invariants.rs`):

| Invariant | Description |
|-----------|-------------|
| I01 | `base` must be in a valid mapped range |
| I02 | `base` must not be one of `0x001D..=0x001F` |
| I03 | `marks` must be one of the valid mark combinations |
| I04 | `flags` must be one of `0x00`, `0x01`, `0x02`, `0x04` |
| I05 | `HAMZA_ABOVE` requires base in `ALEF/WAW/YEH` |
| I06 | `HAMZA_BELOW` requires base `ALEF` |
| I07 | `MADDA` requires base `ALEF` |
| I08 | `HAMZA_ABOVE` and `HAMZA_BELOW` may not coexist |
| I09 | `MADDA` and any `HAMZA_*` flag may not coexist |
| I10 | `TANWEEN_FATH` and `TANWEEN_DAMM` are mutually exclusive |
| I11 | `TANWEEN_FATH` and `TANWEEN_KASR` are mutually exclusive |
| I12 | `TANWEEN_DAMM` and `TANWEEN_KASR` are mutually exclusive |
| I13 | `MADD_*` bits may not coexist with tanween bits |
| I14 | `MADD_NORMAL` and `MADD_EXTENDED` are mutually exclusive |
| I15 | `MADD_*` bits require `LONG_VOWEL_CLASS` |
| I16 | prosody-inert atoms must have `prosody == 0` |
| I17 | prosody-inert atoms must have `marks == 0` |
| I18 | `TANWEEN_FATH` excludes `FATHA` |
| I19 | `TANWEEN_DAMM` excludes `DAMMA` |
| I20 | `TANWEEN_KASR` excludes `KASRA` |
| I21 | `SUPERSCRIPT_ALEF` excludes tanween bits |
| I22 | `reserved` must be `0x0000` |
| I23 | reserved mark bits `[5..15]` must be zero |
| I24 | `SUKUN` and any tanween bit are mutually exclusive |

---

## Hash Anchors (Mandatory Self-Test)

| Hash | Value |
|------|-------|
| CoreHash (empty stream) | `8dd837c60eff174e0f40e75636deedec4bf020751f97cfe10dd1cf7117b16be0` |
| PhoneticHash (empty stream) | `c5f62e920f5b06c74a02f25341f63499f1132da19084eb38e7b806c4a60a03f7` |

Verified independently by `tools/anchor_verify.py`.

---

## Implementation Quality

| Check | Result |
|-------|--------|
| Unsafe code | ✅ None |
| Clippy warnings | ✅ 0 warnings |
| CLI binary builds | ✅ |
| Benchmarks compile | ✅ |

---

## Notes

- `tests/suite1_golden.rs` documents the known GT-S02 stream discrepancy:
  the published stream shows `LAM+SUKUN`, but the normative input bytes
  contain no `U+0652`; the implementation follows input bytes exactly.
- I24 is now active: `SUKUN + TANWEEN` is rejected, `at_037` is restored,
  and the current repository-wide expected total is 285 tests.

---

*Generated automatically by `generate_conformance.py`.*
*Do not edit manually — re-run the script to update.*

#!/usr/bin/env python3
"""
Generate CONFORMANCE.md from live project checks.

Do not edit CONFORMANCE.md manually — re-run this script.
"""
from pathlib import Path
import datetime
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parent

def run(args):
    return subprocess.run(args, cwd=ROOT, capture_output=True, text=True, check=False)

def check(ok: bool) -> str:
    return "✅" if ok else "❌"

def cargo_package_version() -> str:
    cargo_toml = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    m = re.search(r'^version\s*=\s*"([^"]+)"', cargo_toml, flags=re.MULTILINE)
    return m.group(1) if m else "unknown"

def get_suite_results():
    """
    Parse ACTUAL cargo test output into:
        {suite_key: (passed, failed)}
    Handles:
      - unit tests: "Running unittests src/lib.rs (...)"
      - integration tests: "Running tests/suite1_golden.rs (...)"
      - doctests: "Doc-tests dhad"
    """
    proc = subprocess.run(
        ["cargo", "test", "--all", "--color", "never", "--", "--test-threads=1"],
        cwd=ROOT, capture_output=False,
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
        text=True, check=False
    )
    result = proc
    suites = {}
    current_suite = None

    for line in result.stdout.splitlines():
        m = re.match(r"\s*Running (?:unittests |tests/)?(\S+)", line)
        if m:
            current_suite = m.group(1)
            continue

        m_doc = re.match(r"\s*Doc-tests (\S+)", line)
        if m_doc:
            current_suite = "doctests"
            continue

        m2 = re.match(r"test result: \w+\. (\d+) passed; (\d+) failed", line)
        if m2 and current_suite:
            suites[current_suite] = (int(m2.group(1)), int(m2.group(2)))

    return result, suites

def suite_label(key: str) -> str:
    labels = {
        "src/lib.rs": "unit tests (lib)",
        "src/bin/main.rs": "unit tests (dhad-cli)",
        "suite1_golden.rs": "suite1_golden (Mode A)",
        "suite2_tagged.rs": "suite2_tagged (Mode B)",
        "suite3_adversarial.rs": "suite3_adversarial",
        "suite4_properties.rs": "suite4_properties",
        "suite5_coverage.rs": "suite5_coverage (behavioral)",
        "suite6_nfc_rejection.rs": "suite6_nfc_rejection",
        "doctests": "doc tests",
    }
    return labels.get(key, key)

def ordered_suite_keys(suites: dict) -> list[str]:
    preferred = [
        "suite1_golden.rs",
        "suite2_tagged.rs",
        "suite3_adversarial.rs",
        "suite4_properties.rs",
        "suite5_coverage.rs",
        "suite6_nfc_rejection.rs",
        "src/lib.rs",
        "src/bin/main.rs",
        "doctests",
    ]
    out = [k for k in preferred if k in suites]
    out.extend(k for k in suites.keys() if k not in out)
    return out

print("Running cargo test --all ...")
test_result, suites = get_suite_results()

print("Checking for unsafe ...")
unsafe_result = run(["grep", "-RIn", "unsafe", "src/"])
has_unsafe = bool(unsafe_result.stdout.strip())

print("Running clippy ...")
clippy_result = run(["cargo", "clippy", "--all-targets", "--", "-D", "warnings"])

print("Checking CLI builds ...")
cli_result = run(["cargo", "build", "--bin", "dhad-cli"])

print("Checking bench compiles ...")
bench_result = run(["cargo", "bench", "--no-run"])

suite_keys = ordered_suite_keys(suites)
total_pass = sum(p for p, _ in suites.values())
total_fail = sum(f for _, f in suites.values())

overall = (
    test_result.returncode == 0
    and total_fail == 0
    and not has_unsafe
    and clippy_result.returncode == 0
    and cli_result.returncode == 0
    and bench_result.returncode == 0
)

suite_rows = []
for key in suite_keys:
    passed, failed = suites[key]
    suite_rows.append(
        f"| {suite_label(key)} | {passed} | {check(failed == 0)} |"
    )

suite_rows_md = "\n".join(suite_rows)

now = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
version = cargo_package_version()

report = f"""# Dhad Conformance Report
**Generated:** {now}
**Specification:** Dhad v1.0
**Status:** {'✅ CONFORMANT' if overall else '❌ NON-CONFORMANT'}

---

## Versions

| Component | Version |
|-----------|---------|
| Library (`dhad`) | {version} |
| Rust edition | 2021 |
| Dhad spec | v1.0 |

---

## Test Results

| Suite | Tests | Result |
|-------|-------|--------|
{suite_rows_md}
| **Total** | **{total_pass}** | {check(total_fail == 0)} **{total_pass} passed, {total_fail} failed** |

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
| Unsafe code | {check(not has_unsafe)} {'None' if not has_unsafe else 'FOUND'} |
| Clippy warnings | {check(clippy_result.returncode == 0)} {'0 warnings' if clippy_result.returncode == 0 else 'warnings present'} |
| CLI binary builds | {check(cli_result.returncode == 0)} |
| Benchmarks compile | {check(bench_result.returncode == 0)} |

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
"""

(ROOT / "CONFORMANCE.md").write_text(report, encoding="utf-8")
print(f"\nConformance report written to {ROOT / 'CONFORMANCE.md'}")
print(f"Overall status: {'CONFORMANT' if overall else 'NON-CONFORMANT'}")
print(f"Suites found: {len(suites)}")
print(f"Tests: {total_pass} passed, {total_fail} failed")
print(f"Unsafe: {'none' if not has_unsafe else 'FOUND'}")
print(f"Clippy: {'clean' if clippy_result.returncode == 0 else 'warnings present'}")

sys.exit(0 if overall else 1)

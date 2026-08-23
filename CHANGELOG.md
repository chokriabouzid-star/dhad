# CHANGELOG — Dhad (ضاد) Reference Implementation

All notable changes to this project are documented here, strictly bound to the historical git tags and verified commit hashes.

---

## [v1.2.3] — 2026-08-21
### Added
- **Invariant I24 Integration:** Enforced mutual exclusion between `SUKUN` and `TANWEEN_*` on the same atom across both Rust core and Python references.
- **Adversarial Corpus Expansion:** Registered and exported the live adversarial test vector `at_037_sukun_plus_tanween`.
- **Dynamic Diagnostics:** Refactored `examples/export_vectors.rs` to compute file vector counts dynamically using `.len()` instead of hardcoded numbers.

### Fixed
- **Python Ref Parity:** Closed the parity gap on `I24` in both Mode A and Mode B execution paths of `python_ref/dhad_ref.py`.
- **Cargo.toml Health:** Added `rust-version = "1.75"` to declare the minimum supported Rust compiler explicitly.

---

## [v1.2.2] — 2026-08-21
### Added
- **Automated Governance Checks:** Added `tools/verify_plan_status.py` and consolidated check `check_11_doc_stats_parity` to automatically monitor document version drift and vector statistics discrepancy.
- **Formatting Lock:** Added automatic formatting checks and applied systematic code-base format realignment.

---

## [v1.2.1] — 2026-08-20
### Added
- **Invariant I25:** Enforced that reserved prosody bits 6–7 (mask `0xC0`) must be zero on all atoms.
- **Corpus Expansion:** Synced specifications and verifiers to 187 active vectors.

---

## [v1.2.0] — 2026-06-15
### Added
- **Phase 2 Bootstrap:** Officially exported the canonical test suites into the cross-language public conformance corpus (`golden.json`, `adversarial.json`, `tagged.json` with 185 initial vectors).
- **Python Reference Implementation:** Shipped the independent, dependency-free Python reference implementation (`dhad_ref.py`) with 100% exact stream and error object parity.

---

## [v1.1.2] — 2026-06-13
### Added
- **Test Suite 5 & 6 Reinforcement:** Converted raw code coverage probes into deterministic empirical behavioral proofs.
- **System Handoff:** Shipped `HANDOFF.md` to formalize repository handover and lock stable development state.

---

## [v1.1.1] — 2026-06-12
### Fixed
- **Packaging:** Updated the cargo packaging configuration to explicitly include and ship the `CONFORMANCE.md` document to crates.io.

---

## [v1.1.0] — 2026-06-12
### Added
- **Honesty Documentation:** Shipped the "Honesty Clause" under `specification.md` explicitly detailing what the engine does *not* do yet (e.g., relaxed Quranic profiles, malformed frame recovery).

---

## [v1.0.1] — 2026-06-06
### Fixed
- **Metadata:** Restored valid repository links in `Cargo.toml` and synced transitive dependency lockfiles.

---

## [v1.0.0] — 2026-06-06
### Added
- **Initial Stable Release:** Fully stable core implementation of the 12-stage Dhad Arabic Text Canonicalization Pipeline.
- **Validation Engine:** Fully implemented the 23 baseline invariants (I01–I23).
- **Cryptographic Hashes:** Implemented deterministic `CoreHash` (SHA-256) and `PhoneticHash` (SHA-256 over CoreHash + prosody).

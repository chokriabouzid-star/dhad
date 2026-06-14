# Dhad — Project Handoff Document (v3)

**As of:** End of session that shipped `v1.1.2`
**Owner:** Chokri Abouzid
**Repository:** https://github.com/chokriabouzid-star/dhad
**Crate:** https://crates.io/crates/dhad
**Latest published version:** `v1.1.2`

---

## 1) Project summary

Dhad is a deterministic Arabic text canonicalization library in Rust.

It converts Arabic Unicode input into a canonical `AtomStream` of fixed
8-byte atoms, then derives two domain-separated SHA-256 hashes:

- **CoreHash** — orthographic identity (base + marks + flags)
- **PhoneticHash** — prosodic identity (CoreHash + prosody)

Two modes:

- **Mode A** — UTF-8 Arabic text input
- **Mode B** — tagged binary frame (for pre-annotated MADD-style content)

Design axioms: A1, A2, A3, A5, A6, A7.

---

## 2) Current released state

| Version | Status     | Notes                                                 |
|---------|------------|-------------------------------------------------------|
| 1.0.0   | published  | legacy                                                |
| 1.0.1   | published  | legacy                                                |
| 1.1.0   | published  | hardening + honesty docs                              |
| 1.1.1   | published  | packaging fix (ships `CONFORMANCE.md`)                |
| 1.1.2   | published  | test-contract reinforcement (suite5 + NFC suite6)     |

Repository invariants currently maintained:

- `cargo fmt --check` clean
- `cargo clippy --all-targets` clean
- `cargo test` all green
- `cargo publish --dry-run` clean
- No `unsafe`
- Stable Rust; nightly only required for `cargo-fuzz`

---

## 3) What has been completed (cumulative)

### 3.1 ✅ v1.0.2 — Bug fixes (fully done)
- Reject duplicate prosody marks on the same atom
- `checked_mul` / `checked_add` in Mode B length math
- Centralize `MAX_INPUT_BYTES` in `src/constants.rs`
- Remove duplicate `gt_087` golden test
- Clean up self-contradicting `MalformedUtf8` comment in Mode B

### 3.2 ✅ v1.1.0 — Honesty documentation (mostly done)
- NFC policy decided (Option B: strict NFC profile documented)
- `Known Limitations` section in README
- Removed `quran` keyword, added `arabic-script`
- Documented identity stages explicitly
- Updated `src/lib.rs` top-level docs

### 3.3 ✅ v1.1.1 — Packaging fix
- Track `CONFORMANCE.md` in git
- Drop `CONFORMANCE.md` from `Cargo.toml` exclude
- Bump to 1.1.1, publish to crates.io, GitHub release created

### 3.4 ✅ v1.1.2 — Test-contract reinforcement
- `tests/suite5_coverage.rs` rewritten from coverage probes into
  behavioral assertions:
  - exact `ErrorKind` variants
  - exact atom counts
  - exact base ids and flag bits
  - exact 8-byte little-endian wire format
- New file `tests/suite6_nfc_rejection.rs`:
  - bare `U+0653 / U+0654 / U+0655` → `UnmappedCodepoint`
  - decomposed `ALEF + U+0653`, `ALEF + U+0654`, `ALEF + U+0655`,
    `WAW + U+0654`, `YEH + U+0654` → `UnmappedCodepoint` at the
    correct position
  - precomposed `U+0622 / U+0623 / U+0625` still accepted
- Mode B structural-error tests now pin the v1.x
  `MalformedUtf8 { byte_offset }` contract so the future
  `MalformedFrame` migration cannot happen silently.
- `benches/throughput.rs` fixed to cut Arabic input on UTF-8
  codepoint boundaries (no library change).
- Updated `README.md` to note the NFC contract is now enforced
  by `tests/suite6_nfc_rejection.rs`.
- Updated `CONFORMANCE.md` Test Results table.

---

## 4) Anchor conformance — first independent verification

At the end of the v1.1.2 session, an independent Python reference was
executed against the Rust implementation for a small **anchor set**.
The Python code (`tools/anchor_verify.py`) reimplements `CoreHash` and
`PhoneticHash` directly from the Dhad Specification v1.0 (+ CR-01..CR-07),
without using the Rust crate.

Result:

| Anchor       | Description     | CoreHash | PhoneticHash |
|--------------|-----------------|----------|--------------|
| ANCHOR-001   | empty stream    | OK       | OK           |
| ANCHOR-002   | ALEF bare       | OK       | OK           |
| ANCHOR-003   | BEH bare        | OK       | OK           |
| ANCHOR-004   | BEH + FATHA     | OK       | OK           |

Significance:

- The Rust output is no longer only self-consistent.
- A spec-only Python reimplementation reproduces the same hashes.
- This is the project's first true cross-implementation conformance
  evidence, even before the full `dhad-conformance-suite` is built.

This anchor set will be the unshakeable seed of every future vector
file in the conformance suite: every new vector must remain consistent
with these four anchors.

---

## 5) What is NOT yet done

### From the original v1.1.x plan
- Nothing remains. v1.1.x is now fully closed.

### From v1.2.0 (next milestone — not started)
- New repository: `dhad-conformance-suite`
  - `vectors/golden.json`
  - `vectors/adversarial.json`
  - `vectors/tagged.json`
  - Language-agnostic format
- Minimal **Python** reference implementation (~200 lines) that:
  - parses the same vectors
  - reproduces atom streams, CoreHash, PhoneticHash, error classes
- Goal: convert “self-consistency regression” into true
  cross-implementation conformance.

### From v1.3.0 — not started
- `process_mode_a_with_profile(input, profile)`
- `enum Profile { Strict, QuranicRelaxed }`
- `QuranicRelaxed` returns `DhadResult + Vec<IgnoredMark { cp, offset }>`
- Re-introduce `quran` keyword only at this version.

### From v2.0.0 — not started
- `ErrorKind::MalformedFrame { byte_offset, reason }`
- Consider `#[non_exhaustive]` on `ErrorKind`
- Begin **DhadIR** (word-level IR with `DHAD-WORD-V1` hash)
- Begin **Wethaq** (signing / verification layer)

---

## 6) Open GitHub Issues (authoritative roadmap)

1. Support decomposed `U+0653/U+0654/U+0655` or formalize strict NFC input profile
2. Introduce dedicated `MalformedFrame` error in v2.0
3. Strengthen `suite5` coverage tests with behavioral assertions
   *(Issue can now be closed — addressed in v1.1.2.)*
4. Create independent conformance suite and second implementation
   *(This is the v1.2.0 milestone.)*

---

## 7) Known limitations (still valid in v1.1.2)

These are documented, intentional, and now enforced by tests:

- Input must be **NFC-precomposed** Arabic.
- `U+0653`, `U+0654`, `U+0655` are **not mapped** → `UnmappedCodepoint`.
  *(Enforced by `tests/suite6_nfc_rejection.rs`.)*
- Extended Quranic recitation/pause marks (e.g. `U+06D6–U+06ED`) are
  **out of scope** for strict Mode A in v1.x.
- Mode B structural errors are reported as
  `MalformedUtf8 { byte_offset }` for v1.x API compatibility.
  *(Pinned by `tests/suite5_coverage.rs` Mode B tests.)*
- Some specification stages (7, 8, 9 resolve, 11, 12) are
  **identity stages** in v1.x because the work happens earlier.

These should not be treated as bugs in v1.x.

---

## 8) Agreed plan for the next session

The next session begins **v1.2.0**.
Do not skip steps.

### Step 1 — Define vector format
- Design a JSON schema for vectors:
  - `name`
  - `mode`            (`"A"` or `"B"`)
  - `input_hex`       (bytes for Mode A or full frame for Mode B)
  - `expected_result` (`"ok"` or `"err"`)
  - if `ok`:
    - `stream_hex`    (atom stream bytes)
    - `core_hash_hex`
    - `phonetic_hash_hex`
  - if `err`:
    - `error_kind`    (`"MalformedUtf8" | "UnmappedCodepoint" | ...`)
    - optional fields (`codepoint`, `position`, `atom_index`, etc.)

### Step 2 — Create `dhad-conformance-suite` repository
- New repo (separate from `dhad`).
- Initial vector files:
  - `vectors/golden.json`
  - `vectors/adversarial.json`
  - `vectors/tagged.json`
- README explicitly stating: language-agnostic conformance vectors.

### Step 3 — Generate vectors from the Rust implementation
- Add a tiny exporter (binary or test) inside `dhad` that emits
  vectors in the JSON schema from the current test inputs.
- The Rust implementation **produces** the vectors; the conformance
  suite then becomes the canonical artifact for everyone else.

### Step 4 — Minimal Python reference implementation
- ~200 lines, no external dependencies beyond `hashlib`.
- Loads the JSON vectors.
- Computes:
  - atom stream
  - `CoreHash`
  - `PhoneticHash`
  - error classification
- Asserts equivalence with the Rust output.

### Step 5 — Cross-validate
- Run both implementations against the same vector set.
- Lock the result; if they ever disagree, that is a spec or
  implementation bug.

When Steps 1–5 are complete, the README wording can move from:
> self-consistency regression suite

to:
> independently verified conformance against a second implementation.

That is the first real *conformance* milestone of the project.

### Step 6 — Then (and only then) release v1.2.0
- Bump version, update README to reference the conformance repo,
  publish on crates.io, tag, GitHub release.

---

## 9) Engineering principles to preserve

- One branch per intent.
- Bug fixes, documentation, and formatting are never mixed.
- `--no-ff` merges, so history shows intent.
- Failing test first, then fix.
- crates.io publishing is irreversible — always `--dry-run` first.
- Documentation lies are bugs and are treated as such.
- Spec, tests, and code must agree. When they disagree, the spec
  is amended explicitly (no silent reinterpretation).

---

## 10) How to resume in the next chat session

Open a new chat and paste this opening message:

> I am continuing work on **Dhad** (https://crates.io/crates/dhad).
> Latest published version: **1.1.2**.
> Repository: https://github.com/chokriabouzid-star/dhad
> I will paste the project handoff document below.
> Then I want to start **v1.2.0**: the independent conformance suite
> and the Python reference implementation, as described in Section 7.

Then paste this entire handoff document immediately after.

---

## 11) Final note

After v1.1.2 the v1.x line is **fully closed and consistent**:

- Bugs found in earlier sessions are fixed.
- Limitations are documented in README and enforced in tests.
- Test files no longer claim to test what they don't actually test.
- The published crate, the GitHub repo, the tags, the releases, and
  the conformance file all agree with each other.

v1.2.0 is no longer about saving v1.x. It is about **earning the
right to use the word *conformance* without quotes**, by adding a
second, independent implementation.

End of handoff document.

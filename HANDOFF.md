Dhad — Project Handoff Document
As of: End of session that shipped v1.1.0 and v1.1.1
Owner: Chokri Abouzid
Repository: https://github.com/chokriabouzid-star/dhad
Crate: https://crates.io/crates/dhad

1) What this project is
Dhad is a deterministic Arabic text canonicalization library in Rust.

It converts Arabic Unicode input into a canonical AtomStream of fixed 8-byte atoms and produces two independent SHA-256 hashes:

CoreHash — orthographic identity (base + marks + flags)
PhoneticHash — prosodic identity (CoreHash + prosody layer)
Design axioms:

A1 Determinism
A2 No silent correction
A3 Hash separation
A5 Glyph independence
A6 Mark order independence
A7 Digit source independence
Two processing modes:

Mode A — UTF-8 Arabic text
Mode B — tagged binary frame (for MADD-annotated input)
2) Current state (at handoff)
Published versions
v1.0.0 — initial release (legacy)
v1.0.1 — minor update (legacy)
v1.1.0 — hardening + honesty docs
v1.1.1 — packaging fix (ships CONFORMANCE.md)
Repository state
Default branch: main
All work merged
All tags pushed
All releases created on GitHub
crates.io reflects v1.1.1
Quality gates
cargo fmt --check clean
cargo clippy --all-targets clean
cargo test all green
cargo publish --dry-run clean
No unsafe
No nightly required (fuzzing only)
3) What was accomplished in the last session
3.1 Bug fixes (P0)
Reject duplicate prosody marks on the same atom
(previously: silent |= collapse — violated A2)
Hardened Mode B frame length math with checked_mul and checked_add
Centralized MAX_INPUT_BYTES into src/constants.rs
Removed a duplicate golden test vector (gt_087 ≡ gt_084)
Cleaned up self-contradicting comment around MalformedUtf8 in Mode B
3.2 Honesty / documentation
Added Known Limitations section to README.md:
NFC-oriented input profile
U+0653, U+0654, U+0655 are not mapped in v1.x
Extended Quranic marks (U+06D6–U+06ED etc.) are out of scope for strict Mode A
Mode B structural errors are reported as MalformedUtf8 { byte_offset } in v1.x
Some pipeline stages are identity stages in v1.x
Reframed conformance language honestly:
vectors are a self-consistency regression suite
independent cross-implementation conformance is planned
Removed quran from crate keywords, added arabic-script
Updated top-level lib.rs docs to mirror the new contract
3.3 Packaging
Removed CONFORMANCE.md from .gitignore
Removed CONFORMANCE.md from Cargo.toml exclude list
Confirmed the published 1.1.1 package contains 40 files (vs 39 before)
3.4 Process hygiene
Used focused branches:
fix/v1.0.2-p0
docs/v1.1-honesty
fix/conformance-file-shipping
All branches merged with --no-ff
Clear commit messages
GitHub Releases for v1.1.0 and v1.1.1
4 GitHub Issues opened to formalize the roadmap
4) Open GitHub Issues (roadmap as of handoff)
Support decomposed U+0653/U+0654/U+0655 or formalize strict NFC input profile
Introduce dedicated MalformedFrame error in v2.0
Strengthen suite5 coverage tests with behavioral assertions
Create independent conformance suite and second implementation
These are the authoritative remaining engineering items at handoff.

5) Known limitations (carried forward)
These are intentional or documented limitations as of v1.1.1:

Input must be precomposed (NFC-oriented profile).
Decomposed combining marks U+0653, U+0654, U+0655 are unmapped → UnmappedCodepoint.
Extended Quranic recitation/pause marks are out of scope for Mode A in v1.x.
Mode B structural errors are reported as MalformedUtf8 { byte_offset } for v1.x API compatibility.
Some specification stages (7, 8, 9 resolve, 11, 12) are identity stages because the work happens earlier in the pipeline.
These should not be treated as bugs in v1.x.
They are tracked items, see Section 4.

6) Things explicitly NOT done yet
No Python (or other) reference implementation exists.
No language-agnostic conformance repository exists.
No HN / X / Reddit post has been published.
No DhadIR (word-level IR) has been started.
No Wethaq (signing layer) has been started.
No Quranic-relaxed processing profile has been implemented.
7) Recommended next steps (priority-ordered)
This is the suggested resumption plan for the next session.

Priority 1 — Communication
Draft and publish an announcement post:
HN
r/rust
X / Mastodon
Tone: honest, no overclaiming, foreground the “Known Limitations” section.
Mention 4 open Issues as the public roadmap.
Priority 2 — Strengthen test integrity
Address GitHub Issue #3 (suite5 behavioral assertions).
Low risk, high value, no API impact.
Priority 3 — Decide NFC posture formally
Address GitHub Issue #1.
Either:
Codify “strict NFC profile” as a formal spec rule with explicit rejection tests, OR
Add support for U+0653/U+0654/U+0655 with explicit mapping into existing flags.
Priority 4 — Start the next layer (only after the above)
Begin DhadIR:
Word-level IR built on top of AtomStream
New domain-separated hash, e.g. DHAD-WORD-V1
Spec-first, code second
This is the bridge toward the larger vision (research over Quranic / Hadith / Arabic corpora).
Priority 5 — Long-term credibility
Address GitHub Issue #4:
Independent conformance repo
Minimal second implementation (Python is fine)
This is what converts the project from “well-engineered crate” into “reference implementation of a public spec.”
Priority 6 — Major version
Plan v2.0.0:
Introduce ErrorKind::MalformedFrame (GitHub Issue #2)
Consider marking ErrorKind as #[non_exhaustive]
Any other breaking cleanups
8) Engineering principles to keep
These were reinforced in the last session and should be preserved:

Branch per intent. Never mix bug fixes with documentation with formatting.
--no-ff merges for visible history.
Failing test first, then fix.
Run cargo fmt --check, cargo clippy --all-targets, cargo test, and cargo publish --dry-run before any tag/release.
crates.io publishing is irreversible — always dry-run first.
Documentation lies are bugs. Treat them with the same seriousness.
9) How to resume in the next session
Open a new chat and paste exactly this opening message:

I am continuing work on Dhad (https://crates.io/crates/dhad).
Latest published version: 1.1.1.
Repository: https://github.com/chokriabouzid-star/dhad
Please refer to the project handoff document I will paste below.
Then I will tell you which next step I want to take from Section 7.

Then paste this entire document as a follow-up message.

That alone will be enough for the next assistant (or future you) to instantly take over with full context.

10) Final note
The project is now in a clean, defensible, and honest state.
What used to be marketed beyond its actual scope is now explicitly bounded.
What used to be silently incorrect (duplicate prosody, frame length math) is now strictly enforced.
What used to be invisible (CONFORMANCE.md) is now properly shipped.

This is no longer just code.
It is a maintained, versioned, documented public artifact.

The next steps are choices, not emergencies.


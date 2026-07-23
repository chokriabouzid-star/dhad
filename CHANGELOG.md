# CHANGELOG

## Post-release correction (2026-07-23)

Commit `5b0c325` ("ci: regenerate Cargo.msrv.lock against current
Cargo.toml") carried a slightly inaccurate rationale in its message:
`Cargo.toml` did **not** change between commits `3982e0f` and `5b0c325`,
and has in fact been untouched since `a706066` (release v1.2.0).

The actual, verified reason for regenerating `Cargo.msrv.lock` was
different: the file first committed in `3982e0f` was produced inside a
separate working copy (`~/dhad-msrv-test`), which resolved the identical
`Cargo.toml` to a different transitive dependency set (93 packages, no
`rayon`) than `~/dhad` itself did on Rust 1.75.0 (115 packages, with
`rayon`). `cargo test --all --locked` in CI correctly refused that
mismatch. The regeneration in `5b0c325` was performed *inside `~/dhad`*
to align the committed lockfile with the actual project directory, and
the fourth CI run then passed 285/285 with `--locked` as expected.

The implementation plan's Fix 1.3 has since been updated to state
explicitly, as its first step, that `Cargo.msrv.lock` must always be
generated inside the real project directory itself — never copied in
from a separate working copy, even one with identical `Cargo.toml`
content. This closes the class of drift that produced this correction.


## Unreleased

- Retired the historical `CR-01`..`CR-07` correction labels from live
  source comments and reporting. The rules themselves remain in force;
  only the citation style changed.
- Activated invariant **I24**: `SUKUN` and any `TANWEEN_*` bit are now
  mutually exclusive on the same atom.
- Restored adversarial test `at_037_sukun_plus_tanween`.
- Repository-wide total test count increased from **284** to **285**.

## Historical corrections folded into v1.0

| Historical label | Summary | Current home |
|------------------|---------|--------------|
| CR-01 | Reserved Base IDs `0x001D..=0x001F` explicitly rejected | I02 |
| CR-02 | MADD bits accepted only through Mode B / long-vowel rules | §6.2, I15 |
| CR-03 | Hash values independently verified | `tools/anchor_verify.py`, conformance workflow |
| CR-04 | `MAX_INPUT_BYTES = 4,194,304` enforced | pipeline pre-stage |
| CR-05 | P2 means hash-stream consistency, not idempotency | Property list / test suite |
| CR-06 | `U+0670` after a prosody-inert atom rejected | I16 |
| CR-07 | Mode B nonzero `reserved` rejected | I22 |
| Former CR-08 candidate | `SUKUN + TANWEEN` gap closed | I24 |


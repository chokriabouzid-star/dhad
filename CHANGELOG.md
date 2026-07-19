# CHANGELOG

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


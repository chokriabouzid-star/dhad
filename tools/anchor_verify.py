#!/usr/bin/env python3
"""
Dhad — Anchor Conformance Verifier
----------------------------------
Independent re-implementation of Dhad's CoreHash and PhoneticHash
following only the published specification (Dhad Implementation
Specification v1.0 + corrections CR-01..CR-07).

This file does NOT depend on the Rust crate. It is intentionally
written from the spec alone, so that a successful run constitutes
true cross-implementation conformance, not self-consistency.

The Rust crate is checked against the constants below; if they
ever diverge, either the Rust implementation or the specification
has changed and must be reconciled.

Run:
    python3 tools/anchor_verify.py
"""

import hashlib
import struct
import sys


def core_hash(atoms):
    """
    SHA-256(
        b"DHAD-CORE-V1"
        || LE_u32(n_atoms)
        || for each atom:
               LE_u16(base) || LE_u16(marks) || u8(flags)
    )
    """
    h = hashlib.sha256()
    h.update(b"DHAD-CORE-V1")
    h.update(struct.pack("<I", len(atoms)))
    for a in atoms:
        h.update(struct.pack("<H", a["base"]))
        h.update(struct.pack("<H", a["marks"]))
        h.update(bytes([a["flags"]]))
    return h.hexdigest()


def phonetic_hash(atoms, core_hex):
    """
    SHA-256(
        b"DHAD-PROSODY-V1"
        || raw_core_hash_32_bytes
        || LE_u32(n_atoms)
        || for each atom: u8(prosody)
    )
    """
    h = hashlib.sha256()
    h.update(b"DHAD-PROSODY-V1")
    h.update(bytes.fromhex(core_hex))
    h.update(struct.pack("<I", len(atoms)))
    for a in atoms:
        h.update(bytes([a["prosody"]]))
    return h.hexdigest()


ANCHORS = [
    {
        "id":   "ANCHOR-001",
        "name": "empty stream",
        "atoms": [],
        "rust_core":
            "8dd837c60eff174e0f40e75636deedec4bf020751f97cfe10dd1cf7117b16be0",
        "rust_phonetic":
            "c5f62e920f5b06c74a02f25341f63499f1132da19084eb38e7b806c4a60a03f7",
    },
    {
        "id":   "ANCHOR-002",
        "name": "ALEF bare",
        "atoms": [
            {"base": 0x0001, "marks": 0x0000, "flags": 0x00, "prosody": 0x00},
        ],
        "rust_core":
            "68d32b955388e186a3ad963008c4aed8f9d957d9fe72ad0e29ad5012d57e140d",
        "rust_phonetic":
            "984a596fe5175c6413a180a8d1f09891fb53675f5a8b9daac5a1dd4a2ea784d0",
    },
    {
        "id":   "ANCHOR-003",
        "name": "BEH bare",
        "atoms": [
            {"base": 0x0002, "marks": 0x0000, "flags": 0x00, "prosody": 0x00},
        ],
        "rust_core":
            "4cd5488d16f55023d7a6816009777bac5297dbb57a0f5315085693a1dfb438ac",
        "rust_phonetic":
            "2e5317b842f738a15e3aaf04cb527e61cb7979a44ac1c99f6a4b464fb50056a3",
    },
    {
        "id":   "ANCHOR-004",
        "name": "BEH + FATHA",
        "atoms": [
            {"base": 0x0002, "marks": 0x0001, "flags": 0x00, "prosody": 0x00},
        ],
        "rust_core":
            "f4226a79f1c62998559c44298ad718388045dc6cd5c096a9bc197175268d2a04",
        "rust_phonetic":
            "97b6eb001215607e9e8424d96893106a648b21f345d06c8819591a481c2b6ec8",
    },
]


def main() -> int:
    print(f"{'ID':<12} {'NAME':<16} {'CORE':<8} {'PHONETIC':<8}")
    print("-" * 50)
    all_ok = True
    for v in ANCHORS:
        c = core_hash(v["atoms"])
        p = phonetic_hash(v["atoms"], c)
        core_ok = (c == v["rust_core"])
        pho_ok = (p == v["rust_phonetic"])
        print(
            f"{v['id']:<12} {v['name']:<16} "
            f"{'OK' if core_ok else 'FAIL':<8} "
            f"{'OK' if pho_ok else 'FAIL':<8}"
        )
        if not core_ok:
            print(f"   manual core     = {c}")
            print(f"   rust   core     = {v['rust_core']}")
        if not pho_ok:
            print(f"   manual phonetic = {p}")
            print(f"   rust   phonetic = {v['rust_phonetic']}")
        all_ok = all_ok and core_ok and pho_ok

    print()
    if all_ok:
        print("ALL ANCHORS MATCH")
        return 0
    else:
        print("DIVERGENCE DETECTED")
        return 1


if __name__ == "__main__":
    sys.exit(main())

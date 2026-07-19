#!/usr/bin/env bash
# Dhad Self-Test — verifies mandatory anchor constants
# Correct redirection: stderr carries text, stdout carries binary atoms
# Usage: ./self_test.sh [binary_path]

set -euo pipefail

BINARY="${1:-./target/release/dhad-cli}"

if [ ! -x "$BINARY" ]; then
    echo "❌ Binary not found or not executable: $BINARY"
    exit 1
fi

echo '=== Dhad Self-Test ==='
echo "Binary: $BINARY"
echo

# Helper: run binary, discard stdout (binary atoms), capture stderr text only.
# CRITICAL: order matters — "2>&1 >/dev/null" means:
#   1. redirect stderr → current stdout (the pipe)
#   2. redirect stdout → /dev/null
# Result: pipe receives stderr; /dev/null receives binary atoms.
dhad_stderr() {
    printf '%b' "$1" | "$BINARY" 2>&1 >/dev/null || true
}

PASS=0
FAIL=0

check() {
    local label="$1" got="$2" expected="$3"
    if [ "$got" = "$expected" ]; then
        echo "✅ $label: PASS"
        PASS=$((PASS+1))
    else
        echo "❌ $label: FAIL"
        echo "   got:      '$got'"
        echo "   expected: '$expected'"
        FAIL=$((FAIL+1))
    fi
}

# ─── Test 1: Empty input → CoreHash anchor ─────────────────────────────
OUT=$(dhad_stderr '')
check "Empty CoreHash" \
    "$(echo "$OUT" | grep 'core:'     | awk '{print $2}')" \
    "8dd837c60eff174e0f40e75636deedec4bf020751f97cfe10dd1cf7117b16be0"

# ─── Test 2: Empty input → PhoneticHash anchor ─────────────────────────
check "Empty PhoneticHash" \
    "$(echo "$OUT" | grep 'phonetic:' | awk '{print $2}')" \
    "c5f62e920f5b06c74a02f25341f63499f1132da19084eb38e7b806c4a60a03f7"

# ─── Test 3: NOON bare → CoreHash ─────────────────────────────────────
OUT=$(dhad_stderr '\xd9\x86')
check "NOON CoreHash" \
    "$(echo "$OUT" | grep 'core:' | awk '{print $2}')" \
    "4163ae2243aed7a756f03504fc966e69677299245331d6381be8c45b60511019"

# ─── Test 4: ALEF bare → CoreHash ─────────────────────────────────────
OUT=$(dhad_stderr '\xd8\xa7')
check "ALEF CoreHash" \
    "$(echo "$OUT" | grep 'core:' | awk '{print $2}')" \
    "68d32b955388e186a3ad963008c4aed8f9d957d9fe72ad0e29ad5012d57e140d"

# ─── Test 5: Digit '5' ASCII → CoreHash (A8) ──────────────────────────
OUT=$(printf '5' | "$BINARY" 2>&1 >/dev/null || true)
check "Digit 5 ASCII CoreHash (A8)" \
    "$(echo "$OUT" | grep 'core:' | awk '{print $2}')" \
    "de0abdb12eda178b594dddca3646466589a24f95799e4fe839b154a9c0a407d5"

# ─── Test 6: بِسْمِ → atom count = 3 ────────────────────────────────
OUT=$(dhad_stderr '\xd8\xa8\xd9\x90\xd8\xb3\xd9\x92\xd9\x85\xd9\x90')
ATOMS=$(echo "$OUT" | grep 'atoms:' | awk '{print $2}')
check "Bismi atom count = 3" "$ATOMS" "3"

# ─── Test 7: invalid UTF-8 → non-zero exit ─────────────────────────────
set +e
printf '\xC1\x41' | "$BINARY" >/dev/null 2>/dev/null
EXIT_CODE=$?
set -e
if [ "$EXIT_CODE" -ne "0" ]; then
    echo "✅ Invalid UTF-8 → exit($EXIT_CODE) non-zero: PASS"
    PASS=$((PASS+1))
else
    echo "❌ Invalid UTF-8 must return non-zero exit"
    FAIL=$((FAIL+1))
fi

# ─── Summary ────────────────────────────────────────────────────────────
echo
echo "=== Results: $PASS passed, $FAIL failed ==="

if [ "$FAIL" -eq "0" ]; then
    echo "=== All self-tests PASSED ($PASS/$((PASS+FAIL))) ==="
    exit 0
else
    echo "=== FAILED: $FAIL test(s) did not pass ==="
    exit 1
fi

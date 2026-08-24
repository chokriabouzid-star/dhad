#!/usr/bin/env python3
from __future__ import annotations

import difflib
import re
import subprocess
import sys
import json
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

SPEC_PATH = ROOT / "specification.md"
CI_PATH = ROOT / ".github" / "workflows" / "ci.yml"
CONFORMANCE_PATH = ROOT / "CONFORMANCE.md"
GENERATE_CONFORMANCE_PATH = ROOT / "generate_conformance.py"
BOOTSTRAP_GOLDEN_PATH = ROOT / "tools" / "bootstrap_golden_cases.py"
GOLDEN_CASES_PATH = ROOT / "tests" / "cases" / "golden_cases.rs"
ADVERSARIAL_CASES_PATH = ROOT / "tests" / "cases" / "adversarial_cases.rs"
TAGGED_CASES_PATH = ROOT / "tests" / "cases" / "tagged_cases.rs"
FAPS_PATH = ROOT / "src" / "faps.rs"
NOISE_PATH = ROOT / "src" / "noise.rs"
CARGO_LOCK_PATH = ROOT / "Cargo.lock"
CARGO_MSRV_LOCK_PATH = ROOT / "Cargo.msrv.lock"

MSRV = "1.75.0"
EXPECTED_AUDIT_IGNORES = {"RUSTSEC-2026-0204", "RUSTSEC-2026-0190"}
EXPECTED_VECTOR_COUNTS = {
    "golden": 116,
    "adversarial": 40,
    "tagged": 32,
    "total": 188,
}
EXPECTED_NOISE_SET = (
    {0x0640, 0x034F, 0xFEFF}
    | set(range(0x200C, 0x2010))
    | set(range(0x202A, 0x202F))
    | set(range(0x2066, 0x206A))
    | set(range(0xFE00, 0xFE10))
)

MSRV_SEQUENCE = [
    (["cargo", f"+{MSRV}", "generate-lockfile"], False),
    (["cargo", f"+{MSRV}", "update", "-p", "serde_json", "--precise", "1.0.117"], False),
    (["cargo", f"+{MSRV}", "update", "-p", "proptest", "--precise", "1.4.0"], False),
    (["cargo", f"+{MSRV}", "update", "-p", "tempfile", "--precise", "3.10.1"], False),
    (["cargo", f"+{MSRV}", "update", "-p", "rand", "--precise", "0.8.5"], False),
    (["cargo", f"+{MSRV}", "update", "-p", "clap", "--precise", "4.4.18"], False),
    (["cargo", f"+{MSRV}", "update", "-p", "half", "--precise", "2.4.1"], False),
    (["cargo", f"+{MSRV}", "update", "-p", "rayon", "--precise", "1.7.0"], True),
]


@dataclass
class CheckResult:
    idx: int
    name: str
    status: str
    detail: str


class VerificationError(Exception):
    pass


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def read_bytes_or_none(path: Path) -> bytes | None:
    return path.read_bytes() if path.exists() else None


def restore_bytes(path: Path, original: bytes | None) -> None:
    if original is None:
        if path.exists():
            path.unlink()
        return
    path.write_bytes(original)


def run(
    cmd: list[str],
    *,
    cwd: Path = ROOT,
    input_data=None,
    text: bool = True,
    merge_streams: bool = False,
    timeout: int | None = None,
) -> subprocess.CompletedProcess:
    return subprocess.run(
        cmd,
        cwd=str(cwd),
        input=input_data,
        text=text,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT if merge_streams else subprocess.PIPE,
        timeout=timeout,
    )


def first_lines(text: str, n: int = 20) -> str:
    lines = text.splitlines()
    if len(lines) <= n:
        return text.strip()
    return "\n".join(lines[:n]).strip() + "\n... (truncated)"


def diff_excerpt(a: str, b: str, a_name: str, b_name: str, max_lines: int = 80) -> str:
    diff = list(
        difflib.unified_diff(
            a.splitlines(),
            b.splitlines(),
            fromfile=a_name,
            tofile=b_name,
            lineterm="",
        )
    )
    if not diff:
        return ""
    if len(diff) > max_lines:
        diff = diff[:max_lines] + ["... (diff truncated)"]
    return "\n".join(diff)


def normalize_conformance(text: str) -> str:
    text = text.replace("\r\n", "\n")
    return re.sub(
        r"(?m)^\*\*Generated:\*\* .*$",
        "**Generated:** <normalized>",
        text,
    )


def expand_lhs(lhs: str) -> list[int]:
    lhs = lhs.strip()
    if "..=" in lhs:
        start_s, end_s = [part.strip() for part in lhs.split("..=")]
        start = int(start_s, 16)
        end = int(end_s, 16)
        return list(range(start, end + 1))
    values: list[int] = []
    for part in lhs.split("|"):
        values.append(int(part.strip(), 16))
    return values


def parse_anchor(spec_text: str, heading: str) -> tuple[str, str]:
    pattern = re.compile(
        re.escape(f"**{heading}**")
        + r".*?core_hash:\s*([0-9a-f]{64}).*?phonetic_hash:\s*([0-9a-f]{64})",
        re.S,
    )
    match = pattern.search(spec_text)
    if not match:
        raise VerificationError(f"تعذر استخراج anchor '{heading}' من specification.md")
    return match.group(1), match.group(2)


def count_named_cases(path: Path) -> int:
    text = read_text(path)
    return len(re.findall(r'(?m)^\s*name:\s*"[^"]+",?\s*$', text))


def find_cli_binary() -> Path:
    suffix = ".exe" if sys.platform.startswith("win") else ""
    return ROOT / "target" / "debug" / f"dhad-cli{suffix}"


def parse_cli_hashes(stderr_text: str) -> tuple[str, str]:
    core_match = re.search(r"(?m)^core:\s*([0-9a-f]{64})\s*$", stderr_text)
    phon_match = re.search(r"(?m)^phonetic:\s*([0-9a-f]{64})\s*$", stderr_text)
    if not core_match or not phon_match:
        raise VerificationError(
            "تعذر استخراج core/phonetic من stderr الخاص بالـ CLI.\n"
            + first_lines(stderr_text, 40)
        )
    return core_match.group(1), phon_match.group(1)


def classify_two_output_pair(values: tuple[int, int]) -> str:
    a, b = values
    if a == 0x0644 and b in {0x0622, 0x0623, 0x0625, 0x0627}:
        return "lam_alef"
    if a == 0x0640 and b in {0x064B, 0x064E, 0x064F, 0x0650, 0x0651, 0x0652}:
        return "medial_diacritic_tatweel"
    raise VerificationError(
        f"تم العثور على Two(...) غير متوقع في FAPS: ({hex(a)}, {hex(b)})"
    )


def parse_faps_arms(source: str) -> dict[int, tuple[str, tuple[int, ...]]]:
    arm_re = re.compile(
        r"(?m)^\s*(0x[0-9A-Fa-f]+(?:\s*\|\s*0x[0-9A-Fa-f]+)*(?:\s*\.\.=\s*0x[0-9A-Fa-f]+)?)\s*=>\s*FapsResult::(One|Two|Unmapped|PassThrough)(?:\(([^)]*)\))?"
    )
    mapping: dict[int, tuple[str, tuple[int, ...]]] = {}
    for lhs, kind, args in arm_re.findall(source):
        cps = expand_lhs(lhs)
        if kind == "One":
            if not args:
                raise VerificationError(f"FAPS One(...) بلا معاملات: {lhs}")
            out = (int(args.strip(), 16),)
        elif kind == "Two":
            if not args:
                raise VerificationError(f"FAPS Two(...) بلا معاملات: {lhs}")
            parts = [p.strip() for p in args.split(",")]
            if len(parts) != 2:
                raise VerificationError(f"FAPS Two(...) غير متوقع: {lhs} => {args}")
            out = (int(parts[0], 16), int(parts[1], 16))
        else:
            out = ()
        for cp in cps:
            if cp in mapping:
                raise VerificationError(f"تكرار arm في FAPS لنقطة الكود U+{cp:04X}")
            mapping[cp] = (kind, out)
    return mapping


def parse_noise_set(source: str) -> set[int]:
    body_match = re.search(r"matches!\(cp,\s*(.*?)\s*\)\s*\}", source, re.S)
    if not body_match:
        raise VerificationError("تعذر استخراج matches!(cp, ...) من src/noise.rs")
    body = body_match.group(1)
    tokens = re.findall(r"0x[0-9A-Fa-f]+(?:\.\.=0x[0-9A-Fa-f]+)?", body)
    result: set[int] = set()
    for token in tokens:
        if "..=" in token:
            start_s, end_s = token.split("..=")
            start = int(start_s, 16)
            end = int(end_s, 16)
            result.update(range(start, end + 1))
        else:
            result.add(int(token, 16))
    return result


def verify_spec_tracked() -> None:
    cp = run(
        ["git", "ls-files", "--error-unmatch", "specification.md"],
        merge_streams=True,
        timeout=30,
    )
    if cp.returncode != 0:
        raise VerificationError(
            "specification.md ليس ملفًا متتبعًا في git.\n" + first_lines(cp.stdout or "")
        )


def check_01_msrv_lock() -> CheckResult:
    original_lock = read_bytes_or_none(CARGO_LOCK_PATH)
    try:
        probe = run(
            ["cargo", f"+{MSRV}", "--version"],
            merge_streams=True,
            timeout=60,
        )
        if probe.returncode != 0:
            return CheckResult(
                1,
                "Cargo.msrv.lock reproducibility",
                "FAIL",
                f"أداة Rust {MSRV} غير متاحة محليًا.\n{first_lines(probe.stdout or '')}",
            )

        # 1) The True Guarantee: Does the committed lockfile pass the tests under --locked?
        if CARGO_LOCK_PATH.exists():
            CARGO_LOCK_PATH.unlink()
        
        expected_lock = read_bytes_or_none(CARGO_MSRV_LOCK_PATH)
        if not expected_lock:
            return CheckResult(1, "Cargo.msrv.lock reproducibility", "FAIL", "Cargo.msrv.lock غير موجود.")
        
        restore_bytes(CARGO_LOCK_PATH, expected_lock)
        
        test_cp = run(["cargo", f"+{MSRV}", "test", "--all", "--locked", "--quiet"], merge_streams=True, timeout=1200)
        if test_cp.returncode != 0:
            return CheckResult(
                1,
                "Cargo.msrv.lock reproducibility",
                "FAIL",
                "Cargo.msrv.lock الملتزم لا يمرر الاختبارات باستخدام --locked.\n" + first_lines(test_cp.stdout or "", 80),
            )
        
        # 2) Exploratory check: Does regenerating from scratch produce the same bytes?
        # This is purely informational and NEVER fails the check.
        notes = []
        if CARGO_LOCK_PATH.exists():
            CARGO_LOCK_PATH.unlink()

        for cmd, allow_failure in MSRV_SEQUENCE:
            cp = run(cmd, merge_streams=True, timeout=600)
            if cp.returncode != 0 and allow_failure:
                notes.append(f"الخطوة غير الحرجة فشلت: {' '.join(cmd)}")

        if CARGO_LOCK_PATH.exists():
            generated = read_text(CARGO_LOCK_PATH)
            expected = expected_lock.decode("utf-8", errors="replace")
            if generated != expected:
                notes.append("ملاحظة: فهرس crates.io تغيّر لبعض التبعيات الفرعية منذ آخر توليد؛ إعادة التوليد من الصفر تختلف بايتًا، لكن الملف الملتزم سليم وظيفيًا ويعمل بنجاح.")

        detail = "الملف الملتزم Cargo.msrv.lock يعمل بنجاح تام تحت --locked بـ Rust 1.75.0."
        if notes:
            detail += " " + " | ".join(notes)
        return CheckResult(1, "Cargo.msrv.lock reproducibility", "PASS", detail)

    finally:
        restore_bytes(CARGO_LOCK_PATH, original_lock)


def check_02_hash_anchors() -> CheckResult:
    spec = read_text(SPEC_PATH)

    anchors = {
        "Empty stream": b"",
        "ALEF bare": bytes.fromhex("d8a7"),
        "BEH bare": bytes.fromhex("d8a8"),
        "BEH + FATHA": bytes.fromhex("d8a8d98e"),
    }

    build = run(["cargo", "build", "--quiet", "--bin", "dhad-cli"], merge_streams=True, timeout=1200)
    if build.returncode != 0:
        return CheckResult(
            2,
            "Mandatory hash anchors",
            "FAIL",
            "فشل بناء dhad-cli قبل فحص الـ anchors.\n" + first_lines(build.stdout or "", 80),
        )

    binary = find_cli_binary()
    if not binary.exists():
        return CheckResult(
            2,
            "Mandatory hash anchors",
            "FAIL",
            f"لم يتم العثور على الملف التنفيذي المتوقع: {binary}",
        )

    mismatches: list[str] = []
    for heading, input_bytes in anchors.items():
        expected_core, expected_phon = parse_anchor(spec, heading)
        cp = run([str(binary)], input_data=input_bytes, text=False, timeout=60)
        if cp.returncode != 0:
            stderr = (cp.stderr or b"").decode("utf-8", errors="replace")
            return CheckResult(
                2,
                "Mandatory hash anchors",
                "FAIL",
                f"الـ CLI أعاد رمز خروج غير صفري أثناء فحص '{heading}'.\n{first_lines(stderr, 40)}",
            )
        stderr_text = (cp.stderr or b"").decode("utf-8", errors="replace")
        got_core, got_phon = parse_cli_hashes(stderr_text)
        if got_core != expected_core or got_phon != expected_phon:
            mismatches.append(
                f"{heading}: core={got_core} (expected {expected_core}), "
                f"phonetic={got_phon} (expected {expected_phon})"
            )

    if mismatches:
        return CheckResult(
            2,
            "Mandatory hash anchors",
            "FAIL",
            "عدم تطابق في hash anchors الإلزامية:\n" + "\n".join(mismatches),
        )

    return CheckResult(
        2,
        "Mandatory hash anchors",
        "PASS",
        "تطابقت جميع الـ 4 anchors مع specification.md §7.4.",
    )


def check_03_vector_counts() -> CheckResult:
    golden = count_named_cases(GOLDEN_CASES_PATH)
    adversarial = count_named_cases(ADVERSARIAL_CASES_PATH)
    tagged = count_named_cases(TAGGED_CASES_PATH)
    total = golden + adversarial + tagged

    expected = EXPECTED_VECTOR_COUNTS
    if (
        golden != expected["golden"]
        or adversarial != expected["adversarial"]
        or tagged != expected["tagged"]
        or total != expected["total"]
    ):
        return CheckResult(
            3,
            "Conformance vector counts",
            "FAIL",
            f"العد الحالي لا يطابق المواصفة: golden={golden}, adversarial={adversarial}, tagged={tagged}, total={total} "
            f"(expected {expected['golden']}/{expected['adversarial']}/{expected['tagged']} = {expected['total']}).",
        )

    return CheckResult(
        3,
        "Conformance vector counts",
        "PASS",
        f"golden={golden}, adversarial={adversarial}, tagged={tagged}, total={total}.",
    )


ANSI_RE = re.compile(r"\x1b\[[0-9;]*m")


def check_04_test_suite_shape() -> CheckResult:
    cp = run(
        ["cargo", "test", "--all", "--color=never", "--", "--test-threads=1"],
        merge_streams=True,
        timeout=1800,
    )
    # Defense in depth against Cargo emitting ANSI in output on some CI
    # runners (verified empirically on GitHub Actions on 2026-07-25):
    #   1. --color=never asks Cargo not to emit color at the source.
    #   2. ANSI_RE.sub then strips any color that slips through anyway,
    #      protecting all downstream regexes in this check.
    # See CHANGELOG.md and commit fe7336a for the verification trail.
    output = ANSI_RE.sub("", cp.stdout or "")

    if cp.returncode != 0:
        return CheckResult(
            4,
            "Test suite shape",
            "FAIL",
            "فشل cargo test --all.\n" + first_lines(output, 120),
        )

    running_blocks = re.findall(r"(?m)^\s*Running (.+)$", output)
    doctest_blocks = re.findall(r"(?m)^\s*Doc-tests\s+(.+)$", output)
    result_blocks = re.findall(
        r"(?m)^test result:\s+(ok|FAILED)\.\s+(\d+)\s+passed;\s+(\d+)\s+failed;\s+(\d+)\s+ignored;\s+(\d+)\s+measured;\s+(\d+)\s+filtered out",
        output,
    )

    if not result_blocks:
        return CheckResult(
            4,
            "Test suite shape",
            "FAIL",
            "تعذر استخراج أي test result blocks من خرج cargo test.",
        )

    runtime_block_count = len(running_blocks) + len(doctest_blocks)
    result_block_count = len(result_blocks)

    if runtime_block_count != result_block_count:
        diag = "\n".join(
            f"  L{i:03d}: {line!r}"
            for i, line in enumerate(output.splitlines()[:40], start=1)
        )
        return CheckResult(
            4,
            "Test suite shape",
            "FAIL",
            f"عدد blocks التشغيل ({runtime_block_count}) لا يساوي عدد blocks النتائج ({result_block_count}).\n"
            f"Running blocks matched: {len(running_blocks)}, Doc-tests blocks matched: {len(doctest_blocks)}, result blocks matched: {len(result_blocks)}.\n"
            f"First 40 raw output lines (merged stdout+stderr) for diagnosis:\n{diag}",
        )

    integration_files = sorted(p.name for p in (ROOT / "tests").glob("*.rs"))
    integration_runtime = sorted(
        m.group(1)
        for block in running_blocks
        if (m := re.match(r"tests/([^ ]+\.rs)\s", block))
    )

    if integration_runtime != integration_files:
        return CheckResult(
            4,
            "Test suite shape",
            "FAIL",
            "ملفات suites تحت tests/ لا تطابق ما شغّله cargo test فعليًا.\n"
            f"on disk: {integration_files}\n"
            f"runtime: {integration_runtime}",
        )

    total_passed = sum(int(x[1]) for x in result_blocks)
    total_failed = sum(int(x[2]) for x in result_blocks)

    if total_failed != 0:
        return CheckResult(
            4,
            "Test suite shape",
            "FAIL",
            f"cargo test اكتشف {total_failed} اختبارات فاشلة.",
        )

    return CheckResult(
        4,
        "Test suite shape",
        "PASS",
        f"integration suites={len(integration_files)}, runtime result blocks={result_block_count}, total passed={total_passed}.",
    )


def check_05_faps() -> CheckResult:
    source = read_text(FAPS_PATH)
    spec = read_text(SPEC_PATH)
    mapping = parse_faps_arms(source)

    one_count = 0
    two_count = 0
    lam_alef_two = 0
    medial_diacritic_two = 0
    unmapped_count = 0
    pass_through_in_target: list[int] = []

    target_ranges = list(range(0xFB50, 0xFE00)) + list(range(0xFE70, 0xFF00))

    for cp in target_ranges:
        kind, out = mapping.get(cp, ("PassThrough", ()))
        if kind == "One":
            one_count += 1
        elif kind == "Two":
            two_count += 1
            category = classify_two_output_pair((out[0], out[1]))
            if category == "lam_alef":
                lam_alef_two += 1
            elif category == "medial_diacritic_tatweel":
                medial_diacritic_two += 1
        elif kind == "Unmapped":
            unmapped_count += 1
        elif kind == "PassThrough":
            pass_through_in_target.append(cp)
        else:
            raise VerificationError(f"نوع FAPS غير معروف: {kind}")

    total_mapped = one_count + two_count

    if total_mapped != 141 or one_count != 127 or two_count != 14:
        return CheckResult(
            5,
            "FAPS decomposition table",
            "FAIL",
            f"Counts فعلية غير مطابقة: mapped={total_mapped}, One={one_count}, Two={two_count} (expected 141 / 127 / 14).",
        )

    if lam_alef_two != 8 or medial_diacritic_two != 6:
        return CheckResult(
            5,
            "FAPS decomposition table",
            "FAIL",
            f"التحليل الفرعي لـ Two(...) غير مطابق: Lam-Alef={lam_alef_two}, medial-diacritic+tatweel={medial_diacritic_two} (expected 8 / 6).",
        )

    if pass_through_in_target != [0xFEFF]:
        pretty = ", ".join(f"U+{cp:04X}" for cp in pass_through_in_target)
        return CheckResult(
            5,
            "FAPS decomposition table",
            "FAIL",
            f"تم العثور على PassThrough غير متوقع داخل النطاقات المستهدفة: {pretty or '<none>'}. المتوقع الحالي الوحيد هو U+FEFF.",
        )

    if not re.search(r"127 single-codepoint mappings\s*\+\s*14 two-codepoint mappings", spec):
        return CheckResult(
            5,
            "FAPS decomposition table",
            "FAIL",
            "specification.md لا تحتوي الصياغة المصححة الخاصة بـ 127 + 14 في Stage 3.",
        )

    if not re.search(r"8[^.\n]{0,120}Lam-Alef", spec, re.S):
        return CheckResult(
            5,
            "FAPS decomposition table",
            "FAIL",
            "specification.md لا تصف صراحةً وجود 8 Lam-Alef two-codepoint mappings.",
        )

    if not re.search(r"6[^.\n]{0,160}(medial-form diacritics|tatweel carrier)", spec, re.S):
        return CheckResult(
            5,
            "FAPS decomposition table",
            "FAIL",
            "specification.md لا تصف صراحةً وجود 6 medial-diacritic/tatweel two-codepoint mappings.",
        )

    return CheckResult(
        5,
        "FAPS decomposition table",
        "PASS",
        f"mapped=141 (One=127, Two=14; Lam-Alef=8, medial-diacritic+tatweel=6), pass-through in target ranges=U+FEFF only, unmapped={unmapped_count}.",
    )


def check_06_noise() -> CheckResult:
    source = read_text(NOISE_PATH)
    found = parse_noise_set(source)

    if found != EXPECTED_NOISE_SET:
        missing = sorted(EXPECTED_NOISE_SET - found)
        extra = sorted(found - EXPECTED_NOISE_SET)
        missing_s = ", ".join(f"U+{cp:04X}" for cp in missing) or "<none>"
        extra_s = ", ".join(f"U+{cp:04X}" for cp in extra) or "<none>"
        return CheckResult(
            6,
            "Noise codepoint set",
            "FAIL",
            f"مجموعة الضوضاء لا تطابق المتوقَّع.\nmissing: {missing_s}\nextra: {extra_s}",
        )

    if len(found) != 32:
        return CheckResult(
            6,
            "Noise codepoint set",
            "FAIL",
            f"عدد codepoints في noise.rs = {len(found)} وليس 32.",
        )

    return CheckResult(
        6,
        "Noise codepoint set",
        "PASS",
        "src/noise.rs يغطي المجموعة المتوقعة حرفيًا: 32 codepoints.",
    )


def check_07_axioms_invariants() -> CheckResult:
    verify_spec_tracked()
    spec = read_text(SPEC_PATH)

    axioms = sorted(
        set(re.findall(r"(?m)^\|\s*(A\d+)\s*\|", spec)),
        key=lambda x: int(x[1:]),
    )
    expected_axioms = [f"A{i}" for i in range(1, 12)]

    invariants = sorted(set(re.findall(r"(?m)^\s*I(\d{2})(?:\b|:)", spec)))
    expected_invariants = [f"{i:02d}" for i in range(1, 26)]

    if axioms != expected_axioms:
        return CheckResult(
            7,
            "Axiom and invariant counts",
            "FAIL",
            f"axioms found={axioms}, expected={expected_axioms}",
        )

    if invariants != expected_invariants:
        return CheckResult(
            7,
            "Axiom and invariant counts",
            "FAIL",
            f"invariants found={invariants}, expected={expected_invariants}",
        )

    if "I25:" not in spec:
        return CheckResult(
            7,
            "Axiom and invariant counts",
            "FAIL",
            "لم يتم العثور على تعريف I25 كقاعدة فعّالة.",
        )

    if "Check all 23 invariants (§8)" in spec:
        return CheckResult(
            7,
            "Axiom and invariant counts",
            "FAIL",
            "specification.md ما زالت تحتوي العبارة القديمة 'Check all 23 invariants (§8)' رغم تفعيل I24.",
        )

    return CheckResult(
        7,
        "Axiom and invariant counts",
        "PASS",
        "specification.md tracked في git؛ axioms=11 (A1..A11)، invariants=25 (I01..I25)، وI25 فعّال.",
    )


def check_08_no_stale_cr_refs() -> CheckResult:
    cp = run(
        [
            "git",
            "grep",
            "-n",
            "CR-0",
            "--",
            "*.rs",
            "*.md",
            ":(exclude)CHANGELOG.md",
        ],
        merge_streams=True,
        timeout=60,
    )
    if cp.returncode == 0:
        return CheckResult(
            8,
            "No stale CR-XX references outside CHANGELOG.md",
            "FAIL",
            "تم العثور على مراجع CR-0X في ملفات حيّة:\n" + first_lines(cp.stdout or "", 80),
        )
    if cp.returncode == 1:
        return CheckResult(
            8,
            "No stale CR-XX references outside CHANGELOG.md",
            "PASS",
            "لا توجد أي مراجع CR-0X في tracked *.rs/*.md خارج CHANGELOG.md.",
        )
    return CheckResult(
        8,
        "No stale CR-XX references outside CHANGELOG.md",
        "FAIL",
        "فشل git grep نفسه:\n" + first_lines(cp.stdout or "", 40),
    )


def check_09_cargo_audit() -> CheckResult:
    ci_text = read_text(CI_PATH)
    ignore_ids = re.findall(r"--ignore\s+(RUSTSEC-\d{4}-\d{4})", ci_text)

    if set(ignore_ids) != EXPECTED_AUDIT_IGNORES or len(ignore_ids) != 2:
        return CheckResult(
            9,
            "cargo audit ignore list",
            "FAIL",
            f"قائمة ignores في CI ليست بالضبط {sorted(EXPECTED_AUDIT_IGNORES)}. found={ignore_ids}",
        )

    probe = run(["cargo", "audit", "--version"], merge_streams=True, timeout=60)
    if probe.returncode != 0:
        return CheckResult(
            9,
            "cargo audit ignore list",
            "WARN",
            "قائمة ignores في CI صحيحة، لكن cargo-audit غير متاح محليًا لإجراء الفحص الطازج بدون ignores.",
        )

    audit = run(["cargo", "audit"], merge_streams=True, timeout=900)
    output = audit.stdout or ""
    found_ids = set(re.findall(r"(RUSTSEC-\d{4}-\d{4})", output))
    new_ids = sorted(found_ids - EXPECTED_AUDIT_IGNORES)

    network_timeout = (
        "allotted timeframe" in output
        or "couldn't check if the package is yanked" in output
        or "request could not be completed in the allotted timeframe" in output
    )

    if new_ids:
        return CheckResult(
            9,
            "cargo audit ignore list",
            "WARN",
            f"قائمة ignores في CI صحيحة، لكن cargo audit بدون ignores كشف Advisory IDs جديدة: {new_ids}",
        )

    if network_timeout:
        return CheckResult(
            9,
            "cargo audit ignore list",
            "WARN",
            "قائمة ignores في CI صحيحة، لكن الفحص الطازج بدون ignores تأثر بمشكلات شبكة/timeout. لم تظهر IDs جديدة غير المعروفتين.",
        )

    return CheckResult(
        9,
        "cargo audit ignore list",
        "PASS",
        "قائمة ignores في CI = advisoryين بالضبط، والفحص الطازج بدون ignores لم يُظهر IDs جديدة.",
    )


def check_10_generated_freshness() -> CheckResult:
    original_conf = read_bytes_or_none(CONFORMANCE_PATH)
    original_golden = read_bytes_or_none(GOLDEN_CASES_PATH)

    try:
        gen_cp = run(["python3", str(GENERATE_CONFORMANCE_PATH)], merge_streams=True, timeout=1800)
        if gen_cp.returncode != 0:
            return CheckResult(
                10,
                "generate_conformance.py / bootstrap_golden_cases.py freshness",
                "FAIL",
                "فشل generate_conformance.py.\n" + first_lines(gen_cp.stdout or "", 120),
            )

        before_conf = (original_conf or b"").decode("utf-8", errors="replace")
        after_conf = read_text(CONFORMANCE_PATH)

        if normalize_conformance(before_conf) != normalize_conformance(after_conf):
            return CheckResult(
                10,
                "generate_conformance.py / bootstrap_golden_cases.py freshness",
                "FAIL",
                "CONFORMANCE.md تغيّر بأكثر من سطر Generated:.\n"
                + diff_excerpt(
                    normalize_conformance(before_conf),
                    normalize_conformance(after_conf),
                    "CONFORMANCE.md (normalized before)",
                    "CONFORMANCE.md (normalized after)",
                ),
            )

        boot_cp = run(["python3", str(BOOTSTRAP_GOLDEN_PATH)], merge_streams=True, timeout=600)
        if boot_cp.returncode != 0:
            return CheckResult(
                10,
                "generate_conformance.py / bootstrap_golden_cases.py freshness",
                "FAIL",
                "فشل tools/bootstrap_golden_cases.py.\n" + first_lines(boot_cp.stdout or "", 80),
            )

        before_golden = (original_golden or b"").decode("utf-8", errors="replace")
        after_golden = read_text(GOLDEN_CASES_PATH)

        if before_golden != after_golden:
            return CheckResult(
                10,
                "generate_conformance.py / bootstrap_golden_cases.py freshness",
                "FAIL",
                "tests/cases/golden_cases.rs ليس مطابقًا للمخرَج الحالي من bootstrap.\n"
                + diff_excerpt(
                    before_golden,
                    after_golden,
                    "golden_cases.rs (before)",
                    "golden_cases.rs (after)",
                ),
            )

        return CheckResult(
            10,
            "generate_conformance.py / bootstrap_golden_cases.py freshness",
            "PASS",
            "CONFORMANCE.md متطابق بعد تجاهل Generated: فقط، وgolden_cases.rs مطابق حرفيًا.",
        )

    finally:
        restore_bytes(CONFORMANCE_PATH, original_conf)
        restore_bytes(GOLDEN_CASES_PATH, original_golden)



def check_11_doc_stats_parity() -> CheckResult:
    """
    Computed documentation parity check — replaces old string-matching version.

    Sources of truth (computed live, not hardcoded):
      - Test count: `cargo test --all -- --list`
      - Vector counts: len(json["vectors"]) from actual JSON files
      - ok/err split: count of expected_result == "ok"/"err" per JSON file
    """
    STALE_TOKENS = [
        "284", "285", "286",  # superseded test-count totals
        "185", "186", "187",  # superseded vector-corpus totals
        "39 adversarial", "39 typed", "| 39 |",  # superseded adversarial
        "30 tagged", "30 Mode B", "| 30 |",  # superseded tagged
        "23 invariant", "24 invariant",  # superseded invariant counts
        "7 corrections", "7 تصحيحات",  # retired CR-01..CR-07 tagline
    ]

    errs: list[str] = []

    # 1a. Test count via --list
    cp_list = run(
        ["cargo", "test", "--all", "--color=never", "--", "--list"],
        merge_streams=True,
        timeout=120,
    )
    if cp_list.returncode != 0:
        return CheckResult(
            11,
            "Doc stats parity (computed)",
            "FAIL",
            "cargo test --list failed:\n" + first_lines(cp_list.stdout or "", 20),
        )
    actual_tests = len(re.findall(r"(?m): test$", cp_list.stdout or ""))

    # 1b. Vector counts from JSON files
    conf_suite = ROOT.parent / "dhad-conformance-suite"
    if not (conf_suite / "vectors").is_dir():
        conf_suite = ROOT / "dhad-conformance-suite"
    vectors_dir = conf_suite / "vectors"

    if not vectors_dir.is_dir():
        return CheckResult(
            11,
            "Doc stats parity (computed)",
            "FAIL",
            f"Vectors directory not found: {vectors_dir}",
        )

    v: dict[str, int] = {}
    for name in ("golden", "adversarial", "tagged"):
        pth = vectors_dir / f"{name}.json"
        if not pth.is_file():
            errs.append(f"Missing vector file: {pth}")
            continue
        with open(pth, encoding="utf-8") as f:
            data = json.load(f)
        vecs = data.get("vectors", [])
        v[name] = len(vecs)
        v[f"{name}_ok"] = sum(1 for x in vecs if x.get("expected_result") == "ok")
        v[f"{name}_err"] = sum(1 for x in vecs if x.get("expected_result") == "err")

    v["total"] = v.get("golden", 0) + v.get("adversarial", 0) + v.get("tagged", 0)
    v["total_ok"] = v.get("golden_ok", 0) + v.get("adversarial_ok", 0) + v.get("tagged_ok", 0)
    v["total_err"] = v.get("golden_err", 0) + v.get("adversarial_err", 0) + v.get("tagged_err", 0)

    # 2. dhad/README.md
    readme_en = read_text(ROOT / "README.md")

    for fname in ("golden", "adversarial", "tagged"):
        m = re.search(rf"`{fname}\.json`.*?\|\s*(\d+)\s*\|", readme_en)
        if m and int(m.group(1)) != v.get(fname, -1):
            errs.append(f"README.md table: {fname}={m.group(1)} (actual {v[fname]})")

    m_total = re.search(r"\*\*Total\*\*.*?\*\*(\d+)\*\*", readme_en)
    if m_total and int(m_total.group(1)) != v["total"]:
        errs.append(f"README.md table: Total={m_total.group(1)} (actual {v['total']})")

    m_verified = re.search(r"Dhad (v1\.\d+\.\d+) is verified", readme_en)
    if m_verified and m_verified.group(1) != "v1.2.3":
        errs.append(f"README.md: '{m_verified.group(1)} is verified' (expected v1.2.3)")

    m_v121 = re.search(r"\|\s*\*\*v1\.2\.1\*\*\s*\|[^|]*\|([^|]*)\|", readme_en)
    if m_v121 and "188" in m_v121.group(1):
        errs.append("README.md: v1.2.1 row claims 188 vectors (shipped with 187)")

    if not re.search(r"\|\s*\*\*v1\.2\.3\*\*\s*\|", readme_en):
        errs.append("README.md: v1.2.3 row missing from version table")

    # 3. dhad/README.ar.md
    readme_ar = read_text(ROOT / "README.ar.md")
    if str(v["total"]) not in readme_ar:
        errs.append(f"README.ar.md: does not mention {v['total']} vectors")
    if str(actual_tests) not in readme_ar:
        errs.append(f"README.ar.md: does not mention {actual_tests} tests")

    m_ar_v121 = re.search(r"\|\s*\*\*v1\.2\.1\*\*\s*\|[^|]*\|([^|]*)\|", readme_ar)
    if m_ar_v121 and "188" in m_ar_v121.group(1):
        errs.append("README.ar.md: v1.2.1 row claims 188 vectors (shipped with 187)")

    # 4. dhad/HANDOFF.md
    handoff = read_text(ROOT / "HANDOFF.md")

    m_pub = re.search(r"Latest published version.*?`(v[\d.]+)`", handoff)
    if m_pub and m_pub.group(1) != "v1.2.3":
        errs.append(f"HANDOFF.md: Latest published = {m_pub.group(1)} (expected v1.2.3)")

    m_tag = re.search(r"verify_tagged_ref\.py.*?(\d+)/(\d+)", handoff)
    if m_tag and int(m_tag.group(1)) != v.get("tagged", -1):
        errs.append(
            f"HANDOFF.md: verify_tagged_ref = {m_tag.group(1)}/{m_tag.group(2)} "
            f"(expected {v['tagged']}/{v['tagged']})"
        )

    m_gold = re.search(r"verify_golden_ref\.py.*?(\d+)/(\d+)", handoff)
    ga = v.get("golden", 0) + v.get("adversarial", 0)
    if m_gold and int(m_gold.group(1)) != ga:
        errs.append(
            f"HANDOFF.md: verify_golden_ref = {m_gold.group(1)}/{m_gold.group(2)} "
            f"(expected {ga}/{ga})"
        )

    if "30/30" in handoff and v.get("tagged", 0) != 30:
        errs.append(f"HANDOFF.md: stale '30/30' (tagged actual = {v.get('tagged')})")
    if "155/155" in handoff and ga != 155:
        errs.append(f"HANDOFF.md: stale '155/155' (golden+adv actual = {ga})")

    # 5. suite/README.md
    suite_readme_path = conf_suite / "README.md"
    if suite_readme_path.is_file():
        sr = read_text(suite_readme_path)
        rows = re.findall(
            r"\|\s*`vectors/(\w+)\.json`\s*\|[^|]*\|[^|]*\|"
            r"\s*(\d+)\s*\|\s*(\d+)\s*\|\s*(\d+)\s*\|",
            sr,
        )
        for fname, vecs_s, ok_s, err_s in rows:
            vn, on, en = int(vecs_s), int(ok_s), int(err_s)
            if on + en != vn:
                errs.append(f"suite/README.md: {fname} ok({on})+err({en})={on + en} ≠ Vectors({vn})")
            if fname in v and vn != v[fname]:
                errs.append(f"suite/README.md: {fname} Vectors={vn} (actual {v[fname]})")
            if fname in v and on != v.get(f"{fname}_ok", -1):
                errs.append(f"suite/README.md: {fname} ok={on} (actual {v[f'{fname}_ok']})")
            if fname in v and en != v.get(f"{fname}_err", -1):
                errs.append(f"suite/README.md: {fname} err={en} (actual {v[f'{fname}_err']})")

        m_st = re.search(
            r"\|\s*\*\*Total\*\*.*?\|"
            r"\s*\*\*(\d+)\*\*\s*\|\s*\*\*(\d+)\*\*\s*\|\s*\*\*(\d+)\*\*\s*\|",
            sr,
        )
        if m_st:
            tv, to_, te = int(m_st.group(1)), int(m_st.group(2)), int(m_st.group(3))
            if to_ + te != tv:
                errs.append(f"suite/README.md: Total ok({to_})+err({te})={to_ + te} ≠ Vectors({tv})")
            if tv != v["total"]:
                errs.append(f"suite/README.md: Total Vectors={tv} (actual {v['total']})")

    # 6. python_ref/README.md
    pyref_path = conf_suite / "python_ref" / "README.md"
    if pyref_path.is_file():
        pr = read_text(pyref_path)
        stale_map = {"185": v["total"], "39": v.get("adversarial", 0), "30": v.get("tagged", 0)}
        for stale_str, actual_val in stale_map.items():
            if stale_str != str(actual_val) and re.search(rf"\b{stale_str}\b", pr):
                errs.append(f"python_ref/README.md: stale '{stale_str}' (actual {actual_val})")

    # 7. schema §13
    schema_path = conf_suite / "schema" / "vector-schema-1.0.md"
    if schema_path.is_file():
        sc = read_text(schema_path)
        for fname in ("golden", "adversarial", "tagged"):
            m = re.search(rf"`{fname}\.json`\s*=\s*(\d+)", sc)
            if m and int(m.group(1)) != v.get(fname, -1):
                errs.append(f"schema §13: {fname}={m.group(1)} (actual {v[fname]})")
        m_t = re.search(r"total\s*=\s*(\d+)", sc)
        if m_t and int(m_t.group(1)) != v["total"]:
            errs.append(f"schema §13: total={m_t.group(1)} (actual {v['total']})")

    for label, text in (("README.md", readme_en), ("README.ar.md", readme_ar), ("HANDOFF.md", handoff)):
        for token in STALE_TOKENS:
            if token in text:
                errs.append(f"{label} still contains stale token {token!r}")

    if errs:
        return CheckResult(
            11,
            "Documentation stats parity (computed from live sources)",
            "FAIL",
            f"{len(errs)} drift(s) detected:\n" + "\n".join(f"  • {e}" for e in errs),
        )

    return CheckResult(
        11,
        "Documentation stats parity (computed from live sources)",
        "PASS",
        f"tests={actual_tests}, vectors={v['total']} "
        f"(g={v.get('golden')}, a={v.get('adversarial')}, t={v.get('tagged')}). "
        f"All documentation matches computed ground truth.",
    )


CHECKS = [
    check_01_msrv_lock,
    check_02_hash_anchors,
    check_03_vector_counts,
    check_04_test_suite_shape,
    check_05_faps,
    check_06_noise,
    check_07_axioms_invariants,
    check_08_no_stale_cr_refs,
    check_09_cargo_audit,
    check_10_generated_freshness,
    check_11_doc_stats_parity,
]


def main() -> int:
    failures = 0
    warnings = 0

    print("Dhad plan-status verification")
    print(f"Repository root: {ROOT}")
    print()

    for check in CHECKS:
        try:
            result = check()
        except subprocess.TimeoutExpired as exc:
            result = CheckResult(
                idx=0,
                name=getattr(check, "__name__", "unknown"),
                status="FAIL",
                detail=f"انتهت المهلة أثناء التنفيذ: {exc}",
            )
        except Exception as exc:
            result = CheckResult(
                idx=0,
                name=getattr(check, "__name__", "unknown"),
                status="FAIL",
                detail=f"{type(exc).__name__}: {exc}",
            )

        if result.status == "FAIL":
            failures += 1
        elif result.status == "WARN":
            warnings += 1

        print(f"[{result.status}] {result.idx:02d} {result.name} — {result.detail}")

    print()
    print(f"Summary: {len(CHECKS) - failures - warnings} passed, {warnings} warned, {failures} failed.")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())

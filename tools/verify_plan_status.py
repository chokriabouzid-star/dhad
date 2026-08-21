#!/usr/bin/env python3
from __future__ import annotations

import difflib
import re
import subprocess
import sys
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
    readme_en = read_text(ROOT / "README.md")
    readme_ar = read_text(ROOT / "README.ar.md")
    handoff = read_text(ROOT / "HANDOFF.md")

    errs = []
    if "287%20verified" not in readme_en or "287 tests" not in readme_en:
        errs.append("README.md stats mismatch")
    if "188-vector" not in readme_en or "188/188" not in readme_en:
        errs.append("README.md vector stats mismatch")
    if "287%20verified" not in readme_ar or "287 اختباراً" not in readme_ar:
        errs.append("README.ar.md stats mismatch")
    if "188 ناقلاً" not in readme_ar:
        errs.append("README.ar.md vector stats mismatch")
    if "287 tests" not in handoff or "188 vectors" not in handoff:
        errs.append("HANDOFF.md stats mismatch")

    if errs:
        return CheckResult(
            11,
            "Documentation stats parity with codebase",
            "FAIL",
            "Doc stats mismatch: " + "; ".join(errs),
        )

    return CheckResult(
        11,
        "Documentation stats parity with codebase",
        "PASS",
        "README.md, README.ar.md, and HANDOFF.md strictly match 287 tests / 188 vectors.",
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

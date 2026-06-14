from pathlib import Path

SRC_PATH = Path("tests/suite1_golden.rs")
OUT_PATH = Path("tests/cases/golden_cases.rs")


def find_macro_invocations(src: str, macro_name: str):
    needle = macro_name + "!("
    i = 0
    blocks = []

    while True:
        start = src.find(needle, i)
        if start == -1:
            break

        j = start + len(needle)
        depth = 1
        in_string = False
        in_char = False
        escape = False
        in_line_comment = False
        in_block_comment = False

        while j < len(src):
            ch = src[j]
            nxt = src[j + 1] if j + 1 < len(src) else ""

            if in_line_comment:
                if ch == "\n":
                    in_line_comment = False
                j += 1
                continue

            if in_block_comment:
                if ch == "*" and nxt == "/":
                    in_block_comment = False
                    j += 2
                else:
                    j += 1
                continue

            if in_string:
                if escape:
                    escape = False
                elif ch == "\\":
                    escape = True
                elif ch == '"':
                    in_string = False
                j += 1
                continue

            if in_char:
                if escape:
                    escape = False
                elif ch == "\\":
                    escape = True
                elif ch == "'":
                    in_char = False
                j += 1
                continue

            if ch == "/" and nxt == "/":
                in_line_comment = True
                j += 2
                continue

            if ch == "/" and nxt == "*":
                in_block_comment = True
                j += 2
                continue

            if ch == '"':
                in_string = True
                j += 1
                continue

            if ch == "'":
                in_char = True
                j += 1
                continue

            if ch == "(":
                depth += 1
            elif ch == ")":
                depth -= 1
                if depth == 0:
                    body = src[start + len(needle):j]
                    blocks.append(body)
                    j += 1
                    while j < len(src) and src[j].isspace():
                        j += 1
                    if j < len(src) and src[j] == ";":
                        j += 1
                    i = j
                    break

            j += 1
        else:
            raise RuntimeError(f"Unclosed macro invocation starting at offset {start}")

    return blocks


def split_top_level_args(body: str):
    args = []
    buf = []

    paren = 0
    bracket = 0
    brace = 0

    in_string = False
    in_char = False
    escape = False
    in_line_comment = False
    in_block_comment = False

    i = 0
    while i < len(body):
        ch = body[i]
        nxt = body[i + 1] if i + 1 < len(body) else ""

        if in_line_comment:
            buf.append(ch)
            if ch == "\n":
                in_line_comment = False
            i += 1
            continue

        if in_block_comment:
            buf.append(ch)
            if ch == "*" and nxt == "/":
                buf.append(nxt)
                in_block_comment = False
                i += 2
            else:
                i += 1
            continue

        if in_string:
            buf.append(ch)
            if escape:
                escape = False
            elif ch == "\\":
                escape = True
            elif ch == '"':
                in_string = False
            i += 1
            continue

        if in_char:
            buf.append(ch)
            if escape:
                escape = False
            elif ch == "\\":
                escape = True
            elif ch == "'":
                in_char = False
            i += 1
            continue

        if ch == "/" and nxt == "/":
            buf.append(ch)
            buf.append(nxt)
            in_line_comment = True
            i += 2
            continue

        if ch == "/" and nxt == "*":
            buf.append(ch)
            buf.append(nxt)
            in_block_comment = True
            i += 2
            continue

        if ch == '"':
            buf.append(ch)
            in_string = True
            i += 1
            continue

        if ch == "'":
            buf.append(ch)
            in_char = True
            i += 1
            continue

        if ch == "(":
            paren += 1
        elif ch == ")":
            paren -= 1
        elif ch == "[":
            bracket += 1
        elif ch == "]":
            bracket -= 1
        elif ch == "{":
            brace += 1
        elif ch == "}":
            brace -= 1
        elif ch == "," and paren == 0 and bracket == 0 and brace == 0:
            arg = "".join(buf).strip()
            if arg:
                args.append(arg)
            buf = []
            i += 1
            continue

        buf.append(ch)
        i += 1

    tail = "".join(buf).strip()
    if tail:
        args.append(tail)

    return args


def strip_leading_trivia(expr: str) -> str:
    i = 0
    n = len(expr)

    while True:
        while i < n and expr[i].isspace():
            i += 1

        if expr.startswith("//", i):
            j = expr.find("\n", i)
            if j == -1:
                return ""
            i = j + 1
            continue

        if expr.startswith("/*", i):
            j = expr.find("*/", i + 2)
            if j == -1:
                raise ValueError("Unterminated block comment in expression")
            i = j + 2
            continue

        break

    return expr[i:].strip()


def rust_bytes_to_hex(expr: str) -> str:
    expr = strip_leading_trivia(expr.strip())

    if expr == 'b""':
        return ""

    if expr.startswith('b"') and expr.endswith('"'):
        body = expr[2:-1]
        body = body.encode("utf-8").decode("unicode_escape")
        return body.encode("latin1").hex()

    if expr.startswith("&[") and expr.endswith("]"):
        body = expr[2:-1]
        nums = []
        for raw in body.split(","):
            part = raw.strip()
            if not part:
                continue
            if "//" in part:
                part = part.split("//", 1)[0].strip()
            if not part:
                continue
            nums.append(int(part, 0))
        return bytes(nums).hex()

    raise ValueError(f"Unsupported byte expression: {expr}")


def build_preview(input_hex: str):
    if not input_hex:
        return "None"
    try:
        s = bytes.fromhex(input_hex).decode("utf-8")
    except Exception:
        return "None"

    s = (
        s.replace("\\", "\\\\")
         .replace('"', '\\"')
         .replace("\n", "\\n")
    )
    return f'Some("{s}")'


def main():
    src = SRC_PATH.read_text(encoding="utf-8")
    blocks = find_macro_invocations(src, "golden")
    print(f"golden! invocations found: {len(blocks)}")

    cases = []
    for body in blocks:
        args = split_top_level_args(body)
        if len(args) != 5:
            raise RuntimeError(
                "golden! macro did not split into 5 args:\n"
                f"{body}\n--- split -> {len(args)} args"
            )

        name, input_expr, stream_expr, core_hash, phonetic_hash = args

        cases.append({
            "name": name.strip(),
            "input_expr": input_expr.strip(),
            "stream_expr": stream_expr.strip(),
            "core_hash": core_hash.strip().strip('"'),
            "phonetic_hash": phonetic_hash.strip().strip('"'),
        })

    out = []
    out.append("/// Declarative Mode A golden vectors extracted from suite1_golden.rs.")
    out.append("///")
    out.append("/// Bootstrap-generated from the current test suite, then checked in")
    out.append("/// as explicit case data for exporters and future conformance tooling.")
    out.append("")
    out.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
    out.append("pub struct GoldenCase {")
    out.append("    pub name: &'static str,")
    out.append("    pub input: &'static [u8],")
    out.append("    pub stream_hex: &'static str,")
    out.append("    pub core_hash: &'static str,")
    out.append("    pub phonetic_hash: &'static str,")
    out.append("    pub utf8_preview: Option<&'static str>,")
    out.append("}")
    out.append("")
    out.append("pub const GOLDEN_CASES: &[GoldenCase] = &[")
    for c in cases:
        input_hex = rust_bytes_to_hex(c["input_expr"])
        stream_hex = rust_bytes_to_hex(c["stream_expr"])
        preview_expr = build_preview(input_hex)

        out.append("    GoldenCase {")
        out.append(f'        name: "{c["name"]}",')
        out.append(f"        input: {c['input_expr']},")
        out.append(f'        stream_hex: "{stream_hex}",')
        out.append(f'        core_hash: "{c["core_hash"]}",')
        out.append(f'        phonetic_hash: "{c["phonetic_hash"]}",')
        out.append(f"        utf8_preview: {preview_expr},")
        out.append("    },")

    out.append("];")
    out.append("")

    OUT_PATH.write_text("\n".join(out), encoding="utf-8")
    print(f"Wrote {OUT_PATH} with {len(cases)} cases.")


if __name__ == "__main__":
    main()

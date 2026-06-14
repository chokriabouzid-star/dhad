use std::fs;
use std::path::Path;

use dhad::mode_b::build_frame;
use dhad::model::{DhadAtom, ErrorKind};
use dhad::modes::{process_mode_a, process_mode_b};
use serde_json::{json, Map, Value};

#[path = "../tests/cases/adversarial_cases.rs"]
mod adversarial_cases;
#[path = "../tests/cases/golden_cases.rs"]
mod golden_cases;
#[path = "../tests/cases/tagged_cases.rs"]
mod tagged_cases;

fn hex(bytes: &[u8]) -> String {
    const T: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        s.push(T[(b >> 4) as usize] as char);
        s.push(T[(b & 0x0f) as usize] as char);
    }
    s
}

fn small_utf8_preview(bytes: &[u8]) -> Option<String> {
    if bytes.len() > 96 {
        return None;
    }
    std::str::from_utf8(bytes).ok().map(|s| s.to_string())
}

fn input_json(bytes: &[u8]) -> Value {
    if !bytes.is_empty() && bytes.len() >= 1024 && bytes.iter().all(|&b| b == bytes[0]) {
        json!({
            "encoding": "repeat_byte",
            "byte_hex": format!("{:02x}", bytes[0]),
            "count": bytes.len(),
        })
    } else {
        json!({
            "encoding": "hex",
            "hex": hex(bytes),
        })
    }
}

fn error_kind_name(err: &ErrorKind) -> &'static str {
    match err {
        ErrorKind::InputTooLarge(_) => "InputTooLarge",
        ErrorKind::MalformedUtf8 { .. } => "MalformedUtf8",
        ErrorKind::UnmappedCodepoint { .. } => "UnmappedCodepoint",
        ErrorKind::OrphanDiacritic { .. } => "OrphanDiacritic",
        ErrorKind::InvalidMarkCombo { .. } => "InvalidMarkCombo",
        ErrorKind::InvalidFlagCombo { .. } => "InvalidFlagCombo",
        ErrorKind::InvalidProsody { .. } => "InvalidProsody",
        ErrorKind::ReservedFieldNonZero { .. } => "ReservedFieldNonZero",
    }
}

fn error_json(err: &ErrorKind) -> Value {
    match err {
        ErrorKind::InputTooLarge(_) => json!({
            "kind": "InputTooLarge"
        }),
        ErrorKind::MalformedUtf8 { byte_offset } => json!({
            "kind": "MalformedUtf8",
            "byte_offset": byte_offset
        }),
        ErrorKind::UnmappedCodepoint {
            codepoint,
            position,
        } => json!({
            "kind": "UnmappedCodepoint",
            "codepoint": codepoint,
            "position": position
        }),
        ErrorKind::OrphanDiacritic {
            codepoint,
            position,
        } => json!({
            "kind": "OrphanDiacritic",
            "codepoint": codepoint,
            "position": position
        }),
        ErrorKind::InvalidMarkCombo { marks, atom_index } => json!({
            "kind": "InvalidMarkCombo",
            "marks": marks,
            "atom_index": atom_index
        }),
        ErrorKind::InvalidFlagCombo { flags, atom_index } => json!({
            "kind": "InvalidFlagCombo",
            "flags": flags,
            "atom_index": atom_index
        }),
        ErrorKind::InvalidProsody {
            prosody,
            atom_index,
            reason,
        } => json!({
            "kind": "InvalidProsody",
            "prosody": prosody,
            "atom_index": atom_index,
            "reason": reason
        }),
        ErrorKind::ReservedFieldNonZero {
            atom_index,
            reserved,
        } => json!({
            "kind": "ReservedFieldNonZero",
            "atom_index": atom_index,
            "reserved": reserved
        }),
    }
}

fn ok_vector_json(
    name: &str,
    mode: &str,
    input: &[u8],
    stream_hex: String,
    core_hash_hex: String,
    phonetic_hash_hex: String,
) -> Value {
    let mut obj = Map::new();
    obj.insert("name".into(), Value::String(name.to_string()));
    obj.insert("mode".into(), Value::String(mode.to_string()));
    obj.insert("input".into(), input_json(input));
    if mode == "A" {
        if let Some(preview) = small_utf8_preview(input) {
            obj.insert("input_utf8_preview".into(), Value::String(preview));
        }
    }
    obj.insert("expected_result".into(), Value::String("ok".to_string()));
    obj.insert("stream_hex".into(), Value::String(stream_hex));
    obj.insert("core_hash_hex".into(), Value::String(core_hash_hex));
    obj.insert("phonetic_hash_hex".into(), Value::String(phonetic_hash_hex));
    Value::Object(obj)
}

fn err_vector_json(name: &str, mode: &str, input: &[u8], err: &ErrorKind) -> Value {
    let mut obj = Map::new();
    obj.insert("name".into(), Value::String(name.to_string()));
    obj.insert("mode".into(), Value::String(mode.to_string()));
    obj.insert("input".into(), input_json(input));
    if mode == "A" {
        if let Some(preview) = small_utf8_preview(input) {
            obj.insert("input_utf8_preview".into(), Value::String(preview));
        }
    }
    obj.insert("expected_result".into(), Value::String("err".to_string()));
    obj.insert("error".into(), error_json(err));
    Value::Object(obj)
}

fn tagged_input_bytes(input: tagged_cases::TaggedInput) -> Vec<u8> {
    match input {
        tagged_cases::TaggedInput::Atoms(atoms) => {
            let atoms: Vec<DhadAtom> = atoms
                .iter()
                .map(|a| DhadAtom {
                    base: a.base,
                    marks: a.marks,
                    flags: a.flags,
                    prosody: a.prosody,
                    reserved: 0,
                })
                .collect();
            build_frame(&atoms)
        }
        tagged_cases::TaggedInput::RawFrame(bytes) => bytes.to_vec(),
        tagged_cases::TaggedInput::GeneratedFrame(f) => f(),
    }
}

fn adversarial_input_bytes(input: adversarial_cases::AdversarialInput) -> Vec<u8> {
    match input {
        adversarial_cases::AdversarialInput::Bytes(bytes) => bytes.to_vec(),
        adversarial_cases::AdversarialInput::Generated(f) => f(),
    }
}

fn export_golden() -> Vec<Value> {
    let mut out = Vec::with_capacity(golden_cases::GOLDEN_CASES.len());

    for case in golden_cases::GOLDEN_CASES {
        let result = process_mode_a(case.input)
            .unwrap_or_else(|e| panic!("golden case {} unexpectedly failed: {:?}", case.name, e));

        let stream_hex = hex(&result.stream.to_bytes());
        let core_hash_hex = hex(&result.core_hash);
        let phonetic_hash_hex = hex(&result.phonetic_hash);

        assert_eq!(
            stream_hex, case.stream_hex,
            "golden stream mismatch: {}",
            case.name
        );
        assert_eq!(
            core_hash_hex, case.core_hash,
            "golden core hash mismatch: {}",
            case.name
        );
        assert_eq!(
            phonetic_hash_hex, case.phonetic_hash,
            "golden phonetic hash mismatch: {}",
            case.name
        );

        out.push(ok_vector_json(
            case.name,
            "A",
            case.input,
            stream_hex,
            core_hash_hex,
            phonetic_hash_hex,
        ));
    }

    out
}

fn export_adversarial() -> Vec<Value> {
    let mut out = Vec::with_capacity(adversarial_cases::ADVERSARIAL_CASES.len());

    for case in adversarial_cases::ADVERSARIAL_CASES {
        let input = adversarial_input_bytes(case.input);

        match case.expected {
            adversarial_cases::AdversarialExpected::Ok {
                stream_hex,
                core_hash,
                phonetic_hash,
            } => {
                let result = process_mode_a(&input).unwrap_or_else(|e| {
                    panic!(
                        "adversarial ok-case {} unexpectedly failed: {:?}",
                        case.name, e
                    )
                });

                let got_stream_hex = hex(&result.stream.to_bytes());
                let got_core_hash_hex = hex(&result.core_hash);
                let got_phonetic_hash_hex = hex(&result.phonetic_hash);

                assert_eq!(
                    got_stream_hex, stream_hex,
                    "adversarial stream mismatch: {}",
                    case.name
                );
                assert_eq!(
                    got_core_hash_hex, core_hash,
                    "adversarial core hash mismatch: {}",
                    case.name
                );
                assert_eq!(
                    got_phonetic_hash_hex, phonetic_hash,
                    "adversarial phonetic hash mismatch: {}",
                    case.name
                );

                out.push(ok_vector_json(
                    case.name,
                    "A",
                    &input,
                    got_stream_hex,
                    got_core_hash_hex,
                    got_phonetic_hash_hex,
                ));
            }
            adversarial_cases::AdversarialExpected::Err { error_kind } => {
                let err = process_mode_a(&input).err().unwrap_or_else(|| {
                    panic!("adversarial err-case {} unexpectedly succeeded", case.name)
                });

                assert_eq!(
                    error_kind_name(&err),
                    error_kind,
                    "adversarial error kind mismatch: {}",
                    case.name
                );

                out.push(err_vector_json(case.name, "A", &input, &err));
            }
        }
    }

    out
}

fn export_tagged() -> Vec<Value> {
    let mut out = Vec::with_capacity(tagged_cases::TAGGED_CASES.len());

    for case in tagged_cases::TAGGED_CASES {
        let input = tagged_input_bytes(case.input);

        match case.expected {
            tagged_cases::TaggedExpected::Ok {
                stream_hex,
                core_hash,
                phonetic_hash,
            } => {
                let result = process_mode_b(&input).unwrap_or_else(|e| {
                    panic!("tagged ok-case {} unexpectedly failed: {:?}", case.name, e)
                });

                let got_stream_hex = hex(&result.stream.to_bytes());
                let got_core_hash_hex = hex(&result.core_hash);
                let got_phonetic_hash_hex = hex(&result.phonetic_hash);

                assert_eq!(
                    got_stream_hex, stream_hex,
                    "tagged stream mismatch: {}",
                    case.name
                );
                assert_eq!(
                    got_core_hash_hex, core_hash,
                    "tagged core hash mismatch: {}",
                    case.name
                );
                assert_eq!(
                    got_phonetic_hash_hex, phonetic_hash,
                    "tagged phonetic hash mismatch: {}",
                    case.name
                );

                out.push(ok_vector_json(
                    case.name,
                    "B",
                    &input,
                    got_stream_hex,
                    got_core_hash_hex,
                    got_phonetic_hash_hex,
                ));
            }
            tagged_cases::TaggedExpected::Err { error_kind } => {
                let err = process_mode_b(&input).err().unwrap_or_else(|| {
                    panic!("tagged err-case {} unexpectedly succeeded", case.name)
                });

                assert_eq!(
                    error_kind_name(&err),
                    error_kind,
                    "tagged error kind mismatch: {}",
                    case.name
                );

                out.push(err_vector_json(case.name, "B", &input, &err));
            }
        }
    }

    out
}

fn write_suite(
    out_dir: &Path,
    file_name: &str,
    suite_name: &str,
    mode: &str,
    source_suite: &str,
    vectors: Vec<Value>,
) {
    let root = json!({
        "schema_version": "1.0",
        "dhad_spec": "v1.0+CR-01..CR-07",
        "generated_by": format!("dhad-rust-{}", env!("CARGO_PKG_VERSION")),
        "suite": suite_name,
        "mode": mode,
        "source_suite": source_suite,
        "vector_count": vectors.len(),
        "vectors": vectors,
    });

    let path = out_dir.join(file_name);
    let body = serde_json::to_string_pretty(&root).expect("serialize json");
    fs::write(&path, body).unwrap_or_else(|e| panic!("write {} failed: {}", path.display(), e));
}

fn main() {
    assert_eq!(
        golden_cases::GOLDEN_CASES.len(),
        116,
        "golden manifest count changed"
    );
    assert_eq!(
        tagged_cases::TAGGED_CASES.len(),
        30,
        "tagged manifest count changed"
    );
    assert_eq!(
        adversarial_cases::ADVERSARIAL_CASES.len(),
        39,
        "adversarial manifest count changed"
    );

    let out_dir = Path::new("target/dhad-conformance-vectors");
    fs::create_dir_all(out_dir).expect("create output directory");

    let golden = export_golden();
    let adversarial = export_adversarial();
    let tagged = export_tagged();

    write_suite(
        out_dir,
        "golden.json",
        "golden",
        "A",
        "tests/suite1_golden.rs",
        golden,
    );
    write_suite(
        out_dir,
        "adversarial.json",
        "adversarial",
        "A",
        "tests/suite3_adversarial.rs",
        adversarial,
    );
    write_suite(
        out_dir,
        "tagged.json",
        "tagged",
        "B",
        "tests/suite2_tagged.rs",
        tagged,
    );

    println!("wrote target/dhad-conformance-vectors/golden.json       (116 vectors)");
    println!("wrote target/dhad-conformance-vectors/adversarial.json  (39 vectors)");
    println!("wrote target/dhad-conformance-vectors/tagged.json       (30 vectors)");
    println!("total vectors: 185");
}

//! Dhad CLI — Mode A / Mode B processor
//! Reads UTF-8 text (Mode A, default) or a tagged binary frame (Mode B,
//! with --mode-b) from stdin, writes the atom stream to stdout, and
//! writes the human-readable summary (atom count, both hashes) to stderr.
//! Enforces MAX_INPUT_BYTES before allocation (A2, §5 pre-stage).

use dhad::constants::MAX_INPUT_BYTES;
use dhad::model::DhadResult;
use dhad::modes::{process_mode_a, process_mode_b};
use std::io::{Read, Write};

fn print_help() {
    println!("dhad-cli — deterministic Arabic text canonicalization");
    println!();
    println!("USAGE:");
    println!("    echo -n TEXT | dhad-cli          Mode A: UTF-8 text (default)");
    println!("    dhad-cli --mode-b < frame.bin     Mode B: tagged binary frame");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help       Print this help and exit");
    println!("    -V, --version    Print version and exit");
    println!("        --mode-b     Read a Mode B tagged binary frame from stdin");
    println!("                     instead of Mode A UTF-8 text");
    println!();
    println!("OUTPUT:");
    println!("    stdout: the raw AtomStream bytes (n * 8 bytes)");
    println!("    stderr: atom count, CoreHash, and PhoneticHash (human-readable)");
    println!();
    println!("    echo -n \"بسم\" | dhad-cli > atoms.bin      # atoms.bin: binary only");
    println!("    echo -n \"بسم\" | dhad-cli 2>&1 >/dev/null   # prints only the summary");
}

fn print_version() {
    println!("dhad-cli {}", env!("CARGO_PKG_VERSION"));
}

fn emit_result(result: DhadResult) {
    eprintln!("atoms:    {}", result.stream.len());
    eprintln!("core:     {}", hex::encode(result.core_hash));
    eprintln!("phonetic: {}", hex::encode(result.phonetic_hash));

    if let Err(e) = std::io::stdout().write_all(&result.stream.to_bytes()) {
        eprintln!("dhad: write error: {e}");
        std::process::exit(2);
    }
}

fn main() {
    let mut use_mode_b = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-V" | "--version" => {
                print_version();
                return;
            }
            "--mode-b" => {
                use_mode_b = true;
            }
            other => {
                eprintln!("dhad: unknown argument '{other}'");
                eprintln!("dhad: try 'dhad-cli --help'");
                std::process::exit(2);
            }
        }
    }

    // حماية الذاكرة: لا نقرأ أكثر من MAX_INPUT_BYTES + 1
    // الـ +1 يكشف ما إذا كان المدخل أكبر من الحد
    let stdin = std::io::stdin();
    let mut input = Vec::with_capacity(4096);

    match stdin
        .lock()
        .take((MAX_INPUT_BYTES + 1) as u64)
        .read_to_end(&mut input)
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("dhad: read error: {e}");
            std::process::exit(2);
        }
    }

    // فحص الحجم بعد القراءة المحدودة
    if input.len() > MAX_INPUT_BYTES {
        eprintln!(
            "dhad: input exceeds maximum ({} bytes > {} bytes)",
            input.len(),
            MAX_INPUT_BYTES
        );
        std::process::exit(2);
    }

    let outcome = if use_mode_b {
        process_mode_b(&input)
    } else {
        process_mode_a(&input)
    };

    match outcome {
        Ok(result) => emit_result(result),
        Err(e) => {
            eprintln!("dhad: error: {e}");
            std::process::exit(1);
        }
    }
}

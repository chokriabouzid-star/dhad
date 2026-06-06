//! Dhad CLI — Mode A processor
//! Reads UTF-8 from stdin, writes atom stream to stdout
//! Enforces MAX_INPUT_BYTES before allocation (A2, CR-04)

use dhad::modes::process_mode_a;
use std::io::{Read, Write};

const MAX_INPUT_BYTES: usize = 4_194_304;

fn main() {
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

    match process_mode_a(&input) {
        Ok(result) => {
            eprintln!("atoms:    {}", result.stream.len());
            eprintln!("core:     {}", hex::encode(result.core_hash));
            eprintln!("phonetic: {}", hex::encode(result.phonetic_hash));

            if let Err(e) = std::io::stdout().write_all(&result.stream.to_bytes()) {
                eprintln!("dhad: write error: {e}");
                std::process::exit(2);
            }
        }
        Err(e) => {
            eprintln!("dhad: error: {e}");
            std::process::exit(1);
        }
    }
}

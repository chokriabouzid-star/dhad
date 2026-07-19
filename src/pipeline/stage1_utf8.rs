use crate::model::ErrorKind;

pub fn decode(input: &[u8]) -> Result<Vec<u32>, ErrorKind> {
    match std::str::from_utf8(input) {
        Ok(s) => {
            let mut codepoints = Vec::new();
            for (byte_pos, c) in s.char_indices() {
                let cp = c as u32;
                if (0xD800..=0xDFFF).contains(&cp) {
                    // Defense in depth: std::str::from_utf8 already rejects any byte
                    // sequence that would decode to a surrogate, so this branch is not
                    // reachable today. Kept active in every build profile (not
                    // debug_assert!) — a correctness check for this protocol must not
                    // depend on debug_assertions being on (Axiom A2).
                    return Err(ErrorKind::MalformedUtf8 {
                        byte_offset: byte_pos,
                    });
                }
                codepoints.push(cp);
            }
            Ok(codepoints)
        }
        Err(e) => Err(ErrorKind::MalformedUtf8 {
            byte_offset: e.valid_up_to(),
        }),
    }
}

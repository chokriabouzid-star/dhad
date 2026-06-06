use crate::model::ErrorKind;

pub fn decode(input: &[u8]) -> Result<Vec<u32>, ErrorKind> {
    match std::str::from_utf8(input) {
        Ok(s) => {
            let mut codepoints = Vec::new();
            for c in s.chars() {
                let cp = c as u32;
                if (0xD800..=0xDFFF).contains(&cp) {
                    return Err(ErrorKind::MalformedUtf8 { byte_offset: 0 }); // handled by from_utf8 anyway
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

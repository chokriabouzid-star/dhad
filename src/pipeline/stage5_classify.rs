use crate::base_map::unicode_to_base;
use crate::model::{marks, prosody, ErrorKind};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Token {
    Base {
        cp: u32,
        base_id: u16,
        flags: u8,
        position: usize,
    },
    Diacritic {
        cp: u32,
        mark_bit: u16,
        position: usize,
    },
    Prosodic {
        cp: u32,
        prosody_bit: u8,
        position: usize,
    },
}

pub fn classify(cps: &[u32]) -> Result<Vec<Token>, ErrorKind> {
    let mut tokens = Vec::with_capacity(cps.len());
    for (position, &cp) in cps.iter().enumerate() {
        match cp {
            0x064E => tokens.push(Token::Diacritic {
                cp,
                mark_bit: marks::FATHA,
                position,
            }),
            0x064F => tokens.push(Token::Diacritic {
                cp,
                mark_bit: marks::DAMMA,
                position,
            }),
            0x0650 => tokens.push(Token::Diacritic {
                cp,
                mark_bit: marks::KASRA,
                position,
            }),
            0x0652 => tokens.push(Token::Diacritic {
                cp,
                mark_bit: marks::SUKUN,
                position,
            }),
            0x0651 => tokens.push(Token::Diacritic {
                cp,
                mark_bit: marks::SHADDA,
                position,
            }),

            0x064B => tokens.push(Token::Prosodic {
                cp,
                prosody_bit: prosody::TANWEEN_FATH,
                position,
            }),
            0x064C => tokens.push(Token::Prosodic {
                cp,
                prosody_bit: prosody::TANWEEN_DAMM,
                position,
            }),
            0x064D => tokens.push(Token::Prosodic {
                cp,
                prosody_bit: prosody::TANWEEN_KASR,
                position,
            }),
            0x0670 => tokens.push(Token::Prosodic {
                cp,
                prosody_bit: prosody::SUPERSCRIPT_ALEF,
                position,
            }),

            _ => {
                if let Some((base_id, flags)) = unicode_to_base(cp) {
                    tokens.push(Token::Base {
                        cp,
                        base_id,
                        flags,
                        position,
                    });
                } else {
                    return Err(ErrorKind::UnmappedCodepoint {
                        codepoint: cp,
                        position,
                    });
                }
            }
        }
    }
    Ok(tokens)
}

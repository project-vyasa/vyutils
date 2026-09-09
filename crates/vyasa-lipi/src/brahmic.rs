//! Generic parser and emitter for Brahmic scripts (Devanagari, Telugu, Kannada).

use crate::accents::{format_svara, is_svara_mark, parse_svara};
use crate::model::{AccentMode, AksharaToken, PunctuationToken, Token};
use crate::script::Script;
use vyasa_phonetics::{Ayogavaha, Consonant, VowelLength, VowelQuality};

/// Returns the Unicode base code point for the given Brahmic script.
pub const fn script_base(script: Script) -> Option<u32> {
    match script {
        Script::Devanagari => Some(0x0900),
        Script::Bengali => Some(0x0980),
        Script::Telugu => Some(0x0C00),
        Script::Kannada => Some(0x0C80),
        Script::Malayalam => Some(0x0D00),
        Script::Grantha => Some(0x11300),
        _ => None,
    }
}

/// Returns the virama character for the given Brahmic script.
pub const fn script_virama(script: Script) -> Option<char> {
    match script {
        Script::Devanagari => Some('\u{094D}'),
        Script::Bengali => Some('\u{09CD}'),
        Script::Telugu => Some('\u{0C4D}'),
        Script::Kannada => Some('\u{0CCD}'),
        Script::Malayalam => Some('\u{0D4D}'),
        Script::Grantha => Some('\u{1134D}'),
        _ => None,
    }
}

/// Maps an independent vowel to its Unicode character offset from the script base.
pub const fn independent_vowel_offset(quality: VowelQuality, length: VowelLength) -> Option<u32> {
    match (quality, length) {
        (VowelQuality::A, VowelLength::Hrasva) => Some(0x05),
        (VowelQuality::A, _) => Some(0x06), // ā
        (VowelQuality::I, VowelLength::Hrasva) => Some(0x07),
        (VowelQuality::I, _) => Some(0x08), // ī
        (VowelQuality::U, VowelLength::Hrasva) => Some(0x09),
        (VowelQuality::U, _) => Some(0x0A),                   // ū
        (VowelQuality::R, VowelLength::Hrasva) => Some(0x0B), // ṛ
        (VowelQuality::R, _) => Some(0x60),                   // ṝ
        (VowelQuality::L, VowelLength::Hrasva) => Some(0x0C), // ḷ
        (VowelQuality::L, _) => Some(0x61),                   // ॡ
        (VowelQuality::ShortE, _) => Some(0x0E),              // Dravidian short e
        (VowelQuality::E, _) => Some(0x0F),                   // Sanskrit / Dravidian long ē
        (VowelQuality::Ai, _) => Some(0x10),                  // ai
        (VowelQuality::ShortO, _) => Some(0x12),              // Dravidian short o
        (VowelQuality::O, _) => Some(0x13),                   // Sanskrit / Dravidian long ō
        (VowelQuality::Au, _) => Some(0x14),                  // au
    }
}

/// Maps an offset from script base back to independent vowel quality and length.
pub const fn offset_to_independent_vowel(offset: u32) -> Option<(VowelQuality, VowelLength)> {
    match offset {
        0x05 => Some((VowelQuality::A, VowelLength::Hrasva)),
        0x06 => Some((VowelQuality::A, VowelLength::Dirgha)),
        0x07 => Some((VowelQuality::I, VowelLength::Hrasva)),
        0x08 => Some((VowelQuality::I, VowelLength::Dirgha)),
        0x09 => Some((VowelQuality::U, VowelLength::Hrasva)),
        0x0A => Some((VowelQuality::U, VowelLength::Dirgha)),
        0x0B => Some((VowelQuality::R, VowelLength::Hrasva)),
        0x60 => Some((VowelQuality::R, VowelLength::Dirgha)),
        0x0C => Some((VowelQuality::L, VowelLength::Hrasva)),
        0x61 => Some((VowelQuality::L, VowelLength::Dirgha)),
        0x0E => Some((VowelQuality::ShortE, VowelLength::Hrasva)),
        0x0F => Some((VowelQuality::E, VowelLength::Dirgha)),
        0x10 => Some((VowelQuality::Ai, VowelLength::Dirgha)),
        0x12 => Some((VowelQuality::ShortO, VowelLength::Hrasva)),
        0x13 => Some((VowelQuality::O, VowelLength::Dirgha)),
        0x14 => Some((VowelQuality::Au, VowelLength::Dirgha)),
        _ => None,
    }
}

/// Maps a dependent vowel sign (mātrā) to its offset from script base.
pub const fn matra_offset(quality: VowelQuality, length: VowelLength) -> Option<u32> {
    match (quality, length) {
        (VowelQuality::A, VowelLength::Hrasva) => None, // Inherent vowel (no mark)
        (VowelQuality::A, _) => Some(0x3E),             // ā mātrā
        (VowelQuality::I, VowelLength::Hrasva) => Some(0x3F),
        (VowelQuality::I, _) => Some(0x40),
        (VowelQuality::U, VowelLength::Hrasva) => Some(0x41),
        (VowelQuality::U, _) => Some(0x42),
        (VowelQuality::R, VowelLength::Hrasva) => Some(0x43),
        (VowelQuality::R, _) => Some(0x44),
        (VowelQuality::L, VowelLength::Hrasva) => Some(0x62),
        (VowelQuality::L, _) => Some(0x63),
        (VowelQuality::ShortE, _) => Some(0x46),
        (VowelQuality::E, _) => Some(0x47),
        (VowelQuality::Ai, _) => Some(0x48),
        (VowelQuality::ShortO, _) => Some(0x4A),
        (VowelQuality::O, _) => Some(0x4B),
        (VowelQuality::Au, _) => Some(0x4C),
    }
}

/// Maps an offset from script base back to dependent vowel quality and length.
pub const fn offset_to_matra(offset: u32) -> Option<(VowelQuality, VowelLength)> {
    match offset {
        0x3E => Some((VowelQuality::A, VowelLength::Dirgha)),
        0x3F => Some((VowelQuality::I, VowelLength::Hrasva)),
        0x40 => Some((VowelQuality::I, VowelLength::Dirgha)),
        0x41 => Some((VowelQuality::U, VowelLength::Hrasva)),
        0x42 => Some((VowelQuality::U, VowelLength::Dirgha)),
        0x43 => Some((VowelQuality::R, VowelLength::Hrasva)),
        0x44 => Some((VowelQuality::R, VowelLength::Dirgha)),
        0x62 => Some((VowelQuality::L, VowelLength::Hrasva)),
        0x63 => Some((VowelQuality::L, VowelLength::Dirgha)),
        0x46 => Some((VowelQuality::ShortE, VowelLength::Hrasva)),
        0x47 => Some((VowelQuality::E, VowelLength::Dirgha)),
        0x48 => Some((VowelQuality::Ai, VowelLength::Dirgha)),
        0x4A => Some((VowelQuality::ShortO, VowelLength::Hrasva)),
        0x4B => Some((VowelQuality::O, VowelLength::Dirgha)),
        0x4C => Some((VowelQuality::Au, VowelLength::Dirgha)),
        _ => None,
    }
}

/// Maps a consonant to its offset from script base.
pub const fn consonant_offset(c: Consonant) -> u32 {
    match c {
        Consonant::K => 0x15,
        Consonant::Kh => 0x16,
        Consonant::G => 0x17,
        Consonant::Gh => 0x18,
        Consonant::Ng => 0x19,
        Consonant::C => 0x1A,
        Consonant::Ch => 0x1B,
        Consonant::J => 0x1C,
        Consonant::Jh => 0x1D,
        Consonant::Ny => 0x1E,
        Consonant::Tt => 0x1F,
        Consonant::Tth => 0x20,
        Consonant::Dd => 0x21,
        Consonant::Ddh => 0x22,
        Consonant::Nn => 0x23,
        Consonant::T => 0x24,
        Consonant::Th => 0x25,
        Consonant::D => 0x26,
        Consonant::Dh => 0x27,
        Consonant::N => 0x28,
        Consonant::P => 0x2A,
        Consonant::Ph => 0x2B,
        Consonant::B => 0x2C,
        Consonant::Bh => 0x2D,
        Consonant::M => 0x2E,
        Consonant::Y => 0x2F,
        Consonant::R => 0x30,
        Consonant::L => 0x32,
        Consonant::V => 0x35,
        Consonant::Sh => 0x36,
        Consonant::Ss => 0x37,
        Consonant::S => 0x38,
        Consonant::H => 0x39,
        Consonant::LVedic => 0x33,  // ळ / ళ / ಳ
        Consonant::LhVedic => 0x34, // ळ्ह (Devanagari 0x0934 or 0x0933+halant+0x0939)
    }
}

/// Maps an offset from script base back to a Consonant.
pub const fn offset_to_consonant(offset: u32) -> Option<Consonant> {
    match offset {
        0x15 => Some(Consonant::K),
        0x16 => Some(Consonant::Kh),
        0x17 => Some(Consonant::G),
        0x18 => Some(Consonant::Gh),
        0x19 => Some(Consonant::Ng),
        0x1A => Some(Consonant::C),
        0x1B => Some(Consonant::Ch),
        0x1C => Some(Consonant::J),
        0x1D => Some(Consonant::Jh),
        0x1E => Some(Consonant::Ny),
        0x1F => Some(Consonant::Tt),
        0x20 => Some(Consonant::Tth),
        0x21 => Some(Consonant::Dd),
        0x22 => Some(Consonant::Ddh),
        0x23 => Some(Consonant::Nn),
        0x24 => Some(Consonant::T),
        0x25 => Some(Consonant::Th),
        0x26 => Some(Consonant::D),
        0x27 => Some(Consonant::Dh),
        0x28 => Some(Consonant::N),
        0x2A => Some(Consonant::P),
        0x2B => Some(Consonant::Ph),
        0x2C => Some(Consonant::B),
        0x2D => Some(Consonant::Bh),
        0x2E => Some(Consonant::M),
        0x2F => Some(Consonant::Y),
        0x30 => Some(Consonant::R),
        0x32 => Some(Consonant::L),
        0x35 => Some(Consonant::V),
        0x36 => Some(Consonant::Sh),
        0x37 => Some(Consonant::Ss),
        0x38 => Some(Consonant::S),
        0x39 => Some(Consonant::H),
        0x33 => Some(Consonant::LVedic),
        0x34 => Some(Consonant::LhVedic),
        _ => None,
    }
}

/// Parses a Brahmic script text into a stream of IR Tokens.
pub fn parse_brahmic(text: &str, script: Script) -> Vec<Token> {
    let base = match script_base(script) {
        Some(b) => b,
        None => return Vec::new(),
    };
    let virama_offset = 0x4D;

    let mut tokens = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];
        let cp = c as u32;

        // 1. Check for Sacred Om
        if c == 'ॐ' || c == '\u{0C50}' || c == '\u{11350}' {
            tokens.push(Token::Punctuation(PunctuationToken::Om));
            i += 1;
            continue;
        }

        // Script-specific Om sequences:
        if (script == Script::Telugu
            && c == '\u{0C13}'
            && i + 1 < len
            && chars[i + 1] == '\u{0C02}')
            || (script == Script::Kannada
                && c == '\u{0C93}'
                && i + 1 < len
                && chars[i + 1] == '\u{0C82}')
            || (script == Script::Malayalam
                && c == '\u{0D13}'
                && i + 1 < len
                && chars[i + 1] == '\u{0D02}')
            || (script == Script::Bengali
                && c == '\u{0993}'
                && i + 1 < len
                && (chars[i + 1] == '\u{0981}' || chars[i + 1] == '\u{0982}'))
        {
            tokens.push(Token::Punctuation(PunctuationToken::Om));
            i += 2;
            continue;
        }

        // 2. Check for Punctuation: Daṇḍa, Double Daṇḍa, Avagraha
        if c == '।' || c == '|' {
            tokens.push(Token::Punctuation(PunctuationToken::Danda));
            i += 1;
            continue;
        }
        if c == '॥' || (c == '|' && i + 1 < len && chars[i + 1] == '|') {
            tokens.push(Token::Punctuation(PunctuationToken::DoubleDanda));
            i += if c == '|' { 2 } else { 1 };
            continue;
        }
        if (cp >= base && cp <= base + 0x7F && (cp - base) == 0x3D)
            || c == 'ऽ'
            || c == 'ఽ'
            || c == 'ಽ'
        {
            tokens.push(Token::Punctuation(PunctuationToken::Avagraha));
            i += 1;
            continue;
        }

        // 3. Check for Whitespace
        if c.is_whitespace() {
            tokens.push(Token::Whitespace(c));
            i += 1;
            continue;
        }

        // 4. Check for standalone Ayogavāhas (e.g. Jihvāmūlīya \u{1CF5}, Upadhmānīya \u{1CF6}, Ardhavisarga \u{1CF2})
        if c == '\u{1CF5}' {
            tokens.push(Token::Akshara(AksharaToken {
                consonants: Vec::new(),
                vowel: None,
                ayogavaha: Some(Ayogavaha::Jihvamuliya),
                svara: None,
            }));
            i += 1;
            continue;
        }
        if c == '\u{1CF6}' {
            tokens.push(Token::Akshara(AksharaToken {
                consonants: Vec::new(),
                vowel: None,
                ayogavaha: Some(Ayogavaha::Upadhmaniya),
                svara: None,
            }));
            i += 1;
            continue;
        }
        if c == '\u{1CF2}' {
            tokens.push(Token::Akshara(AksharaToken {
                consonants: Vec::new(),
                vowel: None,
                ayogavaha: Some(Ayogavaha::Ardhavisarga),
                svara: None,
            }));
            i += 1;
            continue;
        }

        // Check if within script range
        if cp >= base && cp <= base + 0x7F {
            let offset = cp - base;

            // 5. Independent Vowel
            if let Some((quality, length)) = offset_to_independent_vowel(offset) {
                let mut akshara = AksharaToken::vowel(quality, length);
                i += 1;

                // Check for trailing svara or ayogavāha
                while i < len {
                    let next_c = chars[i];
                    let next_cp = next_c as u32;

                    if is_svara_mark(next_c) {
                        if akshara.svara.is_none() {
                            akshara.svara = parse_svara(next_c);
                        }
                        i += 1;
                    } else if next_cp >= base && next_cp <= base + 0x7F {
                        let next_off = next_cp - base;
                        if next_off == 0x02 && akshara.ayogavaha.is_none() {
                            akshara.ayogavaha = Some(Ayogavaha::Anusvara);
                            i += 1;
                        } else if next_off == 0x03 && akshara.ayogavaha.is_none() {
                            akshara.ayogavaha = Some(Ayogavaha::Visarga);
                            i += 1;
                        } else if next_off == 0x01 && akshara.ayogavaha.is_none() {
                            akshara.ayogavaha = Some(Ayogavaha::Candrabindu);
                            i += 1;
                        } else {
                            break;
                        }
                    } else if next_c == '\u{1CE9}' && akshara.ayogavaha.is_none() {
                        akshara.ayogavaha = Some(Ayogavaha::GomukhaAnusvara);
                        i += 1;
                    } else if next_c == '\u{1CEA}' && akshara.ayogavaha.is_none() {
                        akshara.ayogavaha = Some(Ayogavaha::DvibinduAnusvara);
                        i += 1;
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Akshara(akshara));
                continue;
            }

            // 6. Consonant (or consonant cluster)
            if let Some(first_consonant) = offset_to_consonant(offset) {
                let mut consonants = vec![first_consonant];
                i += 1;

                let mut vowel = Some((VowelQuality::A, VowelLength::Hrasva)); // default inherent 'a'

                // Check for conjuncts: consonant + virama + consonant
                loop {
                    // Detect if Vedic flap aspirate was written as LVedic + virama + H
                    if consonants.len() == 1
                        && consonants[0] == Consonant::LVedic
                        && i + 1 < len
                        && chars[i] as u32 >= base
                        && (chars[i] as u32 - base) == virama_offset
                        && chars[i + 1] as u32 >= base
                        && (chars[i + 1] as u32 - base) == 0x39
                    {
                        consonants[0] = Consonant::LhVedic;
                        i += 2;
                        continue;
                    }

                    if i < len
                        && chars[i] as u32 >= base
                        && (chars[i] as u32 - base) == virama_offset
                    {
                        // Consumed virama
                        i += 1;

                        // Check if followed by another consonant
                        if i < len && chars[i] as u32 >= base && chars[i] as u32 <= base + 0x7F {
                            let next_off = chars[i] as u32 - base;
                            if let Some(next_c) = offset_to_consonant(next_off) {
                                consonants.push(next_c);
                                i += 1;
                                continue;
                            }
                        }

                        // No following consonant: this is a pure halant consonant!
                        vowel = None;
                        break;
                    }

                    // Check for vowel sign (mātrā)
                    if i < len && chars[i] as u32 >= base && chars[i] as u32 <= base + 0x7F {
                        let next_off = chars[i] as u32 - base;
                        if let Some((q, l)) = offset_to_matra(next_off) {
                            vowel = Some((q, l));
                            i += 1;
                        }
                    }
                    break;
                }

                let mut ayogavaha = None;
                let mut svara = None;

                // Check for trailing svara or ayogavāha
                while i < len {
                    let next_c = chars[i];
                    let next_cp = next_c as u32;

                    if is_svara_mark(next_c) {
                        if svara.is_none() {
                            svara = parse_svara(next_c);
                        }
                        i += 1;
                    } else if next_cp >= base && next_cp <= base + 0x7F {
                        let next_off = next_cp - base;
                        if next_off == 0x02 && ayogavaha.is_none() {
                            ayogavaha = Some(Ayogavaha::Anusvara);
                            i += 1;
                        } else if next_off == 0x03 && ayogavaha.is_none() {
                            ayogavaha = Some(Ayogavaha::Visarga);
                            i += 1;
                        } else if next_off == 0x01 && ayogavaha.is_none() {
                            ayogavaha = Some(Ayogavaha::Candrabindu);
                            i += 1;
                        } else {
                            break;
                        }
                    } else if next_c == '\u{1CE9}' && ayogavaha.is_none() {
                        ayogavaha = Some(Ayogavaha::GomukhaAnusvara);
                        i += 1;
                    } else if next_c == '\u{1CEA}' && ayogavaha.is_none() {
                        ayogavaha = Some(Ayogavaha::DvibinduAnusvara);
                        i += 1;
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Akshara(AksharaToken {
                    consonants,
                    vowel,
                    ayogavaha,
                    svara,
                }));
                continue;
            }
        }

        // 7. Any other character
        tokens.push(Token::Other(c));
        i += 1;
    }

    tokens
}

/// Emits IR tokens into a Brahmic script string.
pub fn emit_brahmic(tokens: &[Token], script: Script, accent_mode: AccentMode) -> String {
    let base = match script_base(script) {
        Some(b) => b,
        None => return String::new(),
    };
    let virama_char = match script_virama(script) {
        Some(v) => v,
        None => return String::new(),
    };

    let mut out = String::new();

    for token in tokens {
        match token {
            Token::Punctuation(p) => match p {
                PunctuationToken::Danda => out.push('।'),
                PunctuationToken::DoubleDanda => out.push('॥'),
                PunctuationToken::Avagraha => {
                    let av_char = char::from_u32(base + 0x3D).unwrap_or('\'');
                    out.push(av_char);
                }
                PunctuationToken::Om => match script {
                    Script::Devanagari => out.push('ॐ'),
                    Script::Telugu => {
                        out.push('\u{0C13}');
                        out.push('\u{0C02}');
                    }
                    Script::Kannada => {
                        out.push('\u{0C93}');
                        out.push('\u{0C82}');
                    }
                    Script::Malayalam => {
                        out.push('\u{0D13}');
                        out.push('\u{0D02}');
                    }
                    Script::Bengali => {
                        out.push('\u{0993}');
                        out.push('\u{0982}');
                    }
                    Script::Grantha => out.push('\u{11350}'),
                    _ => out.push_str("om"),
                },
            },
            Token::Whitespace(c) => out.push(*c),
            Token::Other(c) => out.push(*c),
            Token::Akshara(ak) => {
                // 1. Consonants or Independent Vowel
                if ak.consonants.is_empty() {
                    if let Some((q, l)) = ak.vowel {
                        if let Some(off) = independent_vowel_offset(q, l) {
                            if let Some(ch) = char::from_u32(base + off) {
                                out.push(ch);
                            }
                        }
                    }
                } else {
                    for (idx, &c) in ak.consonants.iter().enumerate() {
                        if idx > 0 {
                            out.push(virama_char);
                        }

                        // Special case for LhVedic if target is Telugu/Kannada or Devanagari
                        if c == Consonant::LhVedic {
                            if script == Script::Devanagari {
                                // ळ्ह as U+0933 + virama + U+0939
                                out.push('\u{0933}');
                                out.push('\u{094D}');
                                out.push('\u{0939}');
                            } else {
                                let l_char = char::from_u32(base + 0x33).unwrap_or(' ');
                                let h_char = char::from_u32(base + 0x39).unwrap_or(' ');
                                out.push(l_char);
                                out.push(virama_char);
                                out.push(h_char);
                            }
                        } else if script == Script::Bengali && c == Consonant::V {
                            // Bengali uses ব (0x09AC = 0x0980 + 0x2C) for v
                            if let Some(ch) = char::from_u32(base + 0x2C) {
                                out.push(ch);
                            }
                        } else if script == Script::Bengali && c == Consonant::LVedic {
                            // Bengali uses ল (0x09B2 = 0x0980 + 0x32) for Vedic ḷ
                            if let Some(ch) = char::from_u32(base + 0x32) {
                                out.push(ch);
                            }
                        } else {
                            let off = consonant_offset(c);
                            if let Some(ch) = char::from_u32(base + off) {
                                out.push(ch);
                            }
                        }
                    }

                    // 2. Vowel Mātrā
                    if let Some((q, l)) = ak.vowel {
                        if let Some(off) = matra_offset(q, l) {
                            if let Some(ch) = char::from_u32(base + off) {
                                out.push(ch);
                            }
                        }
                    } else {
                        // Pure halant consonant
                        out.push(virama_char);
                    }
                }

                // 3. Ayogavāha
                if let Some(ay) = ak.ayogavaha {
                    match ay {
                        Ayogavaha::Anusvara => {
                            if let Some(ch) = char::from_u32(base + 0x02) {
                                out.push(ch);
                            }
                        }
                        Ayogavaha::Visarga => {
                            if let Some(ch) = char::from_u32(base + 0x03) {
                                out.push(ch);
                            }
                        }
                        Ayogavaha::Candrabindu => {
                            if let Some(ch) = char::from_u32(base + 0x01) {
                                out.push(ch);
                            }
                        }
                        Ayogavaha::Jihvamuliya => out.push('\u{1CF5}'),
                        Ayogavaha::Upadhmaniya => out.push('\u{1CF6}'),
                        Ayogavaha::Ardhavisarga => out.push('\u{1CF2}'),
                        Ayogavaha::GomukhaAnusvara => out.push('\u{1CE9}'),
                        Ayogavaha::DvibinduAnusvara => out.push('\u{1CEA}'),
                    }
                }

                // 4. Vedic Svara Accent
                if let Some(sv) = ak.svara {
                    let s_str = format_svara(sv, script, accent_mode);
                    out.push_str(s_str);
                }
            }
        }
    }

    out
}

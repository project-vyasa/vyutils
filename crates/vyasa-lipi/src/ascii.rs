//! Parsers and emitters for ASCII Sanskrit transliteration schemes:
//! SLP1 (Sanskrit Library Phonetic Basic), Harvard-Kyoto (HK), and WX.

use crate::accents::{is_svara_mark, parse_svara};
use crate::model::{AksharaToken, PunctuationToken, Token};
use crate::script::Script;
use vyasa_phonetics::{Ayogavaha, Consonant, Svara, VowelLength, VowelQuality};

/// Emits IR tokens into an ASCII scheme string (SLP1, Harvard-Kyoto, or WX).
pub fn emit_ascii(tokens: &[Token], script: Script) -> String {
    let mut out = String::new();

    for token in tokens {
        match token {
            Token::Punctuation(p) => match p {
                PunctuationToken::Danda => match script {
                    Script::Slp1 => out.push('.'),
                    _ => out.push('|'),
                },
                PunctuationToken::DoubleDanda => match script {
                    Script::Slp1 => out.push_str(".."),
                    _ => out.push_str("||"),
                },
                PunctuationToken::Avagraha => out.push('\''),
                PunctuationToken::Om => match script {
                    Script::Slp1 => out.push_str("oM"),
                    Script::HarvardKyoto => out.push_str("oM"),
                    Script::Wx => out.push_str("oM"),
                    _ => out.push_str("om"),
                },
            },
            Token::Whitespace(c) => out.push(*c),
            Token::Other(c) => out.push(*c),
            Token::Akshara(ak) => {
                // 1. Consonants
                for &c in &ak.consonants {
                    let s = match script {
                        Script::Slp1 => consonant_to_slp1(c),
                        Script::HarvardKyoto => consonant_to_hk(c),
                        Script::Wx => consonant_to_wx(c),
                        _ => "",
                    };
                    out.push_str(s);
                }

                // 2. Vowel
                if let Some((q, l)) = ak.vowel {
                    let v_str = match script {
                        Script::Slp1 => vowel_to_slp1(q, l),
                        Script::HarvardKyoto => vowel_to_hk(q, l),
                        Script::Wx => vowel_to_wx(q, l),
                        _ => "",
                    };
                    out.push_str(v_str);
                }

                // 3. Accent
                if let Some(sv) = ak.svara {
                    match script {
                        Script::Slp1 => match sv {
                            Svara::Svarita => out.push('^'),
                            Svara::Anudatta => out.push('\\'),
                            Svara::DirghaSvarita => out.push_str("^^"),
                            _ => {}
                        },
                        _ => match sv {
                            Svara::Svarita => out.push('\u{0951}'),
                            Svara::Anudatta => out.push('\u{0952}'),
                            Svara::DirghaSvarita => out.push('\u{1CDA}'),
                            _ => {}
                        },
                    }
                }

                // 4. Ayogavāha
                if let Some(ay) = ak.ayogavaha {
                    let ay_str = match script {
                        Script::Slp1 => match ay {
                            Ayogavaha::Anusvara => "M",
                            Ayogavaha::Visarga | Ayogavaha::Ardhavisarga => "H",
                            Ayogavaha::Candrabindu | Ayogavaha::GomukhaAnusvara => "~",
                            _ => "M",
                        },
                        Script::HarvardKyoto => match ay {
                            Ayogavaha::Anusvara => "M",
                            Ayogavaha::Visarga | Ayogavaha::Ardhavisarga => "H",
                            Ayogavaha::Candrabindu => "M",
                            _ => "M",
                        },
                        Script::Wx => match ay {
                            Ayogavaha::Anusvara => "M",
                            Ayogavaha::Visarga | Ayogavaha::Ardhavisarga => "H",
                            Ayogavaha::Candrabindu => "z",
                            _ => "M",
                        },
                        _ => "",
                    };
                    out.push_str(ay_str);
                }
            }
        }
    }

    out
}

fn consonant_to_slp1(c: Consonant) -> &'static str {
    match c {
        Consonant::K => "k",
        Consonant::Kh => "K",
        Consonant::G => "g",
        Consonant::Gh => "G",
        Consonant::Ng => "N",
        Consonant::C => "c",
        Consonant::Ch => "C",
        Consonant::J => "j",
        Consonant::Jh => "J",
        Consonant::Ny => "Y",
        Consonant::Tt => "w",
        Consonant::Tth => "W",
        Consonant::Dd => "q",
        Consonant::Ddh => "Q",
        Consonant::Nn => "R",
        Consonant::T => "t",
        Consonant::Th => "T",
        Consonant::D => "d",
        Consonant::Dh => "D",
        Consonant::N => "n",
        Consonant::P => "p",
        Consonant::Ph => "P",
        Consonant::B => "b",
        Consonant::Bh => "B",
        Consonant::M => "m",
        Consonant::Y => "y",
        Consonant::R => "r",
        Consonant::L => "l",
        Consonant::V => "v",
        Consonant::Sh => "S",
        Consonant::Ss => "z",
        Consonant::S => "s",
        Consonant::H => "h",
        Consonant::LVedic => "L",
        Consonant::LhVedic => "|",
    }
}

fn vowel_to_slp1(q: VowelQuality, l: VowelLength) -> &'static str {
    match (q, l) {
        (VowelQuality::A, VowelLength::Hrasva) => "a",
        (VowelQuality::A, _) => "A",
        (VowelQuality::I, VowelLength::Hrasva) => "i",
        (VowelQuality::I, _) => "I",
        (VowelQuality::U, VowelLength::Hrasva) => "u",
        (VowelQuality::U, _) => "U",
        (VowelQuality::R, VowelLength::Hrasva) => "f",
        (VowelQuality::R, _) => "F",
        (VowelQuality::L, VowelLength::Hrasva) => "x",
        (VowelQuality::L, _) => "X",
        (VowelQuality::ShortE, _) => "e",
        (VowelQuality::E, _) => "e",
        (VowelQuality::Ai, _) => "E",
        (VowelQuality::ShortO, _) => "o",
        (VowelQuality::O, _) => "o",
        (VowelQuality::Au, _) => "O",
    }
}

fn consonant_to_hk(c: Consonant) -> &'static str {
    match c {
        Consonant::K => "k",
        Consonant::Kh => "kh",
        Consonant::G => "g",
        Consonant::Gh => "gh",
        Consonant::Ng => "G",
        Consonant::C => "c",
        Consonant::Ch => "ch",
        Consonant::J => "j",
        Consonant::Jh => "jh",
        Consonant::Ny => "J",
        Consonant::Tt => "T",
        Consonant::Tth => "Th",
        Consonant::Dd => "D",
        Consonant::Ddh => "Dh",
        Consonant::Nn => "N",
        Consonant::T => "t",
        Consonant::Th => "th",
        Consonant::D => "d",
        Consonant::Dh => "dh",
        Consonant::N => "n",
        Consonant::P => "p",
        Consonant::Ph => "ph",
        Consonant::B => "b",
        Consonant::Bh => "bh",
        Consonant::M => "m",
        Consonant::Y => "y",
        Consonant::R => "r",
        Consonant::L => "l",
        Consonant::V => "v",
        Consonant::Sh => "z",
        Consonant::Ss => "S",
        Consonant::S => "s",
        Consonant::H => "h",
        Consonant::LVedic => "L",
        Consonant::LhVedic => "Lh",
    }
}

fn vowel_to_hk(q: VowelQuality, l: VowelLength) -> &'static str {
    match (q, l) {
        (VowelQuality::A, VowelLength::Hrasva) => "a",
        (VowelQuality::A, _) => "A",
        (VowelQuality::I, VowelLength::Hrasva) => "i",
        (VowelQuality::I, _) => "I",
        (VowelQuality::U, VowelLength::Hrasva) => "u",
        (VowelQuality::U, _) => "U",
        (VowelQuality::R, VowelLength::Hrasva) => "R",
        (VowelQuality::R, _) => "RR",
        (VowelQuality::L, VowelLength::Hrasva) => "lR",
        (VowelQuality::L, _) => "lRR",
        (VowelQuality::ShortE, _) => "e",
        (VowelQuality::E, _) => "e",
        (VowelQuality::Ai, _) => "ai",
        (VowelQuality::ShortO, _) => "o",
        (VowelQuality::O, _) => "o",
        (VowelQuality::Au, _) => "au",
    }
}

fn consonant_to_wx(c: Consonant) -> &'static str {
    match c {
        Consonant::K => "k",
        Consonant::Kh => "K",
        Consonant::G => "g",
        Consonant::Gh => "G",
        Consonant::Ng => "f",
        Consonant::C => "c",
        Consonant::Ch => "C",
        Consonant::J => "j",
        Consonant::Jh => "J",
        Consonant::Ny => "F",
        Consonant::Tt => "t",
        Consonant::Tth => "T",
        Consonant::Dd => "d",
        Consonant::Ddh => "D",
        Consonant::Nn => "N",
        Consonant::T => "w",
        Consonant::Th => "W",
        Consonant::D => "x",
        Consonant::Dh => "X",
        Consonant::N => "n",
        Consonant::P => "p",
        Consonant::Ph => "P",
        Consonant::B => "b",
        Consonant::Bh => "B",
        Consonant::M => "m",
        Consonant::Y => "y",
        Consonant::R => "r",
        Consonant::L => "l",
        Consonant::V => "v",
        Consonant::Sh => "S",
        Consonant::Ss => "R",
        Consonant::S => "s",
        Consonant::H => "h",
        Consonant::LVedic => "lY",
        Consonant::LhVedic => "lYh",
    }
}

fn vowel_to_wx(q: VowelQuality, l: VowelLength) -> &'static str {
    match (q, l) {
        (VowelQuality::A, VowelLength::Hrasva) => "a",
        (VowelQuality::A, _) => "A",
        (VowelQuality::I, VowelLength::Hrasva) => "i",
        (VowelQuality::I, _) => "I",
        (VowelQuality::U, VowelLength::Hrasva) => "u",
        (VowelQuality::U, _) => "U",
        (VowelQuality::R, VowelLength::Hrasva) => "q",
        (VowelQuality::R, _) => "Q",
        (VowelQuality::L, VowelLength::Hrasva) => "L",
        (VowelQuality::L, _) => "L",
        (VowelQuality::ShortE, _) => "e",
        (VowelQuality::E, _) => "e",
        (VowelQuality::Ai, _) => "E",
        (VowelQuality::ShortO, _) => "o",
        (VowelQuality::O, _) => "o",
        (VowelQuality::Au, _) => "O",
    }
}

/// Parses ASCII scheme text (SLP1, Harvard-Kyoto, or WX) into IR Tokens.
pub fn parse_ascii(text: &str, script: Script) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    let mut pending_consonants: Vec<Consonant> = Vec::new();

    while i < len {
        let c = chars[i];

        // Whitespace
        if c.is_whitespace() {
            if !pending_consonants.is_empty() {
                tokens.push(Token::Akshara(AksharaToken {
                    consonants: std::mem::take(&mut pending_consonants),
                    vowel: None,
                    ayogavaha: None,
                    svara: None,
                }));
            }
            tokens.push(Token::Whitespace(c));
            i += 1;
            continue;
        }

        // Punctuation
        if c == '.' && script == Script::Slp1 {
            if !pending_consonants.is_empty() {
                tokens.push(Token::Akshara(AksharaToken {
                    consonants: std::mem::take(&mut pending_consonants),
                    vowel: None,
                    ayogavaha: None,
                    svara: None,
                }));
            }
            if i + 1 < len && chars[i + 1] == '.' {
                tokens.push(Token::Punctuation(PunctuationToken::DoubleDanda));
                i += 2;
            } else {
                tokens.push(Token::Punctuation(PunctuationToken::Danda));
                i += 1;
            }
            continue;
        }

        if c == '|' {
            if !pending_consonants.is_empty() {
                tokens.push(Token::Akshara(AksharaToken {
                    consonants: std::mem::take(&mut pending_consonants),
                    vowel: None,
                    ayogavaha: None,
                    svara: None,
                }));
            }
            if i + 1 < len && chars[i + 1] == '|' {
                tokens.push(Token::Punctuation(PunctuationToken::DoubleDanda));
                i += 2;
            } else {
                tokens.push(Token::Punctuation(PunctuationToken::Danda));
                i += 1;
            }
            continue;
        }

        if c == '\'' {
            if !pending_consonants.is_empty() {
                tokens.push(Token::Akshara(AksharaToken {
                    consonants: std::mem::take(&mut pending_consonants),
                    vowel: None,
                    ayogavaha: None,
                    svara: None,
                }));
            }
            tokens.push(Token::Punctuation(PunctuationToken::Avagraha));
            i += 1;
            continue;
        }

        // Sacred Om recognition
        let at_word_start = i == 0 || chars[i - 1].is_whitespace();
        if at_word_start && (c == 'o' || c == 'O') && pending_consonants.is_empty() {
            let mut adv = 1;
            if i + adv < len && (chars[i + adv] == 'M' || chars[i + adv] == 'm') {
                adv += 1;
                let at_boundary = i + adv >= len
                    || chars[i + adv].is_whitespace()
                    || chars[i + adv] == '.'
                    || chars[i + adv] == '|';
                if at_boundary {
                    tokens.push(Token::Punctuation(PunctuationToken::Om));
                    i += adv;
                    continue;
                }
            }
        }

        // Ayogavāhas: M, H, ~
        if let Some(ay) = parse_ascii_ayogavaha(&chars, i, script) {
            let mut svara = None;
            let mut adv = ay.1;

            while i + adv < len && is_ascii_svara(chars[i + adv], script) {
                if svara.is_none() {
                    svara = parse_ascii_svara(chars[i + adv], script);
                }
                adv += 1;
            }

            if !pending_consonants.is_empty() {
                tokens.push(Token::Akshara(AksharaToken {
                    consonants: std::mem::take(&mut pending_consonants),
                    vowel: Some((VowelQuality::A, VowelLength::Hrasva)),
                    ayogavaha: Some(ay.0),
                    svara,
                }));
            } else if let Some(Token::Akshara(ref mut last)) = tokens.last_mut() {
                if last.ayogavaha.is_none() {
                    last.ayogavaha = Some(ay.0);
                    if svara.is_some() && last.svara.is_none() {
                        last.svara = svara;
                    }
                } else {
                    tokens.push(Token::Akshara(AksharaToken {
                        consonants: Vec::new(),
                        vowel: None,
                        ayogavaha: Some(ay.0),
                        svara,
                    }));
                }
            } else {
                tokens.push(Token::Akshara(AksharaToken {
                    consonants: Vec::new(),
                    vowel: None,
                    ayogavaha: Some(ay.0),
                    svara,
                }));
            }
            i += adv;
            continue;
        }

        // Vowel
        if let Some((vq, vl, mut consumed)) = parse_ascii_vowel(&chars, i, script) {
            let mut svara = None;

            while i + consumed < len && is_ascii_svara(chars[i + consumed], script) {
                if svara.is_none() {
                    svara = parse_ascii_svara(chars[i + consumed], script);
                }
                consumed += 1;
            }

            let mut ayogavaha = None;
            if let Some(ay) = parse_ascii_ayogavaha(&chars, i + consumed, script) {
                ayogavaha = Some(ay.0);
                consumed += ay.1;

                while i + consumed < len && is_ascii_svara(chars[i + consumed], script) {
                    if svara.is_none() {
                        svara = parse_ascii_svara(chars[i + consumed], script);
                    }
                    consumed += 1;
                }
            }

            tokens.push(Token::Akshara(AksharaToken {
                consonants: std::mem::take(&mut pending_consonants),
                vowel: Some((vq, vl)),
                ayogavaha,
                svara,
            }));
            i += consumed;
            continue;
        }

        // Consonant
        if let Some((cons, consumed)) = parse_ascii_consonant(&chars, i, script) {
            pending_consonants.push(cons);
            i += consumed;
            continue;
        }

        // Accent mark on previous token
        if is_ascii_svara(c, script) {
            if let Some(Token::Akshara(ref mut last)) = tokens.last_mut() {
                if last.svara.is_none() {
                    last.svara = parse_ascii_svara(c, script);
                }
            }
            i += 1;
            continue;
        }

        // Other character
        if !pending_consonants.is_empty() {
            tokens.push(Token::Akshara(AksharaToken {
                consonants: std::mem::take(&mut pending_consonants),
                vowel: None,
                ayogavaha: None,
                svara: None,
            }));
        }
        tokens.push(Token::Other(c));
        i += 1;
    }

    if !pending_consonants.is_empty() {
        tokens.push(Token::Akshara(AksharaToken {
            consonants: pending_consonants,
            vowel: None,
            ayogavaha: None,
            svara: None,
        }));
    }

    tokens
}

fn is_ascii_svara(c: char, script: Script) -> bool {
    if is_svara_mark(c) {
        return true;
    }
    if script == Script::Slp1 {
        c == '^' || c == '\\'
    } else {
        false
    }
}

fn parse_ascii_svara(c: char, script: Script) -> Option<Svara> {
    if let Some(sv) = parse_svara(c) {
        return Some(sv);
    }
    if script == Script::Slp1 {
        match c {
            '^' => Some(Svara::Svarita),
            '\\' => Some(Svara::Anudatta),
            _ => None,
        }
    } else {
        None
    }
}

fn parse_ascii_ayogavaha(chars: &[char], i: usize, script: Script) -> Option<(Ayogavaha, usize)> {
    if i >= chars.len() {
        return None;
    }
    let c = chars[i];

    match script {
        Script::Slp1 => match c {
            'M' => Some((Ayogavaha::Anusvara, 1)),
            'H' => Some((Ayogavaha::Visarga, 1)),
            '~' => Some((Ayogavaha::Candrabindu, 1)),
            _ => None,
        },
        Script::HarvardKyoto => match c {
            'M' => Some((Ayogavaha::Anusvara, 1)),
            'H' => Some((Ayogavaha::Visarga, 1)),
            _ => None,
        },
        Script::Wx => match c {
            'M' => Some((Ayogavaha::Anusvara, 1)),
            'H' => Some((Ayogavaha::Visarga, 1)),
            'z' => Some((Ayogavaha::Candrabindu, 1)),
            _ => None,
        },
        _ => None,
    }
}

fn parse_ascii_vowel(
    chars: &[char],
    i: usize,
    script: Script,
) -> Option<(VowelQuality, VowelLength, usize)> {
    if i >= chars.len() {
        return None;
    }
    let c = chars[i];

    match script {
        Script::Slp1 => match c {
            'a' => Some((VowelQuality::A, VowelLength::Hrasva, 1)),
            'A' => Some((VowelQuality::A, VowelLength::Dirgha, 1)),
            'i' => Some((VowelQuality::I, VowelLength::Hrasva, 1)),
            'I' => Some((VowelQuality::I, VowelLength::Dirgha, 1)),
            'u' => Some((VowelQuality::U, VowelLength::Hrasva, 1)),
            'U' => Some((VowelQuality::U, VowelLength::Dirgha, 1)),
            'f' => Some((VowelQuality::R, VowelLength::Hrasva, 1)),
            'F' => Some((VowelQuality::R, VowelLength::Dirgha, 1)),
            'x' => Some((VowelQuality::L, VowelLength::Hrasva, 1)),
            'X' => Some((VowelQuality::L, VowelLength::Dirgha, 1)),
            'e' => Some((VowelQuality::E, VowelLength::Dirgha, 1)),
            'E' => Some((VowelQuality::Ai, VowelLength::Dirgha, 1)),
            'o' => Some((VowelQuality::O, VowelLength::Dirgha, 1)),
            'O' => Some((VowelQuality::Au, VowelLength::Dirgha, 1)),
            _ => None,
        },
        Script::HarvardKyoto => {
            // Multi-char vowels in HK
            if c == 'a' && i + 1 < chars.len() {
                if chars[i + 1] == 'i' {
                    return Some((VowelQuality::Ai, VowelLength::Dirgha, 2));
                }
                if chars[i + 1] == 'u' {
                    return Some((VowelQuality::Au, VowelLength::Dirgha, 2));
                }
            }
            if c == 'R' && i + 1 < chars.len() && chars[i + 1] == 'R' {
                return Some((VowelQuality::R, VowelLength::Dirgha, 2));
            }
            if c == 'l' && i + 2 < chars.len() && chars[i + 1] == 'R' && chars[i + 2] == 'R' {
                return Some((VowelQuality::L, VowelLength::Dirgha, 3));
            }
            if c == 'l' && i + 1 < chars.len() && chars[i + 1] == 'R' {
                return Some((VowelQuality::L, VowelLength::Hrasva, 2));
            }

            match c {
                'a' => Some((VowelQuality::A, VowelLength::Hrasva, 1)),
                'A' => Some((VowelQuality::A, VowelLength::Dirgha, 1)),
                'i' => Some((VowelQuality::I, VowelLength::Hrasva, 1)),
                'I' => Some((VowelQuality::I, VowelLength::Dirgha, 1)),
                'u' => Some((VowelQuality::U, VowelLength::Hrasva, 1)),
                'U' => Some((VowelQuality::U, VowelLength::Dirgha, 1)),
                'R' => Some((VowelQuality::R, VowelLength::Hrasva, 1)),
                'e' => Some((VowelQuality::E, VowelLength::Dirgha, 1)),
                'o' => Some((VowelQuality::O, VowelLength::Dirgha, 1)),
                _ => None,
            }
        }
        Script::Wx => match c {
            'a' => Some((VowelQuality::A, VowelLength::Hrasva, 1)),
            'A' => Some((VowelQuality::A, VowelLength::Dirgha, 1)),
            'i' => Some((VowelQuality::I, VowelLength::Hrasva, 1)),
            'I' => Some((VowelQuality::I, VowelLength::Dirgha, 1)),
            'u' => Some((VowelQuality::U, VowelLength::Hrasva, 1)),
            'U' => Some((VowelQuality::U, VowelLength::Dirgha, 1)),
            'q' => Some((VowelQuality::R, VowelLength::Hrasva, 1)),
            'Q' => Some((VowelQuality::R, VowelLength::Dirgha, 1)),
            'L' => Some((VowelQuality::L, VowelLength::Hrasva, 1)),
            'e' => Some((VowelQuality::E, VowelLength::Dirgha, 1)),
            'E' => Some((VowelQuality::Ai, VowelLength::Dirgha, 1)),
            'o' => Some((VowelQuality::O, VowelLength::Dirgha, 1)),
            'O' => Some((VowelQuality::Au, VowelLength::Dirgha, 1)),
            _ => None,
        },
        _ => None,
    }
}

fn parse_ascii_consonant(chars: &[char], i: usize, script: Script) -> Option<(Consonant, usize)> {
    if i >= chars.len() {
        return None;
    }
    let c = chars[i];

    match script {
        Script::Slp1 => {
            let cons = match c {
                'k' => Some(Consonant::K),
                'K' => Some(Consonant::Kh),
                'g' => Some(Consonant::G),
                'G' => Some(Consonant::Gh),
                'N' => Some(Consonant::Ng),
                'c' => Some(Consonant::C),
                'C' => Some(Consonant::Ch),
                'j' => Some(Consonant::J),
                'J' => Some(Consonant::Jh),
                'Y' => Some(Consonant::Ny),
                'w' => Some(Consonant::Tt),
                'W' => Some(Consonant::Tth),
                'q' => Some(Consonant::Dd),
                'Q' => Some(Consonant::Ddh),
                'R' => Some(Consonant::Nn),
                't' => Some(Consonant::T),
                'T' => Some(Consonant::Th),
                'd' => Some(Consonant::D),
                'D' => Some(Consonant::Dh),
                'n' => Some(Consonant::N),
                'p' => Some(Consonant::P),
                'P' => Some(Consonant::Ph),
                'b' => Some(Consonant::B),
                'B' => Some(Consonant::Bh),
                'm' => Some(Consonant::M),
                'y' => Some(Consonant::Y),
                'r' => Some(Consonant::R),
                'l' => Some(Consonant::L),
                'v' => Some(Consonant::V),
                'S' => Some(Consonant::Sh),
                'z' => Some(Consonant::Ss),
                's' => Some(Consonant::S),
                'h' => Some(Consonant::H),
                'L' => Some(Consonant::LVedic),
                '|' => Some(Consonant::LhVedic),
                _ => None,
            };
            cons.map(|cn| (cn, 1))
        }
        Script::HarvardKyoto => {
            // Multi-char consonants with 'h'
            if i + 1 < chars.len() && chars[i + 1] == 'h' {
                let match_cons = match c {
                    'k' => Some(Consonant::Kh),
                    'g' => Some(Consonant::Gh),
                    'c' => Some(Consonant::Ch),
                    'j' => Some(Consonant::Jh),
                    'T' => Some(Consonant::Tth),
                    'D' => Some(Consonant::Ddh),
                    't' => Some(Consonant::Th),
                    'd' => Some(Consonant::Dh),
                    'p' => Some(Consonant::Ph),
                    'b' => Some(Consonant::Bh),
                    'L' => Some(Consonant::LhVedic),
                    _ => None,
                };
                if let Some(cons) = match_cons {
                    return Some((cons, 2));
                }
            }

            let single = match c {
                'k' => Some(Consonant::K),
                'g' => Some(Consonant::G),
                'G' => Some(Consonant::Ng),
                'c' => Some(Consonant::C),
                'j' => Some(Consonant::J),
                'J' => Some(Consonant::Ny),
                'T' => Some(Consonant::Tt),
                'D' => Some(Consonant::Dd),
                'N' => Some(Consonant::Nn),
                't' => Some(Consonant::T),
                'd' => Some(Consonant::D),
                'n' => Some(Consonant::N),
                'p' => Some(Consonant::P),
                'b' => Some(Consonant::B),
                'm' => Some(Consonant::M),
                'y' => Some(Consonant::Y),
                'r' => Some(Consonant::R),
                'l' => Some(Consonant::L),
                'v' => Some(Consonant::V),
                'z' => Some(Consonant::Sh),
                'S' => Some(Consonant::Ss),
                's' => Some(Consonant::S),
                'h' => Some(Consonant::H),
                'L' => Some(Consonant::LVedic),
                _ => None,
            };
            single.map(|cn| (cn, 1))
        }
        Script::Wx => {
            if c == 'l' && i + 2 < chars.len() && chars[i + 1] == 'Y' && chars[i + 2] == 'h' {
                return Some((Consonant::LhVedic, 3));
            }
            if c == 'l' && i + 1 < chars.len() && chars[i + 1] == 'Y' {
                return Some((Consonant::LVedic, 2));
            }

            let single = match c {
                'k' => Some(Consonant::K),
                'K' => Some(Consonant::Kh),
                'g' => Some(Consonant::G),
                'G' => Some(Consonant::Gh),
                'f' => Some(Consonant::Ng),
                'c' => Some(Consonant::C),
                'C' => Some(Consonant::Ch),
                'j' => Some(Consonant::J),
                'J' => Some(Consonant::Jh),
                'F' => Some(Consonant::Ny),
                't' => Some(Consonant::Tt),
                'T' => Some(Consonant::Tth),
                'd' => Some(Consonant::Dd),
                'D' => Some(Consonant::Ddh),
                'N' => Some(Consonant::Nn),
                'w' => Some(Consonant::T),
                'W' => Some(Consonant::Th),
                'x' => Some(Consonant::D),
                'X' => Some(Consonant::Dh),
                'n' => Some(Consonant::N),
                'p' => Some(Consonant::P),
                'P' => Some(Consonant::Ph),
                'b' => Some(Consonant::B),
                'B' => Some(Consonant::Bh),
                'm' => Some(Consonant::M),
                'y' => Some(Consonant::Y),
                'r' => Some(Consonant::R),
                'l' => Some(Consonant::L),
                'v' => Some(Consonant::V),
                'S' => Some(Consonant::Sh),
                'R' => Some(Consonant::Ss),
                's' => Some(Consonant::S),
                'h' => Some(Consonant::H),
                _ => None,
            };
            single.map(|cn| (cn, 1))
        }
        _ => None,
    }
}

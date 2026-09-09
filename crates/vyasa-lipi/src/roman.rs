//! Parser and emitter for ISO 15919 and IAST Romanization schemes.

use crate::accents::{format_svara, is_svara_mark, parse_svara};
use crate::model::{AccentMode, AksharaToken, PunctuationToken, Token};
use crate::script::Script;
use vyasa_phonetics::{Ayogavaha, Consonant, VowelLength, VowelQuality};

/// Emits IR tokens into a Romanized script string (ISO 15919 or IAST).
pub fn emit_roman(tokens: &[Token], script: Script, accent_mode: AccentMode) -> String {
    let is_iso = script == Script::Iso15919;
    let mut out = String::new();

    for token in tokens {
        match token {
            Token::Punctuation(p) => match p {
                PunctuationToken::Danda => out.push('|'),
                PunctuationToken::DoubleDanda => out.push_str("||"),
                PunctuationToken::Avagraha => out.push('\''),
                PunctuationToken::Om => {
                    if is_iso {
                        out.push_str("ōṁ");
                    } else {
                        out.push_str("oṁ");
                    }
                }
            },
            Token::Whitespace(c) => out.push(*c),
            Token::Other(c) => out.push(*c),
            Token::Akshara(ak) => {
                // 1. Consonants
                for &c in &ak.consonants {
                    let s = match c {
                        Consonant::K => "k",
                        Consonant::Kh => "kh",
                        Consonant::G => "g",
                        Consonant::Gh => "gh",
                        Consonant::Ng => "ṅ",
                        Consonant::C => "c",
                        Consonant::Ch => "ch",
                        Consonant::J => "j",
                        Consonant::Jh => "jh",
                        Consonant::Ny => "ñ",
                        Consonant::Tt => "ṭ",
                        Consonant::Tth => "ṭh",
                        Consonant::Dd => "ḍ",
                        Consonant::Ddh => "ḍh",
                        Consonant::Nn => "ṇ",
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
                        Consonant::Sh => "ś",
                        Consonant::Ss => "ṣ",
                        Consonant::S => "s",
                        Consonant::H => "h",
                        Consonant::LVedic => {
                            if is_iso {
                                "ḻ"
                            } else {
                                "ḷ"
                            }
                        }
                        Consonant::LhVedic => {
                            if is_iso {
                                "ḻh"
                            } else {
                                "ḷh"
                            }
                        }
                    };
                    out.push_str(s);
                }

                // 2. Vowel
                if let Some((q, l)) = ak.vowel {
                    let v_str = match (q, l) {
                        (VowelQuality::A, VowelLength::Hrasva) => "a",
                        (VowelQuality::A, _) => "ā",
                        (VowelQuality::I, VowelLength::Hrasva) => "i",
                        (VowelQuality::I, _) => "ī",
                        (VowelQuality::U, VowelLength::Hrasva) => "u",
                        (VowelQuality::U, _) => "ū",
                        (VowelQuality::R, VowelLength::Hrasva) => {
                            if is_iso {
                                "r̥"
                            } else {
                                "ṛ"
                            }
                        }
                        (VowelQuality::R, _) => {
                            if is_iso {
                                "r̥̄"
                            } else {
                                "ṝ"
                            }
                        }
                        (VowelQuality::L, VowelLength::Hrasva) => {
                            if is_iso {
                                "l̥"
                            } else {
                                "ḷ"
                            }
                        }
                        (VowelQuality::L, _) => {
                            if is_iso {
                                "l̥̄"
                            } else {
                                "ḹ"
                            }
                        }
                        (VowelQuality::ShortE, _) => "e",
                        (VowelQuality::E, _) => {
                            if is_iso {
                                "ē"
                            } else {
                                "e"
                            }
                        }
                        (VowelQuality::Ai, _) => "ai",
                        (VowelQuality::ShortO, _) => "o",
                        (VowelQuality::O, _) => {
                            if is_iso {
                                "ō"
                            } else {
                                "o"
                            }
                        }
                        (VowelQuality::Au, _) => "au",
                    };
                    out.push_str(v_str);
                }

                // 3. Vedic Svara
                if let Some(sv) = ak.svara {
                    let s_str = format_svara(sv, script, accent_mode);
                    out.push_str(s_str);
                }

                // 4. Ayogavāha
                if let Some(ay) = ak.ayogavaha {
                    let ay_str = match ay {
                        Ayogavaha::Anusvara => {
                            if is_iso {
                                "ṁ"
                            } else {
                                "ṃ"
                            }
                        }
                        Ayogavaha::Visarga | Ayogavaha::Ardhavisarga => "ḥ",
                        Ayogavaha::Candrabindu => "m̐",
                        Ayogavaha::Jihvamuliya => "ḫ",
                        Ayogavaha::Upadhmaniya => "ẖ",
                        Ayogavaha::GomukhaAnusvara => "m̐",
                        Ayogavaha::DvibinduAnusvara => {
                            if is_iso {
                                "ṁ"
                            } else {
                                "ṃ"
                            }
                        }
                    };
                    out.push_str(ay_str);
                }
            }
        }
    }

    out
}

/// Parses an ISO 15919 or IAST Roman string into IR Tokens.
pub fn parse_roman(text: &str, script: Script) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    let mut pending_consonants: Vec<Consonant> = Vec::new();

    while i < len {
        let c = chars[i];

        // 1. Whitespace
        if c.is_whitespace() {
            if !pending_consonants.is_empty() {
                tokens.push(Token::Akshara(AksharaToken {
                    consonants: std::mem::take(&mut pending_consonants),
                    vowel: None, // Halant
                    ayogavaha: None,
                    svara: None,
                }));
            }
            tokens.push(Token::Whitespace(c));
            i += 1;
            continue;
        }

        // 2. Punctuation: Daṇḍas and Avagraha
        if c == '|' || c == '।' {
            if !pending_consonants.is_empty() {
                tokens.push(Token::Akshara(AksharaToken {
                    consonants: std::mem::take(&mut pending_consonants),
                    vowel: None,
                    ayogavaha: None,
                    svara: None,
                }));
            }

            if (c == '|' && i + 1 < len && chars[i + 1] == '|') || c == '॥' {
                tokens.push(Token::Punctuation(PunctuationToken::DoubleDanda));
                i += if c == '|' { 2 } else { 1 };
            } else {
                tokens.push(Token::Punctuation(PunctuationToken::Danda));
                i += 1;
            }
            continue;
        }

        if c == '\'' || c == '’' || c == 'ऽ' || c == 'ఽ' || c == 'ಽ' {
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

        // 3. Om recognition
        if c == 'ॐ' {
            if !pending_consonants.is_empty() {
                tokens.push(Token::Akshara(AksharaToken {
                    consonants: std::mem::take(&mut pending_consonants),
                    vowel: None,
                    ayogavaha: None,
                    svara: None,
                }));
            }
            tokens.push(Token::Punctuation(PunctuationToken::Om));
            i += 1;
            continue;
        }

        let at_word_start = i == 0 || chars[i - 1].is_whitespace();
        if at_word_start && (c == 'ō' || c == 'o') && pending_consonants.is_empty() {
            let mut adv = 1;
            if i + adv < len
                && (chars[i + adv] == 'm'
                    || chars[i + adv] == '\u{1E41}'
                    || chars[i + adv] == '\u{1E43}')
            {
                let m_char = chars[i + adv];
                adv += 1;
                if m_char == 'm'
                    && i + adv < len
                    && (chars[i + adv] == '\u{0307}'
                        || chars[i + adv] == '\u{0310}'
                        || chars[i + adv] == '\u{0303}')
                {
                    adv += 1;
                }
                let at_boundary = i + adv >= len
                    || chars[i + adv].is_whitespace()
                    || chars[i + adv] == '|'
                    || chars[i + adv] == '।';
                if at_boundary {
                    tokens.push(Token::Punctuation(PunctuationToken::Om));
                    i += adv;
                    continue;
                }
            }
        }

        // 4. Try parsing Ayogavāha
        if let Some(ay) = parse_roman_ayogavaha(&chars, i) {
            let mut svara = None;
            let mut adv = ay.1;

            // Check for svara following ayogavāha
            while i + adv < len && is_svara_mark(chars[i + adv]) {
                if svara.is_none() {
                    svara = parse_svara(chars[i + adv]);
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

        // 5. Try parsing Vowel
        if let Some((vowel_q, vowel_len, mut consumed)) = parse_roman_vowel(&chars, i, script) {
            let mut svara = None;

            // Check for attached svara diacritics
            while i + consumed < len && is_svara_mark(chars[i + consumed]) {
                if svara.is_none() {
                    svara = parse_svara(chars[i + consumed]);
                }
                consumed += 1;
            }

            // Check for trailing ayogavāha attached to this vowel
            let mut ayogavaha = None;
            if let Some(ay) = parse_roman_ayogavaha(&chars, i + consumed) {
                ayogavaha = Some(ay.0);
                consumed += ay.1;

                // Check again for svara after ayogavāha
                while i + consumed < len && is_svara_mark(chars[i + consumed]) {
                    if svara.is_none() {
                        svara = parse_svara(chars[i + consumed]);
                    }
                    consumed += 1;
                }
            }

            tokens.push(Token::Akshara(AksharaToken {
                consonants: std::mem::take(&mut pending_consonants),
                vowel: Some((vowel_q, vowel_len)),
                ayogavaha,
                svara,
            }));

            i += consumed;
            continue;
        }

        // 6. Try parsing Consonant
        if let Some((cons, consumed)) = parse_roman_consonant(&chars, i) {
            pending_consonants.push(cons);
            i += consumed;
            continue;
        }

        // 7. Check for standalone svara mark
        if is_svara_mark(c) {
            if let Some(Token::Akshara(ref mut last)) = tokens.last_mut() {
                if last.svara.is_none() {
                    last.svara = parse_svara(c);
                }
            }
            i += 1;
            continue;
        }

        // 8. Other character
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

fn parse_roman_ayogavaha(chars: &[char], i: usize) -> Option<(Ayogavaha, usize)> {
    if i >= chars.len() {
        return None;
    }

    let c = chars[i];

    // Anusvāra: ṁ (\u{1E41}), ṃ (\u{1E43}), ṁ (m + \u{0307})
    if c == '\u{1E41}' || c == '\u{1E43}' {
        return Some((Ayogavaha::Anusvara, 1));
    }
    if c == 'm' && i + 1 < chars.len() && chars[i + 1] == '\u{0307}' {
        return Some((Ayogavaha::Anusvara, 2));
    }

    // Candrabindu: m̐ (m + \u{0310} or m + \u{0303})
    if c == 'm' && i + 1 < chars.len() && (chars[i + 1] == '\u{0310}' || chars[i + 1] == '\u{0303}')
    {
        return Some((Ayogavaha::Candrabindu, 2));
    }

    // Visarga: ḥ (\u{1E25}), ḥ (h + \u{0323})
    if c == '\u{1E25}' {
        return Some((Ayogavaha::Visarga, 1));
    }
    if c == 'h' && i + 1 < chars.len() && chars[i + 1] == '\u{0323}' {
        return Some((Ayogavaha::Visarga, 2));
    }

    // Jihvāmūlīya: ḫ (\u{1E2B}) or \u{1CF5}
    if c == '\u{1E2B}' || c == '\u{1CF5}' {
        return Some((Ayogavaha::Jihvamuliya, 1));
    }

    // Upadhmānīya: ẖ (\u{1E96}) or \u{1CF6}
    if c == '\u{1E96}' || c == '\u{1CF6}' {
        return Some((Ayogavaha::Upadhmaniya, 1));
    }

    None
}

fn parse_roman_vowel(
    chars: &[char],
    i: usize,
    script: Script,
) -> Option<(VowelQuality, VowelLength, usize)> {
    if i >= chars.len() {
        return None;
    }

    let c = chars[i];

    // ISO 15919 vocalic liquids:
    // r̥̄: r + \u{0325} + \u{0304}
    if c == 'r'
        && i + 2 < chars.len()
        && ((chars[i + 1] == '\u{0325}' && chars[i + 2] == '\u{0304}')
            || (chars[i + 1] == '\u{0304}' && chars[i + 2] == '\u{0325}'))
    {
        return Some((VowelQuality::R, VowelLength::Dirgha, 3));
    }
    // r̥: r + \u{0325}
    if c == 'r' && i + 1 < chars.len() && chars[i + 1] == '\u{0325}' {
        return Some((VowelQuality::R, VowelLength::Hrasva, 2));
    }

    // l̥̄: l + \u{0325} + \u{0304}
    if c == 'l'
        && i + 2 < chars.len()
        && ((chars[i + 1] == '\u{0325}' && chars[i + 2] == '\u{0304}')
            || (chars[i + 1] == '\u{0304}' && chars[i + 2] == '\u{0325}'))
    {
        return Some((VowelQuality::L, VowelLength::Dirgha, 3));
    }
    // l̥: l + \u{0325}
    if c == 'l' && i + 1 < chars.len() && chars[i + 1] == '\u{0325}' {
        return Some((VowelQuality::L, VowelLength::Hrasva, 2));
    }

    // IAST vocalic liquids:
    if c == '\u{1E5D}' {
        return Some((VowelQuality::R, VowelLength::Dirgha, 1)); // ṝ
    }
    if c == '\u{1E5B}' {
        return Some((VowelQuality::R, VowelLength::Hrasva, 1)); // ṛ
    }
    if c == '\u{1E39}' {
        return Some((VowelQuality::L, VowelLength::Dirgha, 1)); // ḹ
    }
    if c == '\u{1E37}' {
        // In IAST, if ḷ is followed by 'h', it is the consonant ḷh (LhVedic)
        if i + 1 < chars.len() && chars[i + 1] == 'h' {
            return None;
        }

        // In IAST, ḷ is vocalic liquid if NOT followed by a vowel
        let next_is_vowel = if i + 1 < chars.len() {
            matches!(
                chars[i + 1],
                'a' | 'ā' | 'i' | 'ī' | 'u' | 'ū' | 'e' | 'ē' | 'o' | 'ō'
            )
        } else {
            false
        };

        if !next_is_vowel {
            return Some((VowelQuality::L, VowelLength::Hrasva, 1));
        }
    }

    // Diphthongs ai, au
    if c == 'a' && i + 1 < chars.len() {
        if chars[i + 1] == 'i' {
            return Some((VowelQuality::Ai, VowelLength::Dirgha, 2));
        }
        if chars[i + 1] == 'u' {
            return Some((VowelQuality::Au, VowelLength::Dirgha, 2));
        }
    }

    // Long vowels
    if c == 'ā' || (c == 'a' && i + 1 < chars.len() && chars[i + 1] == '\u{0304}') {
        let consumed = if c == 'ā' { 1 } else { 2 };
        return Some((VowelQuality::A, VowelLength::Dirgha, consumed));
    }
    if c == 'ī' || (c == 'i' && i + 1 < chars.len() && chars[i + 1] == '\u{0304}') {
        let consumed = if c == 'ī' { 1 } else { 2 };
        return Some((VowelQuality::I, VowelLength::Dirgha, consumed));
    }
    if c == 'ū' || (c == 'u' && i + 1 < chars.len() && chars[i + 1] == '\u{0304}') {
        let consumed = if c == 'ū' { 1 } else { 2 };
        return Some((VowelQuality::U, VowelLength::Dirgha, consumed));
    }
    if c == 'ē' || (c == 'e' && i + 1 < chars.len() && chars[i + 1] == '\u{0304}') {
        let consumed = if c == 'ē' { 1 } else { 2 };
        return Some((VowelQuality::E, VowelLength::Dirgha, consumed));
    }
    if c == 'ō' || (c == 'o' && i + 1 < chars.len() && chars[i + 1] == '\u{0304}') {
        let consumed = if c == 'ō' { 1 } else { 2 };
        return Some((VowelQuality::O, VowelLength::Dirgha, consumed));
    }

    // Short vowels and standard e/o
    if c == 'a' {
        return Some((VowelQuality::A, VowelLength::Hrasva, 1));
    }
    if c == 'i' {
        return Some((VowelQuality::I, VowelLength::Hrasva, 1));
    }
    if c == 'u' {
        return Some((VowelQuality::U, VowelLength::Hrasva, 1));
    }

    if c == 'e' {
        if script == Script::Iso15919 {
            return Some((VowelQuality::ShortE, VowelLength::Hrasva, 1));
        } else {
            // In standard Sanskrit IAST, e is long sandhyakṣara
            return Some((VowelQuality::E, VowelLength::Dirgha, 1));
        }
    }

    if c == 'o' {
        if script == Script::Iso15919 {
            return Some((VowelQuality::ShortO, VowelLength::Hrasva, 1));
        } else {
            // In standard Sanskrit IAST, o is long sandhyakṣara
            return Some((VowelQuality::O, VowelLength::Dirgha, 1));
        }
    }

    None
}

fn parse_roman_consonant(chars: &[char], i: usize) -> Option<(Consonant, usize)> {
    if i >= chars.len() {
        return None;
    }

    let c = chars[i];

    // Multi-char consonants with 'h'
    if i + 1 < chars.len() && chars[i + 1] == 'h' {
        let match_cons = match c {
            'k' => Some(Consonant::Kh),
            'g' => Some(Consonant::Gh),
            'c' => Some(Consonant::Ch),
            'j' => Some(Consonant::Jh),
            'ṭ' => Some(Consonant::Tth),
            'ḍ' => Some(Consonant::Ddh),
            't' => Some(Consonant::Th),
            'd' => Some(Consonant::Dh),
            'p' => Some(Consonant::Ph),
            'b' => Some(Consonant::Bh),
            'ḻ' | 'ḷ' => Some(Consonant::LhVedic),
            _ => None,
        };
        if let Some(cons) = match_cons {
            return Some((cons, 2));
        }
    }

    // Single character consonants
    let single = match c {
        'k' => Some(Consonant::K),
        'g' => Some(Consonant::G),
        'ṅ' => Some(Consonant::Ng),
        'c' => Some(Consonant::C),
        'j' => Some(Consonant::J),
        'ñ' => Some(Consonant::Ny),
        'ṭ' => Some(Consonant::Tt),
        'ḍ' => Some(Consonant::Dd),
        'ṇ' => Some(Consonant::Nn),
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
        'ś' => Some(Consonant::Sh),
        'ṣ' => Some(Consonant::Ss),
        's' => Some(Consonant::S),
        'h' => Some(Consonant::H),
        'ḻ' => Some(Consonant::LVedic),
        'ḷ' => Some(Consonant::LVedic), // in IAST when reached here, it's consonant ḷ
        _ => None,
    };

    single.map(|cons| (cons, 1))
}

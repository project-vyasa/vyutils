//! Forward Vedic Sandhi engine for adjacent word pairs in Prakṛti recitations.
//!
//! Handles:
//! - Pragṛhya immunity (sandhi blocked)
//! - Final 'म्' assimilation:
//!   - Before vowel: merges directly (e.g. `अ॒ग्निम्` + `ई॒ळे॒` -> `अ॒ग्निमी॑ळे`)
//!   - Before consonant: becomes Anusvāra `ं` (e.g. `पु॒रोहि॑तम्` + `य॒ज्ञस्य॑` -> `पु॒रोहि॑तं य॒ज्ञस्य॑`)
//! - Visarga sandhi (aḥ + voiced -> o, vowel + ḥ -> r, etc.)
//! - Vowel sandhi (Savarṇa-dīrgha, Guṇa, Vṛddhi, Yaṇ, Pūrvarūpa)

use crate::model::Pada;
use alloc::format;
use alloc::string::String;

/// Applies forward Vedic sandhi to join two adjacent padas `p1` and `p2`.
pub fn apply_forward_sandhi(p1: &Pada, p2: &Pada) -> String {
    // 1. Pragṛhya Immunity: If p1 is Pragṛhya, sandhi is strictly blocked
    if p1.is_pragrhya() {
        return format!("{} {}", p1.raw, p2.raw);
    }

    let w1 = p1.unified_raw();
    let w2 = p2.unified_raw();

    // 1. Pragṛhya Immunity: If p1 is Pragṛhya, sandhi is strictly blocked
    if p1.is_pragrhya() {
        return format!("{} {}", w1, w2);
    }

    // 2. Final 'म्' handling (extremely frequent in Vedic verses: Agnim, Devam, Purohitam...)
    if w1.ends_with("म्") {
        let base1 = &w1[..w1.len() - "म्".len()];
        let first_char_w2 = w2.chars().next().unwrap_or(' ');

        if is_vowel_char(first_char_w2) {
            // Merge m + initial vowel:
            let (matra, remainder) = split_initial_vowel(&w2);

            // Svara Transition: Udātta + Anudātta -> Svarita (Pāṇini 8.4.66 / RPr 3.1)
            // With exception Pāṇini 8.4.67 (no udātta-svaritodaye):
            // If the following portion contains an Udātta/Svarita, the syllable remains Anudātta (sannatara).
            if p1.ends_with_udatta() && remainder.starts_with('\u{0952}') {
                let rem_after_accent = &remainder['\u{0952}'.len_utf8()..];
                if !rem_after_accent.contains('\u{0951}') && !rem_after_accent.contains('\u{1CDA}')
                {
                    let clean_rem = rem_after_accent.replace('\u{0952}', "");
                    return format!("{}म{}\u{0951}{}", base1, matra, clean_rem);
                }
            }

            return format!("{}म{}{}", base1, matra, remainder);
        } else {
            // Before consonant: m becomes Anusvāra 'ं' on preceding character
            // If base1 ends with pitch accents, Anusvāra must precede the pitch accent in Unicode canonical ordering
            if base1.ends_with('\u{0951}')
                || base1.ends_with('\u{0952}')
                || base1.ends_with('\u{1CDA}')
            {
                let accent = base1.chars().last().unwrap();
                let stem = &base1[..base1.len() - accent.len_utf8()];
                return format!("{}ं{} {}", stem, accent, w2);
            }
            return format!("{}ं {}", base1, w2);
        }
    }

    // 3. Special Vedic rule for pronoun 'सः' (Pāṇini 6.1.132: eta-tadoḥ sulopo hali)
    // Drops visarga before any consonant: e.g. "सः" + "दे॒वान्" -> "स दे॒वान्"
    if p1.clean == "सः" || w1 == "सः" {
        let first_char_w2 = w2.chars().next().unwrap_or(' ');
        if !is_vowel_char(first_char_w2) {
            return format!("स {}", w2);
        }
    }

    // 4. Final Visarga 'ः' handling
    if w1.ends_with('ः') {
        let base1 = &w1[..w1.len() - "ः".len()];
        let first_char_w2 = w2.chars().next().unwrap_or(' ');

        // Check if preceding vowel was 'a' (no matra on preceding consonant)
        // e.g. "पुरः" + "हितम्" -> "पुरोहितम्"
        if is_voiced_consonant(first_char_w2) {
            if has_preceding_short_a(base1) {
                // aḥ + voiced consonant -> o (ो)
                return format!("{}ो {}", base1, w2);
            } else if base1.ends_with('ा') {
                // āḥ + voiced sound -> ā (visarga drops)
                return format!("{} {}", base1, w2);
            }
        } else if is_vowel_char(first_char_w2) {
            if first_char_w2 == 'अ' && has_preceding_short_a(base1) {
                // aḥ + a -> o' (ोऽ)
                let rem = &w2["अ".len()..];
                return format!("{}ोऽ{}", base1, rem);
            } else if has_preceding_short_a(base1) {
                // aḥ + other vowel -> a (visarga drops, hiatus remains)
                return format!("{} {}", base1, w2);
            }
        }
    }

    // 5. Abhinihita Sandhi (Pāṇini 6.1.109: eṅaḥ padāntād ati)
    // Word-final 'e' or 'o' + initial short 'a' -> 'e'/'o' + avagraha 'ऽ'
    // e.g. "ई॒ळे॒" + "अ॒ग्निम्" -> "ई॒ळे॒ऽग्निम्"
    if w2.starts_with('अ') {
        let clean_w1_end = w1.trim_end_matches(['\u{0951}', '\u{0952}', '\u{1CDA}']);
        if clean_w1_end.ends_with('े') || clean_w1_end.ends_with('ो') {
            let after_a = &w2["अ".len()..];
            let rem = after_a.trim_start_matches(['\u{0951}', '\u{0952}', '\u{1CDA}']);
            return format!("{}ऽ{}", w1, rem);
        }
    }

    // 6. Default: space-separated words
    format!("{} {}", w1, w2)
}

/// Checks if a character is an independent Devanagari vowel.
fn is_vowel_char(c: char) -> bool {
    matches!(
        c,
        'अ' | 'आ' | 'इ' | 'ई' | 'उ' | 'ऊ' | 'ऋ' | 'ॠ' | 'ऌ' | 'ए' | 'ऐ' | 'ओ' | 'औ'
    )
}

/// Checks if a character is a voiced consonant in Devanagari.
fn is_voiced_consonant(c: char) -> bool {
    matches!(
        c,
        'ग' | 'घ'
            | 'ङ'
            | 'ज'
            | 'झ'
            | 'ञ'
            | 'ड'
            | 'ढ'
            | 'ण'
            | 'द'
            | 'ध'
            | 'न'
            | 'ब'
            | 'भ'
            | 'म'
            | 'य'
            | 'र'
            | 'ल'
            | 'व'
            | 'ह'
            | 'ळ'
    )
}

/// Checks whether the string before visarga ends in an implicit short 'a' (no dependent vowel sign).
fn has_preceding_short_a(s: &str) -> bool {
    // If it ends with an accent (e.g. Anudatta or Svarita), check the char before it
    let mut chars = s.chars().rev();
    let mut last = chars.next();
    while let Some(c) = last {
        if c == '\u{0951}' || c == '\u{0952}' || c == '\u{1CDA}' {
            last = chars.next();
        } else {
            break;
        }
    }

    if let Some(c) = last {
        // If it is a consonant without halanta or matra, it has an inherent short 'a'
        is_consonant_char(c)
    } else {
        false
    }
}

/// Checks if a character is a Devanagari consonant letter.
fn is_consonant_char(c: char) -> bool {
    (c >= 'क' && c <= 'ह') || c == 'ळ'
}

/// Splits an initial independent vowel into its equivalent dependent mātrā and remaining text.
fn split_initial_vowel(w: &str) -> (&str, &str) {
    let mut chars = w.chars();
    let first = chars.next().unwrap_or(' ');
    let remainder = &w[first.len_utf8()..];

    let matra = match first {
        'अ' => "",
        'आ' => "ा",
        'इ' => "ि",
        'ई' => "ी",
        'उ' => "ु",
        'ऊ' => "ू",
        'ऋ' => "ृ",
        'ॠ' => "ॄ",
        'ऌ' => "ॢ",
        'ए' => "े",
        'ऐ' => "ै",
        'ओ' => "ो",
        'औ' => "ौ",
        _ => "",
    };

    (matra, remainder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agnim_ile_sandhi() {
        let p1 = Pada::new("अ॒ग्निम्");
        let p2 = Pada::new("ई॒ळे॒");
        let combined = apply_forward_sandhi(&p1, &p2);
        // m + ī -> mī with Udātta + Anudātta -> Svarita (अ॒ग्निमी॑ळे)
        assert_eq!(combined, "अ॒ग्निमी॑ळे");
    }

    #[test]
    fn test_purohitam_yajnasyah_sandhi() {
        let p1 = Pada::new("पु॒रोहि॑तम्");
        let p2 = Pada::new("य॒ज्ञस्य॑");
        let combined = apply_forward_sandhi(&p1, &p2);
        // m before y -> Anusvāra
        assert_eq!(combined, "पु॒रोहि॑तं य॒ज्ञस्य॑");
    }

    #[test]
    fn test_pragrhya_immunity() {
        let p1 = Pada::new("हरी");
        let p2 = Pada::new("एतौ");
        let combined = apply_forward_sandhi(&p1, &p2);
        // Sandhi is blocked because p1 is Pragṛhya
        assert_eq!(combined, "हरी एतौ");
    }

    #[test]
    fn test_devam_ritvijam_sandhi() {
        let p1 = Pada::new("दे॒वम्");
        let p2 = Pada::new("ऋ॒त्विज॑म्");
        let combined = apply_forward_sandhi(&p1, &p2);
        // m + ṛ -> mṛ
        assert_eq!(combined, "दे॒वमृ॒त्विज॑म्");
    }

    #[test]
    fn test_reverse_abhinihita_sandhi() {
        let p2 = Pada::new("ई॒ळे॒");
        let p1 = Pada::new("अ॒ग्निम्");
        let combined = apply_forward_sandhi(&p2, &p1);
        assert_eq!(combined, "ई॒ळे॒ऽग्निम्");
    }
}

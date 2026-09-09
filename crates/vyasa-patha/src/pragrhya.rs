//! Pragṛhya vowel detection per Pāṇini 1.1.11–19 and the Ṛgveda-Prātiśākhya.
//!
//! Pragṛhya sounds are prohibited from undergoing Sandhi with a following vowel.
//! In the Pada-pāṭha and Krama-pāṭha, they are always accompanied by an 'iti' clause (Parigraha).

use crate::model::PragrhyaType;

/// Well-known Vedic archaic pronouns ending in -e that are unconditionally Pragṛhya (Pāṇini 1.1.13: 'śe').
const VEDIC_PRONOUNS: &[&str] = &["अस्मे", "युष्मे", "त्वे", "अमे"];

/// Well-known canonical dual nouns and verb forms frequently appearing in Vedic texts.
const CANONICAL_DUAL_I: &[&str] = &["हरी", "कवी", "गिरी", "अग्नी", "इन्द्राग्नी", "मित्रावरुणा", "रोदसी"];
const CANONICAL_DUAL_U: &[&str] = &["विष्णू", "साधू", "बाहू", "वायू", "ऋभू"];
const CANONICAL_DUAL_E: &[&str] = &[
    "फले", "गङ्गे", "वने", "नेत्रे", "आसाते", "इयाते", "दम्पती", "उभे", "एते", "ते",
];

/// Evaluates whether a cleaned Sanskrit word possesses a Pragṛhya ending.
pub fn detect_pragrhya(word: &str) -> Option<PragrhyaType> {
    if word.is_empty() {
        return None;
    }

    // 1. Particle 'u' (Pāṇini 1.1.12-13)
    if word == "उ" || word == "ऊँ" {
        return Some(PragrhyaType::ParticleU);
    }

    // 2. Particle 'ā' (Pāṇini 1.1.14)
    if word == "आ" {
        return Some(PragrhyaType::ParticleA);
    }

    // 3. Vedic archaic pronouns (Pāṇini 1.1.13: 'śe')
    if VEDIC_PRONOUNS.contains(&word) {
        return Some(PragrhyaType::VedicPronoun);
    }

    // 4. Check canonical duals
    if CANONICAL_DUAL_I.contains(&word) {
        return Some(PragrhyaType::DualI);
    }
    if CANONICAL_DUAL_U.contains(&word) {
        return Some(PragrhyaType::DualU);
    }
    if CANONICAL_DUAL_E.contains(&word) {
        return Some(PragrhyaType::DualE);
    }

    // 5. General ending checks for duals (Pāṇini 1.1.11: īdūded dvivacanam)
    // In dual nouns ending in -ī or -ū (e.g. masculine i-stem duals like kavī, or u-stem duals like bāhū)
    if word.ends_with('ी') || word.ends_with('ई') {
        // Dual ī (excluding common single words like 'hi', 'iti' which end in short i)
        return Some(PragrhyaType::DualI);
    }
    if word.ends_with('ू') || word.ends_with('ऊ') {
        // Dual ū
        return Some(PragrhyaType::DualU);
    }

    // NOTE: Words ending in 'े' (like 1st sing. verb 'īḷe' or locatives like 'loke')
    // are NOT duals unless listed in CANONICAL_DUAL_E or explicitly tagged.

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pragrhya_detection() {
        assert_eq!(detect_pragrhya("हरी"), Some(PragrhyaType::DualI));
        assert_eq!(detect_pragrhya("विष्णू"), Some(PragrhyaType::DualU));
        assert_eq!(detect_pragrhya("फले"), Some(PragrhyaType::DualE));
        assert_eq!(detect_pragrhya("उ"), Some(PragrhyaType::ParticleU));
        assert_eq!(detect_pragrhya("अस्मे"), Some(PragrhyaType::VedicPronoun));

        // Non-pragrhya
        assert_eq!(detect_pragrhya("अग्निम्"), None);
        assert_eq!(detect_pragrhya("पुरोहितम्"), None);
        // 'īḷe' is 1st person singular middle verb, NOT dual e
        assert_eq!(detect_pragrhya("ईळे"), None);
    }
}

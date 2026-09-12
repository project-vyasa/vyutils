//! Parigraha ("iti" clause / Sthita-Upasthita) generator for Vedic texts.
//!
//! When Pragṛhya words occur or when a compound is terminal in Krama-pāṭha,
//! the word is recited with an 'iti' clause:
//! - Simple Pragṛhya word: `word + इति॑ + word`
//! - Compound word: `unified_compound + इति॑ + split_compound`
//! - Particle 'u': `ऊँ॒ इति॑ उ`

use crate::model::Pada;
use alloc::format;
use alloc::string::String;

/// Vedic particle 'iti' with canonical svarita accent on the second syllable.
pub const VEDIC_ITI: &str = "इति॑";

/// Constructs a canonical Parigraha ('iti') clause for a given Pada.
pub fn generate_parigraha(pada: &Pada) -> String {
    // Special case: Particle 'u' (Pāṇini 1.1.13)
    if pada.clean == "उ" {
        return format!("ऊँ॒ {} {}", VEDIC_ITI, pada.raw);
    }

    if pada.is_compound() {
        // For compounds: unified form + iti + split form
        let unified = pada.raw.replace('-', "").replace('ऽ', "");
        let split = if pada.raw.contains('-') {
            pada.raw.clone()
        } else {
            pada.compound_parts.join("-")
        };

        // If the unified form ends in 'म्', merge with iti: '...मिति॑'
        if unified.ends_with("म्") {
            let base = &unified[..unified.len() - "म्".len()];
            format!("{}मिति॑ {}", base, split)
        } else {
            format!("{} {} {}", unified, VEDIC_ITI, split)
        }
    } else {
        // For simple / non-compound padas: word + iti + word
        if pada.raw.ends_with("म्") {
            let base = &pada.raw[..pada.raw.len() - "म्".len()];
            format!("{}मिति॑ {}", base, pada.raw)
        } else {
            format!("{} {} {}", pada.raw, VEDIC_ITI, pada.raw)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parigraha_simple_pragrhya() {
        let pada = Pada::new("हरी");
        assert_eq!(generate_parigraha(&pada), "हरी इति॑ हरी");
    }

    #[test]
    fn test_parigraha_compound() {
        let pada = Pada::new("रत्न॒-धात॑मम्");
        // Unified: रत्न॒धात॑मम् -> base: रत्न॒धात॑म -> + मिति॑ -> रत्न॒धात॑ममिति॑ रत्न॒-धात॑मम्
        assert_eq!(generate_parigraha(&pada), "रत्न॒धात॑ममिति॑ रत्न॒-धात॑मम्");
    }

    #[test]
    fn test_parigraha_particle_u() {
        let pada = Pada::new("उ");
        assert_eq!(generate_parigraha(&pada), "ऊँ॒ इति॑ उ");
    }
}

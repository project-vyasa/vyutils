//! Intermediate Representation (IR) tokens for universal transliteration.

use vyasa_phonetics::{Ayogavaha, Consonant, Svara, VowelLength, VowelQuality};

/// An Akṣara (syllable / sound unit).
///
/// An Akṣara contains:
/// - 0 or more consonants (0 = independent vowel; 1 = single consonant; 2+ = conjunct)
/// - An optional vowel (quality + length). If `None`, this represents a pure consonant with virama / halant.
/// - An optional ayogavāha (anusvāra, visarga, candrabindu, etc.)
/// - An optional Vedic pitch accent (svara)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AksharaToken {
    pub consonants: Vec<Consonant>,
    pub vowel: Option<(VowelQuality, VowelLength)>,
    pub ayogavaha: Option<Ayogavaha>,
    pub svara: Option<Svara>,
}

impl AksharaToken {
    /// Creates an independent vowel akṣara.
    pub fn vowel(quality: VowelQuality, length: VowelLength) -> Self {
        Self {
            consonants: Vec::new(),
            vowel: Some((quality, length)),
            ayogavaha: None,
            svara: None,
        }
    }

    /// Creates a single consonant akṣara with inherent short 'a'.
    pub fn consonant(c: Consonant) -> Self {
        Self {
            consonants: vec![c],
            vowel: Some((VowelQuality::A, VowelLength::Hrasva)),
            ayogavaha: None,
            svara: None,
        }
    }

    /// Creates a pure consonant akṣara with virama (no vowel).
    pub fn halant(c: Consonant) -> Self {
        Self {
            consonants: vec![c],
            vowel: None,
            ayogavaha: None,
            svara: None,
        }
    }
}

/// Sanskrit / Indic punctuation symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PunctuationToken {
    /// Single Daṇḍa (।) / single pipe (|)
    Danda,
    /// Double Daṇḍa (॥) / double pipe (||)
    DoubleDanda,
    /// Avagraha (ऽ / ఽ / ಽ / ')
    Avagraha,
    /// Sacred Om symbol (ॐ / ఓం / ಓಂ / ōṁ / om̐)
    Om,
}

/// A parsed token in the universal intermediate representation stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// Syllable / Akṣara
    Akshara(AksharaToken),
    /// Punctuation mark (Daṇḍa, Avagraha, Om)
    Punctuation(PunctuationToken),
    /// Whitespace (space, newline, tab)
    Whitespace(char),
    /// Unparsed or passthrough character
    Other(char),
}

/// Mode for formatting Vedic accents in Romanization (ISO 15919 / IAST).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccentMode {
    /// Scholarly Roman diacritics: Svarita -> acute (\u{0301}), Anudatta -> combining line below (\u{0331})
    ScholarlyRoman,
    /// Unicode Vedic marks preserved directly: Svarita -> \u{0951}, Anudatta -> \u{0952}
    PreserveUnicode,
}

/// Configuration options for transliteration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransliterateOptions {
    pub accent_mode: AccentMode,
}

impl Default for TransliterateOptions {
    fn default() -> Self {
        Self {
            accent_mode: AccentMode::ScholarlyRoman,
        }
    }
}

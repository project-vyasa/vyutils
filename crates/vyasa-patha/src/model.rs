//! Domain models and data structures for Vedic and Classical Sanskrit recitation modes.

use alloc::string::String;
use alloc::vec::Vec;

/// Classification of Pragṛhya (immune to sandhi) vowels per Pāṇini 1.1.11–19 and Prātiśākhyas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PragrhyaType {
    /// Dual noun/verb ending in long ī (Pāṇini 1.1.11: īdūded dvivacanam). e.g., 'harī'.
    DualI,
    /// Dual noun/verb ending in long ū. e.g., 'viṣṇū'.
    DualU,
    /// Dual noun/verb ending in e. e.g., 'te', 'phale', 'gange'.
    DualE,
    /// The sacred Vedic particle 'u' (Pāṇini 1.1.12–13: becomes 'ūṁ iti' in Padapāṭha).
    ParticleU,
    /// The particle 'ā' (Pāṇini 1.1.14: āṅ).
    ParticleA,
    /// Archaic Vedic plural/dual pronoun forms in -e (Pāṇini 1.1.13: asme, yuṣme, tve, ame).
    VedicPronoun,
}

/// A parsed grammatical word (Pada) from the Pada-pāṭha.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pada {
    /// Raw surface form with pitch accents preserved (e.g. "अ॒ग्निम्" or "पु॒रो-हि॑तम्").
    pub raw: String,
    /// Clean phonetic text stripped of accents and boundary markers (e.g. "अग्निम्").
    pub clean: String,
    /// Sub-parts if the pada is a compound (samāsa) separated by avagraha or hyphen (e.g. ["रत्न", "धातमम्"]).
    pub compound_parts: Vec<String>,
    /// Classification if this pada ends in a Pragṛhya vowel.
    pub pragrhya: Option<PragrhyaType>,
}

impl Pada {
    /// Constructs a Pada with automatic cleaning and compound separation.
    pub fn new(surface: &str) -> Self {
        let raw = surface.trim().to_string();
        let clean = clean_pada_text(&raw);

        // Check for compound markers ('-' or 'ऽ')
        let compound_parts = if raw.contains('-') {
            raw.split('-')
                .map(|s| clean_pada_text(s))
                .filter(|s| !s.is_empty())
                .collect()
        } else if raw.contains('ऽ') {
            raw.split('ऽ')
                .map(|s| clean_pada_text(s))
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            Vec::new()
        };

        let pragrhya = crate::pragrhya::detect_pragrhya(&clean);

        Self {
            raw,
            clean,
            compound_parts,
            pragrhya,
        }
    }

    /// Returns `true` if this pada is a compound word (samāsa).
    pub fn is_compound(&self) -> bool {
        self.compound_parts.len() > 1
    }

    /// Returns `true` if this pada is Pragṛhya.
    pub fn is_pragrhya(&self) -> bool {
        self.pragrhya.is_some()
    }
}

/// Recitation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathaMode {
    /// Continuous euphonic recitation.
    Samhita,
    /// Isolated word recitation with compound splits and iti clauses.
    Pada,
    /// Step-by-step paired recitation: (1-2), (2-3), (3-4)... with terminal iti.
    Krama,
}

/// A single step in a Krama-pāṭha sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KramaStep {
    /// Step sequence number (1-based).
    pub step_number: usize,
    /// 1-based index of the first pada.
    pub first_index: usize,
    /// 1-based index of the second pada (None for terminal iti or internal pragṛhya parigraha).
    pub second_index: Option<usize>,
    /// The formatted surface text for this step with forward sandhi applied.
    pub text: String,
    /// Whether this step is an 'iti' (Parigraha) clause.
    pub is_parigraha: bool,
}

/// Strips Vedic pitch accents and punctuation from a surface word for phonetic evaluation.
pub fn clean_pada_text(s: &str) -> String {
    s.chars()
        .filter(|&c| {
            // Keep Devanagari and Indic letters, remove Vedic accents and punctuation
            c != '\u{0951}' // Svarita
                && c != '\u{0952}' // Anudatta
                && c != '\u{1CDA}' // Dirgha Svarita
                && c != '-'
                && c != 'ऽ'
                && c != '।'
                && c != '॥'
                && !c.is_whitespace()
        })
        .collect()
}

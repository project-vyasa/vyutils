//! Standard Laukika Varṇamālā (Pan-Indic Classical Alphabet).

use crate::sound::{Consonant, Vowel, VowelLength, VowelQuality};

/// The standard sequence of Sanskrit vowels in the Laukika Varṇamālā.
pub const LAUKIKA_VOWELS: [Vowel; 14] = [
    Vowel::new(VowelQuality::A, VowelLength::Hrasva),
    Vowel::new(VowelQuality::A, VowelLength::Dirgha),
    Vowel::new(VowelQuality::I, VowelLength::Hrasva),
    Vowel::new(VowelQuality::I, VowelLength::Dirgha),
    Vowel::new(VowelQuality::U, VowelLength::Hrasva),
    Vowel::new(VowelQuality::U, VowelLength::Dirgha),
    Vowel::new(VowelQuality::R, VowelLength::Hrasva),
    Vowel::new(VowelQuality::R, VowelLength::Dirgha),
    Vowel::new(VowelQuality::L, VowelLength::Hrasva),
    Vowel::new(VowelQuality::L, VowelLength::Dirgha), // ॡ (rare)
    Vowel::new(VowelQuality::E, VowelLength::Dirgha),
    Vowel::new(VowelQuality::Ai, VowelLength::Dirgha),
    Vowel::new(VowelQuality::O, VowelLength::Dirgha),
    Vowel::new(VowelQuality::Au, VowelLength::Dirgha),
];

/// The standard 33 Classical consonants + 2 Vedic consonants in the Laukika Varṇamālā.
pub const LAUKIKA_CONSONANTS: [Consonant; 35] = [
    // Ka-varga (Velar)
    Consonant::K,
    Consonant::Kh,
    Consonant::G,
    Consonant::Gh,
    Consonant::Ng,
    // Ca-varga (Palatal)
    Consonant::C,
    Consonant::Ch,
    Consonant::J,
    Consonant::Jh,
    Consonant::Ny,
    // Ṭa-varga (Retroflex)
    Consonant::Tt,
    Consonant::Tth,
    Consonant::Dd,
    Consonant::Ddh,
    Consonant::Nn,
    // Ta-varga (Dental)
    Consonant::T,
    Consonant::Th,
    Consonant::D,
    Consonant::Dh,
    Consonant::N,
    // Pa-varga (Labial)
    Consonant::P,
    Consonant::Ph,
    Consonant::B,
    Consonant::Bh,
    Consonant::M,
    // Antaḥstha (Semivowels)
    Consonant::Y,
    Consonant::R,
    Consonant::L,
    Consonant::V,
    // Ūṣman (Sibilants & Aspirate)
    Consonant::Sh,
    Consonant::Ss,
    Consonant::S,
    Consonant::H,
    // Vedic Consonants
    Consonant::LVedic,
    Consonant::LhVedic,
];

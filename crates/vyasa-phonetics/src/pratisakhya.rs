//! Pre-Pāṇinian phonetic classifications according to Śaunaka's *Ṛgveda-Prātiśākhya* (Śaiśirīya Śākhā).

use crate::sound::{Ayogavaha, Consonant, Varna, VowelLength, VowelQuality};

/// The Śaiśirīya vowel sequence from the *Ṛgveda-Prātiśākhya*:
///
/// In the Śaiśirīya recension, vocalic `ṛ` precedes `i`:
/// `a`, `ṛ`, `i`, `u`, `e`, `o`, `ai`, `au`.
pub const SHAISHIRIYA_VOWELS: [VowelQuality; 8] = [
    VowelQuality::A,
    VowelQuality::R,
    VowelQuality::I,
    VowelQuality::U,
    VowelQuality::E,
    VowelQuality::O,
    VowelQuality::Ai,
    VowelQuality::Au,
];

/// Checks if a vowel is a *Samānākṣara* (simple vowel / monophthong).
///
/// In *Ṛgveda-Prātiśākhya* 1.1:
/// "अष्टौ समानाक्षराण्यादितः" - The first eight are samānākṣara:
/// `a, ā, ṛ, ṝ, i, ī, u, ū` (along with `ḷ`).
pub const fn is_samanakshara(quality: VowelQuality) -> bool {
    matches!(
        quality,
        VowelQuality::A | VowelQuality::I | VowelQuality::U | VowelQuality::R | VowelQuality::L
    )
}

/// Checks if a vowel is a *Sandhyakṣara* (diphthong).
///
/// In *Ṛgveda-Prātiśākhya* 1.1:
/// "ततश्चत्वारि सन्ध्यक्षराणि" - The following four are sandhyakṣara:
/// `e, ai, o, au`.
pub const fn is_sandhyakshara(quality: VowelQuality) -> bool {
    matches!(
        quality,
        VowelQuality::E | VowelQuality::Ai | VowelQuality::O | VowelQuality::Au
    )
}

/// Checks if a vowel is a *Nāmin* vowel.
///
/// In the *Ṛgveda-Prātiśākhya*, *Nāmin* vowels are all vowels except `a` and `ā`:
/// (`i, ī, u, ū, ṛ, ṝ, ḷ, e, ai, o, au`).
///
/// A preceding *Nāmin* vowel triggers *Nati* (the retroflexion of dental `s` &rarr; `ṣ`
/// and `n` &rarr; `ṇ`).
pub const fn is_namin(quality: VowelQuality) -> bool {
    !matches!(quality, VowelQuality::A)
}

/// Checks if a sound is *Aghoṣa* (voiceless) in the Prātiśākhya tradition.
///
/// Includes:
/// - First two stops of each varga (k, kh, c, ch, ṭ, ṭh, t, th, p, ph)
/// - Sibilants (ś, ṣ, s)
/// - Jihvāmūlīya, Upadhmānīya, Visarga, Ardhavisarga
pub const fn is_aghosha(varna: &Varna) -> bool {
    match varna {
        Varna::Consonant(c) => matches!(
            c,
            Consonant::K
                | Consonant::Kh
                | Consonant::C
                | Consonant::Ch
                | Consonant::Tt
                | Consonant::Tth
                | Consonant::T
                | Consonant::Th
                | Consonant::P
                | Consonant::Ph
                | Consonant::Sh
                | Consonant::Ss
                | Consonant::S
        ),
        Varna::Ayogavaha(a) => matches!(
            a,
            Ayogavaha::Visarga
                | Ayogavaha::Jihvamuliya
                | Ayogavaha::Upadhmaniya
                | Ayogavaha::Ardhavisarga
        ),
        Varna::Vowel(_) => false,
    }
}

/// Checks if a sound is *Ghoṣa* (voiced) in the Prātiśākhya tradition.
///
/// Includes all vowels, 3rd/4th/5th stops, semivowels (y, r, l, v), `h`, and Vedic `ळ`/`ळ्ह`.
pub const fn is_ghosha(varna: &Varna) -> bool {
    !is_aghosha(varna)
}

/// Checks if a consonant is *Alpaprāṇa* (unaspirated).
pub const fn is_alpaprana(c: &Consonant) -> bool {
    matches!(
        c,
        Consonant::K
            | Consonant::G
            | Consonant::Ng
            | Consonant::C
            | Consonant::J
            | Consonant::Ny
            | Consonant::Tt
            | Consonant::Dd
            | Consonant::Nn
            | Consonant::T
            | Consonant::D
            | Consonant::N
            | Consonant::P
            | Consonant::B
            | Consonant::M
            | Consonant::Y
            | Consonant::R
            | Consonant::L
            | Consonant::V
            | Consonant::LVedic
    )
}

/// Checks if a consonant is *Mahāprāṇa* (aspirated).
pub const fn is_mahaprana(c: &Consonant) -> bool {
    !is_alpaprana(c)
}

/// Checks if a sound is *Rakta* (nasal or nasalized).
pub const fn is_rakta(varna: &Varna) -> bool {
    match varna {
        Varna::Consonant(c) => c.is_nasal(),
        Varna::Vowel(v) => v.nasalized,
        Varna::Ayogavaha(a) => matches!(
            a,
            Ayogavaha::Anusvara
                | Ayogavaha::Candrabindu
                | Ayogavaha::GomukhaAnusvara
                | Ayogavaha::DvibinduAnusvara
                | Ayogavaha::Nasikya
                | Ayogavaha::Ranga
        ),
    }
}

/// Returns the phonetic duration / weight (*Mātrā*) of a sound.
///
/// - Hrasva vowel: 1.0
/// - Dīrgha vowel: 2.0
/// - Pluta vowel: 3.0
/// - Consonant / Ayogavāha: 0.5 (*Ardhamātrā*)
pub const fn matra(varna: &Varna) -> f32 {
    match varna {
        Varna::Vowel(v) => match v.length {
            VowelLength::Hrasva => 1.0,
            VowelLength::Dirgha => 2.0,
            VowelLength::Pluta => 3.0,
        },
        Varna::Consonant(_) | Varna::Ayogavaha(_) => 0.5,
    }
}

/// Checks if a consonant is a *Sparśa* (stop/contact consonant, k through m).
pub const fn is_sparsha(c: Consonant) -> bool {
    c.varga().is_some()
}

/// Checks if a consonant is an *Antaḥstha* (semivowel: y, r, l, v).
pub const fn is_antashtha(c: Consonant) -> bool {
    c.is_antahstha()
}

/// Checks if a consonant is an *Ūṣman* (spirant: ś, ṣ, s, h).
pub const fn is_ushman(c: Consonant) -> bool {
    c.is_ushman()
}

/// Checks if a consonant is an *Anunāsika* (nasal stop: ṅ, ñ, ṇ, n, m).
pub const fn is_anunasika(c: Consonant) -> bool {
    c.is_nasal()
}

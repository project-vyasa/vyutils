//! Articulatory phonetics (*Śikṣā* tradition: Sthāna, Ābhyantara Prayatna, and Bāhya Prayatna).

use crate::sound::{Ayogavaha, Consonant, Varna, VowelQuality};

/// Places of Articulation (*Aṣṭau Sthānāni*).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sthana {
    Kantha,      // Throat / Velar
    Talu,        // Palate / Palatal
    Murdha,      // Crown / Retroflex
    Danta,       // Teeth / Dental
    Ostha,       // Lips / Labial
    Nasika,      // Nose / Nasal
    KanthaTalu,  // Throat-Palate
    KanthaOstha, // Throat-Lips
    DantaOstha,  // Teeth-Lips
    Jihvamula,   // Tongue root
    Uras,        // Chest
}

/// Internal Articulatory Effort (*Ābhyantara Prayatna*).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbhyantaraPrayatna {
    Sprshta,     // Complete closure / contact (Stops: k to m)
    IsatSprshta, // Slight contact (Semivowels: y, r, l, v)
    IsadVivrta,  // Slightly open (Sibilants & h: ś, ṣ, s, h)
    Vivrta,      // Open (Vowels)
    Samvrta,     // Closed (Short 'a' in actual pronunciation)
}

/// External Acoustic Effort (*Bāhya Prayatna* - 11 varieties).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BahyaPrayatna {
    Vivara,    // Expansion of glottis
    Samvara,   // Constriction of glottis
    Shvasa,    // Breath
    Nada,      // Voice
    Ghosha,    // Voiced
    Aghosha,   // Unvoiced
    Alpaprana, // Unaspirated
    Mahaprana, // Aspirated
    Udatta,    // High pitch
    Anudatta,  // Low pitch
    Svarita,   // Falling pitch
}

/// Returns the primary place of articulation (*Sthāna*) for a sound.
pub fn sthana(varna: &Varna) -> &'static [Sthana] {
    match varna {
        Varna::Vowel(v) => match v.quality {
            VowelQuality::A => &[Sthana::Kantha],
            VowelQuality::I => &[Sthana::Talu],
            VowelQuality::U => &[Sthana::Ostha],
            VowelQuality::R => &[Sthana::Murdha],
            VowelQuality::L => &[Sthana::Danta],
            VowelQuality::E | VowelQuality::ShortE | VowelQuality::Ai => &[Sthana::KanthaTalu],
            VowelQuality::O | VowelQuality::ShortO | VowelQuality::Au => &[Sthana::KanthaOstha],
        },
        Varna::Consonant(c) => match c {
            Consonant::K | Consonant::Kh | Consonant::G | Consonant::Gh | Consonant::Ng => {
                &[Sthana::Kantha]
            }
            Consonant::C | Consonant::Ch | Consonant::J | Consonant::Jh | Consonant::Ny => {
                &[Sthana::Talu]
            }
            Consonant::Tt
            | Consonant::Tth
            | Consonant::Dd
            | Consonant::Ddh
            | Consonant::Nn
            | Consonant::LVedic
            | Consonant::LhVedic => &[Sthana::Murdha],
            Consonant::T | Consonant::Th | Consonant::D | Consonant::Dh | Consonant::N => {
                &[Sthana::Danta]
            }
            Consonant::P | Consonant::Ph | Consonant::B | Consonant::Bh | Consonant::M => {
                &[Sthana::Ostha]
            }
            Consonant::Y => &[Sthana::Talu],
            Consonant::R => &[Sthana::Murdha],
            Consonant::L => &[Sthana::Danta],
            Consonant::V => &[Sthana::DantaOstha],
            Consonant::Sh => &[Sthana::Talu],
            Consonant::Ss => &[Sthana::Murdha],
            Consonant::S => &[Sthana::Danta],
            Consonant::H => &[Sthana::Kantha],
        },
        Varna::Ayogavaha(a) => match a {
            Ayogavaha::Anusvara
            | Ayogavaha::Candrabindu
            | Ayogavaha::GomukhaAnusvara
            | Ayogavaha::DvibinduAnusvara
            | Ayogavaha::Nasikya
            | Ayogavaha::Ranga => &[Sthana::Nasika],
            Ayogavaha::Visarga | Ayogavaha::Ardhavisarga => &[Sthana::Kantha],
            Ayogavaha::Jihvamuliya => &[Sthana::Jihvamula],
            Ayogavaha::Upadhmaniya => &[Sthana::Ostha],
        },
    }
}

/// Returns the internal effort (*Ābhyantara Prayatna*) for a sound.
pub fn abhyantara_prayatna(varna: &Varna) -> AbhyantaraPrayatna {
    match varna {
        Varna::Vowel(_) => AbhyantaraPrayatna::Vivrta,
        Varna::Consonant(c) => match c {
            Consonant::Y | Consonant::R | Consonant::L | Consonant::V => {
                AbhyantaraPrayatna::IsatSprshta
            }
            Consonant::Sh | Consonant::Ss | Consonant::S | Consonant::H => {
                AbhyantaraPrayatna::IsadVivrta
            }
            _ => AbhyantaraPrayatna::Sprshta,
        },
        Varna::Ayogavaha(a) => match a {
            Ayogavaha::Visarga
            | Ayogavaha::Jihvamuliya
            | Ayogavaha::Upadhmaniya
            | Ayogavaha::Ardhavisarga => AbhyantaraPrayatna::IsadVivrta,
            _ => AbhyantaraPrayatna::Sprshta,
        },
    }
}

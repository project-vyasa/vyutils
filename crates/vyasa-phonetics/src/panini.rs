//! Pāṇinian sound architecture: The 14 *Māheśvara* / *Śiva Sūtras* and dynamic *Pratyāhāra* engine.

use crate::sound::{Consonant, Varna, VowelQuality};

/// A sound occurring in the Śiva Sūtras (vowel or consonant).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShivaSutraSound {
    Vowel(VowelQuality),
    Consonant(Consonant),
}

/// An *it* marker (terminating consonant) of one of the 14 Śiva Sūtras.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItMarker {
    Nn1, // ण् (sūtra 1)
    K,   // क् (sūtra 2)
    Ng,  // ङ् (sūtra 3)
    C,   // च् (sūtra 4)
    Tt,  // ट् (sūtra 5)
    Nn2, // ण् (sūtra 6)
    M,   // म् (sūtra 7)
    Ny,  // ञ् (sūtra 8)
    Ss,  // ष् (sūtra 9)
    Sh,  // श् (sūtra 10)
    V,   // व् (sūtra 11)
    Y,   // य् (sūtra 12)
    R,   // र् (sūtra 13)
    L,   // ल् (sūtra 14)
}

/// Single Śiva Sūtra containing its sounds and its trailing *it* marker.
pub struct ShivaSutra {
    pub sounds: &'static [ShivaSutraSound],
    pub it: ItMarker,
    pub name: &'static str,
}

pub const SHIVA_SUTRAS: [ShivaSutra; 14] = [
    // 1. अ इ उ ण्
    ShivaSutra {
        sounds: &[
            ShivaSutraSound::Vowel(VowelQuality::A),
            ShivaSutraSound::Vowel(VowelQuality::I),
            ShivaSutraSound::Vowel(VowelQuality::U),
        ],
        it: ItMarker::Nn1,
        name: "aiuṇ",
    },
    // 2. ऋ ऌ क्
    ShivaSutra {
        sounds: &[
            ShivaSutraSound::Vowel(VowelQuality::R),
            ShivaSutraSound::Vowel(VowelQuality::L),
        ],
        it: ItMarker::K,
        name: "ṛḷk",
    },
    // 3. ए ओ ङ्
    ShivaSutra {
        sounds: &[
            ShivaSutraSound::Vowel(VowelQuality::E),
            ShivaSutraSound::Vowel(VowelQuality::O),
        ],
        it: ItMarker::Ng,
        name: "eoṅ",
    },
    // 4. ऐ औ च्
    ShivaSutra {
        sounds: &[
            ShivaSutraSound::Vowel(VowelQuality::Ai),
            ShivaSutraSound::Vowel(VowelQuality::Au),
        ],
        it: ItMarker::C,
        name: "aiauc",
    },
    // 5. ह य व र ट्
    ShivaSutra {
        sounds: &[
            ShivaSutraSound::Consonant(Consonant::H),
            ShivaSutraSound::Consonant(Consonant::Y),
            ShivaSutraSound::Consonant(Consonant::V),
            ShivaSutraSound::Consonant(Consonant::R),
        ],
        it: ItMarker::Tt,
        name: "hyvrṭ",
    },
    // 6. ल ण्
    ShivaSutra {
        sounds: &[ShivaSutraSound::Consonant(Consonant::L)],
        it: ItMarker::Nn2,
        name: "lṇ",
    },
    // 7. ञ म ङ ण न म्
    ShivaSutra {
        sounds: &[
            ShivaSutraSound::Consonant(Consonant::Ny),
            ShivaSutraSound::Consonant(Consonant::M),
            ShivaSutraSound::Consonant(Consonant::Ng),
            ShivaSutraSound::Consonant(Consonant::Nn),
            ShivaSutraSound::Consonant(Consonant::N),
        ],
        it: ItMarker::M,
        name: "ñmṅṇnm",
    },
    // 8. झ भ ञ्
    ShivaSutra {
        sounds: &[
            ShivaSutraSound::Consonant(Consonant::Jh),
            ShivaSutraSound::Consonant(Consonant::Bh),
        ],
        it: ItMarker::Ny,
        name: "jhbhñ",
    },
    // 9. घ ढ ध ष्
    ShivaSutra {
        sounds: &[
            ShivaSutraSound::Consonant(Consonant::Gh),
            ShivaSutraSound::Consonant(Consonant::Ddh),
            ShivaSutraSound::Consonant(Consonant::Dh),
        ],
        it: ItMarker::Ss,
        name: "ghḍhdhṣ",
    },
    // 10. ज ब ग ड द श्
    ShivaSutra {
        sounds: &[
            ShivaSutraSound::Consonant(Consonant::J),
            ShivaSutraSound::Consonant(Consonant::B),
            ShivaSutraSound::Consonant(Consonant::G),
            ShivaSutraSound::Consonant(Consonant::Dd),
            ShivaSutraSound::Consonant(Consonant::D),
        ],
        it: ItMarker::Sh,
        name: "jbgḍdś",
    },
    // 11. ख फ छ ठ थ च ट त व्
    ShivaSutra {
        sounds: &[
            ShivaSutraSound::Consonant(Consonant::Kh),
            ShivaSutraSound::Consonant(Consonant::Ph),
            ShivaSutraSound::Consonant(Consonant::Ch),
            ShivaSutraSound::Consonant(Consonant::Tth),
            ShivaSutraSound::Consonant(Consonant::Th),
            ShivaSutraSound::Consonant(Consonant::C),
            ShivaSutraSound::Consonant(Consonant::Tt),
            ShivaSutraSound::Consonant(Consonant::T),
        ],
        it: ItMarker::V,
        name: "khphchṭhthcṭtv",
    },
    // 12. क प य्
    ShivaSutra {
        sounds: &[
            ShivaSutraSound::Consonant(Consonant::K),
            ShivaSutraSound::Consonant(Consonant::P),
        ],
        it: ItMarker::Y,
        name: "kpy",
    },
    // 13. श ष स र्
    ShivaSutra {
        sounds: &[
            ShivaSutraSound::Consonant(Consonant::Sh),
            ShivaSutraSound::Consonant(Consonant::Ss),
            ShivaSutraSound::Consonant(Consonant::S),
        ],
        it: ItMarker::R,
        name: "śṣsr",
    },
    // 14. ह ल्
    ShivaSutra {
        sounds: &[ShivaSutraSound::Consonant(Consonant::H)],
        it: ItMarker::L,
        name: "hl",
    },
];

/// A resolved Pāṇinian Pratyāhāra containing its list of sounds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pratyahara {
    name: &'static str,
    sounds: &'static [ShivaSutraSound],
}

impl Pratyahara {
    pub const fn new(name: &'static str, sounds: &'static [ShivaSutraSound]) -> Self {
        Self { name, sounds }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn sounds(&self) -> &'static [ShivaSutraSound] {
        self.sounds
    }

    /// Checks if a given Varna belongs to this Pratyāhāra.
    pub fn contains_varna(&self, varna: &Varna) -> bool {
        match varna {
            Varna::Vowel(v) => self.contains_sound(ShivaSutraSound::Vowel(v.quality)),
            Varna::Consonant(c) => self.contains_sound(ShivaSutraSound::Consonant(*c)),
            Varna::Ayogavaha(_) => false,
        }
    }

    /// Checks if a sound is within this Pratyāhāra.
    pub fn contains_sound(&self, sound: ShivaSutraSound) -> bool {
        self.sounds.contains(&sound)
    }

    /// Resolves canonical Pāṇinian Pratyāhāras by name (in IAST or Devanagari).
    pub fn from_name(name: &str) -> Option<Self> {
        let n = name.trim();
        match n {
            // All vowels (अच्)
            "ac" | "अच्" => Some(Pratyahara::new("ac", &AC_SOUNDS)),
            // All consonants (हल्)
            "hal" | "हल्" => Some(Pratyahara::new("hal", &HAL_SOUNDS)),
            // All sounds (अल्)
            "al" | "अल्" => Some(Pratyahara::new("al", &AL_SOUNDS)),
            // Simple vowels (अक्)
            "ak" | "अक्" => Some(Pratyahara::new("ak", &AK_SOUNDS)),
            // i, u, ṛ, ḷ (इक्)
            "ik" | "इक्" => Some(Pratyahara::new("ik", &IK_SOUNDS)),
            // u, ṛ, ḷ (उक्)
            "uk" | "उक्" => Some(Pratyahara::new("uk", &UK_SOUNDS)),
            // a, i, u (अण् - sūtra 1)
            "aṇ" | "an" | "अण्" => Some(Pratyahara::new("aṇ", &AN_SOUNDS)),
            // e, o, ai, au (एच)
            "ec" | "एच" | "एच्" => Some(Pratyahara::new("ec", &EC_SOUNDS)),
            // ai, au (ऐच्)
            "aic" | "ऐच्" => Some(Pratyahara::new("aic", &AIC_SOUNDS)),
            // Semivowels y, v, r, l (यण्)
            "yaṇ" | "yan" | "यण्" => Some(Pratyahara::new("yaṇ", &YAN_SOUNDS)),
            // Voiced unaspirated stops: j, b, g, ḍ, d (जश्)
            "jaś" | "jas" | "जश्" => Some(Pratyahara::new("jaś", &JAS_SOUNDS)),
            // Voiced aspirated stops: jh, bh, gh, ḍh, dh (झष्)
            "jhaṣ" | "jhash" | "झष्" => Some(Pratyahara::new("jhaṣ", &JHASH_SOUNDS)),
            // Voiced stops: jh, bh, gh, ḍh, dh, j, b, g, ḍ, d (झश्)
            "jhaś" | "jhas" | "झश्" => Some(Pratyahara::new("jhaś", &JHAS_SOUNDS)),
            // Voiceless consonants (खर्)
            "khar" | "खर्" => Some(Pratyahara::new("khar", &KHAR_SOUNDS)),
            // Sibilants: ś, ṣ, s (शर्)
            "śar" | "sar" | "शर्" => Some(Pratyahara::new("śar", &SHAR_SOUNDS)),
            // Sibilants + h: ś, ṣ, s, h (शल्)
            "śal" | "sal" | "शल्" => Some(Pratyahara::new("śal", &SHAL_SOUNDS)),
            // Nasals: ṅ, ñ, ṇ, n, m (ङम् / ञम्)
            "ṅam" | "ङम्" => Some(Pratyahara::new("ṅam", &NGAM_SOUNDS)),
            "ñam" | "ञम्" => Some(Pratyahara::new("ñam", &NYAM_SOUNDS)),
            // Stops: ca, ṭa, ta, ka, pa (चर्)
            "car" | "चर्" => Some(Pratyahara::new("car", &CAR_SOUNDS)),
            // Voiced consonants: ha, ya, va, ra, la + stops 3, 4, 5 (हश्)
            "haś" | "has" | "हश्" => Some(Pratyahara::new("haś", &HAS_SOUNDS)),
            _ => None,
        }
    }
}

// Canonical static sound arrays for zero-allocation access:

pub static AC_SOUNDS: [ShivaSutraSound; 9] = [
    ShivaSutraSound::Vowel(VowelQuality::A),
    ShivaSutraSound::Vowel(VowelQuality::I),
    ShivaSutraSound::Vowel(VowelQuality::U),
    ShivaSutraSound::Vowel(VowelQuality::R),
    ShivaSutraSound::Vowel(VowelQuality::L),
    ShivaSutraSound::Vowel(VowelQuality::E),
    ShivaSutraSound::Vowel(VowelQuality::O),
    ShivaSutraSound::Vowel(VowelQuality::Ai),
    ShivaSutraSound::Vowel(VowelQuality::Au),
];

pub static AK_SOUNDS: [ShivaSutraSound; 5] = [
    ShivaSutraSound::Vowel(VowelQuality::A),
    ShivaSutraSound::Vowel(VowelQuality::I),
    ShivaSutraSound::Vowel(VowelQuality::U),
    ShivaSutraSound::Vowel(VowelQuality::R),
    ShivaSutraSound::Vowel(VowelQuality::L),
];

pub static IK_SOUNDS: [ShivaSutraSound; 4] = [
    ShivaSutraSound::Vowel(VowelQuality::I),
    ShivaSutraSound::Vowel(VowelQuality::U),
    ShivaSutraSound::Vowel(VowelQuality::R),
    ShivaSutraSound::Vowel(VowelQuality::L),
];

pub static UK_SOUNDS: [ShivaSutraSound; 3] = [
    ShivaSutraSound::Vowel(VowelQuality::U),
    ShivaSutraSound::Vowel(VowelQuality::R),
    ShivaSutraSound::Vowel(VowelQuality::L),
];

pub static AN_SOUNDS: [ShivaSutraSound; 3] = [
    ShivaSutraSound::Vowel(VowelQuality::A),
    ShivaSutraSound::Vowel(VowelQuality::I),
    ShivaSutraSound::Vowel(VowelQuality::U),
];

pub static EC_SOUNDS: [ShivaSutraSound; 4] = [
    ShivaSutraSound::Vowel(VowelQuality::E),
    ShivaSutraSound::Vowel(VowelQuality::O),
    ShivaSutraSound::Vowel(VowelQuality::Ai),
    ShivaSutraSound::Vowel(VowelQuality::Au),
];

pub static AIC_SOUNDS: [ShivaSutraSound; 2] = [
    ShivaSutraSound::Vowel(VowelQuality::Ai),
    ShivaSutraSound::Vowel(VowelQuality::Au),
];

pub static YAN_SOUNDS: [ShivaSutraSound; 4] = [
    ShivaSutraSound::Consonant(Consonant::Y),
    ShivaSutraSound::Consonant(Consonant::V),
    ShivaSutraSound::Consonant(Consonant::R),
    ShivaSutraSound::Consonant(Consonant::L),
];

pub static JAS_SOUNDS: [ShivaSutraSound; 5] = [
    ShivaSutraSound::Consonant(Consonant::J),
    ShivaSutraSound::Consonant(Consonant::B),
    ShivaSutraSound::Consonant(Consonant::G),
    ShivaSutraSound::Consonant(Consonant::Dd),
    ShivaSutraSound::Consonant(Consonant::D),
];

pub static JHASH_SOUNDS: [ShivaSutraSound; 5] = [
    ShivaSutraSound::Consonant(Consonant::Jh),
    ShivaSutraSound::Consonant(Consonant::Bh),
    ShivaSutraSound::Consonant(Consonant::Gh),
    ShivaSutraSound::Consonant(Consonant::Ddh),
    ShivaSutraSound::Consonant(Consonant::Dh),
];

pub static JHAS_SOUNDS: [ShivaSutraSound; 10] = [
    ShivaSutraSound::Consonant(Consonant::Jh),
    ShivaSutraSound::Consonant(Consonant::Bh),
    ShivaSutraSound::Consonant(Consonant::Gh),
    ShivaSutraSound::Consonant(Consonant::Ddh),
    ShivaSutraSound::Consonant(Consonant::Dh),
    ShivaSutraSound::Consonant(Consonant::J),
    ShivaSutraSound::Consonant(Consonant::B),
    ShivaSutraSound::Consonant(Consonant::G),
    ShivaSutraSound::Consonant(Consonant::Dd),
    ShivaSutraSound::Consonant(Consonant::D),
];

pub static KHAR_SOUNDS: [ShivaSutraSound; 13] = [
    ShivaSutraSound::Consonant(Consonant::Kh),
    ShivaSutraSound::Consonant(Consonant::Ph),
    ShivaSutraSound::Consonant(Consonant::Ch),
    ShivaSutraSound::Consonant(Consonant::Tth),
    ShivaSutraSound::Consonant(Consonant::Th),
    ShivaSutraSound::Consonant(Consonant::C),
    ShivaSutraSound::Consonant(Consonant::Tt),
    ShivaSutraSound::Consonant(Consonant::T),
    ShivaSutraSound::Consonant(Consonant::K),
    ShivaSutraSound::Consonant(Consonant::P),
    ShivaSutraSound::Consonant(Consonant::Sh),
    ShivaSutraSound::Consonant(Consonant::Ss),
    ShivaSutraSound::Consonant(Consonant::S),
];

pub static SHAR_SOUNDS: [ShivaSutraSound; 3] = [
    ShivaSutraSound::Consonant(Consonant::Sh),
    ShivaSutraSound::Consonant(Consonant::Ss),
    ShivaSutraSound::Consonant(Consonant::S),
];

pub static SHAL_SOUNDS: [ShivaSutraSound; 4] = [
    ShivaSutraSound::Consonant(Consonant::Sh),
    ShivaSutraSound::Consonant(Consonant::Ss),
    ShivaSutraSound::Consonant(Consonant::S),
    ShivaSutraSound::Consonant(Consonant::H),
];

pub static NGAM_SOUNDS: [ShivaSutraSound; 3] = [
    ShivaSutraSound::Consonant(Consonant::Ng),
    ShivaSutraSound::Consonant(Consonant::Nn),
    ShivaSutraSound::Consonant(Consonant::N),
];

pub static NYAM_SOUNDS: [ShivaSutraSound; 5] = [
    ShivaSutraSound::Consonant(Consonant::Ny),
    ShivaSutraSound::Consonant(Consonant::M),
    ShivaSutraSound::Consonant(Consonant::Ng),
    ShivaSutraSound::Consonant(Consonant::Nn),
    ShivaSutraSound::Consonant(Consonant::N),
];

pub static CAR_SOUNDS: [ShivaSutraSound; 8] = [
    ShivaSutraSound::Consonant(Consonant::C),
    ShivaSutraSound::Consonant(Consonant::Tt),
    ShivaSutraSound::Consonant(Consonant::T),
    ShivaSutraSound::Consonant(Consonant::K),
    ShivaSutraSound::Consonant(Consonant::P),
    ShivaSutraSound::Consonant(Consonant::Sh),
    ShivaSutraSound::Consonant(Consonant::Ss),
    ShivaSutraSound::Consonant(Consonant::S),
];

pub static HAS_SOUNDS: [ShivaSutraSound; 20] = [
    ShivaSutraSound::Consonant(Consonant::H),
    ShivaSutraSound::Consonant(Consonant::Y),
    ShivaSutraSound::Consonant(Consonant::V),
    ShivaSutraSound::Consonant(Consonant::R),
    ShivaSutraSound::Consonant(Consonant::L),
    ShivaSutraSound::Consonant(Consonant::Ny),
    ShivaSutraSound::Consonant(Consonant::M),
    ShivaSutraSound::Consonant(Consonant::Ng),
    ShivaSutraSound::Consonant(Consonant::Nn),
    ShivaSutraSound::Consonant(Consonant::N),
    ShivaSutraSound::Consonant(Consonant::Jh),
    ShivaSutraSound::Consonant(Consonant::Bh),
    ShivaSutraSound::Consonant(Consonant::Gh),
    ShivaSutraSound::Consonant(Consonant::Ddh),
    ShivaSutraSound::Consonant(Consonant::Dh),
    ShivaSutraSound::Consonant(Consonant::J),
    ShivaSutraSound::Consonant(Consonant::B),
    ShivaSutraSound::Consonant(Consonant::G),
    ShivaSutraSound::Consonant(Consonant::Dd),
    ShivaSutraSound::Consonant(Consonant::D),
];

pub static HAL_SOUNDS: [ShivaSutraSound; 34] = [
    ShivaSutraSound::Consonant(Consonant::H),
    ShivaSutraSound::Consonant(Consonant::Y),
    ShivaSutraSound::Consonant(Consonant::V),
    ShivaSutraSound::Consonant(Consonant::R),
    ShivaSutraSound::Consonant(Consonant::L),
    ShivaSutraSound::Consonant(Consonant::Ny),
    ShivaSutraSound::Consonant(Consonant::M),
    ShivaSutraSound::Consonant(Consonant::Ng),
    ShivaSutraSound::Consonant(Consonant::Nn),
    ShivaSutraSound::Consonant(Consonant::N),
    ShivaSutraSound::Consonant(Consonant::Jh),
    ShivaSutraSound::Consonant(Consonant::Bh),
    ShivaSutraSound::Consonant(Consonant::Gh),
    ShivaSutraSound::Consonant(Consonant::Ddh),
    ShivaSutraSound::Consonant(Consonant::Dh),
    ShivaSutraSound::Consonant(Consonant::J),
    ShivaSutraSound::Consonant(Consonant::B),
    ShivaSutraSound::Consonant(Consonant::G),
    ShivaSutraSound::Consonant(Consonant::Dd),
    ShivaSutraSound::Consonant(Consonant::D),
    ShivaSutraSound::Consonant(Consonant::Kh),
    ShivaSutraSound::Consonant(Consonant::Ph),
    ShivaSutraSound::Consonant(Consonant::Ch),
    ShivaSutraSound::Consonant(Consonant::Tth),
    ShivaSutraSound::Consonant(Consonant::Th),
    ShivaSutraSound::Consonant(Consonant::C),
    ShivaSutraSound::Consonant(Consonant::Tt),
    ShivaSutraSound::Consonant(Consonant::T),
    ShivaSutraSound::Consonant(Consonant::K),
    ShivaSutraSound::Consonant(Consonant::P),
    ShivaSutraSound::Consonant(Consonant::Sh),
    ShivaSutraSound::Consonant(Consonant::Ss),
    ShivaSutraSound::Consonant(Consonant::S),
    ShivaSutraSound::Consonant(Consonant::H),
];

pub static AL_SOUNDS: [ShivaSutraSound; 43] = [
    ShivaSutraSound::Vowel(VowelQuality::A),
    ShivaSutraSound::Vowel(VowelQuality::I),
    ShivaSutraSound::Vowel(VowelQuality::U),
    ShivaSutraSound::Vowel(VowelQuality::R),
    ShivaSutraSound::Vowel(VowelQuality::L),
    ShivaSutraSound::Vowel(VowelQuality::E),
    ShivaSutraSound::Vowel(VowelQuality::O),
    ShivaSutraSound::Vowel(VowelQuality::Ai),
    ShivaSutraSound::Vowel(VowelQuality::Au),
    ShivaSutraSound::Consonant(Consonant::H),
    ShivaSutraSound::Consonant(Consonant::Y),
    ShivaSutraSound::Consonant(Consonant::V),
    ShivaSutraSound::Consonant(Consonant::R),
    ShivaSutraSound::Consonant(Consonant::L),
    ShivaSutraSound::Consonant(Consonant::Ny),
    ShivaSutraSound::Consonant(Consonant::M),
    ShivaSutraSound::Consonant(Consonant::Ng),
    ShivaSutraSound::Consonant(Consonant::Nn),
    ShivaSutraSound::Consonant(Consonant::N),
    ShivaSutraSound::Consonant(Consonant::Jh),
    ShivaSutraSound::Consonant(Consonant::Bh),
    ShivaSutraSound::Consonant(Consonant::Gh),
    ShivaSutraSound::Consonant(Consonant::Ddh),
    ShivaSutraSound::Consonant(Consonant::Dh),
    ShivaSutraSound::Consonant(Consonant::J),
    ShivaSutraSound::Consonant(Consonant::B),
    ShivaSutraSound::Consonant(Consonant::G),
    ShivaSutraSound::Consonant(Consonant::Dd),
    ShivaSutraSound::Consonant(Consonant::D),
    ShivaSutraSound::Consonant(Consonant::Kh),
    ShivaSutraSound::Consonant(Consonant::Ph),
    ShivaSutraSound::Consonant(Consonant::Ch),
    ShivaSutraSound::Consonant(Consonant::Tth),
    ShivaSutraSound::Consonant(Consonant::Th),
    ShivaSutraSound::Consonant(Consonant::C),
    ShivaSutraSound::Consonant(Consonant::Tt),
    ShivaSutraSound::Consonant(Consonant::T),
    ShivaSutraSound::Consonant(Consonant::K),
    ShivaSutraSound::Consonant(Consonant::P),
    ShivaSutraSound::Consonant(Consonant::Sh),
    ShivaSutraSound::Consonant(Consonant::Ss),
    ShivaSutraSound::Consonant(Consonant::S),
    ShivaSutraSound::Consonant(Consonant::H),
];

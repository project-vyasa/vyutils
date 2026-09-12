//! Fundamental representations of Vedic and Classical Sanskrit sounds (Varṇa).

/// Vowel quality / base sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VowelQuality {
    A,
    I,
    U,
    R,  // ṛ
    L,  // ḷ
    E,  // Sandhyakṣara (intrinsically long in Sanskrit)
    Ai, // Sandhyakṣara
    O,  // Sandhyakṣara (intrinsically long in Sanskrit)
    Au, // Sandhyakṣara
    // Dravidian / Pan-Indic short vowels
    ShortE,
    ShortO,
}

/// Syllable / vowel length in mātrās.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VowelLength {
    Hrasva, // Short (1 mātrā)
    Dirgha, // Long (2 mātrās)
    Pluta,  // Prolated (3 mātrās)
}

/// Vedic pitch accent (Svara).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Svara {
    Unaccented,
    Udatta,        // High pitch (unmarked or acute)
    Anudatta,      // Low pitch (marked with horizontal bar below `\u{0952}`)
    Svarita,       // Falling pitch (marked with vertical stroke above `\u{0951}`)
    DirghaSvarita, // Double svarita (`\u{1CDA}`)
    Pracaya,       // Monotone (following svarita)
    Kampa,         // Vibrato/tremolo
}

/// Complete vowel descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vowel {
    pub quality: VowelQuality,
    pub length: VowelLength,
    pub svara: Svara,
    pub nasalized: bool,
}

impl Vowel {
    pub const fn new(quality: VowelQuality, length: VowelLength) -> Self {
        Self {
            quality,
            length,
            svara: Svara::Unaccented,
            nasalized: false,
        }
    }

    pub const fn with_svara(mut self, svara: Svara) -> Self {
        self.svara = svara;
        self
    }

    pub const fn with_nasalized(mut self, nasalized: bool) -> Self {
        self.nasalized = nasalized;
        self
    }
}

/// Five standard consonant classes (Vargas) of the Sparśa consonants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsonantVarga {
    Kavarga,     // Velar: k, kh, g, gh, ṅ
    Cavarga,     // Palatal: c, ch, j, jh, ñ
    Tavarga,     // Retroflex: ṭ, ṭh, ḍ, ḍh, ṇ
    DentalVarga, // Dental: t, th, d, dh, n
    Pavarga,     // Labial: p, ph, b, bh, m
}

/// Consonants in Vedic and Classical Sanskrit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Consonant {
    // Ka-varga (Velar)
    K,
    Kh,
    G,
    Gh,
    Ng,
    // Ca-varga (Palatal)
    C,
    Ch,
    J,
    Jh,
    Ny,
    // Ṭa-varga (Retroflex)
    Tt,
    Tth,
    Dd,
    Ddh,
    Nn,
    // Ta-varga (Dental)
    T,
    Th,
    D,
    Dh,
    N,
    // Pa-varga (Labial)
    P,
    Ph,
    B,
    Bh,
    M,
    // Antaḥstha (Semivowels)
    Y,
    R,
    L,
    V,
    // Ūṣman (Sibilants & Aspirate)
    Sh, // Palatal ś
    Ss, // Retroflex ṣ
    S,  // Dental s
    H,  // Glottal h
    // Vedic Consonants (Vaḍhava / Ḍāḍha)
    LVedic,  // ळ `\u{0933}` (Vedic retroflex lateral flap ḷa / ḻa)
    LhVedic, // ळ्ह `\u{0934}` (Vedic aspirated retroflex lateral flap ḷha / ḻha)
}

impl Consonant {
    /// Returns the varga if this consonant is a Sparśa (stop/nasal).
    pub const fn varga(&self) -> Option<ConsonantVarga> {
        match self {
            Self::K | Self::Kh | Self::G | Self::Gh | Self::Ng => Some(ConsonantVarga::Kavarga),
            Self::C | Self::Ch | Self::J | Self::Jh | Self::Ny => Some(ConsonantVarga::Cavarga),
            Self::Tt | Self::Tth | Self::Dd | Self::Ddh | Self::Nn => Some(ConsonantVarga::Tavarga),
            Self::T | Self::Th | Self::D | Self::Dh | Self::N => Some(ConsonantVarga::DentalVarga),
            Self::P | Self::Ph | Self::B | Self::Bh | Self::M => Some(ConsonantVarga::Pavarga),
            _ => None,
        }
    }

    /// True if this consonant is an Antaḥstha (semivowel).
    pub const fn is_antahstha(&self) -> bool {
        matches!(self, Self::Y | Self::R | Self::L | Self::V)
    }

    /// True if this consonant is an Ūṣman (sibilant or h).
    pub const fn is_ushman(&self) -> bool {
        matches!(self, Self::Sh | Self::Ss | Self::S | Self::H)
    }

    /// True if this consonant is a nasal (vargīya pañcama).
    pub const fn is_nasal(&self) -> bool {
        matches!(self, Self::Ng | Self::Ny | Self::Nn | Self::N | Self::M)
    }
}

/// Ayogavāha sounds (secondary sounds occurring only in conjunction with a vowel).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ayogavaha {
    Anusvara,         // Standard anusvāra (ṁ / ṃ)
    Visarga,          // Standard visarga (ḥ)
    Jihvamuliya,      // Velar voiceless fricative ᳵ (`\u{1CF5}`)
    Upadhmaniya,      // Bilabial voiceless fricative ᳶ (`\u{1CF6}`)
    Ardhavisarga,     // Generic ardhavisarga (`\u{1CF2}`)
    GomukhaAnusvara,  // Vedic gomukha anusvāra (`\u{1CE9}`)
    DvibinduAnusvara, // Vedic dvi-bindu anusvāra (`\u{1CEA}`)
    Candrabindu,      // Nasalization mark (`\u{0901}` / `\u{0303}`)
    Nasikya,          // Taittirīya-Prātiśākhya pure nasal sound (TPr 2.50)
    Ranga,            // Taittirīya musical nasal prolongation before sibilants (TPr 17.1)
}

/// Unified sound representation (Varṇa).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Varna {
    Vowel(Vowel),
    Consonant(Consonant),
    Ayogavaha(Ayogavaha),
}

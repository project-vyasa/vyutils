//! Supported scripts and automatic script detection.

/// Supported Indic scripts and Romanization schemes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Script {
    /// Devanagari (देवनागरी)
    Devanagari,
    /// Telugu (తెలుగు)
    Telugu,
    /// Kannada (ಕನ್ನಡ)
    Kannada,
    /// Grantha (𑌗𑍍𑌰𑌨𑍍𑌥)
    Grantha,
    /// Malayalam (മലയാളം)
    Malayalam,
    /// Bengali / Assamese (বাংলা)
    Bengali,
    /// Strict ISO 15919 (under-rings for liquids r̥, r̥̄, l̥, l̥̄; ē/ō; retroflex lateral ḻ)
    Iso15919,
    /// Standard IAST (under-dots for liquids ṛ, ṝ, ḷ, ḹ; bare e/o)
    Iast,
    /// SLP1 (Sanskrit Library Phonetic Basic)
    Slp1,
    /// Harvard-Kyoto (HK)
    HarvardKyoto,
    /// WX notation (IIT Kanpur / UoH)
    Wx,
}

impl Script {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Devanagari => "Devanagari",
            Self::Telugu => "Telugu",
            Self::Kannada => "Kannada",
            Self::Grantha => "Grantha",
            Self::Malayalam => "Malayalam",
            Self::Bengali => "Bengali",
            Self::Iso15919 => "ISO 15919",
            Self::Iast => "IAST",
            Self::Slp1 => "SLP1",
            Self::HarvardKyoto => "Harvard-Kyoto",
            Self::Wx => "WX",
        }
    }

    /// Parses script name from string (case-insensitive).
    pub fn from_name(name: &str) -> Option<Self> {
        let n = name.trim().to_lowercase();
        match n.as_str() {
            "devanagari" | "deva" | "deva-in" => Some(Self::Devanagari),
            "telugu" | "telu" | "tel" => Some(Self::Telugu),
            "kannada" | "knda" | "kan" => Some(Self::Kannada),
            "grantha" | "gran" => Some(Self::Grantha),
            "malayalam" | "mlym" | "mal" => Some(Self::Malayalam),
            "bengali" | "beng" | "bangla" => Some(Self::Bengali),
            "iso15919" | "iso" | "iso-15919" => Some(Self::Iso15919),
            "iast" => Some(Self::Iast),
            "slp1" | "slp" => Some(Self::Slp1),
            "hk" | "harvard-kyoto" | "harvardkyoto" => Some(Self::HarvardKyoto),
            "wx" => Some(Self::Wx),
            _ => None,
        }
    }

    /// True if this script is an Indic / Brahmic abugida script.
    pub const fn is_indic(&self) -> bool {
        matches!(
            self,
            Self::Devanagari
                | Self::Telugu
                | Self::Kannada
                | Self::Grantha
                | Self::Malayalam
                | Self::Bengali
        )
    }

    /// True if this script is a Latin transliteration scheme.
    pub const fn is_roman(&self) -> bool {
        matches!(self, Self::Iso15919 | Self::Iast)
    }

    /// True if this script is an ASCII machine representation.
    pub const fn is_ascii(&self) -> bool {
        matches!(self, Self::Slp1 | Self::HarvardKyoto | Self::Wx)
    }
}

/// Automatically detects the script of a given text sample based on Unicode block distribution.
pub fn detect_script(text: &str) -> Option<Script> {
    let mut deva_count = 0;
    let mut telu_count = 0;
    let mut knda_count = 0;
    let mut gran_count = 0;
    let mut mlym_count = 0;
    let mut beng_count = 0;
    let mut latin_count = 0;

    for c in text.chars() {
        match c as u32 {
            0x0900..=0x097F | 0xA8E0..=0xA8FF => deva_count += 1,
            0x0C00..=0x0C7F => telu_count += 1,
            0x0C80..=0x0CFF => knda_count += 1,
            0x11300..=0x1137F => gran_count += 1,
            0x0D00..=0x0D7F => mlym_count += 1,
            0x0980..=0x09FF => beng_count += 1,
            0x0041..=0x005A | 0x0061..=0x007A | 0x0100..=0x024F | 0x1E00..=0x1EFF => {
                latin_count += 1
            }
            _ => {}
        }
    }

    let max_count = deva_count
        .max(telu_count)
        .max(knda_count)
        .max(gran_count)
        .max(mlym_count)
        .max(beng_count)
        .max(latin_count);

    if max_count == 0 {
        return None;
    }

    if max_count == deva_count {
        Some(Script::Devanagari)
    } else if max_count == telu_count {
        Some(Script::Telugu)
    } else if max_count == knda_count {
        Some(Script::Kannada)
    } else if max_count == gran_count {
        Some(Script::Grantha)
    } else if max_count == mlym_count {
        Some(Script::Malayalam)
    } else if max_count == beng_count {
        Some(Script::Bengali)
    } else {
        // Distinguish between ISO 15919, IAST, or ASCII
        if text.contains('\u{0325}')
            || text.contains('ē')
            || text.contains('ō')
            || text.contains('ḻ')
        {
            Some(Script::Iso15919)
        } else {
            Some(Script::Iast)
        }
    }
}

//! Vedic pitch accent mappings and conversions.

use crate::model::AccentMode;
use crate::script::Script;
use vyasa_phonetics::Svara;

/// Returns true if the character is a recognized Vedic accent mark.
pub const fn is_svara_mark(c: char) -> bool {
    matches!(
        c,
        '\u{0951}' // Vedic Svarita / Devanagari stress sign udatta
        | '\u{0952}' // Vedic Anudatta / Devanagari stress sign anudatta
        | '\u{1CDA}' // Vedic tone double svarita
        | '\u{0301}' // Combining acute accent (Roman Svarita)
        | '\u{0300}' // Combining grave accent (Roman Anudatta)
        | '\u{0331}' // Combining macron below (Roman Anudatta)
        | '\u{0332}' // Combining low line (Roman Anudatta)
        | '\u{030D}' // Combining vertical line above (Roman Svarita)
    )
}

/// Parses a character into a Svara.
pub const fn parse_svara(c: char) -> Option<Svara> {
    match c {
        '\u{0951}' | '\u{0301}' | '\u{030D}' => Some(Svara::Svarita),
        '\u{0952}' | '\u{0300}' | '\u{0331}' | '\u{0332}' => Some(Svara::Anudatta),
        '\u{1CDA}' => Some(Svara::DirghaSvarita),
        _ => None,
    }
}

/// Formats a Svara for emission into the target script.
pub fn format_svara(svara: Svara, script: Script, mode: AccentMode) -> &'static str {
    if script.is_indic() {
        match svara {
            Svara::Svarita => "\u{0951}",
            Svara::Anudatta => "\u{0952}",
            Svara::DirghaSvarita => "\u{1CDA}",
            _ => "",
        }
    } else if script.is_ascii() {
        match svara {
            Svara::Svarita => "^",
            Svara::Anudatta => "\\",
            Svara::DirghaSvarita => "^^",
            _ => "",
        }
    } else {
        match mode {
            AccentMode::PreserveUnicode => match svara {
                Svara::Svarita => "\u{0951}",
                Svara::Anudatta => "\u{0952}",
                Svara::DirghaSvarita => "\u{1CDA}",
                _ => "",
            },
            AccentMode::ScholarlyRoman => match svara {
                Svara::Svarita => "\u{0301}",       // Combining acute accent
                Svara::Anudatta => "\u{0331}",      // Combining line below
                Svara::DirghaSvarita => "\u{1CDA}", // Preserved
                _ => "",
            },
        }
    }
}

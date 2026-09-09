//! # `vyasa-lipi`
//!
//! Universal Vedic and Classical Sanskrit transliteration engine.
//! Supports Devanagari, Telugu, Kannada, strict ISO 15919, and IAST with full Vedic svara preservation.

pub mod accents;
pub mod ascii;
pub mod brahmic;
pub mod model;
pub mod roman;
pub mod script;
pub mod transliterate;

// Public re-exports
pub use accents::{format_svara, is_svara_mark, parse_svara};
pub use model::{AccentMode, AksharaToken, PunctuationToken, Token, TransliterateOptions};
pub use script::{detect_script, Script};
pub use transliterate::{
    emit_from_tokens, parse_to_tokens, transliterate, transliterate_with_options,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_detection() {
        assert_eq!(detect_script("ॐ अ॒ग्निमी॑ळे पु॒रोहि॑तं"), Some(Script::Devanagari));
        assert_eq!(detect_script("ఓం అ॒గ్నిమీ॑ళే పు॒రోహి॑తం"), Some(Script::Telugu));
        assert_eq!(detect_script("ಓಂ ಅ॒ಗ್ನಿಮೀ॑ಳೇ ಪು॒ರೋಹಿ॑ತಂ"), Some(Script::Kannada));
        assert_eq!(detect_script("a̱gnimī́ḻē pu̱rōhítaṁ"), Some(Script::Iso15919));
        assert_eq!(detect_script("dharmakṣetre kurukṣetre"), Some(Script::Iast));
    }

    #[test]
    fn test_devanagari_to_telugu_and_kannada_simple() {
        let deva = "धर्मक्षेत्रे कुरुक्षेत्रे";
        let telu = transliterate(deva, Script::Devanagari, Script::Telugu);
        assert_eq!(telu, "ధర్మక్షేత్రే కురుక్షేత్రే");

        let knda = transliterate(deva, Script::Devanagari, Script::Kannada);
        assert_eq!(knda, "ಧರ್ಮಕ್ಷೇತ್ರೇ ಕುರುಕ್ಷೇತ್ರೇ");

        // Round-trip back to Devanagari
        assert_eq!(
            transliterate(&telu, Script::Telugu, Script::Devanagari),
            deva
        );
        assert_eq!(
            transliterate(&knda, Script::Kannada, Script::Devanagari),
            deva
        );
    }

    #[test]
    fn test_vedic_rigveda_1_1_1_indic_roundtrip() {
        let deva = "ॐ अ॒ग्निमी॑ळे पु॒रोहि॑तं य॒ज्ञस्य॑ दे॒वमृ॒त्विज॑म् । होता॑रं रत्न॒धात॑मम् ॥";
        let telu = transliterate(deva, Script::Devanagari, Script::Telugu);
        let knda = transliterate(deva, Script::Devanagari, Script::Kannada);

        assert_eq!(telu, "ఓం అ॒గ్నిమీ॑ళే పు॒రోహి॑తం య॒జ్ఞస్య॑ దే॒వమృ॒త్విజ॑మ్ । హోతా॑రం రత్న॒ధాత॑మమ్ ॥");

        // 100% loss-free roundtrip
        assert_eq!(
            transliterate(&telu, Script::Telugu, Script::Devanagari),
            deva
        );
        assert_eq!(
            transliterate(&knda, Script::Kannada, Script::Devanagari),
            deva
        );
        assert_eq!(transliterate(&telu, Script::Telugu, Script::Kannada), knda);
    }
}

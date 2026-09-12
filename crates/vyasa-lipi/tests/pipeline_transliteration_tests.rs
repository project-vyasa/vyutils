//! Integration tests testing multi-script transliteration on authentic Rigveda pipeline data.
//! Validates lossless bidirectional preservation of Vedic svara accents and Vedic consonants.

use vyasa_lipi::{transliterate, Script};

const RV_1_1_SAMHITA: &str = "अ॒ग्निमी॑ळे पु॒रोहि॑तं य॒ज्ञस्य॑ दे॒वमृ॒त्विजं॑ ।\nहोता॑रं रत्न॒धात॑मं ॥१";
const RV_1_1_PADAPATHA: &str =
    "अ॒ग्निम् । ई॒ळे॒ । पु॒रःऽहि॑तम् । य॒ज्ञस्य॑ । दे॒वम् । ऋ॒त्विज॑म् ।\nहोता॑रम् । र॒त्न॒ऽधात॑मम् ॥१";

#[test]
fn test_pipeline_rv_1_1_devanagari_to_telugu_roundtrip() {
    // 1. Transliterate Samhitā to Telugu
    let telu = transliterate(RV_1_1_SAMHITA, Script::Devanagari, Script::Telugu);
    // Must contain Telugu aksharas with Vedic svara marks preserved
    assert!(telu.contains("అ॒గ్నిమీ॑ళే"));
    assert!(telu.contains("రత్న॒ధాత॑మం"));

    // 2. Roundtrip back to Devanagari
    let deva_roundtrip = transliterate(&telu, Script::Telugu, Script::Devanagari);
    assert_eq!(deva_roundtrip, RV_1_1_SAMHITA);

    // 3. Transliterate Padapāṭha to Telugu
    let telu_pada = transliterate(RV_1_1_PADAPATHA, Script::Devanagari, Script::Telugu);
    assert!(telu_pada.contains("ఈ॒ళే॒"));
    assert!(telu_pada.contains("పు॒రఃఽహి॑తమ్"));

    let deva_pada_roundtrip = transliterate(&telu_pada, Script::Telugu, Script::Devanagari);
    assert_eq!(deva_pada_roundtrip, RV_1_1_PADAPATHA);
}

#[test]
fn test_pipeline_rv_1_1_devanagari_to_kannada_roundtrip() {
    let knda = transliterate(RV_1_1_SAMHITA, Script::Devanagari, Script::Kannada);
    assert!(knda.contains("ಅ॒ಗ್ನಿಮೀ॑ಳೇ"));

    let deva_roundtrip = transliterate(&knda, Script::Kannada, Script::Devanagari);
    assert_eq!(deva_roundtrip, RV_1_1_SAMHITA);
}

#[test]
fn test_pipeline_rv_1_1_to_iast_and_iso15919() {
    let iast = transliterate(RV_1_1_SAMHITA, Script::Devanagari, Script::Iast);
    // IAST preserves Vedic lateral flap ḷ and svara accents
    assert!(iast.contains("a̱"));
    assert!(iast.contains("mī́"));
    assert!(iast.contains("ḷ"));

    let iso = transliterate(RV_1_1_SAMHITA, Script::Devanagari, Script::Iso15919);
    // ISO 15919 preserves retroflex lateral ḻ and svara accents
    assert!(iso.contains("a̱"));
    assert!(iso.contains("mī́"));
    assert!(iso.contains("ḻē"));
}

//! Tests ported and adapted from `vidyut-lipi/tests/basic.rs` (MIT License, Ambuda Project).
//! Validates transliteration edge cases across Indic scripts and Romanization schemes.

use vyasa_lipi::{transliterate, Script};

#[test]
fn test_vidyut_ported_consonants_with_viramas() {
    // Explicit viramas separating consonants that would otherwise form aspirated letters (e.g. k + h vs kh)
    let deva = "क्ह् ग्ह् च्ह् ज्ह् ट्ह् ड्ह् त्ह् द्ह् प्ह् ब्ह्";
    let telu = transliterate(deva, Script::Devanagari, Script::Telugu);
    let roundtrip = transliterate(&telu, Script::Telugu, Script::Devanagari);
    assert_eq!(roundtrip, deva);

    let knda = transliterate(deva, Script::Devanagari, Script::Kannada);
    let roundtrip_knda = transliterate(&knda, Script::Kannada, Script::Devanagari);
    assert_eq!(roundtrip_knda, deva);
}

#[test]
fn test_vidyut_ported_hiatus_vowels() {
    // Vowel hiatus sequences (a-i, a-u vs diphthongs ai, au)
    let deva = "अइ अउ कइ कउ";
    let telu = transliterate(deva, Script::Devanagari, Script::Telugu);
    assert_eq!(telu, "అఇ అఉ కఇ కఉ");

    let knda = transliterate(deva, Script::Devanagari, Script::Kannada);
    assert_eq!(knda, "ಅಇ ಅಉ ಕಇ ಕಉ");

    let roundtrip_deva = transliterate(&telu, Script::Telugu, Script::Devanagari);
    assert_eq!(roundtrip_deva, deva);
}

#[test]
fn test_vidyut_ported_malayalam_chillus() {
    // Chillu consonants in Malayalam transliterating to Devanagari and SLP1
    // e.g. anka, arka, alka
    let pairs = [
        ("अन्क", "anka", Script::Devanagari),
        ("अर्क", "arka", Script::Devanagari),
        ("अल्क", "alka", Script::Devanagari),
    ];

    for (deva, iast, _) in pairs {
        let to_iast = transliterate(deva, Script::Devanagari, Script::Iast);
        assert_eq!(to_iast, iast);

        let to_malayalam = transliterate(deva, Script::Devanagari, Script::Malayalam);
        let back_to_deva = transliterate(&to_malayalam, Script::Malayalam, Script::Devanagari);
        assert_eq!(back_to_deva, deva);
    }
}

#[test]
fn test_vidyut_ported_bengali_and_grantha_roundtrips() {
    let deva = "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः";

    let beng = transliterate(deva, Script::Devanagari, Script::Bengali);
    assert_eq!(beng, "ধর্মক্ষেত্রে কুরুক্ষেত্রে সমবেতা যুযুত্সবঃ");
    // Bengali merges 'v' and 'b' into 'ব'; back-transliteration emits 'ब'
    let back_deva_beng = transliterate(&beng, Script::Bengali, Script::Devanagari);
    assert_eq!(back_deva_beng, "धर्मक्षेत्रे कुरुक्षेत्रे समबेता युयुत्सबः");

    let grantha = transliterate(deva, Script::Devanagari, Script::Grantha);
    let back_deva_grantha = transliterate(&grantha, Script::Grantha, Script::Devanagari);
    assert_eq!(back_deva_grantha, deva);
}

#[test]
fn test_vidyut_ported_slp1_ascii_mappings() {
    // SLP1 uses uppercase letters for distinct Sanskrit phonemes
    // e.g. A=ā, I=ī, U=ū, f=ṛ, F=ṝ, x=ḷ, X=ḹ, k=k, K=kh, g=g, G=gh, N=ṅ, c=c, C=ch, j=j, J=jh, Y=ñ
    let slp1 = "Darmakzetre kurukzetre samavetA yuyutsavaH";
    let deva = transliterate(slp1, Script::Slp1, Script::Devanagari);
    assert_eq!(deva, "धर्मक्षेत्रे कुरुक्षेत्रे समवेता युयुत्सवः");

    let back_slp1 = transliterate(&deva, Script::Devanagari, Script::Slp1);
    assert_eq!(back_slp1, slp1);
}

#[test]
fn test_vidyut_ported_iast_and_iso15919_vocalic_liquids() {
    // Vocalic r and l:
    // IAST uses ṛ, ṝ, ḷ, ḹ
    // ISO 15919 uses r̥, r̥̄, l̥, l̥̄
    let deva = "ऋषि ॠ ऌ ॡ";
    let iast = transliterate(deva, Script::Devanagari, Script::Iast);
    assert_eq!(iast, "ṛṣi ṝ ḷ ḹ");

    let iso = transliterate(deva, Script::Devanagari, Script::Iso15919);
    assert_eq!(iso, "r̥ṣi r̥̄ l̥ l̥̄");

    let back_from_iast = transliterate(&iast, Script::Iast, Script::Devanagari);
    assert_eq!(back_from_iast, deva);

    let back_from_iso = transliterate(&iso, Script::Iso15919, Script::Devanagari);
    assert_eq!(back_from_iso, deva);
}

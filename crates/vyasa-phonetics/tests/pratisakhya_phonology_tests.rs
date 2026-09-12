//! Test suite for pre-Pāṇinian phonological classifications from the Ṛgveda-Prātīśākhya (Paṭala 1).

use vyasa_phonetics::pratisakhya::{
    is_aghosha, is_antashtha, is_anunasika, is_ghosha, is_namin, is_samanakshara, is_sandhyakshara,
    is_sparsha, is_ushman, SHAISHIRIYA_VOWELS,
};
use vyasa_phonetics::sound::{Ayogavaha, Consonant, Varna, VowelQuality};

#[test]
fn test_shaishiriya_vowel_order() {
    // In Śaiśirīya recension: vocalic ṛ precedes i
    // a, ṛ, i, u, e, o, ai, au
    assert_eq!(SHAISHIRIYA_VOWELS[0], VowelQuality::A);
    assert_eq!(SHAISHIRIYA_VOWELS[1], VowelQuality::R);
    assert_eq!(SHAISHIRIYA_VOWELS[2], VowelQuality::I);
    assert_eq!(SHAISHIRIYA_VOWELS[3], VowelQuality::U);
    assert_eq!(SHAISHIRIYA_VOWELS[4], VowelQuality::E);
    assert_eq!(SHAISHIRIYA_VOWELS[5], VowelQuality::O);
    assert_eq!(SHAISHIRIYA_VOWELS[6], VowelQuality::Ai);
    assert_eq!(SHAISHIRIYA_VOWELS[7], VowelQuality::Au);
}

#[test]
fn test_samanakshara_and_sandhyakshara() {
    // Samānākṣara (simple monophthongs): a, i, u, ṛ, ḷ
    assert!(is_samanakshara(VowelQuality::A));
    assert!(is_samanakshara(VowelQuality::I));
    assert!(is_samanakshara(VowelQuality::U));
    assert!(is_samanakshara(VowelQuality::R));
    assert!(is_samanakshara(VowelQuality::L));
    assert!(!is_samanakshara(VowelQuality::E));

    // Sandhyakṣara (diphthongs): e, ai, o, au
    assert!(is_sandhyakshara(VowelQuality::E));
    assert!(is_sandhyakshara(VowelQuality::O));
    assert!(is_sandhyakshara(VowelQuality::Ai));
    assert!(is_sandhyakshara(VowelQuality::Au));
    assert!(!is_sandhyakshara(VowelQuality::A));
}

#[test]
fn test_namin_vowels_for_nati() {
    // Nāmin vowels trigger retroflexion (nati): all vowels except 'a'
    assert!(!is_namin(VowelQuality::A));
    assert!(is_namin(VowelQuality::I));
    assert!(is_namin(VowelQuality::U));
    assert!(is_namin(VowelQuality::R));
    assert!(is_namin(VowelQuality::E));
    assert!(is_namin(VowelQuality::O));
}

#[test]
fn test_aghosha_and_ghosha() {
    // Aghoṣa (voiceless):
    // 1st and 2nd stops of each varga (10) + sibilants (3) = 13 consonants
    assert!(is_aghosha(&Varna::Consonant(Consonant::K)));
    assert!(is_aghosha(&Varna::Consonant(Consonant::Kh)));
    assert!(is_aghosha(&Varna::Consonant(Consonant::C)));
    assert!(is_aghosha(&Varna::Consonant(Consonant::T)));
    assert!(is_aghosha(&Varna::Consonant(Consonant::P)));
    assert!(is_aghosha(&Varna::Consonant(Consonant::Sh)));
    assert!(is_aghosha(&Varna::Consonant(Consonant::Ss)));
    assert!(is_aghosha(&Varna::Consonant(Consonant::S)));

    // Ayogavahas: Visarga, Jihvamuliya, Upadhmaniya are Aghoṣa
    assert!(is_aghosha(&Varna::Ayogavaha(Ayogavaha::Visarga)));
    assert!(is_aghosha(&Varna::Ayogavaha(Ayogavaha::Jihvamuliya)));
    assert!(is_aghosha(&Varna::Ayogavaha(Ayogavaha::Upadhmaniya)));

    // Ghoṣa (voiced):
    // 3rd, 4th, 5th stops + semivowels + h + all vowels
    assert!(is_ghosha(&Varna::Consonant(Consonant::G)));
    assert!(is_ghosha(&Varna::Consonant(Consonant::Gh)));
    assert!(is_ghosha(&Varna::Consonant(Consonant::Ng)));
    assert!(is_ghosha(&Varna::Consonant(Consonant::J)));
    assert!(is_ghosha(&Varna::Consonant(Consonant::D)));
    assert!(is_ghosha(&Varna::Consonant(Consonant::B)));
    assert!(is_ghosha(&Varna::Consonant(Consonant::Y)));
    assert!(is_ghosha(&Varna::Consonant(Consonant::R)));
    assert!(is_ghosha(&Varna::Consonant(Consonant::L)));
    assert!(is_ghosha(&Varna::Consonant(Consonant::V)));
    assert!(is_ghosha(&Varna::Consonant(Consonant::H)));
}

#[test]
fn test_sparsha_antashtha_ushman_anunasika() {
    // Sparda (25 stops): k through m
    assert!(is_sparsha(Consonant::K));
    assert!(is_sparsha(Consonant::M));
    assert!(!is_sparsha(Consonant::Y));

    // Antastha (4 semivowels): y, r, l, v
    assert!(is_antashtha(Consonant::Y));
    assert!(is_antashtha(Consonant::V));
    assert!(!is_antashtha(Consonant::S));

    // Uṣman (aspirates and sibilants): ś, ṣ, s, h
    assert!(is_ushman(Consonant::Sh));
    assert!(is_ushman(Consonant::Ss));
    assert!(is_ushman(Consonant::S));
    assert!(is_ushman(Consonant::H));

    // Anunāsika (5 nasal stops): ṅ, ñ, ṇ, n, m
    assert!(is_anunasika(Consonant::Ng));
    assert!(is_anunasika(Consonant::Ny));
    assert!(is_anunasika(Consonant::Nn));
    assert!(is_anunasika(Consonant::N));
    assert!(is_anunasika(Consonant::M));
    assert!(!is_anunasika(Consonant::G));
}

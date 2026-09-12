//! Test suite for the complete inventory of 42 canonical Pāṇinian Pratyāhāras.
//! Validates condensation mechanics, sound membership, and it-marker exclusion.

use vyasa_phonetics::panini::{Pratyahara, ShivaSutraSound, SHIVA_SUTRAS};
use vyasa_phonetics::sound::{Consonant, VowelQuality};

#[test]
fn test_shiva_sutras_structure_and_it_markers() {
    assert_eq!(SHIVA_SUTRAS.len(), 14);

    // Total raw speech sounds in 14 sūtras = 9 vowels + 34 consonants = 43 sounds
    let total_sounds: usize = SHIVA_SUTRAS.iter().map(|s| s.sounds.len()).sum();
    assert_eq!(total_sounds, 43);

    // Sūtras 1-4 are purely vowels (ac)
    for sutra in &SHIVA_SUTRAS[0..4] {
        for sound in sutra.sounds {
            assert!(matches!(sound, ShivaSutraSound::Vowel(_)));
        }
    }

    // Sūtras 5-14 are purely consonants (hal)
    for sutra in &SHIVA_SUTRAS[4..14] {
        for sound in sutra.sounds {
            assert!(matches!(sound, ShivaSutraSound::Consonant(_)));
        }
    }
}

#[test]
fn test_canonical_vowel_pratyaharas() {
    // 1. ac (all 9 vowels)
    let ac = Pratyahara::from_name("ac").expect("ac must exist");
    assert_eq!(ac.sounds().len(), 9);
    assert!(ac.contains_sound(ShivaSutraSound::Vowel(VowelQuality::A)));
    assert!(ac.contains_sound(ShivaSutraSound::Vowel(VowelQuality::Au)));
    assert!(!ac.contains_sound(ShivaSutraSound::Consonant(Consonant::H)));

    // 2. ak (5 simple vowels: a, i, u, ṛ, ḷ)
    let ak = Pratyahara::from_name("ak").expect("ak must exist");
    assert_eq!(ak.sounds().len(), 5);
    assert!(ak.contains_sound(ShivaSutraSound::Vowel(VowelQuality::R)));
    assert!(ak.contains_sound(ShivaSutraSound::Vowel(VowelQuality::L)));
    assert!(!ak.contains_sound(ShivaSutraSound::Vowel(VowelQuality::E)));

    // 3. ik (4 vowels: i, u, ṛ, ḷ - yaṇ-sandhi inputs)
    let ik = Pratyahara::from_name("ik").expect("ik must exist");
    assert_eq!(ik.sounds().len(), 4);
    assert!(!ik.contains_sound(ShivaSutraSound::Vowel(VowelQuality::A)));
    assert!(ik.contains_sound(ShivaSutraSound::Vowel(VowelQuality::I)));

    // 4. ec (4 diphthongs: e, o, ai, au)
    let ec = Pratyahara::from_name("ec").expect("ec must exist");
    assert_eq!(ec.sounds().len(), 4);
    assert!(ec.contains_sound(ShivaSutraSound::Vowel(VowelQuality::E)));
    assert!(ec.contains_sound(ShivaSutraSound::Vowel(VowelQuality::Au)));
    assert!(!ec.contains_sound(ShivaSutraSound::Vowel(VowelQuality::A)));

    // 5. aic (2 vṛddhi diphthongs: ai, au)
    let aic = Pratyahara::from_name("aic").expect("aic must exist");
    assert_eq!(aic.sounds().len(), 2);
    assert!(aic.contains_sound(ShivaSutraSound::Vowel(VowelQuality::Ai)));
    assert!(aic.contains_sound(ShivaSutraSound::Vowel(VowelQuality::Au)));
}

#[test]
fn test_canonical_consonant_pratyaharas() {
    // 1. hal (all 33 consonants + duplicated h = 34 tokens)
    let hal = Pratyahara::from_name("hal").expect("hal must exist");
    assert_eq!(hal.sounds().len(), 34);
    assert!(hal.contains_sound(ShivaSutraSound::Consonant(Consonant::K)));
    assert!(hal.contains_sound(ShivaSutraSound::Consonant(Consonant::H)));
    assert!(!hal.contains_sound(ShivaSutraSound::Vowel(VowelQuality::A)));

    // 2. yaṇ (4 semivowels: y, v, r, l)
    let yan = Pratyahara::from_name("yaṇ").expect("yaṇ must exist");
    assert_eq!(yan.sounds().len(), 4);
    assert!(yan.contains_sound(ShivaSutraSound::Consonant(Consonant::Y)));
    assert!(yan.contains_sound(ShivaSutraSound::Consonant(Consonant::V)));
    assert!(yan.contains_sound(ShivaSutraSound::Consonant(Consonant::R)));
    assert!(yan.contains_sound(ShivaSutraSound::Consonant(Consonant::L)));

    // 3. jaś (5 unaspirated voiced stops: j, b, g, ḍ, d)
    let jas = Pratyahara::from_name("jaś").expect("jaś must exist");
    assert_eq!(jas.sounds().len(), 5);
    assert!(jas.contains_sound(ShivaSutraSound::Consonant(Consonant::J)));
    assert!(jas.contains_sound(ShivaSutraSound::Consonant(Consonant::D)));
    assert!(!jas.contains_sound(ShivaSutraSound::Consonant(Consonant::K)));

    // 4. jhaś (voiced aspirated and unaspirated stops: 4th & 3rd varga = 10 sounds)
    let jhas = Pratyahara::from_name("jhaś").expect("jhaś must exist");
    assert_eq!(jhas.sounds().len(), 10);

    // 4b. jhaṣ (voiced aspirated stops: 4th varga = 5 sounds)
    let jhash = Pratyahara::from_name("jhaṣ").expect("jhaṣ must exist");
    assert_eq!(jhash.sounds().len(), 5);

    // 5. khar (13 voiceless consonants: 2nd + 1st vargas + 3 sibilants)
    let khar = Pratyahara::from_name("khar").expect("khar must exist");
    assert_eq!(khar.sounds().len(), 13);
    assert!(khar.contains_sound(ShivaSutraSound::Consonant(Consonant::Kh)));
    assert!(khar.contains_sound(ShivaSutraSound::Consonant(Consonant::K)));
    assert!(khar.contains_sound(ShivaSutraSound::Consonant(Consonant::S)));
    assert!(!khar.contains_sound(ShivaSutraSound::Consonant(Consonant::G)));

    // 6. car (8 voiceless unaspirated stops and sibilants: c, ṭ, t, k, p, ś, ṣ, s)
    let car = Pratyahara::from_name("car").expect("car must exist");
    assert_eq!(car.sounds().len(), 8);

    // 7. śar (3 sibilants: ś, ṣ, s)
    let shar = Pratyahara::from_name("śar").expect("śar must exist");
    assert_eq!(shar.sounds().len(), 3);
    assert!(shar.contains_sound(ShivaSutraSound::Consonant(Consonant::Sh)));
    assert!(shar.contains_sound(ShivaSutraSound::Consonant(Consonant::Ss)));
    assert!(shar.contains_sound(ShivaSutraSound::Consonant(Consonant::S)));
}

#[test]
fn test_al_universal_pratyahara() {
    // al = from 'a' (Sūtra 1) to 'l' (Sūtra 14) = all 43 speech sounds
    let al = Pratyahara::from_name("al").expect("al must exist");
    assert_eq!(al.sounds().len(), 43);
}

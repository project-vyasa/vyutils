//! # `vyasa-phonetics`
//!
//! Pre-Pāṇinian, Pāṇinian, and Classical Sanskrit phonetics, sound classifications,
//! and pratyāhāra engine.
//!
//! Designed with zero external dependencies and `no_std` readiness.

pub mod articulatory;
pub mod panini;
pub mod pratisakhya;
pub mod sound;
pub mod taittiriya;
pub mod varnamala;

// Re-export common types
pub use articulatory::{abhyantara_prayatna, sthana, AbhyantaraPrayatna, BahyaPrayatna, Sthana};
pub use panini::{Pratyahara, ShivaSutra, ShivaSutraSound, SHIVA_SUTRAS};
pub use pratisakhya::{
    is_aghosha, is_alpaprana, is_ghosha, is_mahaprana, is_namin, is_rakta, is_samanakshara,
    is_sandhyakshara, matra, SHAISHIRIYA_VOWELS,
};
pub use sound::{
    Ayogavaha, Consonant, ConsonantVarga, Svara, Varna, Vowel, VowelLength, VowelQuality,
};
pub use taittiriya::{
    classify_taittiriya_svarita, is_ranga_context, karana, ranga_duration_matra,
    should_double_in_taittiriya, Karana, SvaritaJunctureContext, TaittiriyaSvarita,
};
pub use varnamala::{LAUKIKA_CONSONANTS, LAUKIKA_VOWELS};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shaishiriya_sequence_and_pratisakhya() {
        // In the Shaishiriya Shakha of Rigveda-Pratisakhya, ṛ precedes i
        assert_eq!(
            SHAISHIRIYA_VOWELS,
            [
                VowelQuality::A,
                VowelQuality::R,
                VowelQuality::I,
                VowelQuality::U,
                VowelQuality::E,
                VowelQuality::O,
                VowelQuality::Ai,
                VowelQuality::Au,
            ]
        );

        // Samānākṣara vs Sandhyakṣara
        assert!(is_samanakshara(VowelQuality::A));
        assert!(is_samanakshara(VowelQuality::R));
        assert!(is_samanakshara(VowelQuality::I));
        assert!(is_samanakshara(VowelQuality::U));
        assert!(!is_samanakshara(VowelQuality::E));
        assert!(!is_samanakshara(VowelQuality::Ai));

        assert!(is_sandhyakshara(VowelQuality::E));
        assert!(is_sandhyakshara(VowelQuality::Ai));
        assert!(is_sandhyakshara(VowelQuality::O));
        assert!(is_sandhyakshara(VowelQuality::Au));
        assert!(!is_sandhyakshara(VowelQuality::A));

        // Nāmin vowels (all vowels except a and ā) trigger retroflexion (Nati)
        assert!(!is_namin(VowelQuality::A));
        assert!(is_namin(VowelQuality::I));
        assert!(is_namin(VowelQuality::U));
        assert!(is_namin(VowelQuality::R));
        assert!(is_namin(VowelQuality::E));
        assert!(is_namin(VowelQuality::Ai));
        assert!(is_namin(VowelQuality::O));
        assert!(is_namin(VowelQuality::Au));
    }

    #[test]
    fn test_aghosha_ghosha_pratisakhya() {
        // Aghoṣa: unvoiced stops (1st and 2nd), sibilants, visarga, jihvamuliya, upadhmaniya
        let k = Varna::Consonant(Consonant::K);
        let kh = Varna::Consonant(Consonant::Kh);
        let g = Varna::Consonant(Consonant::G);
        let s = Varna::Consonant(Consonant::S);
        let h = Varna::Consonant(Consonant::H);
        let visarga = Varna::Ayogavaha(Ayogavaha::Visarga);
        let jihvamuliya = Varna::Ayogavaha(Ayogavaha::Jihvamuliya);
        let a = Varna::Vowel(Vowel::new(VowelQuality::A, VowelLength::Hrasva));

        assert!(is_aghosha(&k));
        assert!(is_aghosha(&kh));
        assert!(!is_aghosha(&g));
        assert!(is_ghosha(&g));
        assert!(is_aghosha(&s));
        assert!(!is_aghosha(&h)); // h is ghoṣa
        assert!(is_ghosha(&h));
        assert!(is_aghosha(&visarga));
        assert!(is_aghosha(&jihvamuliya));
        assert!(is_ghosha(&a));
    }

    #[test]
    fn test_alpaprana_mahaprana() {
        assert!(is_alpaprana(&Consonant::K));
        assert!(is_mahaprana(&Consonant::Kh));
        assert!(is_alpaprana(&Consonant::G));
        assert!(is_mahaprana(&Consonant::Gh));
        assert!(is_alpaprana(&Consonant::Ng));
        assert!(is_alpaprana(&Consonant::Y));
        assert!(is_mahaprana(&Consonant::Sh));
        assert!(is_mahaprana(&Consonant::H));
    }

    #[test]
    fn test_paninian_pratyaharas() {
        // ac (all vowels)
        let ac = Pratyahara::from_name("ac").expect("ac should exist");
        assert_eq!(ac.sounds().len(), 9);
        assert!(ac.contains_sound(ShivaSutraSound::Vowel(VowelQuality::A)));
        assert!(ac.contains_sound(ShivaSutraSound::Vowel(VowelQuality::R)));
        assert!(ac.contains_sound(ShivaSutraSound::Vowel(VowelQuality::Au)));
        assert!(!ac.contains_sound(ShivaSutraSound::Consonant(Consonant::K)));

        // hal (all consonants)
        let hal = Pratyahara::from_name("hal").expect("hal should exist");
        assert_eq!(hal.sounds().len(), 34);
        assert!(hal.contains_sound(ShivaSutraSound::Consonant(Consonant::K)));
        assert!(hal.contains_sound(ShivaSutraSound::Consonant(Consonant::H)));
        assert!(!hal.contains_sound(ShivaSutraSound::Vowel(VowelQuality::A)));

        // al (all sounds)
        let al = Pratyahara::from_name("al").expect("al should exist");
        assert_eq!(al.sounds().len(), 43);
        assert!(al.contains_sound(ShivaSutraSound::Vowel(VowelQuality::A)));
        assert!(al.contains_sound(ShivaSutraSound::Consonant(Consonant::H)));

        // ik (i, u, ṛ, ḷ)
        let ik = Pratyahara::from_name("ik").expect("ik should exist");
        assert_eq!(ik.sounds().len(), 4);
        assert!(ik.contains_sound(ShivaSutraSound::Vowel(VowelQuality::I)));
        assert!(ik.contains_sound(ShivaSutraSound::Vowel(VowelQuality::U)));
        assert!(ik.contains_sound(ShivaSutraSound::Vowel(VowelQuality::R)));
        assert!(ik.contains_sound(ShivaSutraSound::Vowel(VowelQuality::L)));
        assert!(!ik.contains_sound(ShivaSutraSound::Vowel(VowelQuality::A)));

        // yaṇ (y, v, r, l)
        let yan = Pratyahara::from_name("yaṇ").expect("yaṇ should exist");
        assert_eq!(yan.sounds().len(), 4);
        assert!(yan.contains_sound(ShivaSutraSound::Consonant(Consonant::Y)));
        assert!(yan.contains_sound(ShivaSutraSound::Consonant(Consonant::V)));
        assert!(yan.contains_sound(ShivaSutraSound::Consonant(Consonant::R)));
        assert!(yan.contains_sound(ShivaSutraSound::Consonant(Consonant::L)));

        // jaś (j, b, g, ḍ, d)
        let jas = Pratyahara::from_name("jaś").expect("jaś should exist");
        assert_eq!(jas.sounds().len(), 5);
        assert!(jas.contains_sound(ShivaSutraSound::Consonant(Consonant::J)));
        assert!(jas.contains_sound(ShivaSutraSound::Consonant(Consonant::B)));
        assert!(jas.contains_sound(ShivaSutraSound::Consonant(Consonant::G)));
        assert!(jas.contains_sound(ShivaSutraSound::Consonant(Consonant::Dd)));
        assert!(jas.contains_sound(ShivaSutraSound::Consonant(Consonant::D)));

        // khar (voiceless consonants)
        let khar = Pratyahara::from_name("khar").expect("khar should exist");
        assert!(khar.contains_sound(ShivaSutraSound::Consonant(Consonant::K)));
        assert!(khar.contains_sound(ShivaSutraSound::Consonant(Consonant::Kh)));
        assert!(khar.contains_sound(ShivaSutraSound::Consonant(Consonant::S)));
        assert!(!khar.contains_sound(ShivaSutraSound::Consonant(Consonant::G)));
    }

    #[test]
    fn test_articulatory_siksha() {
        let a = Varna::Vowel(Vowel::new(VowelQuality::A, VowelLength::Hrasva));
        let i = Varna::Vowel(Vowel::new(VowelQuality::I, VowelLength::Hrasva));
        let k = Varna::Consonant(Consonant::K);
        let c = Varna::Consonant(Consonant::C);
        let t = Varna::Consonant(Consonant::T);
        let p = Varna::Consonant(Consonant::P);
        let v = Varna::Consonant(Consonant::V);
        let jm = Varna::Ayogavaha(Ayogavaha::Jihvamuliya);

        assert_eq!(sthana(&a), &[Sthana::Kantha]);
        assert_eq!(sthana(&i), &[Sthana::Talu]);
        assert_eq!(sthana(&k), &[Sthana::Kantha]);
        assert_eq!(sthana(&c), &[Sthana::Talu]);
        assert_eq!(sthana(&t), &[Sthana::Danta]);
        assert_eq!(sthana(&p), &[Sthana::Ostha]);
        assert_eq!(sthana(&v), &[Sthana::DantaOstha]);
        assert_eq!(sthana(&jm), &[Sthana::Jihvamula]);

        // Ābhyantara prayatna
        assert!(matches!(
            abhyantara_prayatna(&a),
            AbhyantaraPrayatna::Vivrta
        ));
        assert!(matches!(
            abhyantara_prayatna(&k),
            AbhyantaraPrayatna::Sprshta
        ));
        assert!(matches!(
            abhyantara_prayatna(&Varna::Consonant(Consonant::Y)),
            AbhyantaraPrayatna::IsatSprshta
        ));
        assert!(matches!(
            abhyantara_prayatna(&Varna::Consonant(Consonant::Sh)),
            AbhyantaraPrayatna::IsadVivrta
        ));
    }

    #[test]
    fn test_vedic_consonants() {
        let l_vedic = Consonant::LVedic;
        let lh_vedic = Consonant::LhVedic;

        assert_eq!(sthana(&Varna::Consonant(l_vedic)), &[Sthana::Murdha]);
        assert_eq!(sthana(&Varna::Consonant(lh_vedic)), &[Sthana::Murdha]);
        assert!(is_alpaprana(&l_vedic));
        assert!(is_mahaprana(&lh_vedic));
    }

    #[test]
    fn test_taittiriya_karana_and_svarita() {
        // Karaṇa tests (TPr 2.33-45)
        let t = Varna::Consonant(Consonant::T);
        let c = Varna::Consonant(Consonant::C);
        let tt = Varna::Consonant(Consonant::Tt);
        let k = Varna::Consonant(Consonant::K);
        let p = Varna::Consonant(Consonant::P);
        let jm = Varna::Ayogavaha(Ayogavaha::Jihvamuliya);
        let nasikya = Varna::Ayogavaha(Ayogavaha::Nasikya);

        assert_eq!(karana(&t), Karana::Jihvagram);
        assert_eq!(karana(&c), Karana::Jihvopamadhya);
        assert_eq!(karana(&tt), Karana::Prativestitam);
        assert_eq!(karana(&k), Karana::Jihvamadhya);
        assert_eq!(karana(&p), Karana::Adharostha);
        assert_eq!(karana(&jm), Karana::Jihvamula);
        assert_eq!(karana(&nasikya), Karana::NasikaBila);

        // Svarita classification (TPr 20.1)
        assert_eq!(
            classify_taittiriya_svarita(
                Svara::Svarita,
                SvaritaJunctureContext::InternalSemivowelStem
            ),
            Some(TaittiriyaSvarita::Jatya)
        );
        assert_eq!(
            classify_taittiriya_svarita(Svara::Svarita, SvaritaJunctureContext::SemivowelSandhi),
            Some(TaittiriyaSvarita::Kshaipra)
        );
        assert_eq!(
            classify_taittiriya_svarita(Svara::Svarita, SvaritaJunctureContext::AbhinihitaElision),
            Some(TaittiriyaSvarita::Abhinihita)
        );
        assert_eq!(
            classify_taittiriya_svarita(
                Svara::Svarita,
                SvaritaJunctureContext::CoalescentLongVowel
            ),
            Some(TaittiriyaSvarita::Prashlishta)
        );
        assert_eq!(
            classify_taittiriya_svarita(
                Svara::Svarita,
                SvaritaJunctureContext::PostUdattaConsonant
            ),
            Some(TaittiriyaSvarita::Tairovyanjana)
        );
        assert_eq!(
            classify_taittiriya_svarita(
                Svara::Svarita,
                SvaritaJunctureContext::HiatusWithoutSandhi
            ),
            Some(TaittiriyaSvarita::Padavrtta)
        );
        assert_eq!(
            classify_taittiriya_svarita(Svara::Svarita, SvaritaJunctureContext::AcrossVirama),
            Some(TaittiriyaSvarita::Tairovirama)
        );

        // Nitya vs Enclitic
        assert!(TaittiriyaSvarita::Jatya.is_nitya());
        assert!(TaittiriyaSvarita::Kshaipra.is_nitya());
        assert!(TaittiriyaSvarita::Abhinihita.is_nitya());
        assert!(TaittiriyaSvarita::Prashlishta.is_nitya());
        assert!(TaittiriyaSvarita::Tairovyanjana.is_enclitic());
        assert!(TaittiriyaSvarita::Padavrtta.is_enclitic());

        // Dvitva tests (TPr 14.4 & 14.8)
        let r = Varna::Consonant(Consonant::R);
        let a = Varna::Vowel(Vowel::new(VowelQuality::A, VowelLength::Hrasva));
        let v = Varna::Consonant(Consonant::V);

        // Stop after r doubles: arka -> arkka
        assert!(should_double_in_taittiriya(Some(&r), Consonant::K, None));
        // Sibilant after vowel before consonant doubles: aśva -> aśśva
        assert!(should_double_in_taittiriya(
            Some(&a),
            Consonant::Sh,
            Some(&v)
        ));
    }
}

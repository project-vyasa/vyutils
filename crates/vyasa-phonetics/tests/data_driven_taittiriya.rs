//! Data-driven phonological test harness for Krishna Yajurveda (Taittirīya-Prātiśākhya).
//!
//! Loads `tests/data/kyv_corpus.json` containing authentic liturgical passages from the
//! three primary pillars of the Krishna Yajurveda:
//! 1. Taittirīya Saṃhitā (Darśapūrṇamāsa, Śrī Rudram / Namakam, Camakam)
//! 2. Taittirīya Upaniṣad (Śīkṣāvallī, Ānandavallī)
//! 3. Taittirīya Puruṣa Sūkta

use serde::Deserialize;
use std::fs;
use std::path::Path;
use vyasa_phonetics::{
    classify_taittiriya_svarita, karana, should_double_in_taittiriya, Consonant, Karana, Svara,
    SvaritaJunctureContext, TaittiriyaSvarita, Varna, Vowel, VowelLength, VowelQuality,
};

#[derive(Debug, Deserialize)]
struct KyvCorpus {
    collection: String,
    pratisakhya: String,
    texts: Vec<KyvText>,
}

#[derive(Debug, Deserialize)]
struct KyvText {
    text_name: String,
    abbreviation: String,
    sections: Vec<KyvSection>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct KyvSection {
    id: String,
    title: String,
    mantra: String,
    svaritas: Vec<SvaritaCase>,
    dvitva: Vec<DvitvaCase>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SvaritaCase {
    target: String,
    syllable: String,
    context: String,
    expected: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct DvitvaCase {
    word: String,
    consonant: String,
    preceded_by: String,
    followed_by: String,
    expected_double: bool,
}

fn map_context(s: &str) -> SvaritaJunctureContext {
    match s {
        "InternalSemivowelStem" => SvaritaJunctureContext::InternalSemivowelStem,
        "SemivowelSandhi" => SvaritaJunctureContext::SemivowelSandhi,
        "AbhinihitaElision" => SvaritaJunctureContext::AbhinihitaElision,
        "CoalescentLongVowel" => SvaritaJunctureContext::CoalescentLongVowel,
        "PostUdattaConsonant" => SvaritaJunctureContext::PostUdattaConsonant,
        "HiatusWithoutSandhi" => SvaritaJunctureContext::HiatusWithoutSandhi,
        "AcrossVirama" => SvaritaJunctureContext::AcrossVirama,
        other => panic!("Unknown Svarita context in corpus: {other}"),
    }
}

fn map_consonant(s: &str) -> Consonant {
    match s {
        "K" => Consonant::K,
        "Kh" => Consonant::Kh,
        "G" => Consonant::G,
        "Gh" => Consonant::Gh,
        "Ng" => Consonant::Ng,
        "C" => Consonant::C,
        "Ch" => Consonant::Ch,
        "J" => Consonant::J,
        "Jh" => Consonant::Jh,
        "Ny" => Consonant::Ny,
        "Tt" => Consonant::Tt,
        "Tth" => Consonant::Tth,
        "Dd" => Consonant::Dd,
        "Ddh" => Consonant::Ddh,
        "Nn" => Consonant::Nn,
        "T" => Consonant::T,
        "Th" => Consonant::Th,
        "D" => Consonant::D,
        "Dh" => Consonant::Dh,
        "N" => Consonant::N,
        "P" => Consonant::P,
        "Ph" => Consonant::Ph,
        "B" => Consonant::B,
        "Bh" => Consonant::Bh,
        "M" => Consonant::M,
        "Y" => Consonant::Y,
        "R" => Consonant::R,
        "L" => Consonant::L,
        "V" => Consonant::V,
        "Sh" => Consonant::Sh,
        "Ss" => Consonant::Ss,
        "S" => Consonant::S,
        "H" => Consonant::H,
        other => panic!("Unknown consonant in corpus: {other}"),
    }
}

fn map_varna(s: &str) -> Option<Varna> {
    match s {
        "NONE" => None,
        "R" => Some(Varna::Consonant(Consonant::R)),
        "H" => Some(Varna::Consonant(Consonant::H)),
        "T" => Some(Varna::Consonant(Consonant::T)),
        "Y" => Some(Varna::Consonant(Consonant::Y)),
        "A" => Some(Varna::Vowel(Vowel::new(
            VowelQuality::A,
            VowelLength::Hrasva,
        ))),
        "I" => Some(Varna::Vowel(Vowel::new(
            VowelQuality::I,
            VowelLength::Hrasva,
        ))),
        "U" => Some(Varna::Vowel(Vowel::new(
            VowelQuality::U,
            VowelLength::Hrasva,
        ))),
        other => panic!("Unknown varna in corpus: {other}"),
    }
}

#[test]
fn test_data_driven_kyv_corpus() {
    let corpus_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("kyv_corpus.json");

    assert!(
        corpus_path.exists(),
        "Corpus file must exist at {:?}",
        corpus_path
    );

    let raw_json = fs::read_to_string(&corpus_path).expect("Failed to read kyv_corpus.json");
    let corpus: KyvCorpus =
        serde_json::from_str(&raw_json).expect("Failed to parse kyv_corpus.json");

    println!(
        "Testing Krishna Yajurveda Corpus: {} ({})",
        corpus.collection, corpus.pratisakhya
    );

    let mut total_svarita_checks = 0;
    let mut total_dvitva_checks = 0;

    for text in &corpus.texts {
        println!("Checking Text: {} [{}]", text.text_name, text.abbreviation);

        for sec in &text.sections {
            // 1. Validate Svaritas
            for sv in &sec.svaritas {
                let ctx = map_context(&sv.context);
                let detected = classify_taittiriya_svarita(Svara::Svarita, ctx)
                    .unwrap_or_else(|| panic!("Failed to classify svarita for {}", sv.target));

                assert_eq!(
                    detected.id(),
                    sv.expected,
                    "Svarita mismatch for word '{}' in section '{}' (context: {:?})",
                    sv.target,
                    sec.id,
                    sv.context
                );

                // Verify Nitya vs Enclitic consistency
                if matches!(
                    detected,
                    TaittiriyaSvarita::Jatya
                        | TaittiriyaSvarita::Kshaipra
                        | TaittiriyaSvarita::Abhinihita
                        | TaittiriyaSvarita::Prashlishta
                ) {
                    assert!(
                        detected.is_nitya(),
                        "{} must be identified as Nitya Svarita",
                        detected.name_iast()
                    );
                } else {
                    assert!(
                        detected.is_enclitic(),
                        "{} must be identified as Enclitic Svarita",
                        detected.name_iast()
                    );
                }

                total_svarita_checks += 1;
            }

            // 2. Validate Dvitva (Consonant Gemination under TPr Ch. 14)
            for dv in &sec.dvitva {
                let curr_c = map_consonant(&dv.consonant);
                let prev_varna = map_varna(&dv.preceded_by);
                let next_varna = map_varna(&dv.followed_by);

                let is_double =
                    should_double_in_taittiriya(prev_varna.as_ref(), curr_c, next_varna.as_ref());

                assert_eq!(
                    is_double, dv.expected_double,
                    "Dvitva mismatch for '{}' (consonant: {}, prev: {}, next: {}) in section '{}'",
                    dv.word, dv.consonant, dv.preceded_by, dv.followed_by, sec.id
                );

                total_dvitva_checks += 1;
            }
        }
    }

    assert!(
        total_svarita_checks >= 20,
        "Corpus must contain at least 20 Svarita test cases"
    );
    assert!(
        total_dvitva_checks >= 10,
        "Corpus must contain at least 10 Dvitva test cases"
    );

    println!(
        "Successfully verified {} Svaritas and {} Dvitva rules across Krishna Yajurveda corpus!",
        total_svarita_checks, total_dvitva_checks
    );
}

#[test]
fn test_taittiriya_karana_exhaustiveness() {
    // Assert that every consonant maps to a canonical TPr Karaṇa
    let consonants = [
        Consonant::K,
        Consonant::Kh,
        Consonant::G,
        Consonant::Gh,
        Consonant::Ng,
        Consonant::C,
        Consonant::Ch,
        Consonant::J,
        Consonant::Jh,
        Consonant::Ny,
        Consonant::Tt,
        Consonant::Tth,
        Consonant::Dd,
        Consonant::Ddh,
        Consonant::Nn,
        Consonant::T,
        Consonant::Th,
        Consonant::D,
        Consonant::Dh,
        Consonant::N,
        Consonant::P,
        Consonant::Ph,
        Consonant::B,
        Consonant::Bh,
        Consonant::M,
        Consonant::Y,
        Consonant::R,
        Consonant::L,
        Consonant::V,
        Consonant::Sh,
        Consonant::Ss,
        Consonant::S,
        Consonant::H,
    ];

    for c in consonants {
        let v = Varna::Consonant(c);
        let k = karana(&v);
        match c {
            Consonant::K | Consonant::Kh | Consonant::G | Consonant::Gh | Consonant::Ng => {
                assert_eq!(k, Karana::Jihvamadhya)
            }
            Consonant::C
            | Consonant::Ch
            | Consonant::J
            | Consonant::Jh
            | Consonant::Ny
            | Consonant::Y
            | Consonant::Sh => assert_eq!(k, Karana::Jihvopamadhya),
            Consonant::Tt
            | Consonant::Tth
            | Consonant::Dd
            | Consonant::Ddh
            | Consonant::Nn
            | Consonant::R
            | Consonant::Ss => assert_eq!(k, Karana::Prativestitam),
            Consonant::T
            | Consonant::Th
            | Consonant::D
            | Consonant::Dh
            | Consonant::N
            | Consonant::L
            | Consonant::S => assert_eq!(k, Karana::Jihvagram),
            Consonant::P
            | Consonant::Ph
            | Consonant::B
            | Consonant::Bh
            | Consonant::M
            | Consonant::V => assert_eq!(k, Karana::Adharostha),
            Consonant::H => assert_eq!(k, Karana::Hanu),
            _ => {}
        }
    }
}

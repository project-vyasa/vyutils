//! # vyasa-sanskrit-wasm
//!
//! WebAssembly bridge bindings for Project Vyasa's Sanskrit computing engines:
//! - `vyasa-phonetics`: Articulatory phonetics, Śiva Sūtras, and Pratyāhāras
//! - `vyasa-lipi`: Universal transliteration across 11 scripts with Vedic pitch preservation
//! - `vyasa-patha`: Vedic recitation modes, Pada-pāṭha parser, and Krama generation
//!
//! Distributed under the npm package name `@project-vyasa/sanskrit-wasm`.

use wasm_bindgen::prelude::*;

use vyasa_lipi::{
    detect_script as lipi_detect_script, parse_to_tokens, transliterate as lipi_transliterate,
    Script, Token,
};
use vyasa_patha::{
    generate_jata_in_script, generate_jata_patha, generate_krama_in_script, generate_krama_patha,
    parse_pada_patha,
};
use vyasa_phonetics::panini::ItMarker;
use vyasa_phonetics::{
    abhyantara_prayatna, classify_taittiriya_svarita, is_alpaprana, is_ghosha, karana, matra,
    should_double_in_taittiriya, sthana, AbhyantaraPrayatna, Consonant, Karana, Pratyahara,
    ShivaSutraSound, Sthana, SvaritaJunctureContext, TaittiriyaSvarita, Varna, Vowel, VowelLength,
    VowelQuality, SHIVA_SUTRAS,
};

pub mod dto;
use dto::*;

// ============================================================================
// 1. TRANSLITERATION & SCRIPT APIS (vyasa-lipi)
// ============================================================================

/// Transliterates text from a source script to a target script, preserving Vedic pitch accents.
#[wasm_bindgen]
pub fn transliterate(text: &str, from_script: &str, to_script: &str) -> Result<String, JsValue> {
    let from = parse_script(from_script)?;
    let to = parse_script(to_script)?;
    Ok(lipi_transliterate(text, from, to))
}

/// Automatically detects the script of the provided Sanskrit/Indic text.
#[wasm_bindgen]
pub fn detect_script(text: &str) -> Option<String> {
    lipi_detect_script(text).map(|s| s.name().to_string())
}

/// Returns metadata for all supported Indic and Roman scripts.
#[wasm_bindgen]
pub fn get_supported_scripts() -> Result<JsValue, JsValue> {
    let scripts = vec![
        ScriptInfoDto {
            id: "devanagari".to_string(),
            name: "Devanagari (देवनागरी)".to_string(),
            is_indic: true,
            has_vedic_pitch: true,
        },
        ScriptInfoDto {
            id: "telugu".to_string(),
            name: "Telugu (తెలుగు)".to_string(),
            is_indic: true,
            has_vedic_pitch: true,
        },
        ScriptInfoDto {
            id: "kannada".to_string(),
            name: "Kannada (ಕನ್ನಡ)".to_string(),
            is_indic: true,
            has_vedic_pitch: true,
        },
        ScriptInfoDto {
            id: "grantha".to_string(),
            name: "Grantha (𑌗𑍍𑌰𑌨𑍍𑌥)".to_string(),
            is_indic: true,
            has_vedic_pitch: true,
        },
        ScriptInfoDto {
            id: "malayalam".to_string(),
            name: "Malayalam (മലയാളം)".to_string(),
            is_indic: true,
            has_vedic_pitch: true,
        },
        ScriptInfoDto {
            id: "bengali".to_string(),
            name: "Bengali (বাংলা)".to_string(),
            is_indic: true,
            has_vedic_pitch: true,
        },
        ScriptInfoDto {
            id: "iast".to_string(),
            name: "IAST (Roman Diacritics)".to_string(),
            is_indic: false,
            has_vedic_pitch: true,
        },
        ScriptInfoDto {
            id: "iso15919".to_string(),
            name: "ISO 15919".to_string(),
            is_indic: false,
            has_vedic_pitch: true,
        },
        ScriptInfoDto {
            id: "slp1".to_string(),
            name: "SLP1 (ASCII Phonetic)".to_string(),
            is_indic: false,
            has_vedic_pitch: false,
        },
        ScriptInfoDto {
            id: "harvardkyoto".to_string(),
            name: "Harvard-Kyoto (HK)".to_string(),
            is_indic: false,
            has_vedic_pitch: false,
        },
        ScriptInfoDto {
            id: "wx".to_string(),
            name: "WX Notation".to_string(),
            is_indic: false,
            has_vedic_pitch: false,
        },
    ];

    serde_wasm_bindgen::to_value(&scripts).map_err(|e| JsValue::from_str(&e.to_string()))
}

// ============================================================================
// 2. VEDIC RECITATION & KRAMA-PĀṬHA (vyasa-patha)
// ============================================================================

/// Generates formatted Krama-pāṭha text in the requested script.
#[wasm_bindgen]
pub fn generate_krama_text(input_pada_text: &str, target_script: &str) -> Result<String, JsValue> {
    let script = parse_script(target_script)?;
    Ok(generate_krama_in_script(input_pada_text, script))
}

/// Generates step-by-step Krama recitation data structured for Svelte DataGrid and Chanting Trainer.
#[wasm_bindgen]
pub fn generate_krama(input_pada_text: &str, target_script: &str) -> Result<JsValue, JsValue> {
    let script = parse_script(target_script)?;
    let padas = parse_pada_patha(input_pada_text);
    let steps = generate_krama_patha(&padas);

    let dtos: Vec<KramaStepDto> = steps
        .into_iter()
        .map(|step| {
            let formula = if step.is_parigraha {
                format!("{}-iti-{}", step.first_index, step.first_index)
            } else if let Some(sec) = step.second_index {
                format!("{}-{}", step.first_index, sec)
            } else {
                format!("{}", step.first_index)
            };

            let raw_padas_deva = if step.is_parigraha {
                padas
                    .get(step.first_index - 1)
                    .map(|p| p.raw.as_str())
                    .unwrap_or("")
                    .to_string()
            } else if let Some(sec) = step.second_index {
                let p1 = padas
                    .get(step.first_index - 1)
                    .map(|p| p.raw.as_str())
                    .unwrap_or("");
                let p2 = padas.get(sec - 1).map(|p| p.raw.as_str()).unwrap_or("");
                format!("{} {}", p1, p2)
            } else {
                padas
                    .get(step.first_index - 1)
                    .map(|p| p.raw.as_str())
                    .unwrap_or("")
                    .to_string()
            };

            let pragrhya_detected = padas
                .get(step.first_index - 1)
                .map_or(false, |p| p.is_pragrhya())
                || step
                    .second_index
                    .and_then(|idx| padas.get(idx - 1))
                    .map_or(false, |p| p.is_pragrhya());

            let sandhied = if script == Script::Devanagari {
                step.text
            } else {
                lipi_transliterate(&step.text, Script::Devanagari, script)
            };

            let raw_pada = if script == Script::Devanagari {
                raw_padas_deva
            } else {
                lipi_transliterate(&raw_padas_deva, Script::Devanagari, script)
            };

            KramaStepDto {
                step_number: step.step_number,
                formula,
                first_index: step.first_index,
                second_index: step.second_index,
                raw_pada,
                sandhied,
                is_parigraha: step.is_parigraha,
                pragrhya_detected,
            }
        })
        .collect();

    serde_wasm_bindgen::to_value(&dtos).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Generates formatted Jaṭā-pāṭha text in the requested script.
#[wasm_bindgen]
pub fn generate_jata_text(input_pada_text: &str, target_script: &str) -> Result<String, JsValue> {
    let script = parse_script(target_script)?;
    Ok(generate_jata_in_script(input_pada_text, script))
}

/// Generates step-by-step Jaṭā recitation data structured for Svelte DataGrid and Chanting Trainer.
#[wasm_bindgen]
pub fn generate_jata(input_pada_text: &str, target_script: &str) -> Result<JsValue, JsValue> {
    let script = parse_script(target_script)?;
    let padas = parse_pada_patha(input_pada_text);
    let steps = generate_jata_patha(&padas);

    let dtos: Vec<JataStepDto> = steps
        .into_iter()
        .map(|step| {
            let sandhied = if script == Script::Devanagari {
                step.text
            } else {
                lipi_transliterate(&step.text, Script::Devanagari, script)
            };
            let fwd = if script == Script::Devanagari {
                step.forward_text
            } else {
                lipi_transliterate(&step.forward_text, Script::Devanagari, script)
            };
            let rev = if script == Script::Devanagari {
                step.reverse_text
            } else {
                lipi_transliterate(&step.reverse_text, Script::Devanagari, script)
            };

            JataStepDto {
                step_number: step.step_number,
                formula: step.formula,
                first_index: step.first_index,
                second_index: step.second_index,
                forward_text: fwd,
                reverse_text: rev,
                sandhied,
                is_parigraha: step.is_parigraha,
            }
        })
        .collect();

    serde_wasm_bindgen::to_value(&dtos).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Parses Pada-pāṭha text into individual words with compound and Pragṛhya classification.
#[wasm_bindgen]
pub fn parse_padas(input_pada_text: &str) -> Result<JsValue, JsValue> {
    let padas = parse_pada_patha(input_pada_text);
    let dtos: Vec<PadaInfoDto> = padas
        .into_iter()
        .map(|p| PadaInfoDto {
            raw: p.raw.clone(),
            clean: p.clean.clone(),
            compound_parts: p.compound_parts.clone(),
            is_compound: p.is_compound(),
            is_pragrhya: p.is_pragrhya(),
            pragrhya_type: p.pragrhya.map(|pt| format!("{:?}", pt)),
        })
        .collect();

    serde_wasm_bindgen::to_value(&dtos).map_err(|e| JsValue::from_str(&e.to_string()))
}

// ============================================================================
// 3. ARTICULATORY PHONETICS & ŚIVA SŪTRAS (vyasa-phonetics)
// ============================================================================

/// Returns all 14 Māheśvara / Śiva Sūtras with sounds and terminating it-markers.
#[wasm_bindgen]
pub fn get_shiva_sutras() -> Result<JsValue, JsValue> {
    let dtos: Vec<ShivaSutraDto> = SHIVA_SUTRAS
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let sounds_deva: Vec<String> = s
                .sounds
                .iter()
                .map(|snd| sound_to_glyphs(snd).0.to_string())
                .collect();
            let sounds_iast: Vec<String> = s
                .sounds
                .iter()
                .map(|snd| sound_to_glyphs(snd).1.to_string())
                .collect();
            let (it_deva, it_iast) = it_marker_to_glyphs(s.it);

            ShivaSutraDto {
                index: i + 1,
                name: s.name.to_string(),
                it_marker_deva: it_deva.to_string(),
                it_marker_iast: it_iast.to_string(),
                sounds_deva,
                sounds_iast,
            }
        })
        .collect();

    serde_wasm_bindgen::to_value(&dtos).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Resolves a Pāṇinian Pratyāhāra (e.g. "ac", "hal", "yaṇ", "al", "ik") to its sound list.
#[wasm_bindgen]
pub fn get_pratyahara_sounds(pratyahara_name: &str) -> Result<JsValue, JsValue> {
    let p = Pratyahara::from_name(pratyahara_name).ok_or_else(|| {
        JsValue::from_str(&format!(
            "Unknown or unsupported Pratyāhāra: '{}'",
            pratyahara_name
        ))
    })?;

    let dtos: Vec<VarnaAnalysisDto> = p.sounds().iter().map(|s| varna_dto_from_sound(s)).collect();

    serde_wasm_bindgen::to_value(&dtos).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Checks if a sound is contained within a Pāṇinian Pratyāhāra.
#[wasm_bindgen]
pub fn check_pratyahara_contains(
    pratyahara_name: &str,
    sound_symbol: &str,
) -> Result<bool, JsValue> {
    let p = Pratyahara::from_name(pratyahara_name).ok_or_else(|| {
        JsValue::from_str(&format!(
            "Unknown or unsupported Pratyāhāra: '{}'",
            pratyahara_name
        ))
    })?;

    let sound = sound_from_symbol(sound_symbol).ok_or_else(|| {
        JsValue::from_str(&format!("Unrecognized sound symbol: '{}'", sound_symbol))
    })?;

    Ok(p.contains_sound(sound))
}

/// Analyzes an individual Sanskrit Varṇa for Sthāna, Prayatna, Ghoṣa, and Mātrā.
#[wasm_bindgen]
pub fn inspect_varna(varna_symbol: &str) -> Result<JsValue, JsValue> {
    let sound = sound_from_symbol(varna_symbol).ok_or_else(|| {
        JsValue::from_str(&format!("Unrecognized sound symbol: '{}'", varna_symbol))
    })?;

    let dto = varna_dto_from_sound(&sound);
    serde_wasm_bindgen::to_value(&dto).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Syllable-level decomposition of any Sanskrit text into Akṣaras and Varṇas.
#[wasm_bindgen]
pub fn analyze_syllables(text: &str, script_name: &str) -> Result<JsValue, JsValue> {
    let script = parse_script(script_name)?;
    let tokens = parse_to_tokens(text, script);

    let mut dtos: Vec<AksharaAnalysisDto> = Vec::new();

    for token in tokens {
        if let Token::Akshara(akshara) = token {
            let mut cons_dtos = Vec::new();
            let mut total_m: f32 = 0.0;

            for c in &akshara.consonants {
                let varna = Varna::Consonant(*c);
                let dto = varna_dto_from_varna(
                    &varna,
                    &consonant_to_glyphs(c).0,
                    &consonant_to_glyphs(c).1,
                    "consonant",
                );
                total_m += dto.matra;
                cons_dtos.push(dto);
            }

            let vowel_dto = akshara.vowel.map(|(q, l)| {
                let v = Vowel::new(q, l);
                let varna = Varna::Vowel(v);
                let (deva, iast) = vowel_to_glyphs(&q);
                let dto = varna_dto_from_varna(&varna, deva, iast, "vowel");
                total_m += dto.matra;
                dto
            });

            if akshara.ayogavaha.is_some() {
                total_m += 0.5;
            }

            let surface = format!(
                "{}{}",
                cons_dtos
                    .iter()
                    .map(|c| c.glyph_deva.as_str())
                    .collect::<Vec<_>>()
                    .join(""),
                vowel_dto
                    .as_ref()
                    .map(|v| v.glyph_deva.as_str())
                    .unwrap_or("")
            );

            dtos.push(AksharaAnalysisDto {
                surface,
                consonants: cons_dtos,
                vowel: vowel_dto,
                ayogavaha: akshara.ayogavaha.map(|a| format!("{:?}", a)),
                svara: akshara.svara.map(|s| format!("{:?}", s)),
                total_matra: total_m,
            });
        }
    }

    serde_wasm_bindgen::to_value(&dtos).map_err(|e| JsValue::from_str(&e.to_string()))
}

// ============================================================================
// INTERNAL HELPERS
// ============================================================================

fn parse_script(name: &str) -> Result<Script, JsValue> {
    Script::from_name(name)
        .ok_or_else(|| JsValue::from_str(&format!("Unsupported or unknown script: '{}'", name)))
}

fn sthana_to_str(s: Sthana) -> &'static str {
    match s {
        Sthana::Kantha => "Kantha (Velar)",
        Sthana::Talu => "Talu (Palatal)",
        Sthana::Murdha => "Murdha (Retroflex)",
        Sthana::Danta => "Danta (Dental)",
        Sthana::Ostha => "Ostha (Labial)",
        Sthana::Nasika => "Nasika (Nasal)",
        Sthana::KanthaTalu => "Kantha-Talu (Palato-velar)",
        Sthana::KanthaOstha => "Kantha-Ostha (Labio-velar)",
        Sthana::DantaOstha => "Danta-Ostha (Labio-dental)",
        Sthana::Jihvamula => "Jihvamula (Velar root)",
        Sthana::Uras => "Uras (Chest)",
    }
}

fn abhyantara_to_str(a: AbhyantaraPrayatna) -> &'static str {
    match a {
        AbhyantaraPrayatna::Sprshta => "Sprshta (Complete contact)",
        AbhyantaraPrayatna::IsatSprshta => "Isat-Sprshta (Slight contact)",
        AbhyantaraPrayatna::IsadVivrta => "Isad-Vivrta (Slightly open)",
        AbhyantaraPrayatna::Vivrta => "Vivrta (Open)",
        AbhyantaraPrayatna::Samvrta => "Samvrta (Closed)",
    }
}

fn karana_to_str(k: Karana) -> &'static str {
    match k {
        Karana::Jihvagram => "Jihvāgram (Tongue tip)",
        Karana::Jihvopamadhya => "Jihvopamadhya (Tongue blade/edges)",
        Karana::Jihvamadhya => "Jihvāmādhya (Tongue middle)",
        Karana::Jihvamula => "Jihvāmūla (Tongue root)",
        Karana::Prativestitam => "Prativeṣṭitam (Curled tongue tip)",
        Karana::Adharostha => "Adharoṣṭha (Lower lip)",
        Karana::Hanu => "Hanu (Jaws/open vocal tract)",
        Karana::NasikaBila => "Nāsikābila (Nasal cavity)",
    }
}

fn varna_dto_from_varna(
    varna: &Varna,
    deva: &str,
    iast: &str,
    varna_type: &str,
) -> VarnaAnalysisDto {
    let sthana_names: Vec<String> = sthana(varna)
        .iter()
        .map(|&s| sthana_to_str(s).to_string())
        .collect();
    let abhyantara = abhyantara_to_str(abhyantara_prayatna(varna)).to_string();
    let ghosha = is_ghosha(varna);
    let alpaprana = match varna {
        Varna::Consonant(c) => is_alpaprana(c),
        _ => false,
    };
    let m = matra(varna);

    VarnaAnalysisDto {
        glyph_deva: deva.to_string(),
        glyph_iast: iast.to_string(),
        varna_type: varna_type.to_string(),
        sthana: sthana_names,
        abhyantara_prayatna: abhyantara,
        is_ghosha: ghosha,
        is_alpaprana: alpaprana,
        matra: m,
    }
}

fn varna_dto_from_sound(s: &ShivaSutraSound) -> VarnaAnalysisDto {
    match s {
        ShivaSutraSound::Vowel(q) => {
            let v = Vowel::new(*q, VowelLength::Hrasva);
            let varna = Varna::Vowel(v);
            let (deva, iast) = vowel_to_glyphs(q);
            varna_dto_from_varna(&varna, deva, iast, "vowel")
        }
        ShivaSutraSound::Consonant(c) => {
            let varna = Varna::Consonant(*c);
            let (deva, iast) = consonant_to_glyphs(c);
            varna_dto_from_varna(&varna, deva, iast, "consonant")
        }
    }
}

fn sound_to_glyphs(s: &ShivaSutraSound) -> (&'static str, &'static str) {
    match s {
        ShivaSutraSound::Vowel(q) => vowel_to_glyphs(q),
        ShivaSutraSound::Consonant(c) => consonant_to_glyphs(c),
    }
}

fn vowel_to_glyphs(q: &VowelQuality) -> (&'static str, &'static str) {
    match q {
        VowelQuality::A => ("अ", "a"),
        VowelQuality::I => ("इ", "i"),
        VowelQuality::U => ("उ", "u"),
        VowelQuality::R => ("ऋ", "ṛ"),
        VowelQuality::L => ("ऌ", "ḷ"),
        VowelQuality::E => ("ए", "e"),
        VowelQuality::Ai => ("ऐ", "ai"),
        VowelQuality::O => ("ओ", "o"),
        VowelQuality::Au => ("औ", "au"),
        VowelQuality::ShortE => ("ऎ", "e"),
        VowelQuality::ShortO => ("ऒ", "o"),
    }
}

fn consonant_to_glyphs(c: &Consonant) -> (&'static str, &'static str) {
    match c {
        Consonant::K => ("क", "k"),
        Consonant::Kh => ("ख", "kh"),
        Consonant::G => ("ग", "g"),
        Consonant::Gh => ("घ", "gh"),
        Consonant::Ng => ("ङ", "ṅ"),
        Consonant::C => ("च", "c"),
        Consonant::Ch => ("छ", "ch"),
        Consonant::J => ("ज", "j"),
        Consonant::Jh => ("झ", "jh"),
        Consonant::Ny => ("ञ", "ñ"),
        Consonant::Tt => ("ट", "ṭ"),
        Consonant::Tth => ("ठ", "ṭh"),
        Consonant::Dd => ("ड", "ḍ"),
        Consonant::Ddh => ("ढ", "ḍh"),
        Consonant::Nn => ("ण", "ṇ"),
        Consonant::T => ("त", "t"),
        Consonant::Th => ("थ", "th"),
        Consonant::D => ("द", "d"),
        Consonant::Dh => ("ध", "dh"),
        Consonant::N => ("न", "n"),
        Consonant::P => ("प", "p"),
        Consonant::Ph => ("फ", "ph"),
        Consonant::B => ("ब", "b"),
        Consonant::Bh => ("भ", "bh"),
        Consonant::M => ("म", "m"),
        Consonant::Y => ("य", "y"),
        Consonant::R => ("र", "r"),
        Consonant::L => ("ल", "l"),
        Consonant::V => ("व", "v"),
        Consonant::Sh => ("श", "ś"),
        Consonant::Ss => ("ष", "ṣ"),
        Consonant::S => ("स", "s"),
        Consonant::H => ("ह", "h"),
        Consonant::LVedic => ("ळ", "ḷ"),
        Consonant::LhVedic => ("ळ्ह", "ḷh"),
    }
}

fn it_marker_to_glyphs(it: ItMarker) -> (&'static str, &'static str) {
    match it {
        ItMarker::Nn1 => ("ण्", "ṇ"),
        ItMarker::K => ("क्", "k"),
        ItMarker::Ng => ("ङ्", "ṅ"),
        ItMarker::C => ("च्", "c"),
        ItMarker::Tt => ("ट्", "ṭ"),
        ItMarker::Nn2 => ("ण्", "ṇ"),
        ItMarker::M => ("म्", "m"),
        ItMarker::Ny => ("ञ्", "ñ"),
        ItMarker::Ss => ("ष्", "ṣ"),
        ItMarker::Sh => ("श्", "ś"),
        ItMarker::V => ("व्", "v"),
        ItMarker::Y => ("य्", "y"),
        ItMarker::R => ("र्", "r"),
        ItMarker::L => ("ल्", "l"),
    }
}

fn sound_from_symbol(sym: &str) -> Option<ShivaSutraSound> {
    let s = sym.trim();
    match s {
        "a" | "ā" | "अ" | "आ" => Some(ShivaSutraSound::Vowel(VowelQuality::A)),
        "i" | "ī" | "इ" | "ई" => Some(ShivaSutraSound::Vowel(VowelQuality::I)),
        "u" | "ū" | "उ" | "ऊ" => Some(ShivaSutraSound::Vowel(VowelQuality::U)),
        "ṛ" | "ṝ" | "ऋ" | "ॠ" => Some(ShivaSutraSound::Vowel(VowelQuality::R)),
        "ḷ" | "ḹ" | "ऌ" | "ॡ" => Some(ShivaSutraSound::Vowel(VowelQuality::L)),
        "e" | "ए" => Some(ShivaSutraSound::Vowel(VowelQuality::E)),
        "ai" | "ऐ" => Some(ShivaSutraSound::Vowel(VowelQuality::Ai)),
        "o" | "ओ" => Some(ShivaSutraSound::Vowel(VowelQuality::O)),
        "au" | "औ" => Some(ShivaSutraSound::Vowel(VowelQuality::Au)),
        "k" | "क" | "क्" => Some(ShivaSutraSound::Consonant(Consonant::K)),
        "kh" | "ख" | "ख्" => Some(ShivaSutraSound::Consonant(Consonant::Kh)),
        "g" | "ग" | "ग्" => Some(ShivaSutraSound::Consonant(Consonant::G)),
        "gh" | "घ" | "घ्" => Some(ShivaSutraSound::Consonant(Consonant::Gh)),
        "ṅ" | "ङ" | "ङ्" => Some(ShivaSutraSound::Consonant(Consonant::Ng)),
        "c" | "च" | "च्" => Some(ShivaSutraSound::Consonant(Consonant::C)),
        "ch" | "छ" | "छ्" => Some(ShivaSutraSound::Consonant(Consonant::Ch)),
        "j" | "ज" | "ज्" => Some(ShivaSutraSound::Consonant(Consonant::J)),
        "jh" | "झ" | "झ्" => Some(ShivaSutraSound::Consonant(Consonant::Jh)),
        "ñ" | "ञ" | "ञ्" => Some(ShivaSutraSound::Consonant(Consonant::Ny)),
        "ṭ" | "ट" | "ट्" => Some(ShivaSutraSound::Consonant(Consonant::Tt)),
        "ṭh" | "ठ" | "ठ्" => Some(ShivaSutraSound::Consonant(Consonant::Tth)),
        "ḍ" | "ड" | "ड्" => Some(ShivaSutraSound::Consonant(Consonant::Dd)),
        "ḍh" | "ढ" | "ढ्" => Some(ShivaSutraSound::Consonant(Consonant::Ddh)),
        "ṇ" | "ण" | "ण्" => Some(ShivaSutraSound::Consonant(Consonant::Nn)),
        "t" | "त" | "त्" => Some(ShivaSutraSound::Consonant(Consonant::T)),
        "th" | "थ" | "थ्" => Some(ShivaSutraSound::Consonant(Consonant::Th)),
        "d" | "द" | "द्" => Some(ShivaSutraSound::Consonant(Consonant::D)),
        "dh" | "ध" | "ध्" => Some(ShivaSutraSound::Consonant(Consonant::Dh)),
        "n" | "न" | "न्" => Some(ShivaSutraSound::Consonant(Consonant::N)),
        "p" | "प" | "प्" => Some(ShivaSutraSound::Consonant(Consonant::P)),
        "ph" | "फ" | "फ्" => Some(ShivaSutraSound::Consonant(Consonant::Ph)),
        "b" | "ब" | "ब्" => Some(ShivaSutraSound::Consonant(Consonant::B)),
        "bh" | "भ" | "भ्" => Some(ShivaSutraSound::Consonant(Consonant::Bh)),
        "m" | "म" | "म्" => Some(ShivaSutraSound::Consonant(Consonant::M)),
        "y" | "य" | "य्" => Some(ShivaSutraSound::Consonant(Consonant::Y)),
        "r" | "ra" | "र" | "र्" => Some(ShivaSutraSound::Consonant(Consonant::R)),
        "l" | "la" | "ल" | "ल्" => Some(ShivaSutraSound::Consonant(Consonant::L)),
        "v" | "व" | "व्" => Some(ShivaSutraSound::Consonant(Consonant::V)),
        "ś" | "श" | "श्" => Some(ShivaSutraSound::Consonant(Consonant::Sh)),
        "ṣ" | "ष" | "ष्" => Some(ShivaSutraSound::Consonant(Consonant::Ss)),
        "s" | "स" | "स्" => Some(ShivaSutraSound::Consonant(Consonant::S)),
        "h" | "ह" | "ह्" => Some(ShivaSutraSound::Consonant(Consonant::H)),
        _ => None,
    }
}

// ============================================================================
// 4. KRISHNA YAJURVEDA (TAITTIRĪYA-PRĀTIŚĀKHYA) APIS
// ============================================================================

pub fn get_taittiriya_svaritas_list() -> Vec<TaittiriyaSvaritaDto> {
    let svaritas = [
        TaittiriyaSvarita::Jatya,
        TaittiriyaSvarita::Kshaipra,
        TaittiriyaSvarita::Abhinihita,
        TaittiriyaSvarita::Prashlishta,
        TaittiriyaSvarita::Tairovyanjana,
        TaittiriyaSvarita::Tairovirama,
        TaittiriyaSvarita::Padavrtta,
        TaittiriyaSvarita::Tathabhavya,
    ];

    svaritas
        .into_iter()
        .map(|s| TaittiriyaSvaritaDto {
            id: s.id().to_string(),
            name_deva: s.name_deva().to_string(),
            name_iast: s.name_iast().to_string(),
            is_nitya: s.is_nitya(),
        })
        .collect()
}

/// Returns all canonical Svarita varieties defined in *Taittirīya-Prātiśākhya* Ch. 20 as JSON string.
#[wasm_bindgen]
pub fn get_taittiriya_svaritas_json() -> Result<String, JsValue> {
    let dtos = get_taittiriya_svaritas_list();
    serde_json::to_string(&dtos).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Returns all canonical Svarita varieties defined in *Taittirīya-Prātiśākhya* Ch. 20.
#[wasm_bindgen]
pub fn get_taittiriya_svaritas() -> Result<JsValue, JsValue> {
    let dtos = get_taittiriya_svaritas_list();
    serde_wasm_bindgen::to_value(&dtos).map_err(|e| JsValue::from_str(&e.to_string()))
}

pub fn inspect_taittiriya_varna_dto(input: &str) -> Result<TaittiriyaVarnaDto, String> {
    let sound = sound_from_symbol(input).ok_or_else(|| format!("Unrecognized sound: {input}"))?;

    let varna = match &sound {
        ShivaSutraSound::Vowel(q) => Varna::Vowel(Vowel::new(*q, VowelLength::Hrasva)),
        ShivaSutraSound::Consonant(c) => Varna::Consonant(*c),
    };

    let (deva, iast) = sound_to_glyphs(&sound);
    let vtype = match &sound {
        ShivaSutraSound::Vowel(_) => "vowel",
        ShivaSutraSound::Consonant(_) => "consonant",
    };

    let sthana_names: Vec<String> = sthana(&varna)
        .iter()
        .map(|&s| sthana_to_str(s).to_string())
        .collect();
    let karana_name = karana_to_str(karana(&varna)).to_string();
    let abhyantara = abhyantara_to_str(abhyantara_prayatna(&varna)).to_string();
    let ghosha = is_ghosha(&varna);
    let alpaprana = match &varna {
        Varna::Consonant(c) => is_alpaprana(c),
        _ => false,
    };
    let m = matra(&varna);

    Ok(TaittiriyaVarnaDto {
        glyph_deva: deva.to_string(),
        glyph_iast: iast.to_string(),
        varna_type: vtype.to_string(),
        sthana: sthana_names,
        karana: karana_name,
        abhyantara_prayatna: abhyantara,
        is_ghosha: ghosha,
        is_alpaprana: alpaprana,
        matra: m,
    })
}

/// Computes phonetic classification including passive Sthāna and active TPr Karaṇa for a single sound as JSON string.
#[wasm_bindgen]
pub fn inspect_taittiriya_varna_json(input: &str) -> Result<String, JsValue> {
    let dto = inspect_taittiriya_varna_dto(input).map_err(|e| JsValue::from_str(&e))?;
    serde_json::to_string(&dto).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Computes phonetic classification including passive Sthāna and active TPr Karaṇa for a single sound.
#[wasm_bindgen]
pub fn inspect_taittiriya_varna(input: &str) -> Result<JsValue, JsValue> {
    let dto = inspect_taittiriya_varna_dto(input).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&dto).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Checks if a consonant doubles (geminates) in Taittirīya recitation according to TPr Ch. 14.
#[wasm_bindgen]
pub fn check_taittiriya_dvitva(
    prev_glyph: Option<String>,
    curr_glyph: &str,
    next_glyph: Option<String>,
) -> Result<bool, JsValue> {
    let curr_sound = sound_from_symbol(curr_glyph)
        .ok_or_else(|| JsValue::from_str(&format!("Unrecognized consonant: {curr_glyph}")))?;

    let curr_c = match curr_sound {
        ShivaSutraSound::Consonant(c) => c,
        _ => return Ok(false),
    };

    let prev_varna = prev_glyph
        .and_then(|g| sound_from_symbol(&g))
        .map(|s| match s {
            ShivaSutraSound::Vowel(q) => Varna::Vowel(Vowel::new(q, VowelLength::Hrasva)),
            ShivaSutraSound::Consonant(c) => Varna::Consonant(c),
        });

    let next_varna = next_glyph
        .and_then(|g| sound_from_symbol(&g))
        .map(|s| match s {
            ShivaSutraSound::Vowel(q) => Varna::Vowel(Vowel::new(q, VowelLength::Hrasva)),
            ShivaSutraSound::Consonant(c) => Varna::Consonant(c),
        });

    Ok(should_double_in_taittiriya(
        prev_varna.as_ref(),
        curr_c,
        next_varna.as_ref(),
    ))
}

/// Classifies a Svarita by context string under *Taittirīya-Prātiśākhya* Ch. 20.
#[wasm_bindgen]
pub fn classify_taittiriya_svarita_by_context(
    context_str: &str,
) -> Result<Option<String>, JsValue> {
    let context = match context_str {
        "InternalSemivowelStem" => SvaritaJunctureContext::InternalSemivowelStem,
        "SemivowelSandhi" => SvaritaJunctureContext::SemivowelSandhi,
        "AbhinihitaElision" => SvaritaJunctureContext::AbhinihitaElision,
        "CoalescentLongVowel" => SvaritaJunctureContext::CoalescentLongVowel,
        "PostUdattaConsonant" => SvaritaJunctureContext::PostUdattaConsonant,
        "HiatusWithoutSandhi" => SvaritaJunctureContext::HiatusWithoutSandhi,
        "AcrossVirama" => SvaritaJunctureContext::AcrossVirama,
        _ => {
            return Err(JsValue::from_str(&format!(
                "Unknown context: {context_str}"
            )))
        }
    };

    let result = classify_taittiriya_svarita(vyasa_phonetics::Svara::Svarita, context);
    Ok(result.map(|s| s.id().to_string()))
}

use serde::{Deserialize, Serialize};

/// Metadata regarding a supported script scheme.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ScriptInfoDto {
    pub id: String,
    pub name: String,
    pub is_indic: bool,
    pub has_vedic_pitch: bool,
}

/// Linguistic analysis of a single Pada (grammatical word) from the Pada-pāṭha.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PadaInfoDto {
    pub raw: String,
    pub clean: String,
    pub compound_parts: Vec<String>,
    pub is_compound: bool,
    pub is_pragrhya: bool,
    pub pragrhya_type: Option<String>,
}

/// A step in a Krama-pāṭha recitation sequence.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KramaStepDto {
    pub step_number: usize,
    pub formula: String,
    pub first_index: usize,
    pub second_index: Option<usize>,
    pub raw_pada: String,
    pub sandhied: String,
    pub is_parigraha: bool,
    pub pragrhya_detected: bool,
}

/// A single Śiva Sūtra (Māheśvara Sūtra) with its sounds and terminating it-marker.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ShivaSutraDto {
    pub index: usize,
    pub name: String,
    pub it_marker_deva: String,
    pub it_marker_iast: String,
    pub sounds_deva: Vec<String>,
    pub sounds_iast: Vec<String>,
}

/// Articulatory and acoustic properties of a single phoneme (Varṇa).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VarnaAnalysisDto {
    pub glyph_deva: String,
    pub glyph_iast: String,
    pub varna_type: String, // "vowel" | "consonant" | "ayogavaha"
    pub sthana: Vec<String>,
    pub abhyantara_prayatna: String,
    pub is_ghosha: bool,
    pub is_alpaprana: bool,
    pub matra: f32,
}

/// Syllable-level decomposition of an Akṣara.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AksharaAnalysisDto {
    pub surface: String,
    pub consonants: Vec<VarnaAnalysisDto>,
    pub vowel: Option<VarnaAnalysisDto>,
    pub ayogavaha: Option<String>,
    pub svara: Option<String>,
    pub total_matra: f32,
}

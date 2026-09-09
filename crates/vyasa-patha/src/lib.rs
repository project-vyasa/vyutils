//! # vyasa-patha
//!
//! Recitation mode generation engine for Vedic and Classical Sanskrit texts.
//!
//! Part of the `vyutils` workspace.
//!
//! ## Capabilities
//! - **Prakṛti Pāṭhas**: Pada-pāṭha parsing and Krama-pāṭha permutation generation.
//! - **Pragṛhya Detection**: Automatically identifies duals and particles immune to Sandhi (Pāṇini 1.1.11–19).
//! - **Parigraha Engine**: Canonical insertion of 'iti' clauses (Sthita-Upasthita) for Pragṛhya and compound terms.
//! - **Forward Vedic Sandhi**: Euphonic combination on paired tokens with accent preservation.
//! - **Multi-Script Integration**: Seamless generation directly into Telugu, Kannada, Grantha, etc. via `vyasa-lipi`.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod model;
pub mod parigraha;
pub mod pragrhya;
pub mod prakriti;
pub mod sandhi;

pub use model::{KramaStep, Pada, PathaMode, PragrhyaType};
pub use parigraha::{generate_parigraha, VEDIC_ITI};
pub use pragrhya::detect_pragrhya;
pub use prakriti::{
    format_krama_patha, format_pada_patha, generate_krama_patha, parse_pada_patha,
};
pub use sandhi::apply_forward_sandhi;

use alloc::string::String;
use vyasa_lipi::{transliterate, Script};

/// Generates canonical Krama-pāṭha text from a raw Pada-pāṭha input string.
pub fn generate_krama(input_pada_text: &str) -> String {
    let padas = parse_pada_patha(input_pada_text);
    let steps = generate_krama_patha(&padas);
    format_krama_patha(&steps)
}

/// Generates canonical Krama-pāṭha text in a targeted Indic or Roman script.
pub fn generate_krama_in_script(input_pada_text: &str, target_script: Script) -> String {
    // Generate in Devanagari first
    let krama_deva = generate_krama(input_pada_text);

    if target_script == Script::Devanagari {
        krama_deva
    } else {
        transliterate(&krama_deva, Script::Devanagari, target_script)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_level_krama_generation() {
        let input = "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।";
        let krama = generate_krama(input);
        assert_eq!(
            krama,
            "अ॒ग्निमी॒ळे॒ । ई॒ळे॒ पु॒रो-हि॑तम् । पु॒रोहि॑तमिति॑ पु॒रो-हि॑तम् ॥"
        );
    }

    #[test]
    fn test_krama_in_telugu_script() {
        let input = "अ॒ग्निम् । ई॒ळे॒ ।";
        let krama_telu = generate_krama_in_script(input, Script::Telugu);
        assert_eq!(krama_telu, "అ॒గ్నిమీ॒ళే॒ । ఈ॒ళే॒ ఇతి॑ ఈ॒ళే॒ ॥");
    }
}

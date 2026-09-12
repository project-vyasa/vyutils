//! Prakṛti (natural) recitation modes: Pada-pāṭha and Krama-pāṭha.

pub mod krama;
pub mod pada;

pub use krama::{format_krama_patha, generate_krama_for_verse, generate_krama_patha};
pub use pada::{format_pada_patha, parse_pada_patha, parse_verse_hemistichs};

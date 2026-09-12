//! Vikṛti Pāṭhas (the eight modified permutational recitations based on Krama).
//!
//! According to traditional recitation science:
//! "Jaṭā mālā śikhā rekhā dhvajo daṇḍo ratho ghanaḥ |
//! aṣṭau vikṛtayaḥ proktāḥ krama-pūrvā maharṣibhiḥ ||"

pub mod jata;

pub use jata::{format_jata_patha, generate_jata_for_verse, generate_jata_patha, JataStep};

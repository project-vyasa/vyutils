//! Jaṭā-pāṭha (जटापाठ - "Matted Hair") recitation engine.
//!
//! Jaṭā is the primary and foundational mode of the eight Vikṛtis (*Aṣṭa-vikṛtayaḥ*).
//! It permutes every adjacent pair of words forward, backward, and forward again:
//! `1-2, 2-1, 1-2 | 2-3, 3-2, 2-3 | 3-4, 4-3, 3-4 ...`

use crate::model::Pada;
use crate::parigraha::generate_parigraha;
use crate::prakriti::parse_verse_hemistichs;
use crate::sandhi::apply_forward_sandhi;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// A single step in a Jaṭā-pāṭha sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JataStep {
    /// Step sequence number (1-based).
    pub step_number: usize,
    /// 1-based index of the first pada.
    pub first_index: usize,
    /// 1-based index of the second pada (None for Parigraha steps).
    pub second_index: Option<usize>,
    /// Permutation formula (e.g. "1-2-2-1-1-2" or "3-Par").
    pub formula: String,
    /// Forward sandhi text (1-2)
    pub forward_text: String,
    /// Reverse sandhi text (2-1)
    pub reverse_text: String,
    /// Combined surface text for the Jaṭā triplet (1-2 2-1 1-2)
    pub text: String,
    /// Whether this step is an 'iti' (Parigraha) closure clause.
    pub is_parigraha: bool,
}

/// Generates Jaṭā-pāṭha steps for a slice of Padas (typically within a single hemistich).
pub fn generate_jata_patha(padas: &[Pada]) -> Vec<JataStep> {
    let mut steps = Vec::new();
    let n = padas.len();
    if n == 0 {
        return steps;
    }

    let mut step_num = 1;

    for i in 0..n - 1 {
        let p1 = &padas[i];
        let p2 = &padas[i + 1];

        let fwd = apply_forward_sandhi(p1, p2);
        let rev = apply_forward_sandhi(p2, p1);
        let combined = format!("{} {} {}", fwd, rev, fwd);

        steps.push(JataStep {
            step_number: step_num,
            first_index: i + 1,
            second_index: Some(i + 2),
            formula: format!(
                "{}-{}-{}-{}-{}-{}",
                i + 1,
                i + 2,
                i + 2,
                i + 1,
                i + 1,
                i + 2
            ),
            forward_text: fwd,
            reverse_text: rev,
            text: combined,
            is_parigraha: false,
        });
        step_num += 1;

        // If the second word is a compound (samāsa) and not the last word, confirm with Parigraha
        if p2.is_compound() && (i + 1 < n - 1) {
            let parigraha_text = generate_parigraha(p2);
            steps.push(JataStep {
                step_number: step_num,
                first_index: i + 2,
                second_index: None,
                formula: format!("{}-Par", i + 2),
                forward_text: p2.unified_raw(),
                reverse_text: String::new(),
                text: parigraha_text,
                is_parigraha: true,
            });
            step_num += 1;
        }
    }

    // Terminal Parigraha for the final word of the sequence
    if let Some(last_pada) = padas.last() {
        let parigraha_text = generate_parigraha(last_pada);
        steps.push(JataStep {
            step_number: step_num,
            first_index: n,
            second_index: None,
            formula: format!("{}-Par", n),
            forward_text: last_pada.unified_raw(),
            reverse_text: String::new(),
            text: parigraha_text,
            is_parigraha: true,
        });
    }

    steps
}

/// Generates Jaṭā-pāṭha steps for an entire verse, strictly observing Ardharca (hemistich) boundaries.
pub fn generate_jata_for_verse(verse_text: &str) -> Vec<JataStep> {
    let hemistichs = parse_verse_hemistichs(verse_text);
    let mut all_steps = Vec::new();
    let mut global_step_num = 1;
    let mut global_pada_offset = 0;

    for hemistich in hemistichs {
        if hemistich.is_empty() {
            continue;
        }

        let hemi_steps = generate_jata_patha(&hemistich);
        for mut s in hemi_steps {
            s.step_number = global_step_num;
            s.first_index += global_pada_offset;
            if let Some(ref mut idx) = s.second_index {
                *idx += global_pada_offset;
            }
            all_steps.push(s);
            global_step_num += 1;
        }

        global_pada_offset += hemistich.len();
    }

    all_steps
}

/// Formats a list of Jaṭā steps into continuous recitation text.
pub fn format_jata_patha(steps: &[JataStep]) -> String {
    let mut result = String::new();
    for (i, step) in steps.iter().enumerate() {
        result.push_str(&step.text);
        if i + 1 < steps.len() {
            result.push_str(" । ");
        } else {
            result.push_str(" ॥");
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rigveda_1_1_1_first_three_padas_jata() {
        let padas = [Pada::new("अ॒ग्निम्"), Pada::new("ई॒ळे॒"), Pada::new("पु॒रो-हि॑तम्")];
        let steps = generate_jata_patha(&padas);

        // Step 1: (1-2, 2-1, 1-2)
        assert_eq!(steps[0].step_number, 1);
        assert_eq!(steps[0].formula, "1-2-2-1-1-2");
        assert_eq!(steps[0].forward_text, "अ॒ग्निमी॑ळे");
        assert_eq!(steps[0].reverse_text, "ई॒ळे॒ऽग्निम्");
        assert_eq!(steps[0].text, "अ॒ग्निमी॑ळे ई॒ळे॒ऽग्निम् अ॒ग्निमी॑ळे");

        // Step 2: (2-3, 3-2, 2-3)
        assert_eq!(steps[1].step_number, 2);
        assert_eq!(steps[1].formula, "2-3-3-2-2-3");
        assert_eq!(steps[1].forward_text, "ई॒ळे॒ पु॒रोहि॑तम्");
        assert_eq!(steps[1].reverse_text, "पु॒रोहि॑तमी॑ळे");

        // Step 3: Terminal Parigraha on compound purohitam
        assert_eq!(steps[2].step_number, 3);
        assert_eq!(steps[2].formula, "3-Par");
        assert!(steps[2].is_parigraha);
        assert_eq!(steps[2].text, "पु॒रोहि॑तमिति॑ पु॒रो-हि॑तम्");
    }
}

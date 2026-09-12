//! Mathematical generator for Krama-pāṭha (paired step-by-step recitation).
//!
//! Given words 1, 2, 3, 4, 5:
//! - Pair (1, 2)
//! - Pair (2, 3)
//! - Pair (3, 4)
//! - Pair (4, 5)
//! - Terminal clause for word 5: 5 iti 5 (with compound decomposition if applicable)
//!
//! With:
//! - Forward Vedic Sandhi applied between every pair
//! - Parigraha ('iti') clauses inserted for Pragṛhya words and the terminal word.

use crate::model::{KramaStep, Pada};
use crate::parigraha::generate_parigraha;
use crate::sandhi::apply_forward_sandhi;
use alloc::string::String;
use alloc::vec::Vec;

/// Generates the sequence of Krama steps for a given slice of Padas.
pub fn generate_krama_patha(padas: &[Pada]) -> Vec<KramaStep> {
    let mut steps = Vec::new();
    let n = padas.len();

    if n == 0 {
        return steps;
    }

    if n == 1 {
        steps.push(KramaStep {
            step_number: 1,
            first_index: 1,
            second_index: None,
            text: generate_parigraha(&padas[0]),
            is_parigraha: true,
        });
        return steps;
    }

    let mut step_counter = 1;

    for i in 0..(n - 1) {
        let p1 = &padas[i];
        let p2 = &padas[i + 1];

        // 1. Generate the forward sandhi pair (P_i, P_{i+1})
        let paired_text = apply_forward_sandhi(p1, p2);

        steps.push(KramaStep {
            step_number: step_counter,
            first_index: i + 1,
            second_index: Some(i + 2),
            text: paired_text,
            is_parigraha: false,
        });
        step_counter += 1;

        // 2. Pragṛhya Parigraha: If p1 is Pragṛhya and is beyond the first step,
        // it receives an internal Parigraha clause after completing its forward pair
        if i > 0 && p1.is_pragrhya() {
            steps.push(KramaStep {
                step_number: step_counter,
                first_index: i + 1,
                second_index: None,
                text: generate_parigraha(p1),
                is_parigraha: true,
            });
            step_counter += 1;
        }
    }

    // 3. Terminal Step: The final word P_n receives a closing Parigraha ('iti') clause
    let last_pada = &padas[n - 1];
    steps.push(KramaStep {
        step_number: step_counter,
        first_index: n,
        second_index: None,
        text: generate_parigraha(last_pada),
        is_parigraha: true,
    });

    steps
}

/// Generates Krama-pāṭha for an entire verse, respecting Ardharca (hemistich) boundaries.
/// Krama steps never cross an Ardharca boundary ('।'); each hemistich terminates with Parigraha.
pub fn generate_krama_for_verse(verse_text: &str) -> Vec<KramaStep> {
    use crate::prakriti::pada::parse_verse_hemistichs;

    let hemistichs = parse_verse_hemistichs(verse_text);
    let mut all_steps = Vec::new();
    let mut step_offset = 1;
    let mut pada_offset = 1;

    for hemistich in hemistichs {
        if hemistich.is_empty() {
            continue;
        }
        let h_steps = generate_krama_patha(&hemistich);
        for mut step in h_steps {
            step.step_number = step_offset;
            step.first_index += pada_offset - 1;
            if let Some(ref mut idx) = step.second_index {
                *idx += pada_offset - 1;
            }
            all_steps.push(step);
            step_offset += 1;
        }
        pada_offset += hemistich.len();
    }

    all_steps
}

/// Formats a list of Krama steps into canonical recitation text.
pub fn format_krama_patha(steps: &[KramaStep]) -> String {
    let mut out = String::new();
    for (i, step) in steps.iter().enumerate() {
        if i > 0 {
            out.push_str(" । ");
        }
        out.push_str(&step.text);
    }
    if !steps.is_empty() {
        out.push_str(" ॥");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prakriti::pada::parse_pada_patha;

    #[test]
    fn test_krama_rigveda_1_1_1_first_three_padas() {
        let input = "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।";
        let padas = parse_pada_patha(input);
        let steps = generate_krama_patha(&padas);

        // Expect:
        // Step 1: 1-2 (अ॒ग्निमी॑ळे) - Udātta + Anudātta -> Svarita
        // Step 2: 2-3 (ई॒ळे॒ पु॒रोहि॑तम्) - Unified compound without raw hyphens
        // Step 3: Terminal iti on 3 (पु॒रोहि॑तम् इति॑ पु॒रो-हि॑तम्)
        assert_eq!(steps.len(), 3);
        assert_eq!(steps[0].first_index, 1);
        assert_eq!(steps[0].second_index, Some(2));
        assert_eq!(steps[0].text, "अ॒ग्निमी॑ळे");

        assert_eq!(steps[1].first_index, 2);
        assert_eq!(steps[1].second_index, Some(3));
        assert_eq!(steps[1].text, "ई॒ळे॒ पु॒रोहि॑तम्");

        assert_eq!(steps[2].first_index, 3);
        assert_eq!(steps[2].second_index, None);
        assert!(steps[2].is_parigraha);
        assert_eq!(steps[2].text, "पु॒रोहि॑तमिति॑ पु॒रो-हि॑तम्");
    }

    #[test]
    fn test_format_krama_patha() {
        let input = "अ॒ग्निम् । ई॒ळे॒ ।";
        let padas = parse_pada_patha(input);
        let steps = generate_krama_patha(&padas);
        let formatted = format_krama_patha(&steps);

        assert_eq!(formatted, "अ॒ग्निमी॑ळे । ई॒ळे॒ इति॑ ई॒ळे॒ ॥");
    }
}

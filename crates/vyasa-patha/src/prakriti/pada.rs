//! Pada-pāṭha parser and tokenizer.

use crate::model::Pada;
use alloc::string::String;
use alloc::vec::Vec;

/// Parses a raw string of Padas (separated by '।', '॥', or whitespace) into structured Pada tokens.
pub fn parse_pada_patha(input: &str) -> Vec<Pada> {
    let mut padas = Vec::new();

    // Split across punctuation boundaries and whitespace
    for raw_token in input.split(|c: char| c == '।' || c == '॥' || c.is_whitespace()) {
        let trimmed = raw_token.trim();
        if !trimmed.is_empty() {
            padas.push(Pada::new(trimmed));
        }
    }

    padas
}

/// Parses a full verse text into its constituent hemistichs (ardharcas).
/// Each non-empty line (or segment ending with ardharca pause) is parsed into a list of Padas.
pub fn parse_verse_hemistichs(input: &str) -> Vec<Vec<Pada>> {
    let mut hemistichs = Vec::new();

    // Check if input contains line breaks (standard in pipeline and digital corpora)
    if input.contains('\n') {
        for line in input.lines() {
            let padas = parse_pada_patha(line);
            if !padas.is_empty() {
                hemistichs.push(padas);
            }
        }
    } else {
        // Single-line input
        let padas = parse_pada_patha(input);
        if !padas.is_empty() {
            hemistichs.push(padas);
        }
    }

    hemistichs
}

/// Formats a list of Padas back into canonical Pada-pāṭha notation.
pub fn format_pada_patha(padas: &[Pada]) -> String {
    let mut out = String::new();
    for (i, pada) in padas.iter().enumerate() {
        if i > 0 {
            out.push_str(" । ");
        }
        out.push_str(&pada.raw);
    }
    if !padas.is_empty() {
        out.push_str(" ॥");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pada_patha() {
        let input = "अ॒ग्निम् । ई॒ळे॒ । पु॒रो-हि॑तम् ।";
        let padas = parse_pada_patha(input);
        assert_eq!(padas.len(), 3);
        assert_eq!(padas[0].clean, "अग्निम्");
        assert_eq!(padas[1].clean, "ईळे");
        assert_eq!(padas[2].compound_parts, vec!["पुरो", "हितम्"]);
    }
}

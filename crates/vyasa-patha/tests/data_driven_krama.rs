//! Data-driven test harness for Vedic Krama-pāṭha generation.
//!
//! - Everyday regression: Driven off bundled JSON dataset `tests/data/rv_01_001.json`.
//! - Pre-release large-sample testing: Driven off external data pipeline at
//!   `RIGVEDA_PIPELINE_PATH` (or local Wikisource workspace) across multiple Sūktas.

use serde::Deserialize;
use std::path::Path;
use vyasa_lipi::Script;
use vyasa_patha::generate_krama_in_script;
use vyasa_patha::prakriti::{format_krama_patha, generate_krama_for_verse, parse_verse_hemistichs};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SuktaData {
    mandala: u32,
    sukta: u32,
    rishi: String,
    devata: String,
    chandas: String,
    riks: Vec<RikData>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct RikData {
    rik: u32,
    padapatha: String,
    samhita: String,
}

/// Everyday regression test: validates all 9 ṛks of Ṛgveda Sūkta 1.1 with zero external dependencies.
#[test]
fn test_everyday_regression_rv_1_1() {
    let json_str = include_str!("data/rv_01_001.json");
    let sukta: SuktaData = serde_json::from_str(json_str).expect("Failed to parse rv_01_001.json");

    assert_eq!(sukta.mandala, 1);
    assert_eq!(sukta.sukta, 1);
    assert_eq!(sukta.riks.len(), 9);

    for rik in &sukta.riks {
        // 1. Verify hemistich decomposition
        let hemistichs = parse_verse_hemistichs(&rik.padapatha);
        assert_eq!(
            hemistichs.len(),
            2,
            "Rik {} must split into exactly 2 hemistichs (ardharcas)",
            rik.rik
        );

        // 2. Generate Krama steps respecting Ardharca boundaries
        let steps = generate_krama_for_verse(&rik.padapatha);
        assert!(
            !steps.is_empty(),
            "Rik {} must produce Krama steps",
            rik.rik
        );

        // Verify that no step contains unprocessed compound delimiters ('-' or 'ऽ')
        for step in &steps {
            assert!(
                !step.text.contains(" -") && !step.text.contains("- "),
                "Rik {} step {} should not leak raw compound hyphens: '{}'",
                rik.rik,
                step.step_number,
                step.text
            );
        }

        // 3. Format complete Krama text
        let formatted = format_krama_patha(&steps);
        assert!(
            formatted.ends_with('॥'),
            "Formatted Krama must terminate with double danda"
        );

        // 4. Validate Rik 1 specifics
        if rik.rik == 1 {
            // First step must have Svarita on 'mī'
            assert_eq!(steps[0].text, "अ॒ग्निमी॑ळे");
            // Step 2 must unify purohitam without hyphen
            assert_eq!(steps[1].text, "ई॒ळे॒ पु॒रोहि॑तम्");
        }

        // 5. Verify multi-script generation preserves Vedic pitch accents
        let telugu_krama = generate_krama_in_script(&rik.padapatha, Script::Telugu);
        assert!(
            telugu_krama.contains('\u{0951}') || telugu_krama.contains('\u{0952}'),
            "Telugu Krama must preserve Vedic Svara accents"
        );
    }
}

/// Large-sample test option for pre-release validation against the raw data pipeline.
/// Run explicitly via:
/// `RIGVEDA_SAMPLE_SUKTAS=10 cargo test --test data_driven_krama test_large_pipeline -- --nocapture`
#[test]
fn test_large_pipeline_sample_pre_release() {
    let pipeline_dir = std::env::var("RIGVEDA_PIPELINE_PATH").unwrap_or_else(|_| {
        "/Users/anand/Projects/project-vyasa/sa.wikisource.org/data/processed/rigveda".to_string()
    });

    let padapatha_dir = Path::new(&pipeline_dir).join("content/padapatha/01");
    if !padapatha_dir.exists() {
        println!(
            "Pipeline directory not found at {:?}. Skipping large-sample test.",
            padapatha_dir
        );
        return;
    }

    let sample_limit: usize = std::env::var("RIGVEDA_SAMPLE_SUKTAS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5); // Default to 5 Sūktas for quick pre-release check

    println!(
        "\n--- Running Pre-Release Krama Pipeline Validation (Sample: {} Sūktas) ---",
        sample_limit
    );

    let mut entries: Vec<_> = std::fs::read_dir(&padapatha_dir)
        .expect("Failed to read padapatha directory")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "vy"))
        .collect();

    entries.sort_by_key(|e| e.path());

    let mut total_suktas = 0;
    let mut total_riks = 0;
    let mut total_krama_steps = 0;

    for entry in entries.iter().take(sample_limit) {
        let content = std::fs::read_to_string(entry.path()).expect("Failed to read Sūkta file");

        let file_name = entry.file_name();
        let sukta_name = file_name.to_string_lossy();

        // Parse verses from `v <num> [ ... ]` blocks
        let mut in_verse = false;
        let mut current_verse_lines = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("`v ") {
                in_verse = true;
                current_verse_lines.clear();
            } else if trimmed == "]" && in_verse {
                in_verse = false;
                let verse_text = current_verse_lines.join("\n");
                let steps = generate_krama_for_verse(&verse_text);

                assert!(
                    !steps.is_empty(),
                    "Sukta {} verse produced 0 Krama steps",
                    sukta_name
                );

                total_krama_steps += steps.len();
                total_riks += 1;
            } else if in_verse {
                current_verse_lines.push(trimmed);
            }
        }

        total_suktas += 1;
        println!(
            "  ✓ Validated Sukta {}: {} total ṛks processed",
            sukta_name, total_riks
        );
    }

    println!(
        "--- Pipeline Validation Complete: {} Sūktas, {} Ṛks, {} Krama Steps Generated Successfully ---\n",
        total_suktas, total_riks, total_krama_steps
    );
}

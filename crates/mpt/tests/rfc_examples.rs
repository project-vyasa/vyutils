use mpt::{parse, to_string, Document, Format, HeaderBlock, Part};

const MULTI_FORMAT: &str = r#"``[ header
classification = research
``] header

``[ abstract format = yaml
``[ header
title = "Inline annotations"
``] header
summary: >
  Compares inline and out-of-band metadata patterns.
``] abstract

``[ source format = vyasa
``[ header
stream = "mula"
chapter = 1
``] header
  `chapter 1 [
    `verse 1 [ dhṛtarāṣṭra uvāca ]
  ]
``] source

``[ references format = toml
[[cite]]
key = "rfc-019"
title = "Extended semantic annotations"
``] references
"#;

const FEATURE_INVENTORY: &str = r#"``[ header
classification = internal
``] header

``[ metadata
``[ header
name = "Text node rendering"
roles = ["publisher"]
tests = ["vyasac-simple-tests"]
``] header
``] metadata

``[ description
Granular feature: Text node rendering
``] description
"#;

const MACHINE_GENERATED: &str = r#"``[ header
``] header

``[ build-log
2026-08-08T12:00:00Z pack complete
``] build-log
"#;

const PARTS_ONLY: &str = r#"``[ synopsis format = yaml
summary: brief
``] synopsis
"#;

#[test]
fn parses_multi_format_document() {
    let doc = parse(MULTI_FORMAT).expect("multi-format doc");
    assert_eq!(
        doc.file_header.as_ref().map(|h| h.payload.as_str()),
        Some("classification = research")
    );
    assert_eq!(doc.parts.len(), 3);
    assert_eq!(doc.parts[0].id, "abstract");
    assert_eq!(doc.parts[0].body_format, Format::Yaml);
    assert!(doc.parts[0].header.is_some());
    assert_eq!(doc.parts[1].id, "source");
    assert_eq!(doc.parts[1].body_format, Format::Vyasa);
    assert!(doc.parts[1].body.contains("`chapter 1"));
    assert_eq!(doc.parts[2].id, "references");
    assert_eq!(doc.parts[2].body_format, Format::Toml);
}

#[test]
fn parses_feature_inventory() {
    let doc = parse(FEATURE_INVENTORY).expect("feature inventory");
    assert_eq!(doc.parts.len(), 2);
    assert_eq!(doc.parts[0].id, "metadata");
    assert_eq!(doc.parts[0].body_format, Format::Text);
    assert!(doc.parts[0].body.is_empty());
    assert_eq!(doc.parts[1].id, "description");
    assert!(!doc.parts[1].body.is_empty());
}

#[test]
fn parses_empty_file_header() {
    let doc = parse(MACHINE_GENERATED).expect("machine generated");
    assert!(doc.file_header.as_ref().is_some_and(|h| h.payload.is_empty()));
    assert_eq!(doc.parts[0].id, "build-log");
}

#[test]
fn parses_parts_only_without_file_header() {
    let doc = parse(PARTS_ONLY).expect("parts only");
    assert!(doc.file_header.is_none());
    assert_eq!(doc.parts[0].body_format, Format::Yaml);
}

#[test]
fn round_trip_multi_format() {
    let doc = parse(MULTI_FORMAT).expect("parse");
    let serialized = to_string(&doc).expect("serialize");
    let reparsed = parse(&serialized).expect("reparse");
    assert_eq!(doc, reparsed);
}

#[test]
fn canonical_omits_default_formats() {
    let doc = Document {
        file_header: Some(HeaderBlock {
            format: Format::Toml,
            payload: "classification = internal\n".to_string(),
        }),
        parts: vec![Part {
            id: "metadata".to_string(),
            body_format: Format::Text,
            header: None,
            body: String::new(),
        }],
    };
    let out = to_string(&doc).expect("serialize");
    assert!(!out.contains("format = toml"));
    assert!(!out.contains("format = text"));
    assert!(out.contains("``[ metadata\n"));
    assert!(out.contains("``] metadata\n"));
}

#[test]
fn rejects_second_file_header() {
    let input = r#"``[ header
a = 1
``] header

``[ header
b = 2
``] header

``[ x
body
``] x
"#;
    let err = parse(input).unwrap_err();
    assert!(err.to_string().contains("file header"));
}

#[test]
fn rejects_part_close_mismatch() {
    let input = r#"``[ foo
body
``] bar
"#;
    let err = parse(input).unwrap_err();
    assert!(err.to_string().contains("mismatch"));
}

#[test]
fn rejects_duplicate_part_ids() {
    let input = r#"``[ a
one
``] a

``[ a
two
``] a
"#;
    let err = parse(input).unwrap_err();
    assert!(err.to_string().contains("duplicate"));
}

#[test]
fn header_label_opens_metadata_not_part() {
    let input = r#"``[ header
title = "x"
``] header

``[ synopsis
``] synopsis
"#;
    let doc = parse(input).expect("parse");
    assert!(doc.file_header.is_some());
    assert_eq!(doc.parts[0].id, "synopsis");
}

#[test]
fn accepts_crlf_input() {
    let input = "``[ a\r\nbody\r\n``] a\r\n";
    let doc = parse(input).expect("crlf");
    assert_eq!(doc.parts[0].body, "body");
}

#[test]
fn rejects_legacy_keyword_syntax() {
    let input = r#"``part a
body
``part-end a
"#;
    assert!(parse(input).is_err());
}

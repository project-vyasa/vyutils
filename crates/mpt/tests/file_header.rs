use mpt::{parse, parts_count_warning, to_string};

#[test]
fn canonicalize_syncs_parts_count() {
    let input = r#"``[ header
parts = 99
classification = test
``] header

``[ only
body
``] only
"#;
    let doc = parse(input).unwrap();
    assert!(parts_count_warning(&doc).is_some());
    let out = to_string(&doc).unwrap();
    assert!(out.contains("parts = 1\n"));
    assert!(!out.contains("parts = 99"));
}

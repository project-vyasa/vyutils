use mpt::{
    add_part, find_part, merge, parse, remove_part, to_string, Document, Format, HeaderBlock,
    InsertPosition, Part,
};

fn sample_doc() -> Document {
    parse(
        r#"``[ header
classification = test
``] header

``[ alpha
alpha body
``] alpha

``[ beta
beta body
``] beta
"#,
    )
    .expect("sample")
}

#[test]
fn merge_concatenates_parts() {
    let a = parse(
        r#"``[ one
one
``] one
"#,
    )
    .unwrap();
    let b = parse(
        r#"``[ two
two
``] two
"#,
    )
    .unwrap();
    let merged = merge(vec![a, b]).unwrap();
    assert_eq!(merged.parts.len(), 2);
    assert_eq!(merged.parts[0].id, "one");
    assert_eq!(merged.parts[1].id, "two");
}

#[test]
fn merge_rejects_duplicate_part_ids() {
    let a = parse(
        r#"``[ dup
a
``] dup
"#,
    )
    .unwrap();
    let b = parse(
        r#"``[ dup
b
``] dup
"#,
    )
    .unwrap();
    let err = merge(vec![a, b]).unwrap_err();
    assert!(err.to_string().contains("duplicate"));
}

#[test]
fn merge_rejects_two_file_headers() {
    let a = parse(
        r#"``[ header
a = 1
``] header

``[ one
one
``] one
"#,
    )
    .unwrap();
    let b = parse(
        r#"``[ header
b = 2
``] header

``[ two
two
``] two
"#,
    )
    .unwrap();
    let err = merge(vec![a, b]).unwrap_err();
    assert!(err.to_string().contains("file header"));
}

#[test]
fn merge_takes_file_header_from_later_doc_if_first_lacks_one() {
    let a = parse(
        r#"``[ one
one
``] one
"#,
    )
    .unwrap();
    let b = parse(
        r#"``[ header
classification = x
``] header

``[ two
two
``] two
"#,
    )
    .unwrap();
    let merged = merge(vec![a, b]).unwrap();
    assert!(merged.file_header.is_some());
    assert_eq!(merged.parts.len(), 2);
}

#[test]
fn add_part_at_end() {
    let mut doc = sample_doc();
    add_part(
        &mut doc,
        Part {
            id: "gamma".to_string(),
            body_format: Format::Text,
            header: None,
            body: "gamma body".to_string(),
        },
        InsertPosition::End,
    )
    .unwrap();
    assert_eq!(doc.parts.len(), 3);
    assert_eq!(doc.parts[2].id, "gamma");
}

#[test]
fn add_part_before_existing() {
    let mut doc = sample_doc();
    add_part(
        &mut doc,
        Part {
            id: "gamma".to_string(),
            body_format: Format::Text,
            header: None,
            body: "gamma".to_string(),
        },
        InsertPosition::Before("beta".to_string()),
    )
    .unwrap();
    assert_eq!(doc.parts[1].id, "gamma");
    assert_eq!(doc.parts[2].id, "beta");
}

#[test]
fn removes_part_by_id() {
    let mut doc = sample_doc();
    let removed = remove_part(&mut doc, "alpha").unwrap();
    assert_eq!(removed.id, "alpha");
    assert_eq!(doc.parts.len(), 1);
}

#[test]
fn remove_last_part_errors() {
    let doc = parse(
        r#"``[ only
only
``] only
"#,
    )
    .unwrap();
    let mut doc = doc;
    let err = remove_part(&mut doc, "only").unwrap_err();
    assert!(err.to_string().contains("at least one part"));
}

#[test]
fn finds_part_by_id() {
    let doc = sample_doc();
    let part = find_part(&doc, "beta").unwrap();
    assert_eq!(part.body, "beta body");
}

#[test]
fn edit_round_trip_through_serialize() {
    let mut doc = sample_doc();
    add_part(
        &mut doc,
        Part {
            id: "gamma".to_string(),
            body_format: Format::Yaml,
            header: Some(HeaderBlock {
                format: Format::Toml,
                payload: "key = 1".to_string(),
            }),
            body: "y: 1".to_string(),
        },
        InsertPosition::After("alpha".to_string()),
    )
    .unwrap();
    remove_part(&mut doc, "beta").unwrap();
    let serialized = to_string(&doc).unwrap();
    let reparsed = parse(&serialized).unwrap();
    assert_eq!(doc, reparsed);
}

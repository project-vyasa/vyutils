use mpt::{parse, to_string};

#[test]
fn parses_csv_delimiter_option_map() {
    let input = r#"``[ states format = csv { delimiter = "|" }
code|name
CA|California
``] states
"#;
    let doc = parse(input).expect("parse");
    assert_eq!(doc.parts[0].body_format, mpt::Format::Csv);
    assert_eq!(
        doc.parts[0].options.get("delimiter").map(String::as_str),
        Some("|")
    );
    assert!(doc.parts[0].header.is_none());
}

#[test]
fn accepts_toml_literal_string_delimiter() {
    let input = r#"``[ cities format = csv { delimiter = ';' }
code;city
SFO;San Francisco
``] cities
"#;
    let doc = parse(input).expect("parse");
    assert_eq!(
        doc.parts[0].options.get("delimiter").map(String::as_str),
        Some(";")
    );
}

#[test]
fn accepts_tab_delimiter_escape() {
    let input = "``[ t format = csv { delimiter = \"\\t\" }\na\tb\n``] t\n";
    let doc = parse(input).expect("parse");
    assert_eq!(
        doc.parts[0].options.get("delimiter").map(String::as_str),
        Some("\t")
    );
    let out = to_string(&doc).expect("serialize");
    assert!(out.contains(r#"{ delimiter = "\t" }"#));
}

#[test]
fn canonical_omits_default_comma_delimiter() {
    let input = r#"``[ states format = csv { delimiter = "," }
a,b
``] states
"#;
    let doc = parse(input).expect("parse");
    let out = to_string(&doc).expect("serialize");
    assert!(out.contains("``[ states format = csv\n"));
    assert!(!out.contains("delimiter"));
}

#[test]
fn canonical_emits_pipe_delimiter_as_basic_string() {
    let input = r#"``[ states format = csv { delimiter = '|' }
a|b
``] states
"#;
    let doc = parse(input).expect("parse");
    let out = to_string(&doc).expect("serialize");
    assert!(out.contains(r#"{ delimiter = "|" }"#));
    let reparsed = parse(&out).expect("reparse");
    assert_eq!(out, to_string(&reparsed).expect("re-serialize"));
}

#[test]
fn rejects_empty_option_map() {
    let input = r#"``[ states format = csv { }
a,b
``] states
"#;
    let err = parse(input).unwrap_err();
    assert!(err.to_string().contains("empty option-map"));
}

#[test]
fn rejects_unknown_csv_option() {
    let input = r#"``[ states format = csv { encoding = "utf-8" }
a,b
``] states
"#;
    let err = parse(input).unwrap_err();
    assert!(err.to_string().contains("unknown option `encoding`"));
}

#[test]
fn rejects_option_map_on_text_part() {
    let input = r#"``[ notes { delimiter = "|" }
hello
``] notes
"#;
    let err = parse(input).unwrap_err();
    assert!(err.to_string().contains("unknown option `delimiter`"));
}

#[test]
fn rejects_option_map_on_header() {
    let input = r#"``[ header { classification = "x" }
``] header

``[ a
body
``] a
"#;
    let err = parse(input).unwrap_err();
    assert!(err
        .to_string()
        .contains("option-map is not allowed on header"));
}

#[test]
fn rejects_multi_character_delimiter() {
    let input = r#"``[ states format = csv { delimiter = "||" }
a||b
``] states
"#;
    let err = parse(input).unwrap_err();
    assert!(err.to_string().contains("single character"));
}

#[test]
fn rejects_non_string_option_value() {
    let input = r#"``[ states format = csv { delimiter = 1 }
a,b
``] states
"#;
    let err = parse(input).unwrap_err();
    assert!(err.to_string().contains("must be strings"));
}

#[test]
fn rejects_trailing_junk_after_option_map() {
    let input = r#"``[ states format = csv { delimiter = "|" } extra
a|b
``] states
"#;
    let err = parse(input).unwrap_err();
    assert!(err.to_string().contains("invalid option-map"));
}

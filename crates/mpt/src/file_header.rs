use crate::ast::{Document, HeaderBlock};
use crate::format::Format;

const PARTS_KEY: &str = "parts";

/// Declared part count in the file header payload, if present.
pub fn declared_parts_count(doc: &Document) -> Option<usize> {
    doc.file_header
        .as_ref()
        .and_then(|header| parse_parts_count(&header.payload))
}

/// Update or insert `parts = <n>` in the file header to match `doc.parts.len()`.
pub fn sync_parts_count(doc: &mut Document) {
    let count = doc.parts.len();
    if let Some(header) = doc.file_header.as_mut() {
        header.payload = set_parts_count(&header.payload, count);
    } else if count > 0 {
        doc.file_header = Some(HeaderBlock {
            format: Format::HEADER_DEFAULT,
            payload: format!("{PARTS_KEY} = {count}\n"),
        });
    }
}

/// Returns a warning message when declared `parts` does not match actual count.
pub fn parts_count_warning(doc: &Document) -> Option<String> {
    let actual = doc.parts.len();
    let declared = declared_parts_count(doc)?;
    if declared != actual {
        Some(format!(
            "file header declares {PARTS_KEY} = {declared}, but document has {actual} part(s)"
        ))
    } else {
        None
    }
}

fn parse_parts_count(payload: &str) -> Option<usize> {
    for line in payload.lines() {
        let line = line.split('#').next()?.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            if key.trim() == PARTS_KEY {
                return value.trim().parse().ok();
            }
        }
    }
    None
}

fn set_parts_count(payload: &str, count: usize) -> String {
    let mut lines: Vec<String> = payload.lines().map(String::from).collect();
    let assignment = format!("{PARTS_KEY} = {count}");
    let mut replaced = false;
    for line in &mut lines {
        let trimmed = line.split('#').next().unwrap_or("").trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some((key, _)) = trimmed.split_once('=') {
            if key.trim() == PARTS_KEY {
                let suffix = line
                    .find('#')
                    .map(|idx| line[idx..].to_string())
                    .unwrap_or_default();
                *line = format!("{assignment}{suffix}");
                replaced = true;
                break;
            }
        }
    }
    if !replaced {
        if lines.iter().all(|line| line.trim().is_empty()) {
            lines = vec![assignment];
        } else {
            lines.insert(0, assignment);
        }
    }
    let mut out = lines.join("\n");
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_parts_from_toml_payload() {
        let payload = "# comment\nparts = 2\nclassification = x\n";
        assert_eq!(parse_parts_count(payload), Some(2));
    }

    #[test]
    fn sync_inserts_parts_when_missing() {
        let mut doc = Document {
            file_header: Some(HeaderBlock {
                format: Format::Toml,
                payload: "classification = test\n".to_string(),
            }),
            parts: vec![],
        };
        doc.parts.push(crate::ast::Part {
            id: "a".to_string(),
            body_format: Format::Text,
            options: Default::default(),
            header: None,
            body: String::new(),
        });
        sync_parts_count(&mut doc);
        assert!(doc
            .file_header
            .as_ref()
            .unwrap()
            .payload
            .contains("parts = 1"));
    }

    #[test]
    fn sync_creates_file_header_when_absent() {
        let mut doc = Document {
            file_header: None,
            parts: vec![crate::ast::Part {
                id: "a".to_string(),
                body_format: Format::Text,
                options: Default::default(),
                header: None,
                body: "x".to_string(),
            }],
        };
        sync_parts_count(&mut doc);
        assert_eq!(doc.file_header.as_ref().unwrap().payload, "parts = 1\n");
    }

    #[test]
    fn warns_on_mismatch() {
        let doc = Document {
            file_header: Some(HeaderBlock {
                format: Format::Toml,
                payload: "parts = 2\n".to_string(),
            }),
            parts: vec![crate::ast::Part {
                id: "a".to_string(),
                body_format: Format::Text,
                options: Default::default(),
                header: None,
                body: String::new(),
            }],
        };
        let warning = parts_count_warning(&doc).unwrap();
        assert!(warning.contains("parts = 2"));
        assert!(warning.contains("1 part"));
    }
}

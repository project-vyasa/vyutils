use crate::ast::{Document, HeaderBlock, Part};
use crate::error::SerializeError;
use crate::file_header::sync_parts_count;
use crate::format::Format;
use std::collections::BTreeMap;

const SIGIL: &str = "``";

pub fn to_string(doc: &Document) -> Result<String, SerializeError> {
    if doc.parts.is_empty() {
        return Err(SerializeError::NoParts);
    }
    let mut doc = doc.clone();
    sync_parts_count(&mut doc);
    let mut out = String::new();
    if let Some(header) = &doc.file_header {
        write_header_block(&mut out, header);
    }
    for part in &doc.parts {
        write_part(&mut out, part);
    }
    Ok(out)
}

fn write_header_block(out: &mut String, header: &HeaderBlock) {
    write_envelope_open(out, "header", header.format, Format::HEADER_DEFAULT, None);
    if !header.payload.is_empty() {
        out.push_str(&header.payload);
        if !header.payload.ends_with('\n') {
            out.push('\n');
        }
    }
    out.push_str(SIGIL);
    out.push_str("] header\n");
}

fn write_part(out: &mut String, part: &Part) {
    write_envelope_open(
        out,
        &part.id,
        part.body_format,
        Format::BODY_DEFAULT,
        Some(&part.options),
    );
    if let Some(header) = &part.header {
        write_header_block(out, header);
    }
    if !part.body.is_empty() {
        out.push_str(&part.body);
        if !part.body.ends_with('\n') {
            out.push('\n');
        }
    }
    out.push_str(SIGIL);
    out.push_str("] ");
    out.push_str(&part.id);
    out.push('\n');
}

fn write_envelope_open(
    out: &mut String,
    label: &str,
    format: Format,
    default: Format,
    options: Option<&BTreeMap<String, String>>,
) {
    out.push_str(SIGIL);
    out.push('[');
    out.push(' ');
    out.push_str(label);
    if format != default {
        out.push_str(" format = ");
        out.push_str(format.as_str());
    }
    if let Some(options) = options {
        write_option_map(out, format, options);
    }
    out.push('\n');
}

fn write_option_map(out: &mut String, format: Format, options: &BTreeMap<String, String>) {
    let mut pairs = Vec::new();
    for key in format.allowed_option_keys() {
        if let Some(value) = options.get(*key) {
            if !format.is_default_option(key, value) {
                pairs.push((*key, value.as_str()));
            }
        }
    }
    if pairs.is_empty() {
        return;
    }
    out.push_str(" { ");
    for (i, (key, value)) in pairs.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(key);
        out.push_str(" = ");
        out.push_str(&toml_basic_string(value));
    }
    out.push_str(" }");
}

fn toml_basic_string(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            c if c.is_control() => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

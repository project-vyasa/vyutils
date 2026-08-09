use crate::ast::{Document, HeaderBlock, Part};
use crate::error::SerializeError;
use crate::format::Format;

const SIGIL: &str = "``";

pub fn to_string(doc: &Document) -> Result<String, SerializeError> {
    if doc.parts.is_empty() {
        return Err(SerializeError::NoParts);
    }
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
    write_envelope_open(out, "header", header.format, Format::HEADER_DEFAULT);
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

fn write_envelope_open(out: &mut String, label: &str, format: Format, default: Format) {
    out.push_str(SIGIL);
    out.push('[');
    out.push(' ');
    out.push_str(label);
    if format != default {
        out.push_str(" format = ");
        out.push_str(format.as_str());
    }
    out.push('\n');
}

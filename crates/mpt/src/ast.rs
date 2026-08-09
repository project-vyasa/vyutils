use crate::format::Format;

/// Parsed mpt document (core container only; payloads are opaque strings).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub file_header: Option<HeaderBlock>,
    pub parts: Vec<Part>,
}

/// Metadata block (file-level before first part, or part-level).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderBlock {
    pub format: Format,
    pub payload: String,
}

/// Named body region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    pub id: String,
    pub body_format: Format,
    pub header: Option<HeaderBlock>,
    pub body: String,
}

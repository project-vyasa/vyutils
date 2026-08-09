use crate::error::{ParseError, Result};

/// Built-in payload format names (RFC-0001 registry, v1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    Toml,
    Text,
    Yaml,
    Json5,
    Xml,
    Vyasa,
    Csv,
}

impl Format {
    pub const HEADER_DEFAULT: Format = Format::Toml;
    pub const BODY_DEFAULT: Format = Format::Text;

    pub fn parse_name(name: &str) -> Result<Self> {
        match name {
            "toml" => Ok(Self::Toml),
            "text" => Ok(Self::Text),
            "yaml" => Ok(Self::Yaml),
            "json5" => Ok(Self::Json5),
            "xml" => Ok(Self::Xml),
            "vyasa" => Ok(Self::Vyasa),
            "csv" => Ok(Self::Csv),
            other => Err(ParseError::UnknownFormat(other.to_string())),
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Toml => "toml",
            Self::Text => "text",
            Self::Yaml => "yaml",
            Self::Json5 => "json5",
            Self::Xml => "xml",
            Self::Vyasa => "vyasa",
            Self::Csv => "csv",
        }
    }

    pub fn emit_on_header(self) -> bool {
        self != Self::HEADER_DEFAULT
    }

    pub fn emit_on_part_body(self) -> bool {
        self != Self::BODY_DEFAULT
    }
}

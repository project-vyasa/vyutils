use thiserror::Error;

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("document must contain at least one part")]
    NoParts,
    #[error("at most one file header allowed before the first part")]
    SecondFileHeader,
    #[error("duplicate part id `{0}`")]
    DuplicatePartId(String),
    #[error("invalid part id `{0}`")]
    InvalidPartId(String),
    #[error("unknown format `{0}`")]
    UnknownFormat(String),
    #[error("unclosed header block")]
    UnclosedHeader,
    #[error("unclosed part `{0}`")]
    UnclosedPart(String),
    #[error("expected part open, found end of input")]
    ExpectedPart,
    #[error("part-end id mismatch: expected `{expected}`, found `{found}`")]
    PartEndMismatch { expected: String, found: String },
    #[error("unexpected envelope line at line {line}: {content}")]
    UnexpectedEnvelope { line: usize, content: String },
    #[error("invalid envelope line at line {line}: {content}")]
    InvalidEnvelope { line: usize, content: String },
    #[error("envelope line only allows `format = <name>`; found at line {line}: {content}")]
    InvalidFormatClause { line: usize, content: String },
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SerializeError {
    #[error("cannot serialize empty document without parts")]
    NoParts,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EditError {
    #[error("no input documents to merge")]
    NoInputDocuments,
    #[error("document must contain at least one part")]
    NoParts,
    #[error("duplicate part id `{0}`")]
    DuplicatePartId(String),
    #[error("part `{0}` not found")]
    PartNotFound(String),
    #[error("part `{0}` has no header block")]
    PartHeaderNotFound(String),
    #[error("cannot merge: more than one file header across inputs")]
    MergeFileHeaderConflict,
}

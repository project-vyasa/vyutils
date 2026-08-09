use crate::ast::{Document, HeaderBlock, Part};
use crate::error::{ParseError, Result};
use crate::format::Format;

const SIGIL: &str = "``";
const HEADER_LABEL: &str = "header";

#[derive(Debug, Clone, PartialEq, Eq)]
enum EnvelopeKind {
    Header { format: Format },
    HeaderEnd,
    Part { id: String, body_format: Format },
    PartEnd { id: String },
}

pub fn parse(input: &str) -> Result<Document> {
    let normalized = normalize_newlines(input);
    let mut parser = Parser::new(&normalized);
    let file_header = parser.parse_optional_file_header()?;
    let parts = parser.parse_parts()?;
    if parts.is_empty() {
        return Err(ParseError::NoParts);
    }
    Ok(Document { file_header, parts })
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n").replace('\r', "\n")
}

struct Parser<'a> {
    input: &'a str,
    line_starts: Vec<usize>,
    line: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        let mut line_starts = vec![0];
        for (i, c) in input.char_indices() {
            if c == '\n' {
                line_starts.push(i + 1);
            }
        }
        Self {
            input,
            line_starts,
            line: 0,
        }
    }

    fn at_eof(&self) -> bool {
        self.line >= self.line_starts.len()
    }

    fn current_line_number(&self) -> usize {
        self.line + 1
    }

    fn peek_line(&self) -> Option<&'a str> {
        if self.at_eof() {
            return None;
        }
        let start = self.line_starts[self.line];
        let end = self
            .line_starts
            .get(self.line + 1)
            .copied()
            .unwrap_or(self.input.len());
        let mut line = &self.input[start..end];
        if line.ends_with('\n') {
            line = &line[..line.len() - 1];
        }
        Some(line)
    }

    fn consume_line(&mut self) {
        self.line += 1;
    }

    fn parse_optional_file_header(&mut self) -> Result<Option<HeaderBlock>> {
        self.skip_blank_lines();
        if self.at_eof() {
            return Ok(None);
        }
        if matches!(
            self.classify_envelope_line()?,
            Some(EnvelopeKind::Header { .. })
        ) {
            return Ok(Some(self.parse_header_block()?));
        }
        Ok(None)
    }

    fn parse_parts(&mut self) -> Result<Vec<Part>> {
        let mut parts = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();
        while !self.at_eof() {
            self.skip_blank_lines();
            if self.at_eof() {
                break;
            }
            if matches!(
                self.classify_envelope_line()?,
                Some(EnvelopeKind::Header { .. })
            ) {
                return Err(ParseError::SecondFileHeader);
            }
            let part = self.parse_part()?;
            if !seen_ids.insert(part.id.clone()) {
                return Err(ParseError::DuplicatePartId(part.id));
            }
            parts.push(part);
        }
        Ok(parts)
    }

    fn parse_part(&mut self) -> Result<Part> {
        let (id, body_format) = self.expect_part_open()?;
        let header = if matches!(
            self.classify_envelope_line()?,
            Some(EnvelopeKind::Header { .. })
        ) {
            Some(self.parse_header_block()?)
        } else {
            None
        };
        let mut body = String::new();
        while let Some(line) = self.peek_line() {
            if let Some(EnvelopeKind::PartEnd { id: close_id }) =
                self.classify_envelope_line()?
            {
                if close_id != id {
                    return Err(ParseError::PartEndMismatch {
                        expected: id,
                        found: close_id,
                    });
                }
                self.consume_line();
                trim_trailing_newline(&mut body);
                return Ok(Part {
                    id,
                    body_format,
                    header,
                    body,
                });
            }
            if self.is_envelope_line(line) {
                return Err(ParseError::UnexpectedEnvelope {
                    line: self.current_line_number(),
                    content: line.to_string(),
                });
            }
            body.push_str(line);
            body.push('\n');
            self.consume_line();
        }
        Err(ParseError::UnclosedPart(id))
    }

    fn parse_header_block(&mut self) -> Result<HeaderBlock> {
        let format = self.expect_header_open()?;
        let mut payload = String::new();
        while let Some(line) = self.peek_line() {
            if matches!(self.classify_envelope_line()?, Some(EnvelopeKind::HeaderEnd)) {
                self.consume_line();
                trim_trailing_newline(&mut payload);
                return Ok(HeaderBlock { format, payload });
            }
            if self.is_envelope_line(line) {
                return Err(ParseError::UnclosedHeader);
            }
            payload.push_str(line);
            payload.push('\n');
            self.consume_line();
        }
        Err(ParseError::UnclosedHeader)
    }

    fn skip_blank_lines(&mut self) {
        while self.peek_line().is_some_and(|line| line.is_empty()) {
            self.consume_line();
        }
    }

    fn expect_part_open(&mut self) -> Result<(String, Format)> {
        self.skip_blank_lines();
        let line = self.peek_line().ok_or(ParseError::ExpectedPart)?;
        if !self.is_envelope_line(line) {
            return Err(ParseError::ExpectedPart);
        }
        let line_num = self.current_line_number();
        match parse_envelope_kind(line, line_num)? {
            EnvelopeKind::Part { id, body_format } => {
                self.consume_line();
                Ok((id, body_format))
            }
            _ => Err(ParseError::ExpectedPart),
        }
    }

    fn expect_header_open(&mut self) -> Result<Format> {
        let line = self.peek_line().ok_or(ParseError::UnclosedHeader)?;
        let line_num = self.current_line_number();
        match parse_envelope_kind(line, line_num)? {
            EnvelopeKind::Header { format } => {
                self.consume_line();
                Ok(format)
            }
            _ => Err(ParseError::UnclosedHeader),
        }
    }

    fn classify_envelope_line(&self) -> Result<Option<EnvelopeKind>> {
        let Some(line) = self.peek_line() else {
            return Ok(None);
        };
        if !self.is_envelope_line(line) {
            return Ok(None);
        }
        let line_num = self.current_line_number();
        Ok(Some(parse_envelope_kind(line, line_num)?))
    }

    fn is_envelope_line(&self, line: &str) -> bool {
        line.starts_with(SIGIL)
    }
}

fn parse_envelope_kind(line: &str, line_num: usize) -> Result<EnvelopeKind> {
    let rest = line
        .strip_prefix(SIGIL)
        .ok_or_else(|| ParseError::InvalidEnvelope {
            line: line_num,
            content: line.to_string(),
        })?;
    if let Some(after) = rest.strip_prefix('[') {
        return parse_open_envelope(after.trim(), line, line_num);
    }
    if let Some(after) = rest.strip_prefix(']') {
        return parse_close_envelope(after.trim(), line, line_num);
    }
    Err(ParseError::InvalidEnvelope {
        line: line_num,
        content: line.to_string(),
    })
}

fn parse_open_envelope(after: &str, line: &str, line_num: usize) -> Result<EnvelopeKind> {
    if after.is_empty() {
        return Err(ParseError::InvalidEnvelope {
            line: line_num,
            content: line.to_string(),
        });
    }
    if after == HEADER_LABEL {
        return Ok(EnvelopeKind::Header {
            format: Format::HEADER_DEFAULT,
        });
    }
    if let Some(remainder) = after.strip_prefix(HEADER_LABEL) {
        if remainder.is_empty() || remainder.starts_with(' ') {
            let format = parse_format_clause(
                remainder.trim(),
                Format::HEADER_DEFAULT,
                line_num,
                line,
            )?;
            return Ok(EnvelopeKind::Header { format });
        }
    }
    let (id, remainder) = split_id_and_remainder(after, line_num)?;
    let body_format = parse_format_clause(remainder, Format::BODY_DEFAULT, line_num, line)?;
    Ok(EnvelopeKind::Part {
        id: id.to_string(),
        body_format,
    })
}

fn parse_close_envelope(after: &str, line: &str, line_num: usize) -> Result<EnvelopeKind> {
    if after == HEADER_LABEL {
        return Ok(EnvelopeKind::HeaderEnd);
    }
    if after.is_empty() {
        return Err(ParseError::InvalidEnvelope {
            line: line_num,
            content: line.to_string(),
        });
    }
    validate_part_id(after)?;
    Ok(EnvelopeKind::PartEnd {
        id: after.to_string(),
    })
}

fn split_id_and_remainder(after: &str, line_num: usize) -> Result<(&str, &str)> {
    let id = after
        .split_whitespace()
        .next()
        .ok_or_else(|| ParseError::InvalidEnvelope {
            line: line_num,
            content: after.to_string(),
        })?;
    validate_part_id(id)?;
    let remainder = after[id.len()..].trim();
    Ok((id, remainder))
}

fn parse_format_clause(
    remainder: &str,
    default: Format,
    line_num: usize,
    line: &str,
) -> Result<Format> {
    if remainder.is_empty() {
        return Ok(default);
    }
    let Some(eq_pos) = remainder.find('=') else {
        return Err(ParseError::InvalidFormatClause {
            line: line_num,
            content: line.to_string(),
        });
    };
    let key = remainder[..eq_pos].trim();
    let value = remainder[eq_pos + 1..].trim();
    if key != "format" {
        return Err(ParseError::InvalidFormatClause {
            line: line_num,
            content: line.to_string(),
        });
    }
    if value.is_empty() {
        return Err(ParseError::InvalidFormatClause {
            line: line_num,
            content: line.to_string(),
        });
    }
    Format::parse_name(value)
}

fn validate_part_id(id: &str) -> Result<()> {
    if id == HEADER_LABEL {
        return Err(ParseError::InvalidPartId(id.to_string()));
    }
    let valid = if id.len() == 1 {
        id.chars()
            .next()
            .is_some_and(|c| c.is_ascii_lowercase() && c.is_ascii_alphanumeric())
    } else {
        id.chars().all(|c| {
            c.is_ascii_lowercase() || c == '-' || c == '_' || c.is_ascii_digit()
        }) && id
            .starts_with(|c: char| c.is_ascii_lowercase() && c.is_ascii_alphanumeric())
            && id
                .ends_with(|c: char| c.is_ascii_lowercase() && c.is_ascii_alphanumeric())
            && !id.contains("--")
    };
    if valid {
        Ok(())
    } else {
        Err(ParseError::InvalidPartId(id.to_string()))
    }
}

fn trim_trailing_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
    }
}

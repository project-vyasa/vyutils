use crate::ast::{Document, Part};
use crate::error::EditError;

/// Where to insert a new part in a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InsertPosition {
    End,
    Before(String),
    After(String),
}

/// Merge documents in order. Uses the first non-empty file header; errors on duplicate part ids
/// or multiple file headers.
pub fn merge(documents: Vec<Document>) -> Result<Document, EditError> {
    let mut iter = documents.into_iter();
    let Some(mut merged) = iter.next() else {
        return Err(EditError::NoInputDocuments);
    };
    for doc in iter {
        if doc.file_header.is_some() {
            if merged.file_header.is_some() {
                return Err(EditError::MergeFileHeaderConflict);
            }
            merged.file_header = doc.file_header;
        }
        for part in doc.parts {
            if merged.parts.iter().any(|p| p.id == part.id) {
                return Err(EditError::DuplicatePartId(part.id));
            }
            merged.parts.push(part);
        }
    }
    if merged.parts.is_empty() {
        return Err(EditError::NoParts);
    }
    Ok(merged)
}

/// Insert a part into a document.
pub fn add_part(
    doc: &mut Document,
    part: Part,
    position: InsertPosition,
) -> Result<(), EditError> {
    if doc.parts.iter().any(|p| p.id == part.id) {
        return Err(EditError::DuplicatePartId(part.id));
    }
    let idx = match position {
        InsertPosition::End => doc.parts.len(),
        InsertPosition::Before(ref id) => doc
            .parts
            .iter()
            .position(|p| p.id == *id)
            .ok_or_else(|| EditError::PartNotFound(id.clone()))?,
        InsertPosition::After(ref id) => doc
            .parts
            .iter()
            .position(|p| p.id == *id)
            .ok_or_else(|| EditError::PartNotFound(id.clone()))?
            + 1,
    };
    doc.parts.insert(idx, part);
    Ok(())
}

/// Remove a part by id.
pub fn remove_part(doc: &mut Document, id: &str) -> Result<Part, EditError> {
    let idx = doc
        .parts
        .iter()
        .position(|p| p.id == id)
        .ok_or_else(|| EditError::PartNotFound(id.to_string()))?;
    let removed = doc.parts.remove(idx);
    if doc.parts.is_empty() {
        return Err(EditError::NoParts);
    }
    Ok(removed)
}

/// Borrow a part by id.
pub fn find_part<'a>(doc: &'a Document, id: &str) -> Result<&'a Part, EditError> {
    doc.parts
        .iter()
        .find(|p| p.id == id)
        .ok_or_else(|| EditError::PartNotFound(id.to_string()))
}

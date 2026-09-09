//! Top-level transliteration pipeline.

use crate::ascii::{emit_ascii, parse_ascii};
use crate::brahmic::{emit_brahmic, parse_brahmic};
use crate::model::{Token, TransliterateOptions};
use crate::roman::{emit_roman, parse_roman};
use crate::script::Script;

/// Parses an input text in the given script into an intermediate representation token stream.
pub fn parse_to_tokens(text: &str, script: Script) -> Vec<Token> {
    if script.is_indic() {
        parse_brahmic(text, script)
    } else if script.is_ascii() {
        parse_ascii(text, script)
    } else {
        parse_roman(text, script)
    }
}

/// Emits an intermediate representation token stream into the target script.
pub fn emit_from_tokens(
    tokens: &[Token],
    script: Script,
    options: &TransliterateOptions,
) -> String {
    if script.is_indic() {
        emit_brahmic(tokens, script, options.accent_mode)
    } else if script.is_ascii() {
        emit_ascii(tokens, script)
    } else {
        emit_roman(tokens, script, options.accent_mode)
    }
}

/// Transliterates text from one script to another with default options.
pub fn transliterate(text: &str, from: Script, to: Script) -> String {
    transliterate_with_options(text, from, to, &TransliterateOptions::default())
}

/// Transliterates text from one script to another with custom options.
pub fn transliterate_with_options(
    text: &str,
    from: Script,
    to: Script,
    options: &TransliterateOptions,
) -> String {
    if from == to {
        return text.to_string();
    }
    let tokens = parse_to_tokens(text, from);
    emit_from_tokens(&tokens, to, options)
}

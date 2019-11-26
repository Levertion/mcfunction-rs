//! The main lexer for the minecraft commands language.
//! The layout of this file is largely inspired by
//! [m_lexer](https://github.com/matklad/m_lexer/blob/76332c32f87509afc5978ab99a87ab8a29fd272a/src/lib.rs)

use crate::{
    SyntaxKind::{self, *},
    Token,
};
use rowan::{TextRange, TextUnit};

/// Tokenise the given input
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut text = input;
    let mut result = Vec::new();
    while !text.is_empty() {
        let next = next_token(text);
        text = &text[TextRange::from_to(next.len, TextUnit::from_usize(text.len()))];
        result.push(next);
    }
    return result;
}

fn next_token(text: &str) -> Token {
    valid_token(text).unwrap_or_else(|| {
        let mut byte_len = 0;
        for c in text.chars() {
            byte_len += c.len_utf8();
            if text.len() > byte_len && valid_token(&text[byte_len..]).is_some() {
                break;
            }
        }
        let len = TextUnit::from_usize(byte_len);
        Token { kind: ERROR, len }
    })
}

fn valid_token(text: &str) -> Option<Token> {
    static RULES: &[(&str, SyntaxKind)] = &[
        ("{", L_CURLY),
        ("}", R_CURLY),
        ("[", L_SQUARE),
        ("]", R_SQUARE),
        ("@", AT),
        ("=", EQUALS),
        (":", COLON),
        ("..", DOUBLEDOT),
        (".", DOT),
        (",", COMMA),
        ("~", TILDA),
        ("^", CARET),
        ("+", PLUS),
        ("/", SLASH),
        ("-", HYPHEN),
    ];
    for &(token_text, kind) in RULES {
        if text.starts_with(token_text) {
            return Some(Token {
                kind,
                len: TextUnit::of_str(token_text),
            });
        }
    }
    let mut chars = text.char_indices();
    let (_, first) = chars
        .next()
        .expect("next_token should not be called with an empty string");
    if first == '"' || first == '\'' {
        return Some(quoted_string_token(text, first));
    }
    macro_rules! eat_while{
        {$kind: expr, $rule: expr} => {
            if $rule(first) {
                let len = TextUnit::from_usize(
                    chars
                        .skip_while(|&(_, c)| $rule(c))
                        .next()
                        .map(|(i, _)| i)
                        .unwrap_or(text.len()),
                );
                return Some(Token {
                    kind: $kind,
                    len,
                });
            }
        }
    }

    eat_while!(INT, |c: char| c.is_ascii_digit()); // is_ascii_digit has an &self reciever
    eat_while!(WORD, is_allowed_in_word);
    eat_while!(WHITESPACE, char::is_whitespace);
    None
}

fn quoted_string_token(text: &str, quote: char) -> Token {
    let mut indices = text.char_indices().skip(1); // Skip the open quotes
    let mut escaped = false;
    for (_, c) in &mut indices {
        match c {
            '\\' => escaped = !escaped,
            _ if c == quote && !escaped => break,
            // Escape validation occurs elsewhere - when we know we actually have a quoted string and not a greedy string
            _ => escaped = false,
        }
    }
    // Checking for a closing quote comes later
    let len = TextUnit::from_usize(indices.next().map(|(i, _)| i).unwrap_or(text.len()));
    Token {
        kind: QUOTED_STRING,
        len,
    }
}

fn is_allowed_in_word(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic()
}

#[cfg(test)]
mod test;

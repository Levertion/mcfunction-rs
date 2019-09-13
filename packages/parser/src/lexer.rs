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
            if valid_token(&text[byte_len..]).is_some() {
                break;
            }
        }
        let len = TextUnit::from_usize(byte_len);
        Token { kind: OTHER, len }
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
        // (".", DOT),  - This is special cased due to FLOATS like .6
        (",", COMMA),
        ("~", TILDA),
        ("^", CARET),
        ("/", SLASH),
    ];
    for &(token_text, kind) in RULES {
        if text.starts_with(token_text) {
            return Some(Token {
                kind,
                len: TextUnit::of_str(token_text),
            });
        }
    }
    if text.starts_with(".") {
        match text.chars().nth(1) {
            Some(c) if !c.is_ascii_digit() => {
                return Some(Token {
                    len: TextUnit::of_str("."),
                    kind: DOT,
                })
            }
            _ => (),
        };
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
    let mut is_float = false;
    eat_while!(if is_float { FLOAT } else { INT }, |c| is_allowed_number(
        c,
        &mut is_float
    ));
    eat_while!(UNQUOTED_STRING, is_allowed_in_unquoted_string);
    eat_while!(WHITESPACE, char::is_whitespace);
    None
}

fn quoted_string_token(text: &str, quote: char) -> Token {
    let mut indices = text.char_indices().skip(1);
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

fn is_allowed_number(c: char, is_float: &mut bool) -> bool {
    match c {
        '0'..='9' | '-' => true,
        '.' => {
            *is_float = true;
            true
        }
        _ => false,
    }
}

fn is_allowed_in_unquoted_string(c: char) -> bool {
    match c {
        '_' | '0' | '.' | '+' => true,
        _ => c.is_ascii_alphanumeric(),
    }
}

#[cfg(test)]
fn test(input: &str, expected: &str) {
    let mut actual = String::new();
    for t in tokenize(input) {
        actual += &format!("{:?} {}\n", t.kind, t.len.to_usize());
    }
    let expected = expected.trim();
    let actual = actual.trim();

    assert_eq!(
        expected, actual,
        "\nExpected:\n\n\
         {}\n\n\
         Actual:\n\n\
         {}\n\n",
        expected, actual,
    );
}

#[test]
fn test_lexing() {
    test(
        "execute as @a",
        "UNQUOTED_STRING 7
WHITESPACE 1
UNQUOTED_STRING 2
WHITESPACE 1
AT 1
UNQUOTED_STRING 1",
    );
    test(
        "/say unknown chars %*()\"",
        "SLASH 1
UNQUOTED_STRING 3
WHITESPACE 1
UNQUOTED_STRING 7
WHITESPACE 1
UNQUOTED_STRING 5
WHITESPACE 1
OTHER 4
QUOTED_STRING 1",
    );
    test(
        "-10 0 10000000000000000000000000000000000000000000000000",
        "INT 3
WHITESPACE 1
INT 1
WHITESPACE 1
INT 50",
    );
    test(
        "-10.0 .9 123.456",
        "FLOAT 5
WHITESPACE 1
FLOAT 2
WHITESPACE 1
FLOAT 7",
    )
}

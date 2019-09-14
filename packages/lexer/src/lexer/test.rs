use super::tokenize;
use insta::assert_snapshot;
use rowan::{TextRange, TextUnit};

fn pretty_tokenise(input: &str) -> String {
    let mut len: TextUnit = 0u32.into();
    let mut actual = String::new();
    for t in tokenize(input) {
        let end = len + t.len;
        let text = &input[TextRange::from_to(len, end)];
        actual += &format!("{:?}: {:?}\n", text, t.kind);
        len = end;
    }
    return actual.trim().to_owned();
}

macro_rules! lexer_test {
    ($name: ident, $test: expr) => {
        #[test]
        fn $name() {
            assert_snapshot!(pretty_tokenise($test));
        }
    };
}

lexer_test!(execute, "execute as @a");
lexer_test!(unknown_chars, "%*()");
lexer_test!(
    numbers,
    "-10 0 10000000000000000000000000000000000000000000000000 -10.0 .9 123.456"
);
lexer_test!(other_symbols, "{}[]@=:..,.~^+/-");
lexer_test!(spaces, "Single space-Double  space-Many          space");
lexer_test!(
    other_whitespace,
    "Tab:\u{9}. Newline:
Newline and Spaces:    
        "
);
lexer_test!(
    valid_quoted_strings_double,
    r#""Simple valid"
"Valid with \"escaped quotes\""
"Valid with escaped backslashes\\ and \"quotes\""
"Valid with 'Other quotes'""#
);
lexer_test!(
    valid_quoted_strings_single,
    r#"'Simple valid'
'Valid with \'escaped quotes\''
'Valid with escaped backslashes\\ and \'quotes\''
'Valid with "Other quotes"'"#
);
lexer_test!(
    unclosed_quoted_string,
    r#""Even though this quoted string is never \"closed\" it is still accepted."#
);
lexer_test!(
    invalid_escapes,
    r#""In brigadier, the only escapes are for \\ backslashes and \"quotes\". Our lexer allows other escapes too,
e.g. \h \u \i etc., because that allows us to have sane error recovery.""#
);

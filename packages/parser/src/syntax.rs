#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    pub kind: SyntaxKind,
    pub len: rowan::TextUnit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum SyntaxKind {
    // Tokens - produced by the lexer
    // Brackets/braces
    L_CURLY = 0, // {
    R_CURLY,     // }
    L_SQUARE,    // [
    R_SQUARE,    // ]
    // Symbols
    AT,        // @
    EQUALS,    // =
    COLON,     // :
    DOUBLEDOT, // .. - used for ranges
    DOT,       // . - used in NBT paths
    COMMA,     // ,
    TILDA,     // ~
    CARET,     // ^
    SLASH,     // / - used at the beginning of the string
    // Composite tokens produced by the lexer
    // N.B. INT and FLOAT are subsets of UNQUOTED_STRING
    // Also note that INTs and FLOATS are not
    INT,             // An integer. May be negative.
    FLOAT,           // A float. Does *not* include any exponentional
    UNQUOTED_STRING, // Any unquoted words, such as execute, Levertion
    QUOTED_STRING,   // E.g. "this can contain spaces and (potentially invalid) \"escapes\""
    OTHER, // A syntax construct which is unrecognised by the lexer - in most cases this is an error except in greedy strings
    WHITESPACE, // Any whitespace.
    ROOT,  // The final token
}

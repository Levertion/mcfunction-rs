#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    pub kind: SyntaxKind,
    pub len: rowan::TextUnit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
/// The kind of all the syntaxes. This is implemented by the parser
pub enum SyntaxKind {
    // Special kinds
    EOF = 0, // A special syntax kind used to mark the end of the file. Never used in the final tree
    TOMBSTONE, // A special syntax kind used to mark an Event with an unmarked
    ERROR,   // Used to mark an error in lexing or parsing
    // Tokens - produced by the lexer
    L_CURLY,   // {
    R_CURLY,   // }
    L_SQUARE,  // [
    R_SQUARE,  // ]
    AT,        // @
    EQUALS,    // =
    COLON,     // :
    DOUBLEDOT, // .. - used for ranges
    DOT,       // . - used in NBT paths and floats
    COMMA,     // ,
    TILDA,     // ~
    CARET,     // ^
    PLUS,      // + - used in SNBT numbers (not in normal numbers)
    HYPHEN,    // - used to negate an integer or float
    SLASH,     // / - used at the beginning of the command. This will be accepted in parsing
    // but will give an error at validation
    INT,           // An integer. Will not be negative
    WORD,          // Any sequences of letters or _s
    QUOTED_STRING, // E.g. "this can contain spaces and (potentially invalid) \"escapes\"". It also might not be closed
    // If it turns out to be a quoted string in parsing, it will be validated
    WHITESPACE, // Any whitespace.
    ROOT,       // The final 'fixed' syntax kind. The type of the root node
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The kind of syntax node represented in the syntax tree
pub enum SyntaxNodeKind {
    /// A normal syntax kind
    Syntax(SyntaxKind),
    /// A syntax kind which is associated with a node in the command tree
    Command(u16), // Treated as a 'u15'
}

const BIT: usize = 15;
const FLAG: u16 = 1 << BIT;

/// Convert into a Rowan SyntaxKind. The most significant bit represents the kind.
///
/// If it is 1, the node is a command. If it is 0, the node is syntaxkind
impl From<SyntaxNodeKind> for rowan::cursor::SyntaxKind {
    fn from(kind: SyntaxNodeKind) -> Self {
        Self(match kind {
            SyntaxNodeKind::Syntax(kind) => {
                assert_eq!(kind as u16 & FLAG, 0);
                kind as u16
            }
            SyntaxNodeKind::Command(command) => {
                assert_eq!(command as u16 & FLAG, 0);
                command | FLAG
            }
        })
    }
}

impl From<rowan::cursor::SyntaxKind> for SyntaxNodeKind {
    fn from(it: rowan::cursor::SyntaxKind) -> Self {
        let it = it.0;
        match FLAG & it {
            0 => {
                // Required for safety
                assert!(it <= SyntaxKind::ROOT as u16);
                // Safe because we use repr(u16), start from 0 and check the value is less than ROOT, which is a value.
                // We also don't assign discriminants manaully apart from beginning.
                // This could also be written in terms of a massive match, but that causes codegen issues.
                #[allow(unsafe_code)]
                SyntaxNodeKind::Syntax(unsafe { std::mem::transmute(it) })
            }
            result @ _ => {
                assert_eq!(result, FLAG);
                let command = it & !FLAG;
                SyntaxNodeKind::Command(command)
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {}

type SyntaxNode = rowan::SyntaxNode<Lang>;
#[allow(unused)]
type SyntaxToken = rowan::SyntaxToken<Lang>;
#[allow(unused)]
type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

impl rowan::Language for Lang {
    type Kind = SyntaxNodeKind;
    fn kind_from_raw(raw: rowan::cursor::SyntaxKind) -> Self::Kind {
        raw.into()
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::cursor::SyntaxKind {
        kind.into()
    }
}

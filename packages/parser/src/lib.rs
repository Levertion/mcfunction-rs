// Unsafe is allowed ONLY in syntax.rs because of rowan
#![deny(unsafe_code)]

mod lexer;
mod parser;
mod syntax;
pub use lexer::tokenize;
pub use syntax::{SyntaxKind, Token};

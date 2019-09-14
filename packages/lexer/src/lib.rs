mod lexer;
mod syntax;
pub use lexer::tokenize;
pub use syntax::{SyntaxKind, Token};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

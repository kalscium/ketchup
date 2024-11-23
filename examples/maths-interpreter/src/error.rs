//! Error definitions

/// Errors that occur during the lexing, parsing or interpreting
///
/// *(real programming language implementations should probably split this into multiple enums for each phase)*
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Error {
    /// Occurs when there is a character that the lexer doesn't recognise
    #[default]
    UnexpectedCharacter,
}


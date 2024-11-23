//! Error definitions

use crate::{parser::Expr, span::{Span, Spanned}};
use ketchup::error::Error as KError;

/// Errors that occur during the lexing, parsing or interpreting
///
/// *(real programming language implementations should probably split this into multiple enums for each phase)*
#[derive(Debug, Clone)]
pub enum Error {
    /// Occurs when there is a character that the lexer doesn't recognise
    UnexpectedCharacter(Span),

    /// Occurs when there is an unexpected expr
    UnexpectedExpr(Spanned<Expr>),

    /// Occurs when there is an expected expr that is not found, includes the operation that requires the node
    ExpectedExpr(Option<Spanned<Expr>>),

    /// Occurs when there is an expected expr but finds a different token instead
    ExpectedExprFoundOther {
        /// The operation that requires the node
        oper: Option<Spanned<Expr>>,
        /// The expr node it found instead
        found: Spanned<Expr>,
    },

    /// Occurs when a parentheses isn't closed/terminated
    UnclosedParen {
        /// The span of the opening parenthesis
        start_span: Span,
        /// The expected span of the closing parenthesis
        expected_span: Span,
    },
}

impl From<KError<'_, Spanned<Expr>>> for Error {
    fn from(error: KError<Spanned<Expr>>) -> Self {
        match error {
            KError::UnexpectedNode(node) => Error::UnexpectedExpr(node),
            KError::ExpectedNode(oper) => Error::ExpectedExpr(oper.cloned()),
            KError::UnexpectedExpectedNode { oper, found } => Error::ExpectedExprFoundOther { oper: oper.cloned(), found },
        }
    }
}


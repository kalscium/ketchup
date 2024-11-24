//! Error definitions

use crate::{parser::Expr, span::{Span, Spanned}};
use ariadne::{Color, Label, Report};
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
    ExpectedExpr(Spanned<Expr>),

    /// Occurs when there is an expected expr but finds a different token instead
    ExpectedExprFoundOther {
        /// The operation that requires the node
        oper: Option<Spanned<Expr>>,
        /// The expr node it found instead
        found: Spanned<Expr>,
    },

    /// Occurs when there is nothing in a program, includes where it should be
    EmptyProgram(Span),

    /// Occurs when a 'scope' (parentheses) are empty
    EmptyParen {
        /// The span of the parentheses
        span: Span,
        /// The span of where the expr should have been
        expected_span: Span,
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
            KError::ExpectedNode(Some(oper)) => Error::ExpectedExpr(oper.clone()),
            KError::ExpectedNode(None) => unreachable!("this error should've been handled manually long before this"),
            KError::UnexpectedExpectedNode { oper, found } => Error::ExpectedExprFoundOther { oper: oper.cloned(), found },
        }
    }
}

/// Prints a pretty error message
pub fn print(error: Error, src: impl ariadne::Cache<String>) {
    match error {
        Error::UnexpectedCharacter(span) => print_no_context("unexpected or invalid character", span, "consider removing this", src),
        Error::UnexpectedExpr(Spanned { span, .. }) => print_no_context("unexpected expression", span, "consider either removing this or putting an operator before it", src),
        Error::EmptyParen { span, expected_span } => print_single_context("empty parentheses/scope", expected_span, "expected an expression here", span, "while parsing this 'scope'", src),
        Error::UnclosedParen { start_span, expected_span } => print_single_context("unclosed parentheses", expected_span, "expected `)`", start_span, "to complete this 'scope'", src),

        Error::ExpectedExpr(oper) => print_single_context(
            "expected expression",
            Span { filename: oper.span.filename.clone(), range: oper.span.range.end..oper.span.range.end },
            "expected an expr here",
            oper.span,
            &format!("to complete this '{:?}' operation", oper.item),
            src,
        ),

        Error::ExpectedExprFoundOther { oper: Some(oper), found } => print_single_context(
            "expected expression",
            found.span,
            "found this instead",
            oper.span,
            &format!("to complete this '{:?}' operation", oper.item),
            src,
        ),

        Error::ExpectedExprFoundOther { oper: None, found } => print_no_context("expected expression", found.span, "found this instead", src),

        // custom error message
        Error::EmptyProgram(span) => {
            Report::build(ariadne::ReportKind::Error, span.clone())
                .with_message("empty program")
                .with_label(
                    Label::new(span)
                        .with_message("expected an expr here")
                        .with_color(Color::Red)
                )
                .with_note("Meow? (Waiting for something to happen?)") // omori reference
                .finish()
                .eprint(src)
                .unwrap();
        },
    }
}

/// Prints a pretty error message with no extra context spans
fn print_no_context(
    msg: &str,
    span: Span,
    label: &str,
    src: impl ariadne::Cache<String>,
) {
    Report::build(ariadne::ReportKind::Error, span.clone())
        .with_message(msg)
        .with_label(
            Label::new(span)
                .with_message(label)
                .with_color(Color::Red)
        )
        .finish()
        .eprint(src)
        .unwrap();
}

/// Prints a pretty error message with a single piece of additional context
fn print_single_context(
    msg: &str,
    span: Span,
    label: &str,
    ctx_span: Span,
    ctx_label: &str,
    src: impl ariadne::Cache<String>,
) {
    Report::build(ariadne::ReportKind::Error, span.clone())
        .with_message(msg)
        .with_label(
            Label::new(span)
                .with_message(label)
                .with_color(Color::Red)
        )
        .with_label(
            Label::new(ctx_span)
                .with_message(ctx_label)
                .with_color(Color::BrightBlue)
        )
        .finish()
        .eprint(src)
        .unwrap();
}

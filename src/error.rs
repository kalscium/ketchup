//! Errors used by the ketchup parser

use crate::Span;
use std::fmt::Debug;

/// Errors used by the ketchup parser
#[derive(Debug, Clone)]
pub enum KError<Error: Debug + Clone> {
    /// Occurs when there is a node with space of two that is place in way where there first input may never be fullfilled (`/1` or `1 +/ 2`)
    DoubleSpaceConflict {
        /// Span of where a node with zero space was expected (input)
        span: Span,
    },

    /// Occurs when there is a node added to the `ASA` that is not expected by the current node (no space left)
    UnexpectedOper(Span),

    /// Occurs when there is a node in the `ASA` with a missing input (non-zero space)
    ExpectedOper {
        /// Span of the node with missing inputs
        span: Span,
        /// precedence of the node with missing inputs
        precedence: u8,
    },

    /// Custom errors outside of ketchup
    Other(Span, Error),
}

impl<Error: Debug + Clone> KError<Error> {
    #[inline]
    pub fn span(&self) -> &Span {
        use KError as K;
        match self {
            K::DoubleSpaceConflict { span } => span,
            K::UnexpectedOper(span) => span,
            K::ExpectedOper { span, .. } => span,
            K::Other(span, _) => span,
        }
    }
}

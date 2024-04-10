use crate::Span;

#[derive(Debug, Clone)]
pub enum Error<Other> {
    /// occurs when there is a node with a space of two that is placed in a way where the first input may never be fullfilled (`/1` or `1 +/ 2`)
    DoubleSpaceConflict {
        /// span of where a operation of zero space was expected (input) 
        span: Span,
    },

    /// occurs when there is a node added to the `ASA` that is not expected by the current node (negative-space node)
    UnexpectedOper(Span),

    /// occurs when there is a node in the `ASA` with a missing input (non-zero space)
    ExpectedOper {
        /// span of the node with missing inputs
        span: Span,
        /// precedence of the node with missing inputs
        precedence: u8,
    },

    /// custom errors outside of ketchup
    Other(Other),
}

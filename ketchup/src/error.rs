use crate::Span;

#[derive(Debug, Clone)]
pub enum KError<Token, Other> {
    /// Occurs when there is a node with space of two that is place in way where there first input may never be fullfilled (`/1` `1 +/ 2`)
    DoubleSpaceConflict {
        /// Span of where a node with zero space was expected (input)
        span: Span,
    },

    /// Occurs when there is a node added to the `ASA` that is not expected by the current node (no space left)
    UnexpectedOper(Span),

    /// Occurs when there is anode in the `ASA` with a missing input (non-zero space)
    ExpectedOper {
        /// Span of the node with missing inputs
        span: Span,
        /// precedence of the node with missing inputs
        precedence: u8,
    },

    /// Occurs when there is an expected terminator (such as `)`) token that isn't met
    ExpectedEOF(Token),

    /// Custom errors outside of ketchup
    Other(Span, Other),
}

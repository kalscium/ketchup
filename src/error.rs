//! Enums for errors in ketchup

use crate::node;

/// An error that can occur in ketchup
pub enum Error<Node: node::Node> {
    /// Occurs when there is an unexpected node inserted when the ASA is already complete, includes the unexpected node
    UnexpectedNode(Node),
    /// Occurs when there is a required node for an operation that isn't present, includes the operation (unary or binary) node
    ExpectedNode(Node),
}

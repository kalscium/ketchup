//! Enums for errors in ketchup

use std::fmt::Debug;
use crate::node;

/// An error that can occur in ketchup
#[derive(Debug, Clone)]
pub enum Error<'a, Node: node::Node> {
    /// Occurs when there is an unexpected node inserted when the ASA is already complete, includes the unexpected node
    UnexpectedNode(Node),
    /// Occurs when there is a required node for an operation that isn't present, includes the operation (unary right-aligned or binary) node
    ExpectedNode(Option<&'a Node>),
    /// Occurs when there is a required node for an operation, but instead found a unary (right-aligned) or binary node
    UnexpectedExpectedNode {
        /// The unary or binary node that requires the node
        oper: Option<&'a Node>,
        /// The unary (right-aligned) or binary node found instead
        found: Node,
    },
}

use std::fmt::Debug;
use crate::Span;

#[derive(Debug)]
pub struct Node<Oper: Debug> {
    /// The type of operation that the node is
    pub oper: Oper,
    /// Information about the node
    pub info: NodeInfo,
}

#[derive(Debug)]
/// Information about a node in the AST
pub struct NodeInfo {
    /// The location of the node's token in the parsed string
    pub span: Span,
    /// The parent of the node in the `ASA`
    pub parent: Option<usize>,
    /// The precedence of the node in the `ASA`
    pub precedence: u8,
    /// If there is space left for another input node
    pub space: bool,
}

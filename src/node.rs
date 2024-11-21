//! Traits for nodes in the ASA

use std::fmt::Debug;
use crate::Precedence;

/// An element in the **Abstract Syntax Array**
///
/// Each node must have a 'type' the determines it's recursive handed-ness, it's precedence and what kind of node it is; an operand, unary (left-aligned), unary (right-aligned) or a binary node
///
/// Nodes only need to be queried on their precedence and their kind (which is determined by their 'type')
pub trait Node: Debug + Clone {
    /// Queries the precedence of the node
    fn get_precedence(&self) -> Precedence;
    /// Queries the kind of node
    fn get_kind(&self) -> NodeKind;
}

/// Different kinds of nodes in the ASA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    /// A terminal node that doesn't require any 'parameters'
    Operand,
    /// A node that has one 'parameters'
    Unary,
    /// A node that has two 'parameters'
    Binary,
}

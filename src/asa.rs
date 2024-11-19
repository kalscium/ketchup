//! Traits for implementing an ASA

use crate::node;

/// An Abstract Syntax Array
/// 
/// An efficient and cpu cache friendly one dimensional AST (Abstract Synatx Tree)
///
/// Needs to allow for querying of nodes, inserting nodes, pushing to the array and querying the length of the array
pub trait ASA {
    /// The internal node implementation
    type Node: node::Node;

    /// Initialises a new ASA (incomplete)
    fn new() -> Self;

    /// Queries a node in the ASA (panic on out-of-bounds index)
    fn get_node(&self, idx: usize) -> &Self::Node;
    /// Queries the length of the ASA
    fn get_len(&self) -> usize;

    /// Pushes a node to the end of the ASA
    fn push(&mut self, node: Self::Node);
    /// PUshes a node to the start of the ASA
    fn push_start(&mut self, node: Self::Node);

    /// Inserts a node into an index in the ASA (panic on out-of-bounds)
    fn insert(&mut self, idx: usize, node: Self::Node);

    /// Returns a mutable pointer to the 'completed' flag/field of the ASA
    fn completed(&mut self) -> &mut bool;
}

/// An implementation of ASA that uses an underlying vector
#[derive(Debug, Clone)]
pub struct VectorASA<Node: node::Node> {
    complete: bool,
    vector: Vec<Node>,
}

impl<Node: node::Node> ASA for VectorASA<Node> {
    type Node = Node;

    #[inline]
    fn new() -> Self {
        Self {
            complete: false,
            vector: Vec::new(),
        }
    }

    #[inline]
    fn get_len(&self) -> usize {
        self.vector.len()
    }

    #[inline]
    fn get_node(&self, idx: usize) -> &Self::Node {
        &self.vector[idx]
    }

    #[inline]
    fn push(&mut self, node: Self::Node) {
        self.vector.push(node);
    }

    #[inline]
    fn push_start(&mut self, node: Self::Node) {
        self.vector.insert(0, node);
    }

    #[inline]
    fn insert(&mut self, idx: usize, node: Self::Node) {
        self.vector.insert(idx, node);
    }

    #[inline]
    fn completed(&mut self) -> &mut bool {
        &mut self.complete
    }
}

//! Traits for implementing an ASA

use crate::{node, Precedence};

/// An Abstract Syntax Array
/// 
/// An efficient and cpu cache friendly one dimensional AST (Abstract Synatx Tree)
///
/// Needs to allow for querying of nodes, inserting nodes, pushing to the array and querying the length of the array
pub trait ASA {
    /// The internal node implementation
    type Node: node::Node;

    /// Initialises a new ASA (incomplete) (max precedence is the largest precedence value that your parser uses, precedence values MUST be in order with **NO GAPS**)
    fn new(max_precedence: Precedence) -> Self;

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

    /// Returns a mutable pointer to the `completed` flag/field of the ASA
    fn is_complete(&mut self) -> &mut bool;

    /// Returns the **MAXIMUM** possible precedence (precedences **MUST** be in order with **no gaps**)
    fn max_precedence(&self) -> Precedence;

    /// Returns a mutable pointer to the `last_incomplete` field (index in the ASA)
    fn last_incomplete(&mut self) -> &mut Option<usize>;

    /// Returns a mutable pointer to the precedence jumptable array
    fn precedence_jumptable(&mut self) -> &mut [Option<usize>];
}

/// An implementation of ASA that uses an underlying vector
#[derive(Debug, Clone)]
pub struct VectorASA<Node: node::Node> {
    is_complete: bool,
    last_incomplete: Option<usize>,
    max_precedence: Precedence,
    precedence_jumptable: Box<[Option<usize>]>,
    /// The internal vector
    pub vector: Vec<Node>,
}

impl<Node: node::Node> ASA for VectorASA<Node> {
    type Node = Node;

    #[inline]
    fn new(max_precedence: Precedence) -> Self {
        Self {
            is_complete: false,
            last_incomplete: None,
            max_precedence,
            precedence_jumptable: vec![None; max_precedence+1].into_boxed_slice(),
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
    fn is_complete(&mut self) -> &mut bool {
        &mut self.is_complete
    }

    #[inline]
    fn last_incomplete(&mut self) -> &mut Option<usize> {
        &mut self.last_incomplete
    }

    #[inline]
    fn max_precedence(&self) -> Precedence {
        self.max_precedence
    }

    #[inline]
    fn precedence_jumptable(&mut self) -> &mut [Option<usize>] {
        &mut self.precedence_jumptable
    }
}

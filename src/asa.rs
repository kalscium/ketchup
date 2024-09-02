//! The **Abstract Syntax Array** is like an `AST` though optimised for cache locality through the usage of an array instead of pointers to nodes in a tree

use std::fmt::Debug;
use crate::node::Node;

/// The **Abstract Syntax Array**, like an `AST` but in an array
pub trait ASA: IntoIterator<Item = Node<Self::Oper>> + Default + Debug {
    /// The node type used within the array
    type Oper: Debug;

    /// pushes a node to the end of the `ASA`
    fn push(&mut self, node: Node<Self::Oper>);
    /// inserts a node into a position in the `ASA`
    fn insert(&mut self, idx: usize, node: Node<Self::Oper>);
    /// returns a node at a position in the `ASA`
    fn get(&mut self, idx: usize) -> &mut Node<Self::Oper>;
    /// removes a node at a position in the `ASA`
    fn remove(&mut self, idx: usize);
    /// returns the length of the `ASA`
    fn len(&self) -> usize;
    /// returns if the asa is empty
    fn is_empty(&self) -> bool;
}

/// The default implementation of `ASA` for a vector
impl<Oper: Debug> ASA for Vec<Node<Oper>> {
    type Oper = Oper;

    #[inline]
    fn push(&mut self, node: Node<Self::Oper>) {
        self.push(node);
    }

    #[inline]
    fn insert(&mut self, idx: usize, node: Node<Self::Oper>) {
        self.insert(idx, node);
    }

    #[inline]
    fn get(&mut self, idx: usize) -> &mut Node<Self::Oper> {
        &mut self[idx]
    }

    #[inline]
    fn remove(&mut self, idx: usize) {
        self.remove(idx);
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

// /// # TODO
// /// a padded version of an `ASA` vector for extra performance upon ownership changes
// /// because a vector that has capacity at both the end and start of a vector allows for
// /// items to be inserted at the start of the vector without having to shift everything to the right
// pub struct PaddedVec<Token> {
//     offset: usize,
//     array: Box<[Token]>,
//     len: usize,
// }

// impl<Token> PaddedVec<Token> {
//     /// creates a new padded vector
//     #[inline]
//     pub fn new() -> Self {
//         Self {
//             offset: 0,
//             array: Box::new([]),
//             len: 0,
//         }
//     }

//     /// creates a new padded vector with a specified capacity
//     #[inline]
//     pub fn with_capacity(capacity: usize) -> Self {
//         Self {
//             offset: capacity / 2,
//             array: Box::new([])
//         }
//     }
// }

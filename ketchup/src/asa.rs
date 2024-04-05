//! The **Abstract Syntax Array** is like the `AST` though optimised for cache locality through the usage of an array instead of pointers to memory

use crate::node::Node;
use core::fmt::Debug;

/// the **Abstract Syntax Array**, like the `AST` but in an array-like object
pub trait ASA: IntoIterator + Default + Debug {
    /// the token used within the array
    type Token: Debug;

    /// pushes a node to the end of the `ASA`
    fn push(&mut self, node: Node<Self::Token>);
    /// inserts a node into a position in the `ASA`
    fn insert(&mut self, idx: usize, node: Node<Self::Token>);
    /// returns a node at a position in the `ASA`
    fn get(&mut self, range: usize) -> &mut Node<Self::Token>;
    /// returns the length of the `ASA`
    fn len(&self) -> usize;
}

/// a default implementation of `ASA` for a vector
impl<Token: Debug> ASA for Vec<Node<Token>> {
    type Token = Token;

    #[inline]
    fn push(&mut self, node: Node<Self::Token>) {
        self.push(node);
    }

    #[inline]
    fn insert(&mut self, idx: usize, node: Node<Self::Token>) {
        self.insert(idx, node);
    }

    #[inline]
    fn get(&mut self, idx: usize) -> &mut Node<Self::Token> {
        &mut self[idx]
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

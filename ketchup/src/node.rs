use crate::Span;
use core::fmt::Debug;

#[derive(Debug)]
/// a single node in the `ASA`
pub struct Node<Oper: Debug> {
    /// the interal type of the node
    pub oper: Oper,
    /// the location of that node's token
    pub span: Span,
    /// the parent of the node in the `ASA`
    pub parent: Option<usize>,
    /// the precedence of the node's token type in the `ASA`
    pub precedence: u8,
}

impl<Oper: Debug> Node<Oper> {
    /// creates a new node
    #[inline]
    pub fn new(oper: Oper, span: Span, parent: Option<usize>, precedence: u8) -> Self {
        Self {
            oper,
            span,
            parent,
            precedence,
        }
    }
}

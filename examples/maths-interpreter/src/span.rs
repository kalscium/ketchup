//! Span type definitions

use std::fmt::Debug;
use ketchup::node::Node;

/// The location of a node in a source-code file
#[derive(Debug, Clone)]
pub struct Span {
    pub filename: String,
    pub range: std::ops::Range<usize>,
}

/// A value that is tagged with a span
#[derive(Debug, Clone)]
pub struct Spanned<T: Debug + Clone> {
    pub item: T,
    pub span: Span,
}

impl<T: Debug + Clone> Spanned<T> {
    #[inline]
    pub fn new(item: T, span: Span) -> Self {
        Self {
            item,
            span,
        }
    }
}

impl<T: Debug + Clone + Node> Node for Spanned<T> {
    #[inline]
    fn get_kind(&self) -> ketchup::prelude::NodeKind {
        self.item.get_kind()
    }

    #[inline]
    fn get_precedence(&self) -> ketchup::Precedence {
        self.item.get_precedence()
    }
}

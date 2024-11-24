//! Span type definitions

use std::fmt::Debug;
use ketchup::node::Node;

/// The location of a node in a source-code file
#[derive(Debug, Clone)]
pub struct Span {
    pub filename: String,
    pub range: std::ops::Range<usize>,
}

impl ariadne::Span for Span {
    type SourceId = String;

    #[inline]
    fn source(&self) -> &Self::SourceId { &self.filename }

    #[inline]
    fn start(&self) -> usize { self.range.start }

    #[inline]
    fn end(&self) -> usize { self.range.end }

    #[inline]
    fn len(&self) -> usize { self.range.end - self.range.start }

    #[inline]
    fn is_empty(&self) -> bool { self.range.is_empty() }

    #[inline]
    fn contains(&self, offset: usize) -> bool { self.range.contains(&offset) }
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

//! Structures that store information about operations (future nodes)

use std::fmt::Debug;
use crate::{Space, Span};

/// Information about an operation (future node)
#[derive(Debug, Clone)]
pub struct OperInfo<Oper: Debug + Clone> {
    /// The internal operation
    pub oper: Oper,
    /// Span of the operation
    pub span: Span,
    /// The amount of inputs the operation may have
    pub space: Space,
    /// The precedence of the operation
    pub precedence: u8,
}

/// Possible operations from a token
#[derive(Debug, Clone)]
pub struct OperOption<Oper: Debug + Clone> {
    /// A possible operation that requires there be no input nodes (`x`)
    pub no_space    : Option<OperInfo<Oper>>,
    /// A possible operation that requires a single input node to the right of it (`x_`)
    pub single_space: Option<OperInfo<Oper>>,
    /// A possible operation that requires two inputs nodes surrounding it (`_x_`)
    pub double_space: Option<OperInfo<Oper>>,
}

impl<Oper: Debug + Clone> OperOption<Oper> {
    /// Creates a `OperOption` with only one option (no_space)
    #[inline]
    pub fn no_space(info: OperInfo<Oper>) -> Self {
        Self {
            no_space: Some(info),
            single_space: None,
            double_space: None,
        }
    }

    /// Creates a `OperOption` with only one option (single_space)
    #[inline]
    pub fn single_space(info: OperInfo<Oper>) -> Self {
        Self {
            no_space: None,
            single_space: Some(info),
            double_space: None,
        }
    }

    /// Creates a `OperOption` with only one option (single_space)
    #[inline]
    pub fn double_space(info: OperInfo<Oper>) -> Self {
        Self {
            no_space: None,
            single_space: None,
            double_space: Some(info),
        }
    }
}

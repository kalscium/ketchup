//! A parser that can *ketch - up* with your programming language.

#![warn(missing_docs)]

pub mod node;

/// The precedence of an operation / node
pub type Precedence = u8;

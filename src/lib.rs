//! A parser that can *ketch - up* with your programming language.

#![warn(missing_docs)]

pub mod node;
pub mod asa;
pub mod parse;
pub mod error;
pub mod prelude;

/// The precedence of an operation / node
pub type Precedence = u8;

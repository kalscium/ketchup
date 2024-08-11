pub mod node;
pub mod asa;
pub mod error;
pub mod parser;

/// A location in the parsed string
pub type Span = std::ops::Range<usize>;

/// Information about an operation (future node)
#[derive(Debug)]
pub struct OperInfo<Oper: std::fmt::Debug> {
    /// The internal operation
    pub oper: Oper,
    /// The amount of inputs the operation may have
    pub space: Space,
    /// The precedence of the operation
    pub precedence: u8,
}

/// The amount of space a future node may have
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    /// Something with no space for other nodes `x`
    None,
    /// Something with space for only one other node after of it `x _`
    Single,
    /// Something with space for a node before and after it `_ x _`
    Double,
}

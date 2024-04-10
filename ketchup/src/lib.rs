pub mod asa;
pub mod parser;
pub mod node;
pub mod token_info;
pub mod error;

/// a simple type to represent the span of a token
pub type Span = std::ops::Range<usize>;

/// different amounts of space a node can have
#[derive(Debug, Clone, Copy)]
pub enum Space {
    Zero = 0, // `x`
    One = 1,  // `x_`
    Two = 2,  // `_x_`
}

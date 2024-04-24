pub mod asa;
pub mod parser;
pub mod node;
pub mod token_info;
pub mod error;

/// a simple type to represent the span of a token
pub type Span = std::ops::Range<usize>;

/// different amounts of space a node can have
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    Zero, // `x`
    One,  // `x_`
    Two,  // `_x_`
}

/// performs a transformation on a value
#[inline]
pub fn map<T>(mut x: T, f: impl Fn(&mut T)) -> T {
    f(&mut x);
    x
}

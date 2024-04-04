pub mod asa;
pub mod parser;
pub mod node;

/// a simple type to represent the span of a token
pub type Span = std::ops::Range<usize>;

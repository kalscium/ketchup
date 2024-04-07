pub mod asa;
pub mod parser;
pub mod node;
pub mod token_info;

/// a simple type to represent the span of a token
pub type Span = std::ops::Range<usize>;

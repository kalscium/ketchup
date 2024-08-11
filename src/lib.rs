//! ## Example
//! ---
//! *for a full implementation/example check the `examples` directory*
//! ```rust
//! use ketchup::{error::KError, node::Node, parser::Parser, OperInfo, Space, Span};
//! 
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! pub enum Error {
//!     CustomError,
//! }
//! 
//! #[derive(Debug, Clone, PartialEq)]
//! pub enum Token {
//!     Number(u32),
//!     Plus,
//!     Minus,
//!     Star,
//!     Slash,
//! }
//! 
//! #[derive(Debug, Clone)]
//! pub enum Oper {
//!     Num(u32),
//!     Add,
//!     Sub,
//!     Mul,
//!     Div,
//! }
//! 
//! fn oper_generator(token: Token, tokens: &mut impl Iterator<Item = (Result<Token, Error>, Span)>, double_space: bool) -> Result<OperInfo<Oper>, Vec<KError<Token, Error>>> {
//!     use Token as T;
//!     use Oper as O;
//! 
//!     let (precedence, space, oper) = match (token, double_space) {
//!         (T::Number(x), _) => (0, Space::None, O::Num(x)),
//!         (T::Plus, _) => (3, Space::Double, O::Add), // larger precedence changes the order of operations
//!         (T::Minus, _) => (3, Space::Double, O::Sub),
//!         (T::Star, _) => (2, Space::Double, O::Mul),
//!         (T::Slash, _) => (2, Space::Double, O::Div),
//!     };
//! 
//!     Ok(OperInfo {
//!         oper,
//!         span: 0..0, // placeholder for logos `.span()`
//!         space,
//!         precedence,
//!     })
//! }
//! 
//! fn main() {
//!     // source to parse
//!     let mut src = [(Ok(Token::Number(1)), 0..1)].into_iter();
//! 
//!     // initialise parser
//!     let parser = Parser::<'_, Token, Oper, _, Vec<Node<Oper>>, _, Error>::new(&mut src, None, oper_generator);
//! 
//!     // parse and handle errors
//!     let asa = parser.parse().unwrap();
//! 
//!     // print abstract syntax array
//!     println!("{asa:?}");
//! }
//! ```

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
    /// Span of the operation
    pub span: Span,
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

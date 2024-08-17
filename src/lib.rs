//! ## Example
//! ---
//! *for a full implementation/example check the `examples` directory*
//! ```rust
//! use ketchup::{error::KError, node::Node, parser::Parser, OperInfo, Space, Span};
//! 
//! #[derive(Debug, Clone, Default, PartialEq, Eq)]
//! pub enum Error {
//!     #[default]
//!     UnexpectedCharacter,
//!     EmptyParentheses,
//!     UnclosedParentheses,
//!     UnexpectedToken,
//! }
//! 
//! /// A simple logos lexer
//! #[derive(Debug, Clone, PartialEq)]
//! pub enum Token {
//!     Number(u32),
//!     Plus,
//!     Minus,
//!     Star,
//!     Slash,
//! }
//! 
//! /// The operations / nodes that will be used
//! #[derive(Debug, Clone)]
//! pub enum Oper {
//!     Num(u32),
//!     Add,
//!     Sub,
//!     Mul,
//!     Div,
//!     Neg,
//!     Pos,
//! }
//! 
//! fn oper_generator(token: Token, tokens: &mut impl Iterator<Item = (Result<Token, Error>, Span)>, double_space: bool) -> Result<Option<(OperInfo<Oper>, Option<(Result<Token, Error>, Span)>)>, Vec<KError<Token, Error>>> {
//!     use Token as T;
//!     use Oper as O;
//! 
//!     // precedence determines the order of operations, lower the precedence the 'smaller' it is
//!     // space determines how much many input nodes it takes, eg `Space::None` is `x`, `Space::Single` is `x input`, `Space::Double` is `input1 x input2`
//!     // oper is just the kind of operation it is, like a number, addition, etc
//!     let (precedence, space, oper) = match (token, double_space) {
//!         // no space
//!         (T::Number(x), _) => (0, Space::None, O::Num(x)),
//! 
//!         // single space
//!         (T::Plus, false) => (1, Space::Single, O::Pos),
//!         (T::Minus, false) => (1, Space::Single, O::Neg),
//! 
//!         // double space
//!         (T::Plus, true) => (3, Space::Double, O::Add),
//!         (T::Minus, true) => (3, Space::Double, O::Sub),
//!         (T::Star, _) => (2, Space::Double, O::Mul),
//!         (T::Slash, _) => (2, Space::Double, O::Div),
//!     };
//! 
//!     Ok(Some((OperInfo {
//!         oper,
//!         span: 0..0, // should be used with logos to get the actual span
//!         space,
//!         precedence,
//!     }, tokens.next())))
//! }
//! 
//! fn throw(error: KError<Token, Error>) {
//!     println!("err: {error:?}");
//! }
//! 
//! fn main() {
//!     let mut tokens = [(Ok(Token::Number(1)), 0..1), (Ok(Token::Plus), 1..2), (Ok(Token::Number(2)), 2..3), (Ok(Token::Star), 3..4), (Ok(Token::Number(3)), 4..5)].into_iter();
//!     let parser = Parser::<'_, Token, Oper, _, Vec<Node<Oper>>, _, Error>::new(&mut tokens, oper_generator);
//! 
//!     // handle errors
//!     let (asa, trailing_tok) = match parser.parse() {
//!         Ok(asa) => asa,
//!         Err(errs) => {
//!             for err in errs {
//!                 throw(err);
//!             } panic!("an error occured");
//!         },
//!     };
//! 
//!     // make sure that there aren't any tokens that haven't been consumed
//!     if let Some((_, span)) = trailing_tok {
//!         throw(KError::Other(span, Error::UnexpectedToken));
//!         panic!("an error occured");
//!     }
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

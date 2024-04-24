use std::fmt::Debug;
use crate::{error::Error as KError, node::{Node, NodeInfo}, Space, Span};

pub enum TokInfoOrCustom<Oper: Debug, Token, Tokens, Error> where
    Tokens: Iterator<Item = (Result<Token, Error>, Span)>,
{
    TokenInfo(TokenInfo<Oper>),
    #[allow(clippy::type_complexity)]
    Custom(Box<dyn FnOnce(&mut Tokens, Option<usize>) -> Result<(NodeInfo, Vec<Node<Oper>>, bool), Vec<KError<Token, Error>>>>),
}

#[derive(Debug)]
pub struct TokenInfo<Oper> {
    /// the type of operation the token is
    pub oper: Oper,
    /// the location of the token
    pub span: Span,
    /// the amount of inputs the token/node takes
    pub space: Space,
    /// the precedence of the token
    pub precedence: u8,
}

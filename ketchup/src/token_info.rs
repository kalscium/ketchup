use crate::{error::Error as KError, Space, Span};

pub enum TokInfoOrCustom<Oper, Tokens, Error, ASA> {
    TokenInfo(TokenInfo<Oper>),
    Custom(Box<dyn FnOnce(&mut Tokens, &mut ASA) -> Result<(), Vec<KError<Error>>>>),
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

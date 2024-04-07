use crate::Span;

#[derive(Debug)]
pub struct TokenInfo<Oper> {
    /// the type of operation the token is
    pub oper: Oper,
    /// the location of the token
    pub span: Span,
    /// the amount of inputs the token/node takes
    pub space: u8,
    /// the precedence of the token
    pub precedence: u8,
}

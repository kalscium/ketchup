use crate::Span;

#[derive(Debug, Clone)]
pub enum Error<Other> {
    ExpectedOper(Span),
    Other(Other),
}

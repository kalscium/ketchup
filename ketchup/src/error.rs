#[derive(Debug, Clone)]
pub enum Error<Other> {
    Other(Other),
}

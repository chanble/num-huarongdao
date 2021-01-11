
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    ZeroNotFound,
    CannotExchangeNoneZero,
    CannotExchangeNotNeighbouring,
}
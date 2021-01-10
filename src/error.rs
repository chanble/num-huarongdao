
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    CannotExchangeNoneZero,
    CannotExchangeNotNeighbouring,
}
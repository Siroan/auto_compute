#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    NoUnkown,
    MoreThanOneUnknown,
    DivisionByZero,
    SquareForbidden,
    UnknownInDenominator,
}

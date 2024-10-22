#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    NoUnkown,
    SeveralUnknown,
    DivisionByZero,
    SquareForbidden,
    UnknownInDenominator,
}

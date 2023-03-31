use crate::unknown::Unknown;

pub trait EquationAutoCompute {
    fn auto_compute(&self) -> bool;
}

#[derive(Clone, Debug, PartialEq)]
pub enum EquationElement {
    Known(f64),
    Unknown(Unknown),
}

impl EquationElement {
    pub fn new_unknown() -> EquationElement {
        EquationElement::Unknown(Unknown::new())
    }
}

use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{unknown::Unknown, element::Element};

pub trait EquationAutoCompute {
    fn auto_compute(&self) -> bool;
}

#[derive(Clone, Debug)]
pub enum EquationElement {
    Known(f64),
    Unknown(Unknown),
}

impl EquationElement {
    pub fn new_unknown() -> EquationElement {
        EquationElement::Unknown(Unknown::new())
    }
}

impl From<EquationElement> for Element {
    fn from(e: EquationElement) -> Self {
        match e {
            EquationElement::Known(b) => Element::new_known(b),
            EquationElement::Unknown(x) => Element::new_unknown(x.unknown),
        }
    }
}

impl Add for EquationElement {
    type Output = Element;

    fn add(self, rhs: Self) -> Self::Output {
        Element::from(self) + Element::from(rhs)
    }
}

impl Add<f64> for EquationElement {
    type Output = Element;

    fn add(self, rhs: f64) -> Self::Output {
        Element::from(self) + rhs
    }
}

impl Add<EquationElement> for f64 {
    type Output = Element;

    fn add(self, rhs: EquationElement) -> Self::Output {
        Element::from(rhs) + self
    }
}

impl Add<Element> for EquationElement {
    type Output = Element;

    fn add(self, rhs: Element) -> Self::Output {
        Element::from(self) + rhs
    }
}

impl Add<EquationElement> for Element {
    type Output = Element;

    fn add(self, rhs: EquationElement) -> Self::Output {
        Element::from(rhs) + self
    }
}

impl Neg for EquationElement {
    type Output = Element;

    fn neg(self) -> Self::Output {
        -Element::from(self)
    }
}

impl Sub for EquationElement {
    type Output = Element;

    fn sub(self, rhs: Self) -> Self::Output {
        Element::from(self) - Element::from(rhs)
    }
}

impl Sub<f64> for EquationElement {
    type Output = Element;

    fn sub(self, rhs: f64) -> Self::Output {
        Element::from(self) - rhs
    }
}

impl Sub<EquationElement> for f64 {
    type Output = Element;

    fn sub(self, rhs: EquationElement) -> Self::Output {
        -Element::from(rhs) + self
    }
}

impl Sub<Element> for EquationElement {
    type Output = Element;

    fn sub(self, rhs: Element) -> Self::Output {
        Element::from(self) - rhs
    }
}

impl Sub<EquationElement> for Element {
    type Output = Element;

    fn sub(self, rhs: EquationElement) -> Self::Output {
        -Element::from(rhs) + self
    }
}

impl Mul for EquationElement {
    type Output = Element;

    fn mul(self, rhs: Self) -> Self::Output {
        Element::from(self) * Element::from(rhs)
    }
}

impl Mul<f64> for EquationElement {
    type Output = Element;

    fn mul(self, rhs: f64) -> Self::Output {
        Element::from(self) * rhs
    }
}

impl Mul<EquationElement> for f64 {
    type Output = Element;

    fn mul(self, rhs: EquationElement) -> Self::Output {
        Element::from(rhs) * self
    }
}

impl Mul<Element> for EquationElement {
    type Output = Element;

    fn mul(self, rhs: Element) -> Self::Output {
        Element::from(self) * rhs
    }
}

impl Mul<EquationElement> for Element {
    type Output = Element;

    fn mul(self, rhs: EquationElement) -> Self::Output {
        Element::from(rhs) * self
    }
}

impl Div for EquationElement {
    type Output = Element;

    fn div(self, rhs: EquationElement) -> Self::Output {
        Element::from(self) / Element::from(rhs)
    }
}

impl Div<f64> for EquationElement {
    type Output = Element;

    fn div(self, rhs: f64) -> Self::Output {
        Element::from(self) / rhs
    }
}

impl Div<EquationElement> for f64 {
    type Output = Element;

    fn div(self, rhs: EquationElement) -> Self::Output {
        self / Element::from(rhs)
    }
}

impl Div<Element> for EquationElement {
    type Output = Element;

    fn div(self, rhs: Element) -> Self::Output {
        Element::from(self) / rhs
    }
}

impl Div<EquationElement> for Element {
    type Output = Element;

    fn div(self, rhs: EquationElement) -> Self::Output {
        self / Element::from(rhs)
    }
}

impl PartialEq for EquationElement {
    fn eq(&self, rhs: &EquationElement) -> bool {
        Element::from(self.clone()) == Element::from(rhs.clone())
    }
}

impl PartialEq<f64> for EquationElement {
    fn eq(&self, rhs: &f64) -> bool {
        &Element::from(self.clone()) == rhs
    }
}

impl PartialEq<EquationElement> for f64 {
    fn eq(&self, rhs: &EquationElement) -> bool {
        self == &Element::from(rhs.clone())
    }
}

impl PartialEq<Element> for EquationElement {
    fn eq(&self, rhs: &Element) -> bool {
        &Element::from(self.clone()) == rhs
    }
}

impl PartialEq<EquationElement> for Element {
    fn eq(&self, rhs: &EquationElement) -> bool {
        self == &Element::from(rhs.clone())
    }
}

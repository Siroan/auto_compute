use std::cell::RefCell;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::rc::Rc;

use crate::error::Error;

// Represents an element like 'a * x'
#[derive(Clone, Debug, PartialEq)]
pub struct Unknown {
    factor: Option<f64>,
    unknown: Option<Rc<RefCell<f64>>>,
}

impl Unknown {
    pub(crate) fn new(content: Option<(f64, Rc<RefCell<f64>>)>) -> Self {
        content.map_or_else(
            || Unknown {
                factor: None,
                unknown: None,
            },
            |content| Unknown {
                factor: Some(content.0),
                unknown: Some(content.1),
            },
        )
    }

    pub(crate) fn is_some(&self) -> bool {
        self.factor.is_some() && self.unknown.is_some()
    }

    pub(crate) fn solve(&self, other: Unknown, b: f64) {
        let mut temp = self.clone() - other;

        if let Some(x) = &mut temp.unknown {
            let a = temp
                .factor
                .map(|a| if a == 0. { 1. } else { a })
                .unwrap_or(1.);
            *x.borrow_mut() = -b / a;
        }
    }
}

impl Add for Unknown {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            factor: self.factor.map_or(other.factor, |factor1| {
                if let Some(factor2) = other.factor {
                    Some(factor1 + factor2)
                } else {
                    Some(factor1)
                }
            }),
            unknown: self.unknown.or(other.unknown),
        }
    }
}

impl Neg for Unknown {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            factor: self.factor.map(|factor| -factor),
            unknown: self.unknown,
        }
    }
}

impl Sub for Unknown {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            factor: self.factor.map_or_else(
                || other.factor.map(|factor| -factor),
                |factor1| {
                    if let Some(factor2) = other.factor {
                        Some(factor1 - factor2)
                    } else {
                        Some(factor1)
                    }
                },
            ),
            unknown: self.unknown.or(other.unknown),
        }
    }
}

impl Mul<f64> for Unknown {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            factor: self.factor.map(|factor| factor * rhs),
            unknown: self.unknown,
        }
    }
}

impl Mul<Unknown> for f64 {
    type Output = Unknown;

    fn mul(self, rhs: Unknown) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Unknown {
    type Output = (Self, Result<(), Error>);

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0. {
            (self, Err(Error::DivisionByZero))
        } else {
            (
                Self {
                    factor: self.factor.map(|factor| factor / rhs),
                    unknown: self.unknown,
                },
                Ok(()),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Setup {
        rc: Rc<RefCell<f64>>,
    }

    impl Setup {
        fn new() -> Self {
            Self {
                rc: Rc::new(RefCell::new(0.)),
            }
        }
    }

    #[test]
    fn test_add_unknowns() {
        let setup = Setup::new();

        // (x) + (2x) = 3x
        let unknown1 = Unknown::new(Some((1., setup.rc.clone())));
        let unknown2 = Unknown::new(Some((2., setup.rc.clone())));
        let sum = Unknown::new(Some((3., setup.rc.clone())));
        assert_eq!(unknown1 + unknown2, sum);

        // () + (2x) = 2x
        let unknown1 = Unknown::new(None);
        let unknown2 = Unknown::new(Some((2., setup.rc.clone())));
        let sum = Unknown::new(Some((2., setup.rc.clone())));
        assert_eq!(unknown1 + unknown2, sum);

        // (x) + () = x
        let unknown1 = Unknown::new(Some((1., setup.rc.clone())));
        let unknown2 = Unknown::new(None);
        let sum = Unknown::new(Some((1., setup.rc.clone())));
        assert_eq!(unknown1 + unknown2, sum);

        // () + () = ()
        let unknown1 = Unknown::new(None);
        let unknown2 = Unknown::new(None);
        let sum = Unknown::new(None);
        assert_eq!(unknown1 + unknown2, sum);
    }

    #[test]
    fn test_neg_unknowns() {
        let setup = Setup::new();

        // -(x) = -x
        let unknown = Unknown::new(Some((1., setup.rc.clone())));
        let neg = Unknown::new(Some((-1., setup.rc.clone())));
        assert_eq!(-unknown, neg);

        // -() = ()
        let unknown = Unknown::new(None);
        let neg = Unknown::new(None);
        assert_eq!(-unknown, neg);
    }

    #[test]
    fn test_sub_unknowns() {
        let setup = Setup::new();

        // (x) - (2x) = -x
        let unknown1 = Unknown::new(Some((1., setup.rc.clone())));
        let unknown2 = Unknown::new(Some((2., setup.rc.clone())));
        let sub = Unknown::new(Some((-1., setup.rc.clone())));
        assert_eq!(unknown1 - unknown2, sub);

        // () - (2x) = -2x
        let unknown1 = Unknown::new(None);
        let unknown2 = Unknown::new(Some((2., setup.rc.clone())));
        let sub = Unknown::new(Some((-2., setup.rc.clone())));
        assert_eq!(unknown1 - unknown2, sub);

        // (x) - () = x
        let unknown1 = Unknown::new(Some((1., setup.rc.clone())));
        let unknown2 = Unknown::new(None);
        let sub = Unknown::new(Some((1., setup.rc.clone())));
        assert_eq!(unknown1 - unknown2, sub);

        // () - () = ()
        let unknown1 = Unknown::new(None);
        let unknown2 = Unknown::new(None);
        let sub = Unknown::new(None);
        assert_eq!(unknown1 - unknown2, sub);
    }

    #[test]
    fn test_mul_unknowns() {
        let setup = Setup::new();

        // (3) * (2x) = 6x
        let unknown = Unknown::new(Some((2., setup.rc.clone())));
        let factor = 3.;
        let mul = Unknown::new(Some((6., setup.rc.clone())));
        assert_eq!(factor * unknown, mul);

        // (2x) * (3) = 6x
        let unknown = Unknown::new(Some((2., setup.rc.clone())));
        assert_eq!(unknown * factor, mul);

        // (3) * () = ()
        let unknown = Unknown::new(None);
        let factor = 3.;
        let mul = Unknown::new(None);
        assert_eq!(factor * unknown, mul);

        // () * (3) = ()
        let unknown = Unknown::new(None);
        assert_eq!(unknown * factor, mul);
    }

    #[test]
    fn test_div_unknowns() {
        let setup = Setup::new();

        // (4x) / (2) = 2
        let unknown = Unknown::new(Some((4., setup.rc.clone())));
        let factor = 2.;
        let div = Unknown::new(Some((2., setup.rc.clone())));
        assert_eq!(unknown / factor, (div, Ok(())));

        // () / (3) = ()
        let unknown = Unknown::new(None);
        let factor = 3.;
        let div = Unknown::new(None);
        assert_eq!(unknown / factor, (div, Ok(())));

        // (4x) / (0) => error
        let unknown = Unknown::new(Some((4., setup.rc.clone())));
        let factor = 0.;
        let div = Unknown::new(Some((4., setup.rc.clone())));
        assert_eq!(unknown / factor, (div, Err(Error::DivisionByZero)));

        // () / (0) => error
        let unknown = Unknown::new(None);
        let factor = 0.;
        let div = Unknown::new(None);
        assert_eq!(unknown / factor, (div, Err(Error::DivisionByZero)));
    }
}

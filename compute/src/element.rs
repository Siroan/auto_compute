use std::cell::RefCell;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::rc::Rc;

use crate::error::Error;
use crate::unknown::Unknown;

type Ax = (f64, Unknown);

// Represents an element like 'a * x + b'
#[derive(Clone, Debug)]
pub struct Element {
    ax: Option<(f64, Unknown)>,
    b: f64,
    error: Result<(), Error>,
}

impl Element {
    pub fn new_unknown(x: Rc<RefCell<f64>>) -> Self {
        Self {
            ax: Some((1., Unknown::new_with_value(x))),
            b: 0.,
            error: Ok(()),
        }
    }

    pub fn new_known(known: f64) -> Self {
        Self {
            ax: None,
            b: known,
            error: Ok(()),
        }
    }
}

fn combine<F>(first: Option<Ax>, second: Option<Ax>, combinator: F) -> Option<Ax>
    where F: Fn(f64, f64) -> Result<f64, Error> {
        match (first, second) {
            (Some(first), Some(second)) => {
                let mut x = Unknown {
                    status: first.1.status.and_then(|_| second.1.status),
                    unknown: first.1.unknown,
                };

                if x.status.is_ok() {
                    let combined = combinator(first.0, second.0);
                    x.status = x.status.and(combined.clone().map(|_| ()));

                    Some((combined.unwrap_or(first.0), x))
                } else {
                    Some((first.0, x))
                }
            },
            (Some(first), None) => Some(first),
            (None, Some(second)) => Some(second),
            (None, None) => None,
        }
}

impl Add for Element {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let error = self.error.and(rhs.error);

        let mut ax = combine(self.ax, rhs.ax, |ax1, ax2| Ok(ax1 + ax2));
        ax.as_mut().map(|ax| ax.1.status.clone().and(error.clone()));

        Self {
            ax,
            b: self.b + rhs.b,
            error,
        }
    }
}

impl Add<f64> for Element {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Self {
            ax: self.ax,
            b: self.b + rhs,
            error: self.error,
        }
    }
}

impl Add<Element> for f64 {
    type Output = Element;

    fn add(self, rhs: Element) -> Self::Output {
        rhs + self
    }
}

impl Neg for Element {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            ax: self.ax.map(|ax| (-ax.0, ax.1)),
            b: -self.b,
            error: self.error,
        }
    }
}

impl Sub for Element {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let error = self.error.and(rhs.error);

        let mut ax = combine(self.ax.clone(), rhs.ax, |ax1, ax2| Ok(ax1 - ax2));
        ax.as_mut().map(|ax| ax.1.status.clone().and(error.clone()));
        if self.ax.is_none() {
            if let Some(ax) = ax.as_mut() {
                ax.0 = -ax.0;
            }
        }

        Self {
            ax,
            b: self.b - rhs.b,
            error,
        }
    }
}

impl Sub<f64> for Element {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self {
        Self {
            ax: self.ax,
            b: self.b - rhs,
            error: self.error,
        }
    }
}

impl Sub<Element> for f64 {
    type Output = Element;

    fn sub(self, rhs: Element) -> Self::Output {
        -rhs + self
    }
}

impl Mul for Element {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let error = self.error.and(rhs.error);

        let mut ax = combine(self.ax.clone(), rhs.ax, |_, _| Err(Error::SquareForbidden));
        if let Some(ax) = ax.as_mut() {
            ax.0 *= if self.ax.is_some() { rhs.b } else { self.b };
            ax.1.status = ax.1.status.clone().and(error.clone());
        }

        Self {
            ax,
            b: self.b * rhs.b,
            error,
        }
    }
}

impl Mul<f64> for Element {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            ax: self.ax.map(|ax| (ax.0 * rhs, ax.1)),
            b: self.b * rhs,
            error: self.error,
        }
    }
}

impl Mul<Element> for f64 {
    type Output = Element;

    fn mul(self, rhs: Element) -> Self::Output {
        rhs * self
    }
}

fn combine_div(first: Option<Ax>, second: Option<Ax>) -> Option<Ax> {
    match (first, second) {
        (Some(first), Some(second)) => {
            let x = Unknown {
                status: first.1.status.and(second.1.status).and(Err(Error::UnknownInDenominator)),
                unknown: first.1.unknown,
            };

            Some((second.0, x))
        },
        (Some(first), None) => Some(first),
        (None, Some(second)) => {
            let x = Unknown {
                status: second.1.status.and(Err(Error::UnknownInDenominator)),
                unknown: second.1.unknown,
            };

            Some((second.0, x))
        },
        (None, None) => None,
    }
}

impl Div for Element {
    type Output = Self;

    fn div(self, rhs: Element) -> Self {
        let error = self.error
            .and(rhs.error)
            .and_then(|_| {
                if rhs.ax.is_some() {
                    Err(Error::UnknownInDenominator)
                } else if rhs.b == 0. {
                    Err(Error::DivisionByZero)
                } else {
                    Ok(())
                }
            });
        let mut ax = combine_div(self.ax, rhs.ax);
        if let Some(ax) = ax.as_mut() {
            ax.0 /= rhs.b;
            ax.1.status = ax.1.status.clone().and(error.clone());
        }

        Self {
            ax,
            b: if error.is_ok() {
                self.b / rhs.b
            } else {
                self.b
            },
            error,
        }
    }
}

impl Div<f64> for Element {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            ax: self.ax.map(|ax| (ax.0 / rhs, ax.1)),
            b: self.b / rhs,
            error: self.error,
        }
    }
}

impl Div<Element> for f64 {
    type Output = Element;

    fn div(self, rhs: Element) -> Self::Output {
        Element::new_known(self) / rhs
    }
}

impl PartialEq for Element {
    fn eq(&self, rhs: &Self) -> bool {
        let lhs = self.clone() - rhs.clone();
        if let Some(ax) = lhs.ax {
            if ax.0 != 0. && ax.1.status.is_ok() {
                *ax.1.unknown.borrow_mut() = -lhs.b / ax.0;
            }
        }

        let ax_eq = match (self.ax.clone(), rhs.ax.clone()) {
            (Some(ax1), Some(ax2)) => ax1.0 == ax2.0 && ax1.1.status == ax1.1.status,
            (None, None) => true,
            _ => false,
        };
        ax_eq && self.b == rhs.b
    }
}

impl PartialEq<f64> for Element {
    fn eq(&self, rhs: &f64) -> bool {
        self == &Element::new_known(*rhs)
    }
}

impl PartialEq<Element> for f64 {
    fn eq(&self, rhs: &Element) -> bool {
        &Element::new_known(*self) == rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Element {
        fn new(ax: Option<(f64, Rc<RefCell<f64>>)>, b: f64) -> Self {
            Self {
                ax: ax.map(|ax| (ax.0, Unknown::new_with_value(ax.1))),
                b,
                error: Ok(()),
            }
        }
    }

    struct Setup {
        rc: Rc<RefCell<f64>>,
    }

    impl Setup {
        fn new() -> Self {
            Self {
                rc: Rc::new(RefCell::new(0.)),
            }
        }

        fn check_rc(&mut self, value: f64) {
            assert_eq!(*self.rc.borrow(), value);
        }
    }

    #[test]
    fn test_add_elements() {
        let setup = Setup::new();

        // (x + 3) + (2x + 4) = 3x + 7
        let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let element2 = Element::new(Some((2., setup.rc.clone())), 4.);
        let sum = Element::new(Some((3., setup.rc.clone())), 7.);
        assert_eq!(element1 + element2, sum);

        // (3) + (2x + 4) = 2x + 7
        let element1 = Element::new(None, 3.);
        let element2 = Element::new(Some((2., setup.rc.clone())), 4.);
        let sum = Element::new(Some((2., setup.rc.clone())), 7.);
        assert_eq!(element1 + element2, sum);

        // (x + 3) + (4) = x + 7
        let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let element2 = Element::new(None, 4.);
        let sum = Element::new(Some((1., setup.rc.clone())), 7.);
        assert_eq!(element1 + element2, sum);

        // (3) + (4) = 7
        let element1 = Element::new(None, 3.);
        let element2 = Element::new(None, 4.);
        let sum = Element::new(None, 7.);
        assert_eq!(element1 + element2, sum);

        // (x + 3) + 4 = 3x + 7
        let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let sum = Element::new(Some((1., setup.rc.clone())), 7.);
        assert_eq!(element1 + 4., sum);

        // 4 + (x + 3) = 3x + 7
        let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let sum = Element::new(Some((1., setup.rc.clone())), 7.);
        assert_eq!(4. + element1, sum);
    }

    #[test]
    fn test_neg_elements() {
        let setup = Setup::new();

        // -(x + 3) = -x - 3
        let element = Element::new(Some((1., setup.rc.clone())), 3.);
        let neg = Element::new(Some((-1., setup.rc.clone())), -3.);
        assert_eq!(-element, neg);

        // -(3) = -3
        let element = Element::new(None, 3.);
        let neg = Element::new(None, -3.);
        assert_eq!(-element, neg);
    }

    #[test]
    fn test_sub_elements() {
        let setup = Setup::new();

        // (x + 3) - (2x + 4) = -x -1
        let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let element2 = Element::new(Some((2., setup.rc.clone())), 4.);
        let sub = Element::new(Some((-1., setup.rc.clone())), -1.);
        assert_eq!(element1 - element2, sub);

        // (3) - (2x + 4) = -2x - 1
        let element1 = Element::new(None, 3.);
        let element2 = Element::new(Some((2., setup.rc.clone())), 4.);
        let sub = Element::new(Some((-2., setup.rc.clone())), -1.);
        assert_eq!(element1 - element2, sub);

        // (x + 3) - (4) = x - 1
        let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let element2 = Element::new(None, 4.);
        let sub = Element::new(Some((1., setup.rc.clone())), -1.);
        assert_eq!(element1 - element2, sub);

        // (3) - (4) = -1
        let element1 = Element::new(None, 3.);
        let element2 = Element::new(None, 4.);
        let sub = Element::new(None, -1.);
        assert_eq!(element1 - element2, sub);

        // (x + 3) - 4 = x -1
        let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let sub = Element::new(Some((1., setup.rc.clone())), -1.);
        assert_eq!(element1 - 4., sub);

        // 4 - (x + 3) = x -1
        let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let sub = Element::new(Some((-1., setup.rc.clone())), 1.);
        assert_eq!(4. - element1, sub);
    }

    #[test]
    fn test_mul_elements() {
        let setup = Setup::new();

        // (x + 3) * (2x + 4) => error
        let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let element2 = Element::new(Some((2., setup.rc.clone())), 4.);
        assert_eq!((element1 * element2).ax.unwrap().1.status, Err(Error::SquareForbidden));

        // (3) * (2x + 4) = 6x + 12
        let element1 = Element::new(None, 3.);
        let element2 = Element::new(Some((2., setup.rc.clone())), 4.);
        let mul = Element::new(Some((6., setup.rc.clone())), 12.);
        assert_eq!(element1 * element2, mul);

        // (2x + 3) * (4) = 8x + 12
        let element1 = Element::new(Some((2., setup.rc.clone())), 3.);
        let element2 = Element::new(None, 4.);
        let mul = Element::new(Some((8., setup.rc.clone())), 12.);
        assert_eq!(element1 * element2, mul);

        // (3) * (4) = 12
        let element1 = Element::new(None, 3.);
        let element2 = Element::new(None, 4.);
        let mul = Element::new(None, 12.);
        assert_eq!(element1 * element2, mul);
    }

    #[test]
    fn test_div_elements() {
        let setup = Setup::new();

        // (x + 3) / (2x + 4) => error
        let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let element2 = Element::new(Some((2., setup.rc.clone())), 4.);
        assert_eq!(
            (element1 / element2).ax.unwrap().1.status,
            Err(Error::UnknownInDenominator)
        );

        // (x + 3) / (4) = x/4 + 3/4
        let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let element2 = Element::new(None, 4.);
        let div = Element::new(Some((0.25, setup.rc.clone())), 0.75);
        assert_eq!(element1 / element2, div);

        // (x + 3) / (0) => error
        let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let element2 = Element::new(None, 0.);
        assert_eq!((element1 / element2).ax.unwrap().1.status, Err(Error::DivisionByZero));

        // (3) / (4) = x/4 + 3/4
        let element1 = Element::new(None, 3.);
        let element2 = Element::new(None, 4.);
        let div = Element::new(None, 0.75);
        assert_eq!(element1 / element2, div);
    }

    #[test]
    fn test_solve_elements() {
        let mut setup = Setup::new();

        // x + 1 = 0
        let element1 = Element::new(Some((1., setup.rc.clone())), 1.);
        let element2 = Element::new(None, 0.);
        let _ = element1 == element2;
        setup.check_rc(-1.);

        // 2x + 1 = 0
        let element1 = Element::new(Some((2., setup.rc.clone())), 1.);
        let element2 = Element::new(None, 0.);
        let _ = element1 == element2;
        setup.check_rc(-0.5);

        // 2x + 1 = x
        let element1 = Element::new(Some((2., setup.rc.clone())), 1.);
        let element2 = Element::new(Some((1., setup.rc.clone())), 0.);
        let _ = element1 == element2;
        setup.check_rc(-1.);

        // 4x + 1 = 2x + 3
        let element1 = Element::new(Some((4., setup.rc.clone())), 1.);
        let element2 = Element::new(Some((2., setup.rc.clone())), 3.);
        let _ = element1 == element2;
        setup.check_rc(1.);
    }
}

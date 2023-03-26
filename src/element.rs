use std::ops::{Add, Mul, Neg, Sub};
use std::cell::RefCell;
use std::rc::Rc;

use crate::unknown::Unknown;

// Represents an element like 'a * x + b'
#[derive(Debug)]
pub(crate) struct Element {
    ax: Unknown,
    b: f64,
}

impl Element {
    pub(crate) fn new_unknown(rc: Rc<RefCell<f64>>) -> Self {
        Self {
            ax: Unknown::new(Some((1., rc))),
            b: 0.,
        }
    }

    pub(crate) fn new_known(known: f64) -> Self {
        Self {
            ax: Unknown::new(None),
            b: known,
        }
    }
}

impl Add for Element {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            ax: self.ax + other.ax,
            b: self.b + other.b,
        }
    }
}

impl Neg for Element {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            ax: -self.ax,
            b: -self.b,
        }
    }
}

impl Sub for Element {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            ax: self.ax - other.ax,
            b: self.b - other.b,
        }
    }
}

impl Mul for Element {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        if self.ax.is_some() && rhs.ax.is_some() {
            panic!("No square!");
        }
        Self {
            ax: if self.ax.is_some() {
                self.ax * rhs.b
            } else if rhs.ax.is_some() {
                self.b * rhs.ax
            } else {
                Unknown::new(None)
            },
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f64> for Element {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            ax: self.ax * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Element> for f64 {
    type Output = Element;

    fn mul(self, rhs: Element) -> Self::Output {
        rhs * self
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        println!("Solve called");
        self.ax.clone().solve(other.ax.clone(), self.b - other.b);

        self.ax == other.ax && self.b == other.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Element {
        fn new(ax: Option<(f64, Rc<RefCell<f64>>)>, b: f64) -> Self {
            Self {
                ax: Unknown::new(ax),
                b: b,
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

        let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let element2 = Element::new(Some((2., setup.rc.clone())), 4.);
        let sum = Element::new(Some((3., setup.rc.clone())), 7.);
        assert_eq!(element1 + element2, sum);

        let element1 = Element::new(None, 3.);
        let element2 = Element::new(Some((2., setup.rc.clone())), 4.);
        let sum = Element::new(Some((2., setup.rc.clone())), 7.);
        assert_eq!(element1 + element2, sum);

        let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let element2 = Element::new(None, 4.);
        let sum = Element::new(Some((1., setup.rc.clone())), 7.);
        assert_eq!(element1 + element2, sum);

        let element1 = Element::new(None, 3.);
        let element2 = Element::new(None, 4.);
        let sum = Element::new(None, 7.);
        assert_eq!(element1 + element2, sum);
    }

    #[test]
    fn test_neg_elements() {
        let setup = Setup::new();

        let element = Element::new(Some((1., setup.rc.clone())), 3.);
        let neg = Element::new(Some((-1., setup.rc.clone())), -3.);
        assert_eq!(-element, neg);

        let element = Element::new(None, 3.);
        let neg = Element::new(None, -3.);
        assert_eq!(-element, neg);
    }

    #[test]
    fn test_sub_elements() {
        let setup = Setup::new();

        let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let element2 = Element::new(Some((2., setup.rc.clone())), 4.);
        let sub = Element::new(Some((-1., setup.rc.clone())), -1.);
        assert_eq!(element1 - element2, sub);

        let element1 = Element::new(None, 3.);
        let element2 = Element::new(Some((2., setup.rc.clone())), 4.);
        let sub = Element::new(Some((-2., setup.rc.clone())), -1.);
        assert_eq!(element1 - element2, sub);

        let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let element2 = Element::new(None, 4.);
        let sub = Element::new(Some((1., setup.rc.clone())), -1.);
        assert_eq!(element1 - element2, sub);

        let element1 = Element::new(None, 3.);
        let element2 = Element::new(None, 4.);
        let sub = Element::new(None, -1.);
        assert_eq!(element1 - element2, sub);
    }

    #[test]
    fn test_mul_elements() {
        let setup = Setup::new();

        /*let element1 = Element::new(Some((1., setup.rc.clone())), 3.);
        let element2 = Element::new(Some((2., setup.rc.clone())), 4.);
        let mul = Element::new(Some((2., setup.rc.clone())), 4.);
        assert_eq!(element1 * element2, mul);*/

        let element1 = Element::new(None, 3.);
        let element2 = Element::new(Some((2., setup.rc.clone())), 4.);
        let mul = Element::new(Some((6., setup.rc.clone())), 12.);
        assert_eq!(element1 * element2, mul);

        let element1 = Element::new(Some((2., setup.rc.clone())), 3.);
        let element2 = Element::new(None, 4.);
        let mul = Element::new(Some((8., setup.rc.clone())), 12.);
        assert_eq!(element1 * element2, mul);

        let element1 = Element::new(None, 3.);
        let element2 = Element::new(None, 4.);
        let mul = Element::new(None, 12.);
        assert_eq!(element1 * element2, mul);
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
extern crate compute;
use compute::element::Element;

#[macro_use]
extern crate compute_macro;

#[cfg(test)]
mod tests {
    use compute::equation::{EquationAutoCompute, EquationElement};

    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

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
    fn test_add_elements_whatever() {
        let setup = Setup::new();

        // (x + 3) + (2x + 4) = 3x + 7
        let element1 = Element::new_unknown(setup.rc.clone()) + Element::new_known(3.);
        let element2 = 2. * Element::new_unknown(setup.rc.clone()) + Element::new_known(4.);
        let sum = 3. * Element::new_unknown(setup.rc.clone()) + Element::new_known(7.);
        assert_eq!(element1 + element2, sum);
    }

    #[derive(Equation)]
    struct MyEquation {
        element1: f64,
        #[unknown]
        element2: EquationElement,
        #[unknown]
        element3: EquationElement,
    }

    impl EquationAutoCompute for MyEquation {
        fn auto_compute(&self) -> bool {
            self.element1 == self.element2 + self.element3
        }
    }

    #[test]
    fn equation_no_unknown_test() {
        let my_equation = MyEquation {
            element1: 0.,
            element2: EquationElement::Known(0.),
            element3: EquationElement::Known(0.),
        };
        assert_eq!(my_equation.compute(), Err(Error::NoUnkown));
    }

    #[test]
    fn equation_two_unknown_test() {
        let my_equation = MyEquation {
            element1: 0.,
            element2: EquationElement::new_unknown(),
            element3: EquationElement::new_unknown(),
        };
        assert_eq!(my_equation.compute(), Err(Error::MoreThanOneUnknown));
    }

    #[test]
    fn equation_test() {
        let my_equation = MyEquation {
            element1: 100.,
            element2: EquationElement::new_unknown(),
            element3: EquationElement::Known(30.),
        };
        assert_eq!(my_equation.compute(), Ok(70.));
    }
}

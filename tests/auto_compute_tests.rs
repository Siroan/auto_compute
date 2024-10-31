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


        let element2 = 2. * Element::new_unknown(setup.rc.clone()) + Element::new_known(4.);
        let sum = 3. * Element::new_unknown(setup.rc.clone()) + Element::new_known(7.);
        let _ = element2 == sum;
        assert_eq!(*setup.rc.borrow(), -3.);
    }

    #[derive(Equation)]
    struct MyEquation {
        element1: f64,
        #[variable]
        element2: EquationElement,
        #[variable]
        element3: EquationElement,
    }

    impl EquationAutoCompute for MyEquation {
        fn auto_compute(&self) -> bool {
            self.element1 == self.element2.clone() + self.element3.clone()
        }
    }

    #[test]
    fn equation_no_unknown_test() {
        let my_equation = MyEquation {
            element1: 0.,
            element2: EquationElement::known(0.),
            element3: EquationElement::known(0.),
        };
        assert_eq!(my_equation.compute(), Err(Error::NoUnkown));
    }

    #[test]
    fn equation_two_unknown_test() {
        let my_equation = MyEquation {
            element1: 0.,
            element2: EquationElement::unknown(),
            element3: EquationElement::unknown(),
        };
        assert_eq!(my_equation.compute(), Err(Error::SeveralUnknown));
    }

    #[test]
    fn equation_test() {
        let my_equation = MyEquation {
            element1: 100.,
            element2: EquationElement::unknown(),
            element3: EquationElement::known(30.),
        };
        assert_eq!(my_equation.compute(), Ok(70.));
    }
}

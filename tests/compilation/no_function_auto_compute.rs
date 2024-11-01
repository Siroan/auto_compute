extern crate compute;

#[macro_use]
extern crate compute_macro;

use compute::equation::EquationElement;

fn main() {
    #[derive(Equation)]
    struct Struct {
        element1: f64,
        #[variable]
        element2: EquationElement,
    }
}
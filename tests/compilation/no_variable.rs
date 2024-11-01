extern crate compute;

#[macro_use]
extern crate compute_macro;

use compute::equation::{EquationAutoCompute, EquationElement};

fn main() {
    #[derive(Equation)]
    struct NoVariable {
        pub member: EquationElement,
    }

    impl EquationAutoCompute for NoVariable {
        fn auto_compute(&self) -> bool {
            true
        }
    }
}
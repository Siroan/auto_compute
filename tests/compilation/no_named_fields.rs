extern crate compute;

#[macro_use]
extern crate compute_macro;

use compute::equation::EquationAutoCompute;

fn main() {
    #[derive(Equation)]
    struct NoNamedFields(i32, i32);

    impl EquationAutoCompute for NoNamedFields {
        fn auto_compute(&self) -> bool {
            true
        }
    }
}
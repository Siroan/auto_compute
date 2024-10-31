extern crate compute;

#[macro_use]
extern crate compute_macro;

use compute::equation::EquationAutoCompute;

fn main() {
    #[derive(Equation)]
    enum NotAStruct {}

    impl EquationAutoCompute for NotAStruct {
        fn auto_compute(&self) -> bool {
            true
        }
    }
}
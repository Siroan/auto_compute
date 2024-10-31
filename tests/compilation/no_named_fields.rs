extern crate compute;

#[macro_use]
extern crate compute_macro;

fn main() {
    #[derive(Equation)]
    struct TupleStruct(i32, i32);
}
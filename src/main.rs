use std::cell::RefCell;
use std::rc::Rc;

mod unknown;
mod element;
mod equation;

fn main() {
    /*let x = Rc::new(RefCell::new(0.));

    let nominal = element::Element::new_known(1000.);
    let price = element::Element::new_known(10.);
    let quantity = element::Element::new_unknown(x.clone());

    let _ = nominal == price * quantity;

    println!("Quantity: {:?}", x);*/

    let result = solve_equation! {{
        nominal: equation::PodElement::Known(1000.),
        //price: equation::PodElement::Known(10.),
        //quantity: equation::PodElement::Unknown,
        //|nominal, price, quantity| nominal == price * quantity,
    }};
    println!("Result: {:?}", result);
}

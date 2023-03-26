/*use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub enum PodElement {
    Known(f64),
    Unknown,
    //UnknownWithF-1
}

pub type Pod = HashMap<String, PodElement>;

struct Equation {
    x: Rc<RefCell<f64>>,
    pod: Pod,
    compute: Box<dyn Fn() -> bool>,
}

impl Equation {
    fn new(pod: Pod, compute: Box<dyn Fn() -> bool>) -> Self {
        Self {
            x: Rc::new(RefCell::new(0.)),
            pod,
            compute,
        }
    }

    fn compute(&self) -> f64 {
        self.compute();
        *self.x.borrow()
    }
}*/

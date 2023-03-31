use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Unknown {
    pub(crate) status: Result<(), Error>,
    pub(crate) unknown: Rc<RefCell<f64>>,
}

impl Unknown {
    pub(crate) fn new() -> Self {
        Unknown {
            status: Ok(()),
            unknown: Rc::new(RefCell::new(0.)),
        }
    }

    pub(crate) fn new_with_value(unknown: Rc<RefCell<f64>>) -> Self {
        Unknown {
            status: Ok(()),
            unknown,
        }
    }
}

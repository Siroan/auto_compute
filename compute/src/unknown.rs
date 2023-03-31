use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Unknown {
    pub status: Result<(), Error>,
    pub unknown: Rc<RefCell<f64>>,
}

impl Unknown {
    pub(crate) fn new(unknown: Rc<RefCell<f64>>) -> Self {
        Unknown {
            status: Ok(()),
            unknown,
        }
    }
}

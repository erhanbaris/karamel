use std::{cell::RefCell, rc::Rc};

use super::OpcodeLocation;



#[derive(Clone)]
pub struct OpcodeLocationGroup { pub locations: RefCell<Vec<Rc<OpcodeLocation>>> }
impl OpcodeLocationGroup {
    pub fn new() -> Self {
        OpcodeLocationGroup {
            locations: RefCell::new(Vec::new())
        }
    }

    pub fn add(&self, location: Rc<OpcodeLocation>) {
        self.locations.borrow_mut().push(location.clone());
    }

    pub fn clear(&self) {
        #[cfg(debug_assertions)]
        {
            for location in self.locations.borrow().iter() {
                assert_ne!(location.get(), 0);
            }
        }
        self.locations.borrow_mut().clear();
    }
}
use std::{cell::{Cell, RefCell}, rc::Rc};

use super::OpcodeGeneratorTrait;


#[derive(Clone)]
pub struct OpcodeLocation {
    location: Cell<usize>,
    used_location: RefCell<Vec<usize>>
}

impl OpcodeLocation {
    pub fn new(location: usize) -> Self {
        OpcodeLocation {
            location: Cell::new(location),
            used_location: RefCell::new(Vec::new())
        }
    }

    pub fn empty() -> Self {
        OpcodeLocation {
            location: Cell::new(0),
            used_location: RefCell::new(Vec::new())
        }
    }

    pub fn get(&self) -> usize {
        self.location.get()
    }

    pub fn set(&self, location: usize, opcodes: &mut Vec<u8>) {
        self.location.set(location);

        for used_location in self.used_location.borrow().iter() {
            opcodes[*used_location] = location as u8;
            opcodes[*used_location + 1] = (location >> 8) as u8;
        }
    }

    pub fn apply(&self, opcodes: &mut Vec<u8>) {
        // Save position
        self.used_location.borrow_mut().push(opcodes.len());
        opcodes.push(self.get() as u8);
        opcodes.push((self.get() >> 8) as u8);
    }
}

#[derive(Clone)]
pub struct CurrentLocationUpdateGenerator { pub location:  Rc<OpcodeLocation> }
impl OpcodeGeneratorTrait for CurrentLocationUpdateGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        self.location.set(opcodes.len(), opcodes);
    }
}
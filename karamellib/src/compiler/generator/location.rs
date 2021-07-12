use std::{cell::{Cell, RefCell}, rc::Rc};
use std::ops::Sub;

use super::OpcodeGeneratorTrait;

#[derive(Clone)]
pub enum LocationType {
    Fixed(usize),
    Dynamic(Rc<OpcodeLocation>, Rc<OpcodeLocation>)
}

#[derive(Clone)]
pub struct OpcodeLocation {
    location: RefCell<LocationType>,
    used_location: RefCell<Vec<usize>>
}

impl Sub for OpcodeLocation {
    type Output = (OpcodeLocation, OpcodeLocation);
    fn sub(self, rhs: OpcodeLocation) -> Self::Output {
        (self, rhs)
    }
}

impl OpcodeLocation {
    pub fn new(location: usize) -> Self {
        OpcodeLocation {
            location: RefCell::new(LocationType::Fixed(location)),
            used_location: RefCell::new(Vec::new())
        }
    }

    pub fn empty() -> Self {
        OpcodeLocation {
            location: RefCell::new(LocationType::Fixed(0)),
            used_location: RefCell::new(Vec::new())
        }
    }

    pub fn get(&self) -> usize {
        *self.location.borrow().deref()
    }

    pub fn set(&self, location: usize, opcodes: &mut Vec<u8>) {
        self.location.set(LocationType::Fixed(location));

        for used_location in self.used_location.borrow().iter() {
            opcodes[*used_location] = location as u8;
            opcodes[*used_location + 1] = (location >> 8) as u8;
        }
    }

    pub fn dynamic_set(&self, sub_set: (OpcodeLocation, OpcodeLocation)) {

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
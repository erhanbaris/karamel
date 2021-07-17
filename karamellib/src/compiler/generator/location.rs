use std::{cell::RefCell, rc::Rc};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{DumpBuilder, OpcodeGeneratorTrait};

#[cfg(debug_assertions)]
static OPCODE_LOCATION_INDEXER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub enum LocationType {
    Fixed(usize),
    Subtraction {
        left_hand: Rc<OpcodeLocation>, 
        right_hand: Rc<OpcodeLocation>
    }
}

#[derive(Clone)]
pub struct OpcodeLocation {
    #[cfg(debug_assertions)]
    index: usize,
    location: RefCell<LocationType>,
    used_location: RefCell<Vec<usize>>
}

impl OpcodeLocation {
    pub fn new(location: usize) -> Self {
        OpcodeLocation {
            #[cfg(debug_assertions)]
            index: OPCODE_LOCATION_INDEXER.fetch_add(1, Ordering::SeqCst),
            location: RefCell::new(LocationType::Fixed(location)),
            used_location: RefCell::new(Vec::new())
        }
    }

    pub fn empty() -> Self {
        OpcodeLocation {
            #[cfg(debug_assertions)]
            index: OPCODE_LOCATION_INDEXER.fetch_add(1, Ordering::SeqCst),
            location: RefCell::new(LocationType::Fixed(0)),
            used_location: RefCell::new(Vec::new())
        }
    }

    #[cfg(debug_assertions)]
    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get(&self) -> usize {
        match &*self.location.borrow() {
            LocationType::Fixed(location) => *location,
            LocationType::Subtraction { left_hand, right_hand } => {
                println!("{} {}", left_hand.get(), right_hand.get());
                left_hand.get() - right_hand.get()
            }
        }
    }

    pub fn set(&self, location: usize, opcodes: &mut Vec<u8>) {
        *self.location.borrow_mut() = LocationType::Fixed(location);

        for used_location in self.used_location.borrow().iter() {
            opcodes[*used_location] = location as u8;
            opcodes[*used_location + 1] = (location >> 8) as u8;
        }
    }

    pub fn subtraction(&self, left_hand: Rc<OpcodeLocation>, right_hand: Rc<OpcodeLocation>) {
        #[cfg(debug_assertions)]
        assert!(left_hand.get_index() != right_hand.get_index());
        
        *self.location.borrow_mut() = LocationType::Subtraction {
            left_hand,
            right_hand
        };
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

    fn dump<'a>(&self, _: &'a DumpBuilder, _: Rc<AtomicUsize>, _: &Vec<u8>) {
        // Dynamically location calculator
    }
}

#[derive(Clone)]
pub struct DynamicLocationUpdateGenerator { 
    pub target:  Rc<OpcodeLocation>,
    pub source:  Rc<OpcodeLocation>
}

impl OpcodeGeneratorTrait for DynamicLocationUpdateGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        self.target.set(self.source.get(), opcodes);
    }

    fn dump<'a>(&self, _: &'a DumpBuilder, _: Rc<AtomicUsize>, _: &Vec<u8>) {
        // Dynamically location calculator
    }
}

#[derive(Clone)]
pub struct SubtractionGenerator { 
    pub target:  Rc<OpcodeLocation>,
    pub left_hand:  Rc<OpcodeLocation>,
    pub right_hand:  Rc<OpcodeLocation>
}

impl OpcodeGeneratorTrait for SubtractionGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        let left = self.left_hand.get();
        let right = self.right_hand.get();

        self.target.set(left - right, opcodes);
    }

    fn dump<'a>(&self, _: &'a DumpBuilder, _: Rc<AtomicUsize>, _: &Vec<u8>) {
        // Dynamically location calculator
    }
}

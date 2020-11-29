mod compiler;
mod static_storage;
mod storage_builder;

pub mod value;
pub mod ast;

pub use self::compiler::*;
pub use self::static_storage::*;
pub use self::value::*;

use crate::types::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;
use std::mem;

pub trait Storage {

    /// Build memory block with temporary, constant and variable definitions
    fn build(&mut self);
    fn new() -> Self;
    fn get_memory(&mut self) -> Rc<RefCell<Vec<VmObject>>>;
    fn get_constant_size(&self) -> u8;
    fn get_variable_size(&self) -> u8;
    fn get_temp_size(&self) -> u8;
    fn get_free_temp_slot(&mut self) -> u8;
    fn set_temp_size(&mut self, value: u8);

    fn get_temp_counter(&self) -> u8;
    fn set_temp_counter(&mut self, counter: u8);
    fn inc_temp_counter(&mut self);
    fn reset_temp_counter(&mut self);

    fn add_variable(&mut self, name: &String) -> u8;
    fn set_variable_value(&mut self, name: &String, object: VmObject);
    fn add_constant(&mut self, object: Rc<BramaPrimative>);

    fn get_variable_location(&self, name: &String) -> Option<u8>;
    fn get_variable_value(&self, name: &String) -> Option<Rc<BramaPrimative>>;

    fn get_constant_location(&self, object: Rc<BramaPrimative>) -> Option<u8>;

    fn dump(&self);
}

pub struct VmByte(pub u32);
pub struct VmData {
    pub opcode: VmOpCode,
    pub target: u8,
    pub a: u8,
    pub b: u8
}

impl VmData {
    pub fn encode(&self) -> VmByte {
        VmByte((self.opcode as u32) |
            (self.target as u32) << 8 |
            (self.a as u32) << 16 |
            (self.b as u32) << 24)
    }
}

impl VmByte {
    pub fn none() -> VmByte {
        Self::new(VmOpCode::None, 0, 0, 0)
    }

    pub fn new(opcode: VmOpCode, target: u8, a: u8, b: u8) -> VmByte {
        VmData {
            target,
            opcode,
            a,
            b
        }.encode()
    }

    pub fn decode(&self) -> VmData {
        let VmByte(bits) = *self;

        VmData {
            opcode: unsafe { mem::transmute::<_, VmOpCode>((bits & 0xff) as u8) },
            target: (bits >> 8) as u8,
            a: (bits >> 16) as u8,
            b: (bits >> 24) as u8
        }
    }

    pub fn decode_as_tuple(&self) -> (VmOpCode, usize, usize, usize) {
        let VmByte(bits) = *self;
        let opcode = unsafe { mem::transmute::<_, VmOpCode>((bits & 0xff) as u8) };
        let target = (bits >> 8) as u8;
        let a = (bits >> 16) as u8;
        let b = (bits >> 24) as u8;

        (opcode, target as usize, a as usize, b as usize)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VmOpCode {
    None = 0,
    Addition,
    Subraction,
    Multiply,
    Division,
    And,
    Or,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqualThan,
    LessEqualThan,
    AssignAddition,
    AssignSubtraction,
    AssignMultiplication,
    AssignDivision,
    NativeCall,

    Increment,
    Decrement,

    Move
}

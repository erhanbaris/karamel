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
use std::fmt;

pub trait Storage {

    /// Build memory block with temporary, constant and variable definitions
    fn build(&mut self);
    fn new() -> Self;
    fn get_memory(&mut self) -> Rc<RefCell<Vec<VmObject>>>;
    fn get_stack(&mut self) -> Rc<RefCell<Vec<VmObject>>>;
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

pub struct VmByte(pub u8);
impl fmt::Debug for VmByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.decode_opcode())
    }
}

impl VmByte {
    pub fn new_opcode(opcode: VmOpCode) -> VmByte {
        VmByte(opcode as u8)
    }

    #[allow(dead_code)]
    pub fn decode_opcode(&self) -> VmOpCode {
        let VmByte(bits) = *self;
        unsafe { mem::transmute::<_, VmOpCode>((bits & 0xff) as u8) }
    }
}

trait VmByteDecode {
    fn encode(&self) -> VmByte;
}

impl VmByteDecode for VmOpCode {
    fn encode(&self) -> VmByte {
        VmByte::new_opcode(*self)
    }
}

impl VmByteDecode for u8 {
    fn encode(&self) -> VmByte {
        VmByte(*self)
    }
}

impl VmOpCode {
    pub fn encode(&self) -> VmByte {
        VmByte(*self as u8)
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
    NativeCall,

    Increment,
    Decrement,
    Not,
    
    Compare,
    Jump,

    Load,
    Store,
    FastStore,
    CopyToStore,
    Dublicate
}

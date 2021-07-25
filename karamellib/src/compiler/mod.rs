mod compiler;
mod static_storage;
mod storage_builder;
pub mod function;

pub mod value;
pub mod ast;
pub mod module;
pub mod scope;
pub mod context;
pub mod generator;

pub use self::compiler::*;
pub use self::static_storage::*;
pub use self::value::*;
pub use self::context::KaramelCompilerContext;

use std::vec::Vec;
use std::mem;
use std::fmt;

pub trait GetType {
    fn get_type(&self) -> String;
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


#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VmOpCode {
    Addition = 1,
    Subraction = 2,
    Multiply = 3,
    Division = 4,
    Module = 5,
    And = 6,
    Or = 7,
    Equal = 8,
    NotEqual = 9,
    GreaterThan = 10,
    GreaterEqualThan = 12,

    Call = 16,
    CallStack = 17,
    Return = 18,

    Increment = 19,
    Decrement = 20,
    Not = 21,

    /// Compare previous two opcode.
    /// If true, jump over 2 opcode and continue to execution.
    /// If false, read next 2 opcode than calculate false jump location via high and low byte.
    Compare = 22,
    Jump = 23,

    Init = 24,

    /// Copy value from memory to stack.
    Load = 26,
    
    /// Copy stack value to memory and remove value from stack.
    Store = 27,

    /// Dublicate value at memory. Take value from memory and copy to destination location. Stack not involved at this operation.
    FastStore = 28,

    /// Copy last stack value to memory and keep copied value at stack.
    CopyToStore = 29,
    Dublicate = 30,
    GetItem = 31,
    SetItem = 32,
    Constant = 33,
    Halt = 34
}

impl From<VmOpCode> for u8 {
    fn from(opcode: VmOpCode) -> Self {
        opcode as u8
    }
}

impl From<&VmOpCode> for u8 {
    fn from(opcode: &VmOpCode) -> Self {
        *opcode as u8
    }
}


impl fmt::Display for VmOpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
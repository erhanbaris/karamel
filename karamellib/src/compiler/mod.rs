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
    None = 0,
    Addition,
    Subraction,
    Multiply,
    Division,
    Module,
    And,
    Or,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqualThan,
    LessEqualThan,

    Func,
    InitArguments,
    Call,
    CallStack,
    Return,

    Increment,
    Decrement,
    Not,

    /// Compare previous two opcode.
    /// If true, jump over 2 opcode and continue to execution.
    /// If false, read next 2 opcode than calculate false jump location via high and low byte.
    Compare,
    Jump,

    InitList,
    InitDict,

    /// Copy value from memory to stack.
    Load,

    /// Copy stack value to memory and remove value from stack.
    Store,

    /// Dublicate value at memory. Take value from memory and copy to destination location. Stack not involved at this operation.
    FastStore,

    /// Copy last stack value to memory and keep copied value at stack.
    CopyToStore,
    Dublicate,
    GetItem,
    SetItem,
    Halt
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
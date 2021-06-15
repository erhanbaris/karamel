mod compiler;
mod static_storage;
mod storage_builder;
pub mod function;

pub mod value;
pub mod ast;

pub use self::compiler::*;
pub use self::static_storage::*;
pub use self::value::*;

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

    Compare,
    Jump,

    InitList,
    InitDict,

    Load,
    Store,
    FastStore,
    CopyToStore,
    Dublicate,
    GetItem,
    SetItem,
    Halt
}

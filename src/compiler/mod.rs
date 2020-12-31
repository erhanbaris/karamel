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
use std::cell::Cell;
use std::rc::Rc;
use std::vec::Vec;
use std::mem;
use std::fmt;

pub struct FunctionInformation {
    pub name: String,
    pub opcode_location: Cell<u16>,
    pub used_locations: RefCell<Vec<u16>>,
    pub storage_index: u16
}

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
    fn set_parent_index(&mut self, parent_index: usize);
    fn get_parent_index(&self) -> usize;
    fn get_temp_counter(&self) -> u8;
    fn set_temp_counter(&mut self, counter: u8);
    fn inc_temp_counter(&mut self);
    fn reset_temp_counter(&mut self);

    fn add_function(&mut self, name: &String, information: Rc<FunctionInformation>);
    fn add_variable(&mut self, name: &String) -> u8;
    fn set_variable_value(&mut self, name: &String, object: VmObject);
    fn add_constant(&mut self, object: Rc<BramaPrimative>);

    fn get_function(&self, name: &String) -> Option<Rc<FunctionInformation>>;
    fn get_variable_location(&self, name: &String) -> Option<u8>;
    fn get_variable_value(&self, name: &String) -> Option<Rc<BramaPrimative>>;

    fn get_constant_location(&self, object: Rc<BramaPrimative>) -> Option<u8>;
    fn get_function_constant(&self, name: String, module_path: Vec<String>, framework: String) -> Option<u8>;
    fn get_function_constants(&self) -> Vec<(u8, VmObject)>;
    fn update_constant(&self, index: u8, object: VmObject);

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
    GetItem
}

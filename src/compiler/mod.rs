mod compiler;
mod static_storage;
mod dynamic_storage;
pub mod value;
pub mod ast;

pub use self::compiler::*;
pub use self::static_storage::*;
pub use self::dynamic_storage::*;
pub use self::value::*;

use crate::types::*;

use std::rc::Rc;
use std::vec::Vec;


pub trait Storage {

    /// Build memory block with temporary, constant and variable definitions
    fn build(&mut self);
    fn new() -> Self;
    fn get_memory(&mut self) -> &mut Vec<VmObject>;
    fn get_constant_size(&self) -> u16;
    fn get_variable_size(&self) -> u16;
    fn get_temp_size(&self) -> u16;
    fn get_free_temp_slot(&mut self) -> u16;
    fn set_temp_size(&mut self, value: u16);

    fn get_temp_counter(&self) -> u16;
    fn inc_temp_counter(&mut self);
    fn reset_temp_counter(&mut self);

    fn add_variable(&mut self, name: &String) -> u16;
    fn set_variable_value(&mut self, name: &String, object: VmObject);
    fn add_constant(&mut self, object: Rc<BramaPrimative>);

    fn get_variable_location(&self, name: &String) -> Option<u16>;
    fn get_variable_value(&self, name: &String) -> Option<Rc<BramaPrimative>>;

    fn get_constant_location(&self, object: Rc<BramaPrimative>) -> Option<u16>;

    fn dump(&self);
}

#[repr(C)]
pub enum BramaVmOpCode {
    None,
    Addition             {target: i16, left: i16, right: i16},
    Subraction           {target: i16, left: i16, right: i16},
    Multiply             {target: i16, left: i16, right: i16},
    Division             {target: i16, left: i16, right: i16},
    And                  {target: i16, left: i16, right: i16},
    Or                   {target: i16, left: i16, right: i16},
    Equal                {target: i16, left: i16, right: i16},
    NotEqual             {target: i16, left: i16, right: i16},
    GreaterThan          {target: i16, left: i16, right: i16},
    LessThan             {target: i16, left: i16, right: i16},
    GreaterEqualThan     {target: i16, left: i16, right: i16},
    LessEqualThan        {target: i16, left: i16, right: i16},
    Assign               {target: i16, expression: i16},
    AssignAddition       {target: i16, expression: i16},
    AssignSubtraction    {target: i16, expression: i16},
    AssignMultiplication {target: i16, expression: i16},
    AssignDivision       {target: i16, expression: i16}
}
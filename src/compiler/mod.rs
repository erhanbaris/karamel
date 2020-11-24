mod compiler;
mod static_storage;
mod dynamic_storage;

pub use self::compiler::*;
pub use self::static_storage::*;
pub use self::dynamic_storage::*;

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
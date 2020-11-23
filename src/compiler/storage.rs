use crate::types::*;
use crate::compiler::Storage;

use std::collections::HashMap;
use std::rc::Rc;


#[derive(PartialEq, Debug)]
pub struct StaticStorage {
    pub constants             : Vec<VmObject>,
    pub constant_size         : u16,
    pub temp_size             : u16,
    pub temp_counter          : u16,
    pub variables             : HashMap<String, u16>,
    pub memory                : Vec<VmObject>,
    pub total_const_variables : u16
}

impl StaticStorage {
    pub fn new() -> StaticStorage {
        StaticStorage {
            constants: Vec::new(),
            constant_size: 0,
            temp_size: 0,
            temp_counter: 0,
            total_const_variables: 0,
            memory: Vec::new(),
            variables: HashMap::new()
        }
    }
}

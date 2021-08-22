use crate::buildin::Module;
use crate::types::*;
use crate::compiler::*;
use std::rc::Rc;

#[cfg(not(feature = "unittest"))]
use crate::{debug_println};

use std::ptr;

pub struct StaticStorage {
    pub index                 : usize,
    pub constants             : Vec<VmObject>,
    pub constants_ptr         : *const VmObject,
    pub variables             : Vec<String>,
    pub parent_location       : Option<usize>
}

impl StaticStorage {
    pub fn new(index: usize) -> Self {
        let mut storage = StaticStorage {
            index: index,
            constants: Vec::with_capacity(128),
            constants_ptr: ptr::null(),
            variables: Vec::new(),
            parent_location: None
        };
        storage.constants_ptr = storage.constants.as_ptr();
        storage
    }
    pub fn get_variable_size(&self) -> u8 { self.variables.len() as u8 }
    
    pub fn set_parent_location(&mut self, parent_location: usize) {
        self.parent_location = Some(parent_location);
    }
    pub fn get_parent_location(&self) -> Option<usize> {
        self.parent_location
    }
    pub fn add_constant(&mut self, value: Rc<KaramelPrimative>) -> usize {
        let constant_position = self.constants.iter().position(|x| {
            *x.deref() == *value
        });
        
        match constant_position {
            Some(position) => position,
            None => {
                self.constants.push(VmObject::convert(value));
                self.constants.len() -1
            }
        }
    }

    pub fn add_variable(&mut self, name: &str) -> u8 {
        let result = self.variables.iter().position(|key| key == name);
        match result {
            Some(location) => location as u8,
            _ => {
                self.variables.push(name.to_string());
                (self.variables.len()-1) as u8
            }
        }
    }

    pub fn get_variable_location(&self, name: &str) -> Option<u8> {
        let result = self.variables.iter().position(|key| key == name);
        match result {
            Some(location) => Some(location as u8),
            _ => None
        }
    }

    pub fn get_constant_location(&self, value: Rc<KaramelPrimative>) -> Option<u8> {
        return match self.constants.iter().position(|x| { *x.deref() == *value }) {
            Some(number) => Some(number as u8),
            _ => None
        };
    }

    pub fn get_function_constant(&self, name: String, module: Rc<dyn Module>) -> Option<u8> {
        
        for (index, item) in self.constants.iter().enumerate() {
            if let KaramelPrimative::Function(reference, _) = &*item.deref() {
                if reference.name        == name && 
                   reference.module.get_path() == module.get_path() {
                    return Some(index as u8);
                }
            }
        }

        None
    }

    pub fn get_class_constant(&self, name: String, _module_path: Rc<dyn Module>) -> Option<u8> {
        
        for (index, item) in self.constants.iter().enumerate() {
            if let KaramelPrimative::Class(reference) = &*item.deref() {
                if reference.get_class_name() == name {
                    return Some(index as u8);
                }
            }
        }

        None
    }

    #[cfg(feature = "unittest")]
    pub fn dump(&self) {}

    #[cfg(not(feature = "unittest"))]
    pub fn dump(&self) {
        debug_println!("------------------------------------------");
        debug_println!("╔════════════════════════════════════════╗");
        debug_println!("║             VARIABLE DUMP              ║");
        debug_println!("╠════════════════════════════════════════╣");
        for (index, variable) in self.variables.iter().enumerate() {
            debug_println!("║ {:3?} ║ {:32} ║", index, format!("{}", variable));
        }
        debug_println!("╚════════════════════════════════════════╝");
        debug_println!("╔════════════════════════════════════════╗");
        debug_println!("║             CONSTANT DUMP              ║");
        debug_println!("╠═════╦══════════════════════════════════╣");
        for (index, constant) in self.constants.iter().enumerate() {
            debug_println!("║ {:3?} ║ {:32} ║", index, format!("{}", constant));
        }
        debug_println!("╚═════╩══════════════════════════════════╝");
    }
}

use crate::buildin::Module;
use crate::types::*;
use crate::compiler::*;
use std::rc::Rc;

#[cfg(not(feature = "unittest"))]
use crate::{debug_println};

use std::ptr;

unsafe fn from_buf_raw<T>(ptr: *const T, elts: usize) -> Vec<T> {
    let mut dst = Vec::with_capacity(elts);

    // SAFETY: Our precondition ensures the source is aligned and valid,
    // and `Vec::with_capacity` ensures that we have usable space to write them.
    ptr::copy(ptr, dst.as_mut_ptr(), elts);

    // SAFETY: We created it with this much capacity earlier,
    // and the previous `copy` has initialized these elements.
    dst.set_len(elts);
    dst
}

pub struct StaticStorage {
    pub index                 : usize,
    pub constants             : Vec<VmObject>,
    pub constants_ptr         : *const VmObject,
    pub variables             : Vec<String>,
    pub memory                : Vec<VmObject>,
    pub parent_location       : Option<usize>
}

impl StaticStorage {
    pub fn new(index: usize) -> Self {
        let mut storage = StaticStorage {
            index: index,
            constants: Vec::with_capacity(128),
            constants_ptr: ptr::null(),
            memory: Vec::new(),
            variables: Vec::new(),
            parent_location: None
        };
        storage.constants_ptr = storage.constants.as_ptr();
        storage
    }

    pub fn get_mut_memory(&mut self) -> &mut Vec<VmObject> { 
        self.memory.as_mut()
    }
    pub fn get_memory(&mut self) -> Vec<VmObject> { 
        unsafe {
            from_buf_raw(self.memory.as_mut_ptr(), self.memory.len())
        }
    }
    pub fn get_variable_size(&self) -> u8 { self.variables.len() as u8 }
    
    pub fn set_parent_location(&mut self, parent_location: usize) {
        self.parent_location = Some(parent_location);
    }
    pub fn get_parent_location(&self) -> Option<usize> {
        self.parent_location
    }

    /// add variable and constant data same time. Assign constant location to variable reference.
    /// If variable or constant data already assigned before, it will try to update variable value with constant.
    pub fn add_static_data(&mut self, name: &str, value: Rc<KaramelPrimative>) {
        let result = self.variables.iter().position(|key| key == name);
        match result {
            Some(location) => self.memory[location] = VmObject::convert(value),
            _ => {
                self.variables.push(name.to_string());
                self.memory.push(VmObject::convert(value));
            }
        }
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
                self.memory.push(VmObject::convert(Rc::new(KaramelPrimative::Empty)));
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

    #[allow(dead_code)]
    pub fn get_variable_value(&self, name: &str) -> Option<Rc<KaramelPrimative>> {
        match self.get_variable_location(name) {
            Some(loc) => Some(self.memory[loc as usize].deref()),
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
        debug_println!("║               MEMORY DUMP {:10}   ║", format!("#{}", self.index));
        debug_println!("╠═══╦═════╦══════════════════════════════╣");

        for (index, item) in self.memory.iter().enumerate() {
            let last_type = "V";

            debug_println!("║ {} ║ {:3?} ║ {:28} ║", last_type, index, format!("{:?}", *item.deref()));
        }

        debug_println!("╚═══╩═════╩══════════════════════════════╝");
        debug_println!("╔════════════════════════════════════════╗");
        debug_println!("║             VARIABLE DUMP              ║");
        debug_println!("╠════════════════════════════════════════╣");
        for variable in &self.variables {
            debug_println!("║ {:38} ║", format!("{}", variable));
        }
        debug_println!("╚════════════════════════════════════════╝");
        debug_println!("╔════════════════════════════════════════╗");
        debug_println!("║             CONSTANT DUMP              ║");
        debug_println!("╠════════════════════════════════════════╣");
        for constant in &self.constants {
            debug_println!("║ {:38} ║", format!("{}", constant));
        }
        debug_println!("╚════════════════════════════════════════╝");
    }
}

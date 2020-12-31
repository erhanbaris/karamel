use crate::types::*;
use crate::compiler::*;

use std::collections::BTreeMap;
use std::cell::RefCell;
use std::rc::Rc;


pub struct StaticStorage {
    pub constants             : Vec<VmObject>,
    pub constant_size         : u8,
    pub temp_size             : u8,
    pub temp_counter          : u8,
    pub variables             : Vec<(String, u8)>,
    pub memory                : Rc<RefCell<Vec<VmObject>>>,
    pub stack                 : Rc<RefCell<Vec<VmObject>>>,
    pub total_const_variables : u8,
    pub functions             : BTreeMap<String, Rc<FunctionInformation>>,
    pub function_references   : Vec<FunctionReference>,
    pub parent_index          : usize
}


/*
### STORAGE STRUCTURE ###
-------------------------
  CONSTANT VALUES
-------------------------
  VARIABLE VALUES
-------------------------
  TEMP VALUES
-------------------------
*/
impl Storage for StaticStorage {
    fn new() -> StaticStorage {
        StaticStorage {
            constants: Vec::new(),
            constant_size: 0,
            temp_size: 0,
            temp_counter: 0,
            total_const_variables: 0,
            memory: Rc::new(RefCell::new(Vec::new())),
            stack: Rc::new(RefCell::new(Vec::new())),
            variables: Vec::new(),
            functions: BTreeMap::new(),
            parent_index: 0,
            function_references: Vec::new()
        }
    }

    fn build(&mut self) {
        self.constant_size = self.constants.len() as u8;
        let mut memory = self.memory.borrow_mut();
        let mut stack  = self.stack.borrow_mut();

        /* Allocate memory */
        let memory_size = self.get_constant_size() 
                        + self.get_variable_size() 
                        + self.get_temp_size();
        
        memory.reserve(memory_size.into());

        /* Move all constants informations to memory location */
        memory.append(&mut self.constants);

        /*  Allocate variable memory and update references */
        let mut index = self.get_constant_size();
        for (_, value) in self.variables.iter_mut() {
            memory.push(VmObject::convert(Rc::new(BramaPrimative::Empty)));
            *value = index;
            index += 1;
        }

        for _ in 0..self.temp_size {
            stack.push(VmObject::convert(Rc::new(BramaPrimative::Empty)));
        }
    }

    fn get_memory(&mut self) -> Rc<RefCell<Vec<VmObject>>> { self.memory.clone() }
    fn get_stack(&mut self) -> Rc<RefCell<Vec<VmObject>>> { self.stack.clone() }
    fn get_constant_size(&self) -> u8 { self.constant_size }
    fn get_variable_size(&self) -> u8 { self.variables.len() as u8 }
    fn get_temp_size(&self) -> u8     { self.temp_size }
    fn set_parent_index(&mut self, parent_index: usize) {
        self.parent_index = parent_index;
    }
    fn get_parent_index(&self) -> usize { self.parent_index }
    fn set_temp_size(&mut self, value: u8) { self.temp_size = value; }
    fn get_free_temp_slot(&mut self) -> u8 { 
        let index = self.temp_counter;
        self.temp_counter += 1;
        return self.get_constant_size() + self.get_variable_size() + index;
    }

    fn get_temp_counter(&self) -> u8 { self.temp_counter }
    fn set_temp_counter(&mut self, counter: u8) { self.temp_counter = counter; }
    fn inc_temp_counter(&mut self)    { self.temp_counter += 1; }
    fn reset_temp_counter(&mut self)  { self.temp_counter = 0; }

    fn add_function(&mut self, name: &String, information: Rc<FunctionInformation>) {
        self.functions.insert(name.to_string(), information);
    }

    fn add_constant(&mut self, value: Rc<BramaPrimative>) {
        let has = self.constants.iter().any(|x| {
            *x.deref() == *value
        });
        
        if !has { 
            self.constants.push(VmObject::convert(value.clone()));
        };
    }

    fn add_variable(&mut self, name: &String) -> u8 {
        let result = self.variables.iter().position(|(key, _)| key == name);
        match result {
            Some(location) => self.variables[location].1,
            _ => {
                self.variables.push((name.to_string(), 0));
                0
            }
        }
    }

    fn set_variable_value(&mut self, name: &String, object: VmObject) {
        match self.get_variable_location(name) {
            Some(location) => {
                let mut memory = self.memory.borrow_mut();
                memory[location as usize] = object;
            },
            _ => ()
        };
    }

    fn get_function(&self, name: &String) -> Option<Rc<FunctionInformation>> {
        if self.functions.contains_key(name) {
            return Some(self.functions.get(name).unwrap().clone());
        }
        return None;
    }

    fn get_variable_location(&self, name: &String) -> Option<u8> {
        let result = self.variables.iter().position(|(key, _)| key == name);
        match result {
            Some(location) => Some(self.variables[location].1),
            _ => None
        }
    }

    fn get_variable_value(&self, name: &String) -> Option<Rc<BramaPrimative>> {
        match self.get_variable_location(name) {
            Some(loc) => Some(self.memory.borrow_mut()[loc as usize].deref()),
            _ => None
        }
    }

    fn get_constant_location(&self, value: Rc<BramaPrimative>) -> Option<u8> {
        return match self.memory.borrow_mut().iter().position(|x| { return *x.deref() == *value; }) {
            Some(number) => Some(number as u8),
            _ => None
        };
    }

    fn get_function_constant(&self, name: String, module_path: Vec<String>, framework: String) -> Option<u8> {
        
        for (index, item) in self.memory.borrow().iter().enumerate() {
            if let BramaPrimative::Function(reference) = &*item.deref() {
                if reference.name        == name && 
                   reference.module_path == module_path && 
                   reference.framework   == framework {
                    return Some(index as u8);
                }
            }
        }

        None
    }

    fn update_constant(&self, index: u8, object: VmObject) {
        self.memory.borrow_mut()[index as usize] = object;
    }

    fn get_function_constants(&self) -> Vec<(u8, VmObject)> {
        let mut items = Vec::new();

        for (index, item) in self.memory.borrow().iter().enumerate() {
            if let BramaPrimative::Function(_) = &*item.deref() {
                items.push((index as u8, *item));
            }
        }

        return items;
    }

    fn dump(&self) {
        println!("╔════════════════════════════════════════╗");
        println!("║               MEMORY DUMP              ║");
        println!("╠═══╦═════╦══════════════════════════════╣");

        let consts    = self.constant_size;
        let variables = self.constant_size as usize + self.variables.len();

        let mut last_type = 'C';

        for (index, item) in self.memory.borrow().iter().enumerate() {

            if consts as usize == index {
                last_type = 'V';
            } else if variables as usize == index {
                last_type = 'T';
            }

            println!("║ {} ║ {:3?} ║ {:28} ║", last_type.to_string(), index, format!("{:?}", *item.deref()));
        }

        println!("╚═══╩═════╩══════════════════════════════╝");
        println!("╔════════════════════════════════════════╗");
        println!("║             VARIABLE DUMP              ║");
        println!("╠════════════════════════════════════════╣");
        for (variable, value) in &self.variables {
            println!("║ {:38} ║", format!("{} [{}]", variable, value));
        }
        println!("╚════════════════════════════════════════╝");

        println!("╔════════════════════════════════════════╗");
        println!("║                  STACK                 ║");
        println!("╠════════════════════════════════════════╣");
        println!("║ Stack size: {:<10}                 ║", self.stack.borrow().len());
        println!("╠════════════════════════════════════════╣");
        for item in self.stack.borrow().iter() {
            println!("║ {:38} ║", format!("{:?}", item.deref()));
        }
        println!("╚════════════════════════════════════════╝");
    }
}

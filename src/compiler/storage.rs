use crate::types::*;
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

pub trait Storage {
    /// Build memory block with temporary, constant and variable definitions
    fn build(&mut self);
    fn get_memory(&self) -> &Vec<VmObject>;
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
    fn get_constant_location(&self, object: Rc<BramaPrimative>) -> Option<u16>;

    fn dump(&self);
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
    fn build(&mut self) {
        self.constant_size = self.constants.len() as u16;

        /* Allocate memory */
        let memory_size = self.get_constant_size() + self.get_variable_size() + self.get_temp_size();
        self.memory.reserve(memory_size.into());

        /* Move all constants informations to memory location */
        self.memory.append(&mut self.constants);

        /*  Allocate variable memory and update referances */
        let mut index = self.get_constant_size();
        for (_, value) in self.variables.iter_mut() {
            self.memory.push(VmObject::convert(Rc::new(BramaPrimative::Empty)));
            *value = index;
            index += 1;
        }

        let start_index = self.get_temp_size();
        for _ in 0..start_index {
            self.memory.push(VmObject::convert(Rc::new(BramaPrimative::Empty)));
        }
    }

    fn get_memory(&self) -> &Vec<VmObject> { &self.memory }
    fn get_constant_size(&self) -> u16 { self.constant_size }
    fn get_variable_size(&self) -> u16 { self.variables.len() as u16 }
    fn get_temp_size(&self) -> u16     { self.temp_size }
    fn set_temp_size(&mut self, value: u16) { self.temp_size = value; }
    fn get_free_temp_slot(&mut self) -> u16 { 
        let index = self.temp_counter;
        self.temp_counter += 1;
        return self.get_constant_size() + self.get_variable_size() + index;
    }

    fn get_temp_counter(&self) -> u16 { self.temp_counter }
    fn inc_temp_counter(&mut self)    { self.temp_counter += 1; }
    fn reset_temp_counter(&mut self)  { self.temp_counter = 0; }

    fn add_constant(&mut self, value: Rc<BramaPrimative>) {
        let has = self.constants.iter().any(|x| {
            *x.deref() == *value
        });
        
        if !has { 
            self.constants.push(VmObject::convert(value.clone()));
        };
    }

    fn add_variable(&mut self, name: &String) -> u16 { 
        if !self.variables.contains_key(&name[..]) {
            self.variables.insert(name.to_string(), 0);
        }

        return self.get_variable_location(name).unwrap();
    }

    fn set_variable_value(&mut self, name: &String, object: VmObject) {
        match self.get_variable_location(name) {
            Some(location) => {
                self.memory[location as usize] = object;
            },
            _ => ()
        };
    }

    fn get_variable_location(&self, name: &String) -> Option<u16> {
        if self.variables.contains_key(name) {
            return Some(*self.variables.get(name).unwrap());
        }
        return None;
    }

    fn get_constant_location(&self, value: Rc<BramaPrimative>) -> Option<u16> {
        return match self.memory.iter().position(|x| { return *x.deref() == *value; }) {
            Some(number) => Some(number as u16),
            _ => None
        };
    }

    fn dump(&self) {
        println!("-------------------------------");
        println!("        MEMORY DUMP");
        println!("-------------------------------");
        for (index, item) in self.memory.iter().enumerate() {
            println!("| {:?} | {:?}", index, *item.deref());
        }
        println!("-------------------------------");
        println!("-------------------------------");
        println!("        VARIABLE DUMP");
        println!("-------------------------------");
        for (variable, value) in &self.variables {
            println!("| {:?}  [{:?}]", variable, value);
        }
        println!("-------------------------------");
    }
}

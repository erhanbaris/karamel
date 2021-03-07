use crate::buildin::{Class, ClassProperty};
use crate::compiler::{BramaPrimative, function::{FunctionReference}};

use std::collections::HashMap;
use crate::compiler::GetType;
use std::rc::Rc;

pub struct OpcodeClass {
    pub name: String,
    pub storage_index: usize,
    pub properties: HashMap<String, ClassProperty>
 }

impl Class for OpcodeClass {
    fn get_class_name(&self) -> String {
        self.name.to_string()
    }
    
    fn has_property(&self, name: String) -> bool {
        self.properties.contains_key(&name)
    }
    
    fn add_method(&mut self, name: String, function: Rc<FunctionReference>) {
        self.properties.insert(name, ClassProperty::Function(function.clone()));
    }

    fn add_property(&mut self, name: String, property: Rc<BramaPrimative>) {
        self.properties.insert(name, ClassProperty::Field(property.clone()));
    }
    
    fn get_method(&self, name: &String) -> Option<Rc<FunctionReference>> {
        match self.properties.get(name) {
            Some(property) => {
                match property {
                    ClassProperty::Field(_) => None,
                    ClassProperty::Function(function) => Some(function.clone())
                }
            },
            None => None
        }
    }

    fn get_property(&self, name: &String) -> Option<Rc<BramaPrimative>> {
        match self.properties.get(name) {
            Some(property) => {
                match property {
                    ClassProperty::Field(property) => Some(property.clone()),
                    ClassProperty::Function(_) => None
                }
            },
            None => None
        }
    }
}

impl GetType for OpcodeClass {
    fn get_type(&self) -> String {
        self.get_class_name()
    }
}

impl OpcodeClass {
    #[allow(dead_code)]
    fn new(name: String, storage_index: usize) -> OpcodeClass {
        OpcodeClass {
            name,
            storage_index,
            properties: HashMap::new()
        }
    }
}

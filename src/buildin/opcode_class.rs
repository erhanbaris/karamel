use crate::buildin::{Class, ClassProperty};
use crate::compiler::{BramaPrimative, function::{FunctionReference, NativeCall}};

use std::collections::HashMap;
use std::vec::Vec;
use std::rc::Rc;

pub struct OpcodeClass {
    pub name: String,
    pub storage_index: u16,
    pub properties: HashMap<String, ClassProperty>
 }

impl Class for OpcodeClass {
    fn new(name: String, storage_index: usize) -> Class {
        OpcodeClass {
            name: String::new(),
            storage_index: 0,
            properties: HashMap::new()
        }
    }

    fn get_class_name(&self) -> String {
        self.name
    }
    
    fn has_property(&self, name: String) -> bool {
        self.properties.contains_key(name)
    }
    
    fn add_method(&mut self, name: String, function: Rc<FunctionReference>) {
        self.properties.insert(name, ClassProperty::Function(function.clone()));
    }

    fn add_property(&mut self, name: String, property: Rc<BramaPrimative>) {
        self.properties.insert(name, ClassProperty::Field(function.clone()));
    }
    
    fn get_method(&self, name: &String) -> Option<FunctionReference> {
        match self.properties.get(name) {
            Some(property) => {
                match property {
                    ClassProperty::Field(_) => None,
                    ClassProperty::Function(function) => Some(function)
                }
            },
            None => None
        }
    }

    fn get_property(&self, name: &String) -> Option<Rc<BramaPrimative>> {
        match self.properties.get(name) {
            Some(property) => {
                match property {
                    ClassProperty::Field(property) => Some(property),
                    ClassProperty::Function(_) => None
                }
            },
            None => None
        }
    }
}

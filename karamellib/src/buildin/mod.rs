pub mod debug;
pub mod io;
pub mod num;
pub mod base_functions;
pub mod opcode_class;
pub mod number;

use crate::compiler::GetType;

use std::collections::HashMap;
use std::vec::Vec;
use std::rc::Rc;

use crate::compiler::{BramaPrimative, function::{FunctionReference, NativeCall}};

pub trait Module {
    fn new() -> Self where Self: Sized;
    fn get_module_name(&self) -> String;
    
    fn get_method(&self, name: &str) -> Option<NativeCall>;
    fn get_module(&self, name: &str) -> Option<Rc<dyn Module>>;

    fn get_methods(&self) -> Vec<(&'static str, NativeCall)>;
    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>>;

    fn get_classes(&self) -> Vec<Rc<dyn Class>>;
}

pub struct ModuleCollection {
    pub modules: HashMap<String, Rc<dyn Module>>
}

impl ModuleCollection
{
    pub fn new() -> ModuleCollection {
        let mut collection = ModuleCollection {
            modules: HashMap::new()
        };
        collection.add_module(Rc::new(base_functions::BaseFunctionsModule::new()));
        collection.add_module(Rc::new(io::IoModule::new()));
        collection.add_module(Rc::new(num::NumModule::new()));
        collection.add_module(Rc::new(debug::DebugModule::new()));
        collection
    }

    pub fn add_module(&mut self, module: Rc<dyn Module>) {        
        self.modules.insert(module.get_module_name(), module);
    }
}


pub enum ClassProperty {
    Function(Rc<FunctionReference>),
    Field(Rc<BramaPrimative>)
}

pub struct ClassConfig {
    pub name: String,
    pub storage_index: usize,
    pub properties: HashMap<String, ClassProperty>,
    pub is_readonly: bool,
    pub is_buildin: bool,
    pub is_static: bool
}

pub trait Class: GetType {
    fn set_class_config(&mut self, config: ClassConfig);
    fn get_class_name(&self) -> String;
    
    fn has_property(&self, name: String) -> bool;
    
    fn add_method(&mut self, name: String, function: Rc<FunctionReference>);
    fn add_property(&mut self, name: String, property: Rc<BramaPrimative>);
    
    fn get_method(&self, name: &str) -> Option<Rc<FunctionReference>>;
    fn get_property(&self, name: &str) -> Option<Rc<BramaPrimative>>;
}

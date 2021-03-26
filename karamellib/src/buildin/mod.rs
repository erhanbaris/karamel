pub mod debug;
pub mod io;
pub mod num;
pub mod base_functions;

#[macro_use]
pub mod class;

use crate::compiler::GetType;

use std::collections::HashMap;
use std::vec::Vec;
use std::sync::Arc;

use crate::compiler::{BramaPrimative, function::{FunctionReference, NativeCall}};

pub trait Module {
    fn new() -> Self where Self: Sized;
    fn get_module_name(&self) -> String;
    
    fn get_method(&self, name: &str) -> Option<NativeCall>;
    fn get_module(&self, name: &str) -> Option<Arc<dyn Module>>;

    fn get_methods(&self) -> Vec<(&'static str, NativeCall)>;
    fn get_modules(&self) -> HashMap<String, Arc<dyn Module>>;

    fn get_classes(&self) -> Vec<Arc<dyn Class>>;
}

pub struct ModuleCollection {
    pub modules: HashMap<String, Arc<dyn Module>>
}

impl ModuleCollection
{
    pub fn new() -> ModuleCollection {
        let mut collection = ModuleCollection {
            modules: HashMap::new()
        };
        collection.add_module(Arc::new(base_functions::BaseFunctionsModule::new()));
        collection.add_module(Arc::new(io::IoModule::new()));
        collection.add_module(Arc::new(num::NumModule::new()));
        collection.add_module(Arc::new(debug::DebugModule::new()));
        collection
    }

    pub fn add_module(&mut self, module: Arc<dyn Module>) {        
        self.modules.insert(module.get_module_name(), module);
    }
}


pub enum ClassProperty {
    Function(Arc<FunctionReference>),
    Field(Arc<BramaPrimative>)
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
    
    fn add_method(&mut self, name: String, function: Arc<FunctionReference>);
    fn add_property(&mut self, name: String, property: Arc<BramaPrimative>);
    
    fn get_method(&self, name: &str) -> Option<Arc<FunctionReference>>;
    fn get_property(&self, name: &str) -> Option<Arc<BramaPrimative>>;
}

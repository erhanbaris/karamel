pub mod debug;
pub mod io;
pub mod num;
pub mod base_functions;

#[macro_use]
pub mod class;

use crate::{compiler::{GetType, function::{IndexerGetCall, IndexerSetCall, FunctionFlag}}, types::VmObject};

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


#[derive(Clone)]
pub enum ClassProperty {
    Function(Rc<FunctionReference>),
    Field(Rc<BramaPrimative>)
}

#[derive(Default)]
pub struct ClassConfig {
    pub name: String,
    pub storage_index: usize,
    pub properties: HashMap<String, ClassProperty>,
    pub is_readonly: bool,
    pub is_buildin: bool,
    pub is_static: bool,
    pub indexer: Indexer
}

#[derive(Default)]
pub struct Indexer {
    pub get: Option<IndexerGetCall>,
    pub set: Option<IndexerSetCall>
}


pub trait Class: GetType {
    fn set_class_config(&mut self, config: ClassConfig);
    fn get_class_name(&self) -> String;
    
    fn has_element(&self, source: Option<VmObject>, field: Rc<String>) -> bool;
    fn get_element(&self, source: Option<VmObject>, field: Rc<String>) -> Option<ClassProperty>;
    fn property_count(&self) -> usize;
    fn properties(&self) -> std::collections::hash_map::Iter<'_, String, ClassProperty>;
    
    fn add_method(&mut self, name: &str, function: NativeCall, flags: FunctionFlag);
    fn add_property(&mut self, name: &str, property: Rc<BramaPrimative>);
    
    fn set_getter(&mut self, indexer: IndexerGetCall);
    fn get_getter(&self) -> Option<IndexerGetCall>;
    
    fn set_setter(&mut self, indexer: IndexerSetCall);
    fn get_setter(&self) -> Option<IndexerSetCall>;
}

pub mod debug;
pub mod io;
pub mod num;
pub mod base_functions;

use std::collections::hash_map::Iter;

#[macro_use]
pub mod class;

use crate::{compiler::{GetType, function::{IndexerGetCall, IndexerSetCall, FunctionFlag}}, types::VmObject};

use std::collections::HashMap;
use std::vec::Vec;
use std::rc::Rc;

use crate::compiler::{BramaPrimative, function::{FunctionReference, NativeCall}};

pub trait Module {
    fn get_module_name(&self) -> String;
    
    fn get_method(&self, name: &str) -> Option<Rc<FunctionReference>>;
    fn get_module(&self, name: &str) -> Option<Rc<dyn Module>>;

    fn get_methods(&self) -> Vec<(&String, Rc<FunctionReference>)>;
    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>>;

    fn get_classes(&self) -> Vec<Rc<dyn Class>>;
}
pub struct DummyModule;

impl Module for DummyModule {
    fn get_module_name(&self) -> String { unreachable!() }
    
    fn get_method(&self, _: &str) -> Option<Rc<FunctionReference>> { unreachable!() }
    fn get_module(&self, _: &str) -> Option<Rc<dyn Module>> { unreachable!() }

    fn get_methods(&self) -> Vec<(&String, Rc<FunctionReference>)> { unreachable!() }
    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>> { unreachable!() }

    fn get_classes(&self) -> Vec<Rc<dyn Class>> { unreachable!() }
}

pub struct ModuleCollectionIterator<'a> {
    iter: Iter<'a, String, Rc<dyn Module>>
}

pub struct ModuleCollection {
    modules: HashMap<String, Rc<dyn Module>>
}

impl ModuleCollection
{
    pub fn new() -> ModuleCollection {
        ModuleCollection {
            modules: HashMap::new()
        }
    }

    pub fn add_module(&mut self, module: Rc<dyn Module>) {        
        self.modules.insert(module.get_module_name(), module);
    }

    pub fn iter(&self) -> ModuleCollectionIterator {
        ModuleCollectionIterator  { 
            iter: self.modules.iter().clone()
        }
    }

    pub fn has_module(&self, module_name: &String) -> bool {
        self.modules.contains_key(module_name)
    }
}

impl<'a> Iterator for ModuleCollectionIterator<'a> {
    type Item = (&'a String, &'a Rc<dyn Module>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
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

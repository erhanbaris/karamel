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

use crate::compiler::{KaramelPrimative, function::{FunctionReference, NativeCall}};

pub trait Module {
    fn get_module_name(&self) -> String;
    fn get_path(&self) -> &Vec<String>;
    
    fn get_method(&self, name: &str) -> Option<Rc<FunctionReference>>;
    fn get_module(&self, name: &str) -> Option<Rc<dyn Module>>;

    fn get_methods(&self) -> Vec<Rc<FunctionReference>>;
    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>>;

    fn get_classes(&self) -> Vec<Rc<dyn Class>>;
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

    pub fn has_module(&self, module_path: &Vec<String>) -> bool {
        self.modules.iter().find_map(|(key, module)| if module.get_path() == module_path { Some(key) } else { None }).is_some()
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
    Field(Rc<KaramelPrimative>)
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
    fn add_property(&mut self, name: &str, property: Rc<KaramelPrimative>);
    
    fn set_getter(&mut self, indexer: IndexerGetCall);
    fn get_getter(&self) -> Option<IndexerGetCall>;
    
    fn set_setter(&mut self, indexer: IndexerSetCall);
    fn get_setter(&self) -> Option<IndexerSetCall>;
}

pub struct DummyModule {
    name: String,
    path: Vec<String>
}

impl DummyModule {
    pub fn new() -> Self {
        DummyModule {
            name: "!dummy".to_string(),
            path: vec!["!dummy".to_string()]
        }
    }
}

impl Module for DummyModule {
    fn get_module_name(&self) -> String { self.name.to_string() }
    fn get_path(&self) -> &Vec<String> { &self.path }
    
    fn get_method(&self, _: &str) -> Option<Rc<FunctionReference>> { None }
    fn get_module(&self, _: &str) -> Option<Rc<dyn Module>> { None }

    fn get_methods(&self) -> Vec<Rc<FunctionReference>> { Vec::new() }
    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>> { HashMap::new() }

    fn get_classes(&self) -> Vec<Rc<dyn Class>> { Vec::new() }
}

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

pub trait Module<'a> {
    fn get_module_name(&self) -> String;
    fn get_path(&self) -> &Vec<String>;
    
    fn get_method(&self, name: &str) -> Option<Rc<FunctionReference<'a>>>;
    fn get_module(&self, name: &str) -> Option<Rc<dyn Module<'a> + 'a>>;

    fn get_methods(&self) -> Vec<Rc<FunctionReference<'a>>>;
    fn get_modules(&self) -> HashMap<String, Rc<dyn Module<'a> + 'a>>;

    fn get_classes(&self) -> Vec<Rc<dyn Class<'a> + 'a>>;
}

pub struct ModuleCollectionIterator<'iter, 'a> {
    iter: Iter<'iter, String, Rc<dyn Module<'a> + 'a>>
}

pub struct ModuleCollection<'a> {
    modules: HashMap<String, Rc<dyn Module<'a> + 'a>>
}

impl<'a> ModuleCollection<'a>
{
    pub fn new() -> Self {
        ModuleCollection {
            modules: HashMap::new()
        }
    }

    pub fn add_module(&mut self, module: Rc<dyn Module<'a> + 'a>) {        
        self.modules.insert(module.get_module_name(), module);
    }

    pub fn iter(&self) -> ModuleCollectionIterator<'_, 'a> {
        let iter: Iter<'_, String, Rc<dyn Module<'a> + 'a>> = self.modules.iter().clone();

        ModuleCollectionIterator { iter }
    }

    pub fn has_module(&self, module_path: &Vec<String>) -> bool {
        self.modules.iter().find_map(|(key, module)| if module.get_path() == module_path { Some(key) } else { None }).is_some()
    }
}

impl<'a, 'b> Iterator for ModuleCollectionIterator<'a, 'b> {
    type Item = (&'a String, &'a Rc<dyn Module<'b> + 'b>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[derive(Clone)]
pub enum ClassProperty<'a> {
    Function(Rc<FunctionReference<'a>>),
    Field(Rc<KaramelPrimative<'a>>)
}

#[derive(Default)]
pub struct ClassConfig<'a> {
    pub name: String,
    pub storage_index: usize,
    pub properties: HashMap<String, ClassProperty<'a>>,
    pub is_readonly: bool,
    pub is_buildin: bool,
    pub is_static: bool,
    pub indexer: Indexer
}

impl<'a> ClassConfig<'a> {
    pub fn empty() -> Self {
        ClassConfig {
            name: String::new(),
            storage_index: 0,
            properties: HashMap::new(),
            is_readonly: false,
            is_buildin: false,
            is_static: false,
            indexer: Indexer {
                get: None,
                set: None
            }
        }
    }
}

#[derive(Default)]
pub struct Indexer {
    pub get: Option<IndexerGetCall>,
    pub set: Option<IndexerSetCall>
}


pub trait Class<'a>: GetType<'a> {
    fn set_class_config(&mut self, config: ClassConfig<'a>);
    fn get_class_name(&self) -> String;
    
    fn has_element(&self, source: Option<VmObject>, field: Rc<String>) -> bool;
    fn get_element(&self, source: Option<VmObject>, field: Rc<String>) -> Option<ClassProperty<'a>>;
    fn property_count(&self) -> usize;
    fn properties(&self) -> std::collections::hash_map::Iter<'_, String, ClassProperty<'a>>;
    
    fn add_method(&mut self, name: &str, function: NativeCall, flags: FunctionFlag);
    fn add_property(&mut self, name: &str, property: Rc<KaramelPrimative<'a>>);
    
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

impl<'a> Module<'a> for DummyModule {
    fn get_module_name(&self) -> String { self.name.to_string() }
    fn get_path(&self) -> &Vec<String> { &self.path }
    
    fn get_method(&self, _: &str) -> Option<Rc<FunctionReference<'a>>> { None }
    fn get_module(&self, _: &str) -> Option<Rc<dyn Module<'a> + 'a>> { None }

    fn get_methods(&self) -> Vec<Rc<FunctionReference<'a>>> { Vec::new() }
    fn get_modules(&self) -> HashMap<String, Rc<dyn Module<'a> + 'a>> { HashMap::new() }

    fn get_classes(&self) -> Vec<Rc<dyn Class<'a> + 'a>> { Vec::new() }
}

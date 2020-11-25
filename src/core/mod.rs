pub mod buildin;

use std::collections::HashMap;
use std::vec::Vec;

use crate::compiler::StaticStorage;
use crate::compiler::value::BramaPrimative;

pub type NativeCallResult = Result<(), (&'static str, u32, u32)>;
pub type NativeCall       = fn(params: Vec<BramaPrimative>, storage: &mut StaticStorage) -> NativeCallResult;

pub trait Module {
    fn get_module_name(&self) -> String;
    fn get_methods(&self) -> Vec<(&'static str, NativeCall)>;
}

pub struct ModuleCollection {
    modules: HashMap<String, HashMap<String, NativeCall>>
}

impl ModuleCollection
{
    pub fn new() -> ModuleCollection {
        let mut collection = ModuleCollection {
            modules: HashMap::new()
        };
        collection.add_module(&buildin::BuildinModule {});
        collection
    }

    pub fn add_module(&mut self, module: &dyn Module) {
        let mut module_functions: HashMap<String, NativeCall> = HashMap::new();
        
        for (name, func) in module.get_methods() {
            module_functions.insert(name.to_string(), func);
        }
        
        self.modules.insert(module.get_module_name(), module_functions);
    }

    pub fn has_module(&self, module: String) -> bool {
        self.modules.contains_key(&module)
    }

    pub fn get_function(&self, module: String, func_name: String) -> Option<NativeCall> {
        match self.modules.get(&module) {
            Some(module) => {
                match module.get(&func_name) {
                    Some(func) => Some(*func),
                    None => None
                }
            },
            None => None
        }
    }
}
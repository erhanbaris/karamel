pub mod buildin;

use std::collections::HashMap;

use crate::types::BramaPrimative;
use crate::compiler::Storage;

pub type NativeCallResult = Result<(), (&'static str, u32, u32)>;
pub type NativeCall<T> where T: Storage= fn(params: Vec<BramaPrimative>, storage: &mut T) -> NativeCallResult;

pub trait Module<T: Storage> {
    fn get_module_name(&self) -> String;
    fn get_methods(&self) -> Vec<(&'static str, NativeCall<T>)>;
}

pub struct ModuleCollection<T: Storage> {
    modules: HashMap<String, HashMap<String, NativeCall<T>>>
}

impl<T: Storage> ModuleCollection<T>
{
    pub fn new() -> ModuleCollection<T> {
        ModuleCollection {
            modules: HashMap::new()
        }
    }

    pub fn add_module(&mut self, module: &dyn Module<T>) {
        let mut module_functions: HashMap<String, NativeCall<T>> = HashMap::new();
        
        for (name, func) in module.get_methods() {
            module_functions.insert(name.to_string(), func);
        }
        
        self.modules.insert(module.get_module_name(), module_functions);
    }

    pub fn has_module(&self, module: String) -> bool {
        self.modules.contains_key(&module)
    }

    pub fn get_function(&self, module: String, func_name: String) -> Option<NativeCall<T>> {
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
pub mod buildin;

use std::collections::HashMap;
use std::vec::Vec;
use std::rc::Rc;

use crate::compiler::value::NativeCall;
use crate::compiler::value::NativeCallResult;

pub trait Module {
    fn new() -> Self where Self: Sized;
    fn get_module_name(&self) -> String;
    
    fn get_method(&self, name: &String) -> Option<NativeCall>;
    fn get_module(&self, name: &String) -> Option<Rc<dyn Module>>;

    fn get_methods(&self) -> Vec<(&'static str, NativeCall)>;
    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>>;
}

pub struct ModuleCollection {
    modules: HashMap<String, Rc<dyn Module>>
}

impl ModuleCollection
{
    pub fn new() -> ModuleCollection {
        let mut collection = ModuleCollection {
            modules: HashMap::new()
        };
        collection.add_module(Rc::new(buildin::IoModule::new()));
        collection.add_module(Rc::new(buildin::NumModule::new()));
        collection
    }

    pub fn add_module(&mut self, module: Rc<dyn Module>) {        
        self.modules.insert(module.get_module_name(), module);
    }

    pub fn find_method(&self, names: &Vec<String>) -> Option<NativeCall> {
        let mut index = 0;
        let mut modules   = self.modules.clone();

        loop {
            if let Some(next_name) = names.get(index + 1) {
                let name = &names[index];
                match modules.get(name) {
                    Some(module) => {
                        if let Some(method) = module.get_method(next_name) {
                            return Some(method);
                        }

                        else if let Some(new_module) = module.get_module(next_name) {
                            modules = new_module.get_modules();
                            index +=1;
                        }

                        else {
                            return None;
                        }
                    },
                    None => return None
                }
            }
        }
    }
}
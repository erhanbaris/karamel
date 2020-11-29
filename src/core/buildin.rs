use crate::{core::{Module, NativeCall, NativeCallResult}};
use crate::types::VmObject;
use crate::compiler::value::EMPTY_OBJECT;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct IoModule {
    methods: HashMap<String, NativeCall>
}

impl Module for IoModule {
    fn new() -> IoModule where Self: Sized {
        let mut module = IoModule {
            methods: HashMap::new()
        };
        module.methods.insert("print".to_string(), Self::print as NativeCall);
        module.methods.insert("printline".to_string(), Self::printline as NativeCall);
        module
    }

    fn get_module_name(&self) -> String {
        return "io".to_string();
    }

    fn get_method(&self, name: &String) -> Option<NativeCall> {
        match self.methods.get(name) {
            Some(method) => Some(*method),
            None => None
        }
    }

    fn get_module(&self, _: &String) -> Option<Rc<dyn Module>> {
        None
    }

    fn get_methods(&self) -> Vec<(&'static str, NativeCall)> {
        [("print", Self::print as NativeCall),
        ("printline", Self::printline as NativeCall)].to_vec()
    }

    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>> {
        HashMap::new()
    }
}

impl IoModule  {
    pub fn print(arguments: &Vec<VmObject>) -> NativeCallResult {
        for arg in arguments {
            print!("{:?}", arg.deref());
        }

        Ok(EMPTY_OBJECT)
    }
    
    pub fn printline(arguments: &Vec<VmObject>) -> NativeCallResult {
        for arg in arguments {
            println!("{:?}", arg.deref());
        }

        Ok(EMPTY_OBJECT)
    }
}
use crate::{core::{Module, NativeCall, NativeCallResult}};
use crate::types::VmObject;
use crate::compiler::value::BramaPrimative;
use crate::compiler::value::EMPTY_OBJECT;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct IoModule {
    methods: HashMap<String, NativeCall>
}

pub struct NumModule {
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
    pub fn print(arguments: &Vec<VmObject>, last_position: usize, total_args: u8) -> NativeCallResult {
        for arg in arguments {
            print!("{:?}", arg.deref());
        }

        Ok(EMPTY_OBJECT)
    }
    
    pub fn printline(arguments: &Vec<VmObject>, last_position: usize, total_args: u8) -> NativeCallResult {
        for arg in arguments.iter().skip(last_position as usize - (total_args as usize - 1)).take(total_args as usize) {
            println!("{:?}", arg.deref());
        }

        Ok(EMPTY_OBJECT)
    }
}



impl Module for NumModule {
    fn new() -> NumModule where Self: Sized {
        let mut module = NumModule {
            methods: HashMap::new()
        };
        module.methods.insert("parse".to_string(), Self::parse as NativeCall);
        module
    }

    fn get_module_name(&self) -> String {
        return "num".to_string();
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
        [("parse", Self::parse as NativeCall)].to_vec()
    }

    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>> {
        HashMap::new()
    }
}

impl NumModule  {
    pub fn parse(arguments: &Vec<VmObject>, last_position: usize, total_args: u8) -> NativeCallResult {
        if total_args > 1 {
            return Err(("More than 1 argument passed", 0, 0));
        }

        let arg = arguments[last_position].deref();

        return match &*arg {
            BramaPrimative::Number(_) => Ok(arguments[last_position]),
            BramaPrimative::Text(text) => {
                match (*text).parse() {
                    Ok(num) => Ok(VmObject::native_convert(BramaPrimative::Number(num))),
                    _ => Err(("More than 1 argument passed", 0, 0))
                }
            },
            _ => Ok(EMPTY_OBJECT)
        };
    }
}
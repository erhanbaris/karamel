use crate::compiler::{function::{FunctionParameter, NativeCall, NativeCallResult}};
use crate::types::{VmObject};
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::{Module, Class};
use std::collections::HashMap;
use std::rc::Rc;
use std::io;

use log;


#[derive(Clone)]
pub struct IoModule {
    methods: HashMap<String, NativeCall>
}

impl Module for IoModule {
    fn new() -> IoModule where Self: Sized {
        let mut module = IoModule {
            methods: HashMap::new()
        };
        module.methods.insert("satıroku".to_string(), Self::readline as NativeCall);
        module.methods.insert("yaz".to_string(), Self::print as NativeCall);
        module.methods.insert("satıryaz".to_string(), Self::printline as NativeCall);
        module
    }

    fn get_module_name(&self) -> String {
        "gç".to_string()
    }

    fn get_method(&self, name: &str) -> Option<NativeCall> {
        self.methods.get(name).map(|method| *method)
    }

    fn get_module(&self, _: &str) -> Option<Rc<dyn Module>> {
        None
    }

    fn get_methods(&self) -> Vec<(&'static str, NativeCall)> {
        [("satıroku", Self::readline as NativeCall),
         ("yaz", Self::print as NativeCall),
         ("satıryaz", Self::printline as NativeCall)].to_vec()
    }

    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>> {
        HashMap::new()
    }

    fn get_classes(&self) -> Vec<Rc<dyn Class>> {
        Vec::new()
    }
}

impl IoModule  {
    pub fn readline(_: FunctionParameter) -> NativeCallResult {        
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => return Ok(VmObject::from(Rc::new(line.trim().to_string()))),
            _ => Ok(EMPTY_OBJECT)
        }
    }

    pub fn print(parameter: FunctionParameter) -> NativeCallResult {
        let mut buffer = String::new();
        for arg in parameter.iter() {
            buffer.push_str(&format!("{}", arg.deref()));
        }
        log::info!("{}", buffer);
                
        parameter.write_to_stdout(&buffer);
        Ok(EMPTY_OBJECT)
    }
    
    pub fn printline(parameter: FunctionParameter) -> NativeCallResult {
        let mut buffer = String::new();

        for arg in parameter.iter() {
            buffer.push_str(&format!("{}", arg.deref()));
        }

        buffer.push_str(&"\r\n");
        log::info!("{}", buffer);

        parameter.write_to_stdout(&buffer);
        Ok(EMPTY_OBJECT)
    }
}

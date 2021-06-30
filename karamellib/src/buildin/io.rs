use crate::compiler::{function::{FunctionParameter, FunctionReference, NativeCall, NativeCallResult}};
use crate::types::{VmObject};
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::{Module, Class};
use std::{cell::RefCell, collections::HashMap};
use std::rc::Rc;
use std::io;

use log;


#[derive(Clone)]
pub struct IoModule {
    methods: RefCell<HashMap<String, Rc<FunctionReference>>>,
    path: Vec<String>
}

impl Module for IoModule {
    fn get_module_name(&self) -> String {
        "gç".to_string()
    }

    fn get_path(&self) -> &Vec<String> {
        &self.path
    }

    fn get_method(&self, name: &str) -> Option<Rc<FunctionReference>> {
        self.methods.borrow().get(name).map(|method| method.clone())
    }

    fn get_module(&self, _: &str) -> Option<Rc<dyn Module>> {
        None
    }

    fn get_methods(&self) -> Vec<Rc<FunctionReference>> {
        let mut response = Vec::new();
        self.methods.borrow().iter().for_each(|(_, reference)| response.push(reference.clone()));
        response
    }

    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>> {
        HashMap::new()
    }

    fn get_classes(&self) -> Vec<Rc<dyn Class>> {
        Vec::new()
    }
}

impl IoModule  {
    pub fn new() -> Rc<IoModule> {
        let module = IoModule {
            methods: RefCell::new(HashMap::new()),
            path: vec!["gç".to_string()]
        };

        let rc_module = Rc::new(module);
        rc_module.methods.borrow_mut().insert("satıroku".to_string(), FunctionReference::native_function(Self::readline as NativeCall, "satıroku".to_string(), rc_module.clone()));
        rc_module.methods.borrow_mut().insert("satiroku".to_string(), FunctionReference::native_function(Self::readline as NativeCall, "satiroku".to_string(), rc_module.clone()));
        rc_module.methods.borrow_mut().insert("yaz".to_string(), FunctionReference::native_function(Self::print as NativeCall, "yaz".to_string(), rc_module.clone()));
        rc_module.methods.borrow_mut().insert("satıryaz".to_string(), FunctionReference::native_function(Self::printline as NativeCall, "satıryaz".to_string(), rc_module.clone()));
        rc_module.methods.borrow_mut().insert("satiryaz".to_string(), FunctionReference::native_function(Self::printline as NativeCall, "satiryaz".to_string(), rc_module.clone()));
        rc_module.methods.borrow_mut().insert("biçimlendir".to_string(), FunctionReference::native_function(Self::format as NativeCall, "biçimlendir".to_string(), rc_module.clone()));
        rc_module.methods.borrow_mut().insert("bicimlendir".to_string(), FunctionReference::native_function(Self::format as NativeCall, "bicimlendir".to_string(), rc_module.clone()));
        rc_module.clone()
    }

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
    
    pub fn format(parameter: FunctionParameter) -> NativeCallResult {
        if parameter.length() != 1 {
            return Ok(EMPTY_OBJECT);
        }

        Ok(VmObject::from(Rc::new(format!("{}", parameter.iter().next().unwrap().deref()))))
    }
}

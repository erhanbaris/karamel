use crate::compiler::{function::{FunctionParameter, FunctionReference, NativeCall, NativeCallResult}};
use crate::types::VmObject;
use crate::compiler::value::KaramelPrimative;
use crate::compiler::value::EMPTY_OBJECT;
use crate::error::KaramelErrorType;
use crate::buildin::{Module, Class};
use crate::{n_parameter_expected, expected_parameter_type};
use std::{cell::RefCell, collections::HashMap};
use std::rc::Rc;

pub struct NumModule {
    methods: RefCell<HashMap<String, Rc<FunctionReference>>>,
    path: Vec<String>
}

impl Module for NumModule {
    fn get_module_name(&self) -> String {
        "sayı".to_string()
    }

    fn get_path(&self) -> &Vec<String> {
        &self.path
    }

    fn get_method(&self, name: &str) -> Option<Rc<FunctionReference>> {
        match self.methods.borrow().get(name) {
            Some(method) => Some(method.clone()),
            None => None
        }
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

impl NumModule {
    pub fn new() -> Rc<NumModule> {
        let module = NumModule {
            methods: RefCell::new(HashMap::new()),
            path: vec!["sayı".to_string()]
        };

        let rc_module = Rc::new(module);
        rc_module.methods.borrow_mut().insert("oku".to_string(), FunctionReference::native_function(Self::parse as NativeCall, "tür_bilgisi".to_string(), rc_module.clone()));
        rc_module.clone()
    }

    pub fn parse(parameter: FunctionParameter) -> NativeCallResult {
        if parameter.length() > 1 {
            return n_parameter_expected!("oku".to_string(), 1);
        }

        let arg = match parameter.iter().next() {
            Some(arg) => arg.deref(),
            None => return Ok(EMPTY_OBJECT)
        };

        match &*arg {
            KaramelPrimative::Number(_) => Ok(*parameter.iter().next().unwrap()),
            KaramelPrimative::Text(text) => {
                match (*text).parse::<f64>() {
                    Ok(num) => Ok(VmObject::from(num)),
                    _ => expected_parameter_type!("oku".to_string(), "Yazı".to_string())
                }
            },
            _ => Ok(EMPTY_OBJECT)
        }
    }
}

use crate::compiler::{function::{FunctionParameter, FunctionReference, NativeCall, NativeCallResult}};
use crate::types::VmObject;
use crate::compiler::value::BramaPrimative;
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::{Module, Class};
use crate::{n_parameter_expected, expected_parameter_type};
use std::collections::HashMap;
use std::rc::Rc;

pub struct NumModule {
    methods: HashMap<String, Rc<FunctionReference>>
}

impl Module for NumModule {
    fn get_module_name(&self) -> String {
        "sayı".to_string()
    }

    fn get_method(&self, name: &str) -> Option<Rc<FunctionReference>> {
        match self.methods.get(name) {
            Some(method) => Some(method.clone()),
            None => None
        }
    }

    fn get_module(&self, _: &str) -> Option<Rc<dyn Module>> {
        None
    }

    fn get_methods(&self) -> Vec<(&String, Rc<FunctionReference>)> {
        self.methods.iter().map(|(key, value)| (key, value.clone())).collect::<Vec<(&String, Rc<FunctionReference>)>>()
    }

    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>> {
        HashMap::new()
    }
    
    fn get_classes(&self) -> Vec<Rc<dyn Class>> {
        Vec::new()
    }
}

impl NumModule {
    pub fn new() -> NumModule where Self: Sized {
        let mut module = NumModule {
            methods: HashMap::new()
        };
        module.methods.insert("oku".to_string(), FunctionReference::native_function(Self::parse as NativeCall, "tür_bilgisi".to_string(), [module.get_module_name()].to_vec()));
        module
    }

    pub fn parse(parameter: FunctionParameter) -> NativeCallResult {
        if parameter.length() > 1 {
            return n_parameter_expected!("oku", 1);
        }

        let arg = match parameter.iter().next() {
            Some(arg) => arg.deref(),
            None => return Ok(EMPTY_OBJECT)
        };

        match &*arg {
            BramaPrimative::Number(_) => Ok(*parameter.iter().next().unwrap()),
            BramaPrimative::Text(text) => {
                match (*text).parse::<f64>() {
                    Ok(num) => Ok(VmObject::from(num)),
                    _ => expected_parameter_type!("oku", "Yazı")
                }
            },
            _ => Ok(EMPTY_OBJECT)
        }
    }
}

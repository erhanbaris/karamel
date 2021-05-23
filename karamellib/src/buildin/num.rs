use crate::compiler::{function::{FunctionParameter, NativeCall, NativeCallResult}};
use crate::types::VmObject;
use crate::compiler::value::BramaPrimative;
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::{Module, Class};
use crate::{n_parameter_expected, expected_parameter_type};
use std::collections::HashMap;
use std::rc::Rc;

pub struct NumModule {
    methods: HashMap<String, NativeCall>
}

impl Module for NumModule {
    fn new() -> NumModule where Self: Sized {
        let mut module = NumModule {
            methods: HashMap::new()
        };
        module.methods.insert("oku".to_string(), Self::parse as NativeCall);
        module
    }

    fn get_module_name(&self) -> String {
        "sayı".to_string()
    }

    fn get_method(&self, name: &str) -> Option<NativeCall> {
        self.methods.get(name).map(|method| *method)
    }

    fn get_module(&self, _: &str) -> Option<Rc<dyn Module>> {
        None
    }

    fn get_methods(&self) -> Vec<(&'static str, NativeCall)> {
        [("oku", Self::parse as NativeCall)].to_vec()
    }

    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>> {
        HashMap::new()
    }
    
    fn get_classes(&self) -> Vec<Rc<dyn Class>> {
        Vec::new()
    }
}

impl NumModule  {
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

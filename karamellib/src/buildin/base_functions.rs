use crate::compiler::{EMPTY_OBJECT, function::{FunctionParameter, NativeCall, NativeCallResult}};
use crate::types::VmObject;
use crate::buildin::{Module, Class};
use crate::compiler::GetType;
use crate::{n_parameter_expected};
use std::collections::HashMap;
use std::rc::Rc;


#[derive(Clone)]
pub struct BaseFunctionsModule {
    methods: HashMap<String, NativeCall>
}

impl Module for BaseFunctionsModule {
    fn new() -> BaseFunctionsModule where Self: Sized {
        let mut module = BaseFunctionsModule {
            methods: HashMap::new()
        };
        module.methods.insert("tür_bilgisi".to_string(), Self::type_info as NativeCall);
        module
    }

    fn get_module_name(&self) -> String {
        "".to_string()
    }

    fn get_method(&self, name: &str) -> Option<NativeCall> {
        match self.methods.get(name) {
            Some(method) => Some(*method),
            None => None
        }
    }

    fn get_module(&self, _: &str) -> Option<Rc<dyn Module>> {
        None
    }

    fn get_methods(&self) -> Vec<(&'static str, NativeCall)> {
        [("tür_bilgisi", Self::type_info as NativeCall)].to_vec()
    }

    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>> {
        HashMap::new()
    }

    fn get_classes(&self) -> Vec<Rc<dyn Class>> {
        Vec::new()
    }
}

impl BaseFunctionsModule  {
    pub fn type_info(parameter: FunctionParameter) -> NativeCallResult {        
        if parameter.length() > 1 {
            return n_parameter_expected!("tür_bilgisi", 1);
        }

        match parameter.iter().next() {
            Some(arg) => Ok(VmObject::from(Rc::new(arg.deref().get_type()))),
            None => Ok(EMPTY_OBJECT)
        }
    }
}

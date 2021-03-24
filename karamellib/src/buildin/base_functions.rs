use crate::compiler::{EMPTY_OBJECT, function::{FunctionParameter, NativeCall, NativeCallResult}};
use crate::types::VmObject;
use crate::buildin::{Module, Class};
use crate::compiler::GetType;
use std::collections::HashMap;
use std::sync::Arc;


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

    fn get_module(&self, _: &str) -> Option<Arc<dyn Module>> {
        None
    }

    fn get_methods(&self) -> Vec<(&'static str, NativeCall)> {
        [("tür_bilgisi", Self::type_info as NativeCall)].to_vec()
    }

    fn get_modules(&self) -> HashMap<String, Arc<dyn Module>> {
        HashMap::new()
    }

    fn get_classes(&self) -> Vec<Arc<dyn Class>> {
        Vec::new()
    }
}

impl BaseFunctionsModule  {
    pub fn type_info(parameter: FunctionParameter) -> NativeCallResult {        
        if parameter.length() > 1 {
            return Err(("More than 1 argument passed".to_string(), 0, 0));
        }

        match parameter.iter().next() {
            Some(arg) => Ok(VmObject::from(Arc::new(arg.deref().get_type()))),
            None => Ok(EMPTY_OBJECT)
        }
    }
}

use crate::compiler::function::{NativeCallResult, NativeCall};
use crate::types::VmObject;
use crate::buildin::{Module, Class};
use crate::compiler::GetType;
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
    pub fn type_info(arguments: &Vec<VmObject>, last_position: usize, total_args: u8) -> NativeCallResult {        
        if total_args > 1 {
            return Err(("More than 1 argument passed".to_string(), 0, 0));
        }

        let arg = arguments[last_position - 1].deref();
        Ok(VmObject::from(Rc::new(arg.get_type())))
    }
}
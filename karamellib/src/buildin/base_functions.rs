use crate::compiler::{EMPTY_OBJECT, function::{FunctionParameter, FunctionReference, NativeCall, NativeCallResult}};
use crate::types::VmObject;
use crate::buildin::{Module, Class};
use crate::compiler::GetType;
use crate::{n_parameter_expected};
use std::collections::HashMap;
use std::rc::Rc;


#[derive(Clone)]
pub struct BaseFunctionsModule {
    methods: HashMap<String, Rc<FunctionReference>>,
    path: Vec<String>
}

impl Module for BaseFunctionsModule {
    fn get_module_name(&self) -> String {
        "baz".to_string()
    }

    fn get_path(&self) -> &Vec<String> {
        &self.path
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

impl BaseFunctionsModule  {
    pub fn new() -> BaseFunctionsModule where Self: Sized {
        let mut module = BaseFunctionsModule {
            methods: HashMap::new(),
            path: vec!["baz".to_string()]
        };
        module.methods.insert("tür_bilgisi".to_string(), FunctionReference::native_function(Self::type_info as NativeCall, "tür_bilgisi".to_string(), [module.get_module_name()].to_vec()));
        module
    }

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

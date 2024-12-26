use crate::buildin::{Class, Module};
use crate::compiler::GetType;
use crate::types::VmObject;
use crate::{
    compiler::{
        function::{FunctionParameter, FunctionReference, NativeCall, NativeCallResult},
        EMPTY_OBJECT,
    },
    n_parameter_expected,
};
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

#[derive(Clone)]
pub struct BaseFunctionsModule {
    methods: RefCell<HashMap<String, Rc<FunctionReference>>>,
    path: Vec<String>,
}

impl Module for BaseFunctionsModule {
    fn get_module_name(&self) -> String {
        "baz".to_string()
    }

    fn get_path(&self) -> &Vec<String> {
        &self.path
    }

    fn get_method(&self, name: &str) -> Option<Rc<FunctionReference>> {
        self.methods.borrow().get(name).cloned()
    }

    fn get_module(&self, _: &str) -> Option<Rc<dyn Module>> {
        None
    }

    fn get_methods(&self) -> Vec<Rc<FunctionReference>> {
        let mut response = Vec::new();
        self.methods
            .borrow()
            .iter()
            .for_each(|(_, reference)| response.push(reference.clone()));
        response
    }

    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>> {
        HashMap::new()
    }

    fn get_classes(&self) -> Vec<Rc<dyn Class>> {
        Vec::new()
    }
}

impl BaseFunctionsModule {
    pub fn new() -> Rc<BaseFunctionsModule> {
        let module = BaseFunctionsModule {
            methods: RefCell::new(HashMap::new()),
            path: vec!["baz".to_string()],
        };

        let rc_module = Rc::new(module);
        rc_module
            .methods
            .borrow_mut()
            .insert("tür_bilgisi".to_string(), FunctionReference::native_function(Self::type_info as NativeCall, "tür_bilgisi".to_string(), rc_module.clone()));
        rc_module
    }

    pub fn type_info(parameter: FunctionParameter) -> NativeCallResult {
        if parameter.length() > 1 {
            return n_parameter_expected!("tür_bilgisi".to_string(), 1);
        }

        match parameter.iter().next() {
            Some(arg) => Ok(VmObject::from(Rc::new(arg.deref().get_type()))),
            None => Ok(EMPTY_OBJECT),
        }
    }
}

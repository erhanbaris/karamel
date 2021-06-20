use crate::buildin::{Module, Class};
use crate::compiler::function::{FunctionReference, NativeCall, NativeCallResult};
use crate::compiler::function::FunctionParameter;
use crate::compiler::value::EMPTY_OBJECT;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct DebugModule {
    methods: HashMap<String, Rc<FunctionReference>>
}

impl Module for DebugModule {
    fn get_module_name(&self) -> String {
        "hataayıklama".to_string()
    }

    fn get_method(&self, name: &str) -> Option<Rc<FunctionReference>> {
        self.methods.get(name).map(|method| method.clone())
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

impl DebugModule  {
    pub fn new() -> DebugModule where Self: Sized {
        let mut module = DebugModule {
            methods: HashMap::new()
        };

        module.methods.insert("doğrula".to_string(), FunctionReference::native_function(Self::assert as NativeCall, "doğrula".to_string(), [module.get_module_name()].to_vec()));
        module
    }

    pub fn assert(parameter: FunctionParameter) -> NativeCallResult {
        match parameter.length() {
            1 => {
                match parameter.iter().next().unwrap().deref().is_true() {
                    false => Err("Assert failed".to_string()),
                    true  => Ok(EMPTY_OBJECT)
                }
            },
            2 => {
                let mut iter = parameter.iter();
                let left = iter.next().unwrap().deref();
                let right = iter.next().unwrap().deref();
                match left == right {
                    false => Err(format!("Assert failed (left: {:?}, right: {:?})", left, right)),
                    true  => Ok(EMPTY_OBJECT)
                }
            },
            _ => Err("Assert failed".to_string())
        }
    }
}
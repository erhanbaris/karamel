use crate::buildin::{Module, Class};
use crate::compiler::function::{NativeCall, NativeCallResult};
use crate::types::VmObject;
use crate::compiler::value::EMPTY_OBJECT;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct DebugModule {
    methods: HashMap<String, NativeCall>
}

impl Module for DebugModule {
    fn new() -> DebugModule where Self: Sized {
        let mut module = DebugModule {
            methods: HashMap::new()
        };
        module.methods.insert("doğrula".to_string(), Self::assert as NativeCall);
        module
    }

    fn get_module_name(&self) -> String {
        "hataayıklama".to_string()
    }

    fn get_method(&self, name: &str) -> Option<NativeCall> {
        self.methods.get(name).map(|method| *method)
    }

    fn get_module(&self, _: &str) -> Option<Rc<dyn Module>> {
        None
    }

    fn get_methods(&self) -> Vec<(&'static str, NativeCall)> {
        [("doğrula", Self::assert as NativeCall)].to_vec()
    }

    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>> {
        HashMap::new()
    }

    fn get_classes(&self) -> Vec<Rc<dyn Class>> {
        Vec::new()
    }
}

impl DebugModule  {
    pub fn assert(arguments: &Vec<VmObject>, last_position: usize, total_args: u8) -> NativeCallResult {
        match total_args {
            1 => {
                match arguments[last_position - 1].deref().is_true() {
                    false => Err(("Assert failed".to_string(), 0, 0)),
                    true  => Ok(EMPTY_OBJECT)
                }
            },
            2 => {
                let status = arguments[last_position - 2].deref() == arguments[last_position - 1].deref();
                match status {
                    false => Err((format!("Assert failed (left: {:?}, right: {:?})", arguments[last_position - 2].deref(), arguments[last_position - 1].deref()), 0, 0)),
                    true  => Ok(EMPTY_OBJECT)
                }
            },
            _ => Err(("Assert failed".to_string(), 0, 0))
        }
    }
}
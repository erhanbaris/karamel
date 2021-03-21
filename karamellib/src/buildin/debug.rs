use crate::{buildin::{Module, Class}, compiler::{BramaCompiler, BramaPrimative}};
use crate::compiler::function::{NativeCall, NativeCallResult};
use crate::compiler::value::EMPTY_OBJECT;
use std::collections::HashMap;
use std::sync::Arc;

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

    fn get_module(&self, _: &str) -> Option<Arc<dyn Module>> {
        None
    }

    fn get_methods(&self) -> Vec<(&'static str, NativeCall)> {
        [("doğrula", Self::assert as NativeCall)].to_vec()
    }

    fn get_modules(&self) -> HashMap<String, Arc<dyn Module>> {
        HashMap::new()
    }

    fn get_classes(&self) -> Vec<Arc<dyn Class>> {
        Vec::new()
    }
}

impl DebugModule  {
    pub fn assert(compiler: &mut BramaCompiler, _: Option<Arc<BramaPrimative>>, last_position: usize, total_args: u8) -> NativeCallResult {
        unsafe {
            match total_args {
                1 => {
                    match (*compiler.current_scope).stack[last_position - 1].deref().is_true() {
                        false => Err(("Assert failed".to_string(), 0, 0)),
                        true  => Ok(EMPTY_OBJECT)
                    }
                },
                2 => {
                    let status = (*compiler.current_scope).stack[last_position - 2].deref() == (*compiler.current_scope).stack[last_position - 1].deref();
                    match status {
                        false => Err((format!("Assert failed (left: {:?}, right: {:?})", (*compiler.current_scope).stack[last_position - 2].deref(), (*compiler.current_scope).stack[last_position - 1].deref()), 0, 0)),
                        true  => Ok(EMPTY_OBJECT)
                    }
                },
                _ => Err(("Assert failed".to_string(), 0, 0))
            }
        }
    }
}
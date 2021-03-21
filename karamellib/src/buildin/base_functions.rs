use crate::compiler::{BramaCompiler, BramaPrimative, function::{NativeCallResult, NativeCall}};
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
    pub fn type_info(compiler: &mut BramaCompiler, _: Option<Arc<BramaPrimative>>, last_position: usize, total_args: u8) -> NativeCallResult {        
        if total_args > 1 {
            return Err(("More than 1 argument passed".to_string(), 0, 0));
        }

        let arg = unsafe {(*compiler.current_scope).stack[last_position - 1].deref()};
        Ok(VmObject::from(Arc::new(arg.get_type())))
    }
}

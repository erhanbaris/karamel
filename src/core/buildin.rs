use crate::core::{Module, NativeCall, NativeCallResult};
use crate::types::BramaPrimative;
use crate::compiler::Storage;

pub struct BuildinModule;

impl<T: Storage> Module<T> for BuildinModule {

    fn get_module_name(&self) -> String {
        return "buildin".to_string();
    }

    fn get_methods(&self) -> Vec<(&'static str, NativeCall<T>)> {
        [("print", Self::print as NativeCall<T>)].to_vec()
    }
}

impl BuildinModule  {
    pub fn print<T: Storage>(params: Vec<BramaPrimative>, storage: &mut T) -> NativeCallResult {
        storage.set_variable_value(&"erhan".to_string(), BramaPrimative::Number(1024.0).to_object());
        Ok(())
    }
}
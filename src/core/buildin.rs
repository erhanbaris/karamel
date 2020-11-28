use crate::{compiler::StaticStorage, compiler::Storage, core::{Module, NativeCall, NativeCallResult}};
use crate::compiler::BramaPrimative;
use crate::types::VmObject;
use std::rc::Rc;

pub struct BuildinModule;

impl Module for BuildinModule {

    fn get_module_name(&self) -> String {
        return "buildin".to_string();
    }

    fn get_methods(&self) -> Vec<(&'static str, NativeCall)> {
        [("print", Self::print as NativeCall)].to_vec()
    }
}

impl BuildinModule  {
    pub fn print(_: Vec<BramaPrimative>, storage: &mut StaticStorage) -> NativeCallResult {
        storage.set_variable_value(&"erhan".to_string(), VmObject::convert(Rc::new(BramaPrimative::Number(1234.0))));
        Ok(())
    }
}
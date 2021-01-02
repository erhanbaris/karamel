use crate::compiler::function::{NativeCallResult, NativeCall};
use crate::types::VmObject;
use crate::compiler::value::BramaPrimative;
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::Module;
use std::collections::HashMap;
use std::rc::Rc;

pub struct NumModule {
    methods: HashMap<String, NativeCall>
}

impl Module for NumModule {
    fn new() -> NumModule where Self: Sized {
        let mut module = NumModule {
            methods: HashMap::new()
        };
        module.methods.insert("oku".to_string(), Self::parse as NativeCall);
        module
    }

    fn get_module_name(&self) -> String {
        return "sayı".to_string();
    }

    fn get_method(&self, name: &String) -> Option<NativeCall> {
        match self.methods.get(name) {
            Some(method) => Some(*method),
            None => None
        }
    }

    fn get_module(&self, _: &String) -> Option<Rc<dyn Module>> {
        None
    }

    fn get_methods(&self) -> Vec<(&'static str, NativeCall)> {
        [("oku", Self::parse as NativeCall)].to_vec()
    }

    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>> {
        HashMap::new()
    }
}

impl NumModule  {
    pub fn parse(arguments: &Vec<VmObject>, last_position: usize, total_args: u8) -> NativeCallResult {
        if total_args > 1 {
            return Err(("More than 1 argument passed".to_string(), 0, 0));
        }

        let arg = arguments[last_position - 1].deref();

        return match &*arg {
            BramaPrimative::Number(_) => Ok(arguments[last_position - 1]),
            BramaPrimative::Text(text) => {
                match (*text).parse() {
                    Ok(num) => Ok(VmObject::native_convert(BramaPrimative::Number(num))),
                    _ => Err(("More than 1 argument passed".to_string(), 0, 0))
                }
            },
            _ => Ok(EMPTY_OBJECT)
        };
    }
}

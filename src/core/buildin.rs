use crate::{core::{Module, NativeCall, NativeCallResult}};
use crate::types::VmObject;
use crate::compiler::value::BramaPrimative;
use crate::compiler::value::EMPTY_OBJECT;
use std::collections::HashMap;
use std::rc::Rc;
use std::io::{self};


#[derive(Clone)]
pub struct IoModule {
    methods: HashMap<String, NativeCall>
}

pub struct NumModule {
    methods: HashMap<String, NativeCall>
}

#[derive(Clone)]
pub struct DebugModule {
    methods: HashMap<String, NativeCall>
}

impl Module for IoModule {
    fn new() -> IoModule where Self: Sized {
        let mut module = IoModule {
            methods: HashMap::new()
        };
        module.methods.insert("satıroku".to_string(), Self::readline as NativeCall);
        module.methods.insert("yaz".to_string(), Self::print as NativeCall);
        module.methods.insert("satıryaz".to_string(), Self::printline as NativeCall);
        module
    }

    fn get_module_name(&self) -> String {
        return "gç".to_string();
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
        [("satıroku", Self::readline as NativeCall),
         ("yaz", Self::print as NativeCall),
         ("satıryaz", Self::printline as NativeCall)].to_vec()
    }

    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>> {
        HashMap::new()
    }
}

impl IoModule  {
    pub fn readline(_: &Vec<VmObject>, _: usize, _: u8) -> NativeCallResult {        
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => return Ok(VmObject::from(Rc::new(line.trim().to_string()))),
            _ => return Ok(EMPTY_OBJECT)
        }
    }

    pub fn print(arguments: &Vec<VmObject>, _: usize, _: u8) -> NativeCallResult {
        for arg in arguments {
            print!("{:?}", arg.deref());
        }

        Ok(EMPTY_OBJECT)
    }
    
    pub fn printline(arguments: &Vec<VmObject>, last_position: usize, total_args: u8) -> NativeCallResult {
        for arg in arguments.iter().skip((last_position as usize - 1) - (total_args as usize - 1)).take(total_args as usize) {
            println!("{:?}", arg.deref());
        }

        Ok(EMPTY_OBJECT)
    }
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
            return Err(("More than 1 argument passed", 0, 0));
        }

        let arg = arguments[last_position - 1].deref();

        return match &*arg {
            BramaPrimative::Number(_) => Ok(arguments[last_position - 1]),
            BramaPrimative::Text(text) => {
                match (*text).parse() {
                    Ok(num) => Ok(VmObject::native_convert(BramaPrimative::Number(num))),
                    _ => Err(("More than 1 argument passed", 0, 0))
                }
            },
            _ => Ok(EMPTY_OBJECT)
        };
    }
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
        return "hataayıklama".to_string();
    }

    fn get_method(&self, name: &String) -> Option<NativeCall> {
        match self.methods.get(name) {
            Some(method) => Some(*method),
            None         => None
        }
    }

    fn get_module(&self, _: &String) -> Option<Rc<dyn Module>> {
        None
    }

    fn get_methods(&self) -> Vec<(&'static str, NativeCall)> {
        [("doğrula", Self::assert as NativeCall)].to_vec()
    }

    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>> {
        HashMap::new()
    }
}

impl DebugModule  {
    pub fn assert(arguments: &Vec<VmObject>, last_position: usize, total_args: u8) -> NativeCallResult {
        if total_args != 2 {
            return Err(("2 argument required", 0, 0));
        }

        let left  = arguments[last_position - 1].deref();
        let right = arguments[last_position - 2].deref();

        let status = match (&*left, &*right) {
            (BramaPrimative::Empty,               BramaPrimative::Empty)               => true,
            (BramaPrimative::Atom(l_value),       BramaPrimative::Atom(r_value))       => *l_value == *r_value,
            (BramaPrimative::Bool(l_value),       BramaPrimative::Bool(r_value))       => *l_value == *r_value,
            (BramaPrimative::Number(l_value),     BramaPrimative::Number(r_value))     => *l_value == *r_value,
            (BramaPrimative::Text(l_value),       BramaPrimative::Text(r_value))       => *l_value == *r_value,
            _ => false
        };

        return match status {
            false => Err(("Assert failed", 0, 0)),
            true  => Ok(EMPTY_OBJECT)
        };
    }
}
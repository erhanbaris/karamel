pub mod number;
pub mod text;

use crate::buildin::opcode_class::OpcodeClass;
use std::vec::Vec;
use lazy_static::lazy_static;


pub fn get_empty_class() -> OpcodeClass {
    let mut opcode = OpcodeClass::default();
    opcode.set_name("__NO__CLASS__".to_string());
    opcode
}

lazy_static! {
    pub static ref PRIMATIVE_CLASSES: Vec<OpcodeClass> = {
        let mut m = Vec::new();
        m.push(number::get_primative_class());
        m.push(text::get_primative_class());
        m.push(get_empty_class());
        m.push(get_empty_class());
        m.push(get_empty_class());
        m.push(get_empty_class());
        m.push(get_empty_class());
        m.push(get_empty_class());
        m.push(get_empty_class());
        m.push(get_empty_class());
        m
    };
}

#[macro_export]
macro_rules! nativecall_test {
    ($name:ident, $function_name:ident, $query:expr, $result:expr) => {
        #[test]
        fn $name () {
            use std::cell::RefCell;
            let stack: Vec<VmObject> = Vec::new();
            let stdout = Some(RefCell::new(String::new()));
            let stderr = Some(RefCell::new(String::new()));
            
            let parameter = FunctionParameter::new(&stack, Some(Arc::new($query)), 0, 0, &stdout, &stderr);
            let result = $function_name(parameter);
            assert!(result.is_ok());
            let object = result.unwrap().deref();
            assert_eq!(*object, $result);
        }
    };
}

#[macro_export]
macro_rules! n_parameter_check {
    ($function_name:expr, $parameter_size:expr) => {
        if parameter.length() > 1 {
            return n_parameter_expected!("tür_bilgisi", 1);
        }
    };
}

#[macro_export]
macro_rules! n_parameter_expected {
    ($function_name:expr, $parameter_size:expr) => { Err(format!("'{}' fonksiyonu {} parametre kabul ediyor", $function_name, $parameter_size)) };
    ($function_name:expr, $parameter_size:expr, $parameter_found:expr) => { Err(format!("'{}' fonksiyonu {} parametre kabul ediyor, fakat {} adet parametre bulundu", $function_name, $parameter_size, $parameter_found)) };
}

#[macro_export]
macro_rules! expected_parameter_type {
    ($function_name:expr, $expected_type:expr) => { Err((format!("'{}' sadece {} parametresini kabul ediyor", $function_name, $expected_type))) };
}
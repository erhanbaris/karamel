pub mod number;
pub mod text;
pub mod list;
pub mod dict;
pub mod baseclass;
pub mod proxy;

use crate::buildin::class::baseclass::BasicInnerClass;
use std::{rc::Rc, vec::Vec};
use lazy_static::lazy_static;

use super::Class;


pub fn get_empty_class() -> Rc<dyn Class> {
    let mut opcode = BasicInnerClass::default();
    opcode.set_name("__NO__CLASS__");
    Rc::new(opcode)
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
            
            let parameter = FunctionParameter::new(&stack, Some(VmObject::native_convert($query)), 0, 0, &stdout, &stderr);
            let result = $function_name(parameter);
            assert!(result.is_ok());
            let object = result.unwrap().deref();
            assert_eq!(*object, $result);
        }
    };
}

#[macro_export]
macro_rules! primative_text {
    ($text:expr) => {
        BramaPrimative::Text(Rc::new($text.to_string()))
    };
}

#[macro_export]
macro_rules! primative_number {
    ($number:expr) => {
        BramaPrimative::Number($number as f64)
    };
}

#[macro_export]
macro_rules! primative_list {
    ($list:expr) => {
        BramaPrimative::List(RefCell::new($list))
    };
}

#[macro_export]
macro_rules! arc_text {
    ($text:expr) => {
        VmObject::native_convert(primative_text!($text))
    };
}

#[macro_export]
macro_rules! arc_number {
    ($number:expr) => {
        VmObject::from($number as f64)
    };
}

#[macro_export]
macro_rules! arc_bool {
    ($bool:expr) => {
        VmObject::from($bool)
    };
}

#[macro_export]
macro_rules! arc_empty {
    () => {
        EMPTY_OBJECT
    };
}

#[macro_export]
macro_rules! nativecall_test_with_params {
    ($name:ident, $function_name:ident, $query:expr, $params:expr, $result:expr) => {
        #[test]
        fn $name () {
            use std::cell::RefCell;
            let stack: Vec<VmObject> = $params.to_vec();
            let stdout = Some(RefCell::new(String::new()));
            let stderr = Some(RefCell::new(String::new()));
            
            let parameter = FunctionParameter::new(&stack, Some(VmObject::native_convert($query)), stack.len() as usize, stack.len() as u8, &stdout, &stderr);
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

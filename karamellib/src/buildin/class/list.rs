use std::sync::Arc;

use crate::{buildin::Class, compiler::function::{FunctionParameter, NativeCallResult}};
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::class::baseclass::BasicInnerClass;
use crate::compiler::value::BramaPrimative;
use crate::types::VmObject;
use crate::{n_parameter_expected, expected_parameter_type};

pub fn get_primative_class() -> BasicInnerClass {
    let mut opcode = BasicInnerClass::default();
    opcode.set_name("Liste");
    
    opcode.add_method("uzunluk", length);
    opcode.add_method("ekle", add);
    opcode.add_method("temizle", clear);
    opcode.add_method("arayaekle", insert);
    opcode
}

fn length(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::List(list) = &*parameter.source().unwrap() {
        let length = list.borrow().len() as f64;
        return Ok(VmObject::native_convert(BramaPrimative::Number(length)));
    }
    Ok(EMPTY_OBJECT)
}

fn clear(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::List(list) = &*parameter.source().unwrap() {
        list.borrow_mut().clear();
    }
    Ok(EMPTY_OBJECT)
}

fn add(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::List(list) = &*parameter.source().unwrap() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("ekle", 1),
            1 => {
                let length = list.borrow().len() as f64;
                list.borrow_mut().push(parameter.iter().next().unwrap().deref());
                return Ok(VmObject::native_convert(BramaPrimative::Number(length)));
            },
            _ => n_parameter_expected!("ekle", 1, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn insert(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::List(list) = &*parameter.source().unwrap() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("arayaekle", 1),
            2 => {
                let mut iter = parameter.iter();
                let (position_object, item) = (&*iter.next().unwrap().deref(), &*iter.next().unwrap());

                let position = match position_object {
                    BramaPrimative::Number(number) => *number,
                    _ => return expected_parameter_type!("arayaekle", "Sayı")
                };

                list.borrow_mut().insert(position as usize, Arc::new(BramaPrimative::Empty));
                return Ok(VmObject::native_convert(BramaPrimative::Empty));
            },
            _ => n_parameter_expected!("arayaekle", 1, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::compiler::value::BramaPrimative;
    use super::*;

    use crate::nativecall_test_with_params;
    use crate::nativecall_test;
    use crate::primative_list;
    use crate::primative_text;
    use crate::arc_text;
    use crate::arc_bool;
    use crate::arc_empty;
    use crate::arc_number;
    use crate::primative_number;


    nativecall_test!{test_length_1, length,  primative_list!([arc_text!("")].to_vec()), BramaPrimative::Number(1.0)}
    nativecall_test!{test_length_2, length,  primative_list!([].to_vec()), BramaPrimative::Number(0.0)}
    nativecall_test!{test_length_3, length,  primative_list!([arc_text!(""), arc_empty!(), arc_number!(123), arc_bool!(true)].to_vec()), BramaPrimative::Number(4.0)}


    nativecall_test_with_params!{test_add_1, add, primative_list!([arc_text!("")].to_vec()), [VmObject::native_convert(BramaPrimative::Number(8.0))], primative_number!(1)}
    nativecall_test_with_params!{test_add_2, add, primative_list!([].to_vec()), [VmObject::native_convert(BramaPrimative::Bool(true))], primative_number!(0)}

    #[test]
    fn test_clear_1 () {
        use std::cell::RefCell;
        let stack: Vec<VmObject> = Vec::new();
        let stdout = Some(RefCell::new(String::new()));
        let stderr = Some(RefCell::new(String::new()));
        let list = Arc::new(BramaPrimative::List(RefCell::new([arc_bool!(true), arc_empty!(), arc_number!(1)].to_vec())));
        
        let parameter = FunctionParameter::new(&stack, Some(list.clone()), stack.len() as usize, stack.len() as u8, &stdout, &stderr);
        let result = clear(parameter);
        assert!(result.is_ok());

        match &*list {
            BramaPrimative::List(l) => assert_eq!(l.borrow().len(), 0),
            _ => assert_eq!(true, false)
        };
    }
}
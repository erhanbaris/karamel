use std::rc::Rc;

use crate::{buildin::Class, compiler::function::{FunctionParameter, NativeCallResult}};
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::class::baseclass::BasicInnerClass;
use crate::compiler::value::KaramelPrimative;
use crate::error::KaramelErrorType;
use crate::types::VmObject;
use crate::{n_parameter_expected, expected_parameter_type, arc_bool, arc_empty};
use crate::buildin::class::PRIMATIVE_CLASS_NAMES;

pub fn get_primative_class() -> Rc<dyn Class> {
    let mut opcode = BasicInnerClass::default();
    opcode.set_name("liste");
    
    opcode.add_class_method("getir", get);
    opcode.add_class_method("güncelle", set);
    opcode.add_class_method("guncelle", set);
    opcode.add_class_method("uzunluk", length);
    opcode.add_class_method("ekle", add);
    opcode.add_class_method("temizle", clear);
    opcode.add_class_method("arayaekle", insert);
    opcode.add_class_method("pop", pop);
    opcode.add_class_method("sil", remove);
    opcode.set_getter(getter);
    opcode.set_setter(setter);

    PRIMATIVE_CLASS_NAMES.lock().unwrap().insert(opcode.get_class_name());
    Rc::new(opcode)
}

fn get(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::List(list) = &*parameter.source().unwrap().deref() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("getir".to_string(), 1),
            1 => {
                let position = match &*parameter.iter().next().unwrap().deref() {
                    KaramelPrimative::Number(number) => *number as usize,
                    _ => return expected_parameter_type!("sıra".to_string(), "Sayı".to_string())
                };
                
                return match list.borrow().get(position) {
                    Some(item) => Ok(*item),
                    _ => Ok(EMPTY_OBJECT)
                };
            },
            _ => n_parameter_expected!("getir".to_string(), 1, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn set(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::List(list) = &*parameter.source().unwrap().deref() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("güncelle".to_string(), 2),
            2 => {
                let mut iter = parameter.iter();
                let (position_object, item) = (&*iter.next().unwrap().deref(), &*iter.next().unwrap());

                let position = match position_object {
                    KaramelPrimative::Number(number) => *number,
                    _ => return expected_parameter_type!("güncelle".to_string(), "Sayı".to_string())
                };

                let is_in_size = position <= list.borrow().len() as f64;
                return match is_in_size {
                    true => {
                        list.borrow_mut()[position as usize] = *item; 
                        Ok(arc_bool!(true))
                    },
                    false => Ok(arc_bool!(false))
                };
            },
            _ => n_parameter_expected!("güncelle".to_string(), 2, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn getter(source: VmObject, index: f64) -> NativeCallResult {
    let index = match index >= 0.0 {
        true => index as usize,
        false =>  return Ok(EMPTY_OBJECT)
    };

    if let KaramelPrimative::List(list) = &*source.deref() {

        let is_in_size = index <= list.borrow().len();
        return match is_in_size {
            true => match list.borrow().get(index) {
                Some(item) => Ok(*item),
                _ => Ok(EMPTY_OBJECT)
            },
            false => Ok(arc_empty!())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn setter(source: VmObject, index: f64, item: VmObject) -> NativeCallResult {
    let index = match index >= 0.0 {
        true => index as usize,
        false =>  return Ok(EMPTY_OBJECT)
    };

    if let KaramelPrimative::List(list) = &*source.deref() {

        let is_in_size = index <= list.borrow().len();
        return match is_in_size {
            true => {
                list.borrow_mut()[index] = item; 
                Ok(arc_bool!(true))
            },
            false => Ok(arc_bool!(false))
        };
    }
    Ok(EMPTY_OBJECT)
}

fn length(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::List(list) = &*parameter.source().unwrap().deref() {
        let length = list.borrow().len() as f64;
        return Ok(VmObject::from(length));
    }
    Ok(EMPTY_OBJECT)
}

fn clear(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::List(list) = &*parameter.source().unwrap().deref() {
        list.borrow_mut().clear();
    }
    Ok(EMPTY_OBJECT)
}

pub fn add(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::List(list) = &*parameter.source().unwrap().deref() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("ekle".to_string(), 1),
            1 => {
                let length = list.borrow().len() as f64;
                list.borrow_mut().push(*parameter.iter().next().unwrap());
                return Ok(VmObject::from(length));
            },
            _ => n_parameter_expected!("ekle".to_string(), 1, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

pub fn insert(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::List(list) = &*parameter.source().unwrap().deref() {
        match parameter.length() {
            0 => return n_parameter_expected!("arayaekle".to_string(), 1),
            2 => {
                let mut iter = parameter.iter();
                let (position_object, item) = (&*iter.next().unwrap().deref(), &*iter.next().unwrap());

                let position = match position_object {
                    KaramelPrimative::Number(number) => *number,
                    _ => return expected_parameter_type!("arayaekle".to_string(), "Sayı".to_string())
                };

                let is_in_size = position <= list.borrow().len() as f64;
                return match is_in_size {
                    true => {
                        list.borrow_mut().insert(position as usize, *item); 
                        Ok(arc_bool!(true))
                    },
                    false => Ok(arc_bool!(false))
                };
            },
            _ => return n_parameter_expected!("arayaekle".to_string(), 2, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn remove(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::List(list) = &*parameter.source().unwrap().deref() {
        match parameter.length() {
            0 => return n_parameter_expected!("sil".to_string(), 1),
            1 => {
                let position = match &*parameter.iter().next().unwrap().deref() {
                    KaramelPrimative::Number(number) => *number as usize,
                    _ => return expected_parameter_type!("sıra".to_string(), "Sayı".to_string())
                };
                
                let is_in_size = position <= list.borrow().len();
                return match is_in_size {
                    true => Ok(list.borrow_mut().remove(position)),
                    false => Ok(arc_bool!(false))
                };
            },
            _ => return n_parameter_expected!("sil".to_string(), 1, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn pop(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::List(list) = &*parameter.source().unwrap().deref() {
        let item = list.borrow_mut().pop();
        return match item {
            Some(data) => Ok(data),
            _ => Ok(EMPTY_OBJECT)
        };
    }
    Ok(EMPTY_OBJECT)
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::compiler::value::KaramelPrimative;
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


    nativecall_test!{test_length_1, length,  primative_list!([arc_text!("")].to_vec()), KaramelPrimative::Number(1.0)}
    nativecall_test!{test_length_2, length,  primative_list!(Vec::new()), KaramelPrimative::Number(0.0)}
    nativecall_test!{test_length_3, length,  primative_list!([arc_text!(""), arc_empty!(), arc_number!(123), arc_bool!(true)].to_vec()), KaramelPrimative::Number(4.0)}


    nativecall_test_with_params!{test_add_1, add, primative_list!([arc_text!("")].to_vec()), [VmObject::from(8.0)], primative_number!(1)}
    nativecall_test_with_params!{test_add_2, add, primative_list!(Vec::new()), [VmObject::native_convert(KaramelPrimative::Bool(true))], primative_number!(0)}
    #[test]
    fn test_add_3 () {
        use std::cell::RefCell;
        let stack: Vec<VmObject> = [arc_text!("merhaba")].to_vec();
        let stdout = Some(RefCell::new(String::new()));
        let stderr = Some(RefCell::new(String::new()));
        let list = KaramelPrimative::List(RefCell::new(Vec::new()));
        let obj = VmObject::native_convert(list);
        
        let parameter = FunctionParameter::new(&stack, Some(obj), stack.len() as usize, stack.len() as u8, &stdout, &stderr);
        let result = add(parameter);
        assert!(result.is_ok());

        match &*result.unwrap().deref() {
            KaramelPrimative::Number(p) => assert_eq!(*p, 0.0),
            _ => assert_eq!(true, false)
        };
    }

    #[test]
    fn test_insert_1 () {
        use std::cell::RefCell;
        let stdout = Some(RefCell::new(String::new()));
        let stderr = Some(RefCell::new(String::new()));
        let list = Rc::new(KaramelPrimative::List(RefCell::new(Vec::new())));
        let obj = VmObject::native_convert_by_ref(list.clone());
        
        let result = add(FunctionParameter::new(&[arc_text!("dünya")].to_vec(), Some(obj), 1 as usize, 1 as u8, &stdout, &stderr));
        assert!(result.is_ok());

        match &*list {
            KaramelPrimative::List(l) => assert_eq!(l.borrow().len(), 1),
            _ => assert_eq!(true, false)
        };

        let result = insert(FunctionParameter::new(&[arc_number!(0), arc_text!("merhaba")].to_vec(), Some(obj), 2 as usize, 2 as u8, &stdout, &stderr));
        assert!(result.is_ok());

        match &*list {
            KaramelPrimative::List(l) => {
                assert_eq!(l.borrow().len(), 2);
                assert_eq!(l.borrow().get(0).unwrap().deref(), Rc::new(primative_text!("merhaba")));
                assert_eq!(l.borrow().get(1).unwrap().deref(), Rc::new(primative_text!("dünya")));
            },
            _ => assert_eq!(true, false)
        };
    }

    #[test]
    fn test_clear_1 () {
        use std::cell::RefCell;
        let stack: Vec<VmObject> = Vec::new();
        let stdout = Some(RefCell::new(String::new()));
        let stderr = Some(RefCell::new(String::new()));
        let list = Rc::new(KaramelPrimative::List(RefCell::new([arc_bool!(true), arc_empty!(), arc_number!(1)].to_vec())));
        let obj = VmObject::native_convert_by_ref(list.clone());
        
        let result = add(FunctionParameter::new(&[arc_text!("dünya")].to_vec(), Some(obj), 1 as usize, 1 as u8, &stdout, &stderr));
        assert!(result.is_ok());


        let parameter = FunctionParameter::new(&stack, Some(obj), stack.len() as usize, stack.len() as u8, &stdout, &stderr);
        let result = clear(parameter);
        assert!(result.is_ok());

        match &*list {
            KaramelPrimative::List(l) => assert_eq!(l.borrow().len(), 0),
            _ => assert_eq!(true, false)
        };
    }
}
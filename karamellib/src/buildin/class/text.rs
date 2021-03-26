use crate::compiler::{function::{FunctionParameter, NativeCallResult}};
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::class::baseclass::OpcodeClass;
use crate::compiler::value::BramaPrimative;
use crate::types::VmObject;
use crate::{n_parameter_expected, expected_parameter_type};

use std::sync::Arc;


pub fn get_primative_class() -> OpcodeClass {
    let mut opcode = OpcodeClass::default();
    opcode.set_name("Yazı".to_string());
    
    opcode.add_method("uzunluk".to_string(), length);
    opcode.add_method("küçükharf".to_string(), lowercase);
    opcode.add_method("kucukharf".to_string(), lowercase);
    opcode.add_method("büyükharf".to_string(), uppercase);
    opcode.add_method("buyukharf".to_string(), uppercase);
    opcode.add_method("içeriyormu".to_string(), contains);
    opcode.add_method("iceriyormu".to_string(), contains);
    opcode
}

fn length(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(text.chars().count() as f64)));
    }
    Ok(EMPTY_OBJECT)
}

fn contains(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("içeriyormu", 1),
            1 => {
                match &*parameter.iter().next().unwrap().deref() {
                    BramaPrimative::Text(search) =>  Ok(VmObject::native_convert(BramaPrimative::Bool(text.contains(&search[..])))),
                    _ => expected_parameter_type!("içeriyormu", "Yazı")
                }
            },
            _ => n_parameter_expected!("içeriyormu", 1, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn lowercase(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap() {
        let text:String = text.chars()
        .map(|x| match x { 
            'I' => 'ı', 
            'İ' => 'i', 
            'Ü' => 'ü', 
            'Ğ' => 'ğ', 
            'Ş' => 'ş', 
            'Ç' => 'ç', 
            'Ö' => 'ö',
            _ => x
        }).collect();
        return Ok(VmObject::native_convert(BramaPrimative::Text(Arc::new(text.to_lowercase()))));
    }
    Ok(EMPTY_OBJECT)
}

fn uppercase(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap() {
        let text:String = text.chars()
        .map(|x| match x { 
            'ı' => 'I', 
            'i' => 'İ', 
            'ü' => 'Ü', 
            'ğ' => 'Ğ', 
            'ş' => 'Ş', 
            'ç' => 'Ç', 
            'ö' => 'Ö',
            _ => x
        }).collect();
        return Ok(VmObject::native_convert(BramaPrimative::Text(Arc::new(text.to_uppercase()))));
    }
    Ok(EMPTY_OBJECT)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::compiler::value::BramaPrimative;
    use super::*;

    use crate::nativecall_test;


    nativecall_test!{test_length_1, length, BramaPrimative::Text(Arc::new("TÜRKİYE".to_string())), BramaPrimative::Number(7.0)}
    nativecall_test!{test_length_2, length, BramaPrimative::Text(Arc::new("".to_string())), BramaPrimative::Number(0.0)}
    nativecall_test!{test_length_3, length, BramaPrimative::Text(Arc::new("12345".to_string())), BramaPrimative::Number(5.0)}
    nativecall_test!{test_lowercase_1, lowercase, BramaPrimative::Text(Arc::new("TÜRKİYE".to_string())), BramaPrimative::Text(Arc::new("türkiye".to_string()))}
    nativecall_test!{test_lowercase_2, lowercase, BramaPrimative::Text(Arc::new("IĞÜİŞÇÖ".to_string())), BramaPrimative::Text(Arc::new("ığüişçö".to_string()))}
    nativecall_test!{test_lowercase_3, lowercase, BramaPrimative::Text(Arc::new("ERHAN".to_string())), BramaPrimative::Text(Arc::new("erhan".to_string()))}
    nativecall_test!{test_uppercase_1, uppercase, BramaPrimative::Text(Arc::new("türkiye".to_string())), BramaPrimative::Text(Arc::new("TÜRKİYE".to_string()))}
    nativecall_test!{test_uppercase_2, uppercase, BramaPrimative::Text(Arc::new("ığüişçö".to_string())), BramaPrimative::Text(Arc::new("IĞÜİŞÇÖ".to_string()))}
    nativecall_test!{test_uppercase_3, uppercase, BramaPrimative::Text(Arc::new("erhan".to_string())), BramaPrimative::Text(Arc::new("ERHAN".to_string()))}
}
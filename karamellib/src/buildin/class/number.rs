use crate::{buildin::Class, compiler::{function::{FunctionParameter, NativeCallResult}}, n_parameter_expected, expected_parameter_type, error::KaramelErrorType};
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::class::BasicInnerClass;
use crate::compiler::value::KaramelPrimative;
use crate::types::VmObject;
use crate::buildin::class::PRIMATIVE_CLASS_NAMES;

use std::{mem, rc::Rc};

pub fn get_primative_class() -> Rc<dyn Class> {
    let mut opcode = BasicInnerClass::default();
    opcode.set_name("sayı");
    
    opcode.add_class_method("hex", hex);
    opcode.add_class_method("yazı", string);
    opcode.add_class_method("yazi", string);
    opcode.add_class_method("yuvarla", round);
    opcode.add_class_method("tavan", ceil);
    opcode.add_class_method("taban", floor);
    opcode.add_class_method("tamsayı", trunc);
    opcode.add_class_method("kesir", fract);
    opcode.add_class_method("üst", power);

    PRIMATIVE_CLASS_NAMES.lock().unwrap().insert(opcode.get_class_name());
    Rc::new(opcode)
}

fn hex(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::Number(number) = &*parameter.source().unwrap().deref() {
        if number.fract() != 0.0 {
            let as_int: u64 = unsafe { mem::transmute(*number) };
            return Ok(VmObject::native_convert(KaramelPrimative::Text(Rc::new(format!("0x{:x}", as_int)))));
        }

        return Ok(VmObject::native_convert(KaramelPrimative::Text(Rc::new(format!("0x{:x}", number.trunc() as i64)))));
    }
    Ok(EMPTY_OBJECT)
}

fn string(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::Number(number) = &*parameter.source().unwrap().deref() {
        return Ok(VmObject::native_convert(KaramelPrimative::Text(Rc::new(format!("{}", number)))));
    }
    Ok(EMPTY_OBJECT)
}

fn round(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::Number(number) = &*parameter.source().unwrap().deref() {
        return Ok(VmObject::from(number.round()));
    }
    Ok(EMPTY_OBJECT)
}

fn ceil(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::Number(number) = &*parameter.source().unwrap().deref() {
        return Ok(VmObject::from(number.ceil()));
    }
    Ok(EMPTY_OBJECT)
}

fn floor(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::Number(number) = &*parameter.source().unwrap().deref() {
        return Ok(VmObject::from(number.floor()));
    }
    Ok(EMPTY_OBJECT)
}

fn trunc(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::Number(number) = &*parameter.source().unwrap().deref() {
        return Ok(VmObject::from(number.trunc()));
    }
    Ok(EMPTY_OBJECT)
}

fn fract(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::Number(number) = &*parameter.source().unwrap().deref() {
        return Ok(VmObject::from(number.fract()));
    }
    Ok(EMPTY_OBJECT)
}

fn power(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::Number(sayi) = &*parameter.source().unwrap().deref() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("üst".to_string(), 1),
            1 => {
                match &*parameter.iter().next().unwrap().deref() {
                    KaramelPrimative::Number(pow) =>  {
                        Ok(VmObject::native_convert(KaramelPrimative::Number(sayi.powf(*pow) as f64)))
                    },
                    _ => expected_parameter_type!("üst".to_string(), "Sayı".to_string())
                }
            },
            _ => n_parameter_expected!("üst".to_string(), 1, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::compiler::value::KaramelPrimative;
    use super::*;
    use crate::nativecall_test;

    nativecall_test!{test_hex_1, hex, KaramelPrimative::Number(-1.51), KaramelPrimative::Text(Rc::new("0xbff828f5c28f5c29".to_string()))}
    nativecall_test!{test_hex_2, hex, KaramelPrimative::Number(22.0), KaramelPrimative::Text(Rc::new("0x16".to_string()))}
    nativecall_test!{test_hex_3, hex, KaramelPrimative::Number(-16.0), KaramelPrimative::Text(Rc::new("0xfffffffffffffff0".to_string()))}

    nativecall_test!{test_yuvarla_1, round, KaramelPrimative::Number(1.51), KaramelPrimative::Number(2.0)}
    nativecall_test!{test_yuvarla_2, round, KaramelPrimative::Number(1.5), KaramelPrimative::Number(2.0)}
    nativecall_test!{test_yuvarla_3, round, KaramelPrimative::Number(1.4), KaramelPrimative::Number(1.0)}
    nativecall_test!{test_yuvarla_4, round, KaramelPrimative::Number(-1.2), KaramelPrimative::Number(-1.0)}
    nativecall_test!{test_yuvarla_5, round, KaramelPrimative::Number(-1.5), KaramelPrimative::Number(-2.0)}
    nativecall_test!{test_yuvarla_6, round, KaramelPrimative::Number(-1.51), KaramelPrimative::Number(-2.0)}

    nativecall_test!{test_tavan_1, ceil, KaramelPrimative::Number(1.51), KaramelPrimative::Number(2.0)}
    nativecall_test!{test_tavan_2, ceil, KaramelPrimative::Number(1.5), KaramelPrimative::Number(2.0)}
    nativecall_test!{test_tavan_3, ceil, KaramelPrimative::Number(1.4), KaramelPrimative::Number(2.0)}
    nativecall_test!{test_tavan_4, ceil, KaramelPrimative::Number(-1.2), KaramelPrimative::Number(-1.0)}
    nativecall_test!{test_tavan_5, ceil, KaramelPrimative::Number(-1.5), KaramelPrimative::Number(-1.0)}
    nativecall_test!{test_tavan_6, ceil, KaramelPrimative::Number(-1.51), KaramelPrimative::Number(-1.0)}

    nativecall_test!{test_taban_1, floor, KaramelPrimative::Number(1.51), KaramelPrimative::Number(1.0)}
    nativecall_test!{test_taban_2, floor, KaramelPrimative::Number(1.5), KaramelPrimative::Number(1.0)}
    nativecall_test!{test_taban_3, floor, KaramelPrimative::Number(1.4), KaramelPrimative::Number(1.0)}
    nativecall_test!{test_taban_4, floor, KaramelPrimative::Number(-1.2), KaramelPrimative::Number(-2.0)}
    nativecall_test!{test_taban_5, floor, KaramelPrimative::Number(-1.5), KaramelPrimative::Number(-2.0)}
    nativecall_test!{test_taban_6, floor, KaramelPrimative::Number(-1.51), KaramelPrimative::Number(-2.0)}

    nativecall_test!{test_tamsayi_1, trunc, KaramelPrimative::Number(-1.5), KaramelPrimative::Number(-1.0)}
    nativecall_test!{test_tamsayi_2, trunc, KaramelPrimative::Number(122.51), KaramelPrimative::Number(122.0)}

    nativecall_test!{test_kesir_1, fract, KaramelPrimative::Number(-1.5), KaramelPrimative::Number(-0.5)}
}
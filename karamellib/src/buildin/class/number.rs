use crate::{buildin::Class, compiler::{function::{FunctionParameter, NativeCallResult}}};
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::class::BasicInnerClass;
use crate::compiler::value::BramaPrimative;
use crate::types::VmObject;

use std::{mem, sync::Arc};

pub fn get_primative_class() -> Box<dyn Class + Send + Sync> {
    let mut opcode = BasicInnerClass::default();
    opcode.set_name("Sayı");
    
    opcode.add_method("hex", hex);
    opcode.add_method("yuvarla", round);
    opcode.add_method("tavan", ceil);
    opcode.add_method("taban", floor);
    opcode.add_method("tamsayı", trunc);
    opcode.add_method("kesir", fract);
    Box::new(opcode)
}

fn hex(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Number(number) = &*parameter.source().unwrap().deref() {
        if number.fract() != 0.0 {
            let as_int: u64 = unsafe { mem::transmute(*number) };
            return Ok(VmObject::native_convert(BramaPrimative::Text(Arc::new(format!("0x{:x}", as_int)))));
        }

        return Ok(VmObject::native_convert(BramaPrimative::Text(Arc::new(format!("0x{:x}", number.trunc() as i64)))));
    }
    Ok(EMPTY_OBJECT)
}

fn round(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Number(number) = &*parameter.source().unwrap().deref() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(number.round())));
    }
    Ok(EMPTY_OBJECT)
}

fn ceil(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Number(number) = &*parameter.source().unwrap().deref() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(number.ceil())));
    }
    Ok(EMPTY_OBJECT)
}

fn floor(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Number(number) = &*parameter.source().unwrap().deref() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(number.floor())));
    }
    Ok(EMPTY_OBJECT)
}

fn trunc(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Number(number) = &*parameter.source().unwrap().deref() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(number.trunc())));
    }
    Ok(EMPTY_OBJECT)
}

fn fract(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Number(number) = &*parameter.source().unwrap().deref() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(number.fract())));
    }
    Ok(EMPTY_OBJECT)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::compiler::value::BramaPrimative;
    use super::*;
    use crate::nativecall_test;

    nativecall_test!{test_hex_1, hex, BramaPrimative::Number(-1.51), BramaPrimative::Text(Arc::new("0xbff828f5c28f5c29".to_string()))}
    nativecall_test!{test_hex_2, hex, BramaPrimative::Number(22.0), BramaPrimative::Text(Arc::new("0x16".to_string()))}
    nativecall_test!{test_hex_3, hex, BramaPrimative::Number(-16.0), BramaPrimative::Text(Arc::new("0xfffffffffffffff0".to_string()))}

    nativecall_test!{test_yuvarla_1, round, BramaPrimative::Number(1.51), BramaPrimative::Number(2.0)}
    nativecall_test!{test_yuvarla_2, round, BramaPrimative::Number(1.5), BramaPrimative::Number(2.0)}
    nativecall_test!{test_yuvarla_3, round, BramaPrimative::Number(1.4), BramaPrimative::Number(1.0)}
    nativecall_test!{test_yuvarla_4, round, BramaPrimative::Number(-1.2), BramaPrimative::Number(-1.0)}
    nativecall_test!{test_yuvarla_5, round, BramaPrimative::Number(-1.5), BramaPrimative::Number(-2.0)}
    nativecall_test!{test_yuvarla_6, round, BramaPrimative::Number(-1.51), BramaPrimative::Number(-2.0)}

    nativecall_test!{test_tavan_1, ceil, BramaPrimative::Number(1.51), BramaPrimative::Number(2.0)}
    nativecall_test!{test_tavan_2, ceil, BramaPrimative::Number(1.5), BramaPrimative::Number(2.0)}
    nativecall_test!{test_tavan_3, ceil, BramaPrimative::Number(1.4), BramaPrimative::Number(2.0)}
    nativecall_test!{test_tavan_4, ceil, BramaPrimative::Number(-1.2), BramaPrimative::Number(-1.0)}
    nativecall_test!{test_tavan_5, ceil, BramaPrimative::Number(-1.5), BramaPrimative::Number(-1.0)}
    nativecall_test!{test_tavan_6, ceil, BramaPrimative::Number(-1.51), BramaPrimative::Number(-1.0)}

    nativecall_test!{test_taban_1, floor, BramaPrimative::Number(1.51), BramaPrimative::Number(1.0)}
    nativecall_test!{test_taban_2, floor, BramaPrimative::Number(1.5), BramaPrimative::Number(1.0)}
    nativecall_test!{test_taban_3, floor, BramaPrimative::Number(1.4), BramaPrimative::Number(1.0)}
    nativecall_test!{test_taban_4, floor, BramaPrimative::Number(-1.2), BramaPrimative::Number(-2.0)}
    nativecall_test!{test_taban_5, floor, BramaPrimative::Number(-1.5), BramaPrimative::Number(-2.0)}
    nativecall_test!{test_taban_6, floor, BramaPrimative::Number(-1.51), BramaPrimative::Number(-2.0)}

    nativecall_test!{test_tamsayi_1, trunc, BramaPrimative::Number(-1.5), BramaPrimative::Number(-1.0)}
    nativecall_test!{test_tamsayi_2, trunc, BramaPrimative::Number(122.51), BramaPrimative::Number(122.0)}

    nativecall_test!{test_kesir_1, fract, BramaPrimative::Number(-1.5), BramaPrimative::Number(-0.5)}
}
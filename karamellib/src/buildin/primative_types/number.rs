use crate::compiler::{BramaCompiler, function::{NativeCallResult}};
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::opcode_class::OpcodeClass;
use crate::compiler::value::BramaPrimative;
use crate::types::VmObject;

use std::{mem, sync::Arc};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref NUMBER_CLASS: OpcodeClass = {
        let mut opcode = OpcodeClass::default();
        opcode.set_name("sayı".to_string());
        
        opcode.add_method("hex".to_string(), hex);
        opcode.add_method("yuvarla".to_string(), round);
        opcode.add_method("tavan".to_string(), ceil);
        opcode.add_method("taban".to_string(), floor);
        opcode.add_method("tamsayı".to_string(), trunc);
        opcode.add_method("kesir".to_string(), fract);
        opcode
    };
}

fn hex(_: &mut BramaCompiler, source: Option<Arc<BramaPrimative>>, _: usize, _: u8) -> NativeCallResult {
    if let BramaPrimative::Number(number) = &*source.unwrap() {
        if number.fract() != 0.0 {
            let as_int: u64 = unsafe { mem::transmute(*number) };
            return Ok(VmObject::native_convert(BramaPrimative::Text(Arc::new(format!("0x{:x}", as_int)))));
        }

        return Ok(VmObject::native_convert(BramaPrimative::Text(Arc::new(format!("0x{:x}", number.trunc() as i64)))));
    }
    Ok(EMPTY_OBJECT)
}

fn round(_: &mut BramaCompiler, source: Option<Arc<BramaPrimative>>, _: usize, _: u8) -> NativeCallResult {
    if let BramaPrimative::Number(number) = &*source.unwrap() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(number.round())));
    }
    Ok(EMPTY_OBJECT)
}

fn ceil(_: &mut BramaCompiler, source: Option<Arc<BramaPrimative>>, _: usize, _: u8) -> NativeCallResult {
    if let BramaPrimative::Number(number) = &*source.unwrap() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(number.ceil())));
    }
    Ok(EMPTY_OBJECT)
}

fn floor(_: &mut BramaCompiler, source: Option<Arc<BramaPrimative>>, _: usize, _: u8) -> NativeCallResult {
    if let BramaPrimative::Number(number) = &*source.unwrap() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(number.floor())));
    }
    Ok(EMPTY_OBJECT)
}

fn trunc(_: &mut BramaCompiler, source: Option<Arc<BramaPrimative>>, _: usize, _: u8) -> NativeCallResult {
    if let BramaPrimative::Number(number) = &*source.unwrap() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(number.trunc())));
    }
    Ok(EMPTY_OBJECT)
}

fn fract(_: &mut BramaCompiler, source: Option<Arc<BramaPrimative>>, _: usize, _: u8) -> NativeCallResult {
    if let BramaPrimative::Number(number) = &*source.unwrap() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(number.fract())));
    }
    Ok(EMPTY_OBJECT)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::compiler::BramaCompiler;
    use crate::compiler::value::BramaPrimative;
    use super::*;

    #[macro_export]
    macro_rules! nativecall_test {
        ($name:ident, $function_name:ident, $query:expr, $result:expr) => {
            #[test]
            fn $name () {
                let result = $function_name(&mut BramaCompiler::new(), Some(Arc::new($query)), 0, 0);
                assert!(result.is_ok());
                let object = result.unwrap().deref();
                assert_eq!(*object, $result);
            }
        };
    }

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
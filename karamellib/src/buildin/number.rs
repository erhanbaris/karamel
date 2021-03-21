use crate::compiler::{BramaCompiler, function::{NativeCallResult}};
use crate::compiler::value::EMPTY_OBJECT;
use super::opcode_class::OpcodeClass;
use crate::compiler::value::BramaPrimative;
use crate::types::VmObject;

use std::{sync::Arc};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref NUMBER_CLASS: OpcodeClass = {
        let mut opcode = OpcodeClass::default();
        opcode.set_name("sayı".to_string());
        
        opcode.add_method("hex".to_string(), hex);
        opcode.add_method("yuvarla".to_string(), round);
        opcode.add_method("tavan".to_string(), tavan);
        opcode.add_method("taban".to_string(), taban);
        opcode
    };
}

fn hex(_: &mut BramaCompiler, source: Option<Arc<BramaPrimative>>, _: usize, _: u8) -> NativeCallResult {
    if let BramaPrimative::Number(number) = &*source.unwrap() {
        let as_int: u64 = *number as u64;
        return Ok(VmObject::native_convert(BramaPrimative::Text(Arc::new(format!("0x{:x}", as_int)))));
    }
    Ok(EMPTY_OBJECT)
}

fn round(_: &mut BramaCompiler, source: Option<Arc<BramaPrimative>>, _: usize, _: u8) -> NativeCallResult {
    if let BramaPrimative::Number(number) = &*source.unwrap() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(number.round())));
    }
    Ok(EMPTY_OBJECT)
}

fn tavan(_: &mut BramaCompiler, source: Option<Arc<BramaPrimative>>, _: usize, _: u8) -> NativeCallResult {
    if let BramaPrimative::Number(number) = &*source.unwrap() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(number.ceil())));
    }
    Ok(EMPTY_OBJECT)
}

fn taban(_: &mut BramaCompiler, source: Option<Arc<BramaPrimative>>, _: usize, _: u8) -> NativeCallResult {
    if let BramaPrimative::Number(number) = &*source.unwrap() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(number.floor())));
    }
    Ok(EMPTY_OBJECT)
}
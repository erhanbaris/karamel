use crate::compiler::function::FunctionReference;
use crate::compiler::{BramaCompiler, function::{NativeCallResult}};
use crate::compiler::value::EMPTY_OBJECT;
use super::opcode_class::OpcodeClass;
use crate::compiler::value::BramaPrimative;
use crate::types::VmObject;

use std::{mem, rc::Rc};

use lazy_static::lazy_static;
pub static NUMBER_CLASS: OpcodeClass = OpcodeClass::default();

lazy_static! {
    static ref NUMBER_CLASS: OpcodeClass = {
        let mut opcode = OpcodeClass::default();
        opcode.set_name("sayı".to_string());
        let hex_func = FunctionReference::native_function(hex, "hex".to_string(), Vec::new(), "".to_string());
        opcode.add_method("hex".to_string(), hex_func);
        opcode
    };
}


pub fn a() {
}

fn hex(compiler: &mut BramaCompiler, last_position: usize, total_args: u8) -> NativeCallResult {
    if total_args != 1 {
        return Err(("More than 1 argument passed".to_string(), 0, 0));
    }

    let arg = unsafe { (*compiler.current_scope).stack[last_position - 1].deref() };

    if let BramaPrimative::Number(number) = &*arg {
        let as_int: u64 = unsafe { mem::transmute(number) };
        return Ok(VmObject::native_convert(BramaPrimative::Text(Rc::new(format!("{:x}", as_int)))));
    }
    Ok(EMPTY_OBJECT)
}
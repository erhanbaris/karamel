use crate::{buildin::Class, compiler::function::{FunctionParameter, NativeCallResult}};
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::class::baseclass::BasicInnerClass;
use crate::compiler::value::BramaPrimative;
use crate::types::VmObject;
use crate::{n_parameter_expected, expected_parameter_type};
use crate::primative_text;

use unicode_width::UnicodeWidthStr;
use std::{cell::RefCell, sync::Arc};


pub fn get_primative_class() -> BasicInnerClass {
    let mut opcode = BasicInnerClass::default();
    opcode.set_name("Liste");
    
    opcode.add_method("uzunluk", length);
    opcode
}

fn length(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::List(list) = &*parameter.source().unwrap() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(list.borrow().len() as f64)));
    }
    Ok(EMPTY_OBJECT)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::compiler::value::BramaPrimative;
    use super::*;

    use crate::nativecall_test;
    use crate::nativecall_test_with_params;
    use crate::primative_text;


    nativecall_test!{test_length_1, length, BramaPrimative::Text(Arc::new("TÜRKİYE".to_string())), BramaPrimative::Number(7.0)}

}
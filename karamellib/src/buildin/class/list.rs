use crate::{buildin::Class, compiler::function::{FunctionParameter, NativeCallResult}};
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::class::baseclass::BasicInnerClass;
use crate::compiler::value::BramaPrimative;
use crate::types::VmObject;

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

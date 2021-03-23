use crate::compiler::{BramaCompiler, function::{NativeCallResult}};
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::opcode_class::OpcodeClass;
use crate::compiler::value::BramaPrimative;
use crate::types::VmObject;

use std::sync::Arc;


pub fn get_primative_class() -> OpcodeClass {
    let mut opcode = OpcodeClass::default();
    opcode.set_name("yazı".to_string());
    
    opcode.add_method("küçükharf".to_string(), lowercase);
    opcode.add_method("büyükharf".to_string(), uppercase);
    opcode
}

fn lowercase(_: &mut BramaCompiler, source: Option<Arc<BramaPrimative>>, _: usize, _: u8) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*source.unwrap() {
        return Ok(VmObject::native_convert(BramaPrimative::Text(Arc::new(text.to_lowercase()))));
    }
    Ok(EMPTY_OBJECT)
}

fn uppercase(_: &mut BramaCompiler, source: Option<Arc<BramaPrimative>>, _: usize, _: u8) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*source.unwrap() {
        return Ok(VmObject::native_convert(BramaPrimative::Text(Arc::new(text.to_uppercase()))));
    }
    Ok(EMPTY_OBJECT)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::compiler::BramaCompiler;
    use crate::compiler::value::BramaPrimative;
    use super::*;

    use crate::nativecall_test;


    nativecall_test!{test_lowercase_1, lowercase, BramaPrimative::Text(Arc::new("TÜRKİYE".to_string())), BramaPrimative::Text(Arc::new("türkiye".to_string()))}
}
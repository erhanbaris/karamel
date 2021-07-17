use std::{rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use crate::compiler::function::FunctionReference;

use super::{OpcodeGeneratorTrait, DumpBuilder};


#[derive(Clone)]
/// Generate jump opcodes. 
pub struct FunctionGenerator {
    pub function: Rc<FunctionReference>
}

impl OpcodeGeneratorTrait for FunctionGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        (*self.function).opcode_location.set(opcodes.len());
        opcodes.push(self.function.arguments.len() as u8);
    }

    fn dump<'a>(&self, builder: &'a DumpBuilder, index: Rc<AtomicUsize>, opcodes: &Vec<u8>) {
        let opcode_index = index.fetch_add(1, Ordering::SeqCst);
        builder.add_with_text(opcode_index, format!("[FUNCTION: {}]", self.function.name), opcodes[opcode_index].to_string(), "".to_string(), "".to_string());
    }
}
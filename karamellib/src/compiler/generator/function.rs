use std::{rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use crate::compiler::{VmOpCode, function::FunctionReference};

use super::{OpcodeGeneratorTrait, dump_default};


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

    fn dump(&self, index: Rc<AtomicUsize>, opcodes: &Vec<u8>, buffer: &mut String) {
        let opcode_index = index.fetch_add(1, Ordering::SeqCst);
        dump_default(opcode_index, format!("[{}]", self.function.name), buffer, opcodes[opcode_index].to_string(), "", "");
    }
}
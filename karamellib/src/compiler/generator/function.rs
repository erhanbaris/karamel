use std::{rc::Rc, sync::atomic::AtomicUsize};

use crate::compiler::{VmOpCode, function::FunctionReference};

use super::{OpcodeGeneratorTrait};


#[derive(Clone)]
/// Generate jump opcodes. 
pub struct FunctionGenerator {
    pub function: Rc<FunctionReference>
}

impl OpcodeGeneratorTrait for FunctionGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        (*self.function).opcode_location.set(opcodes.len());
        opcodes.push(self.function.arguments.len() as u8);

        if !self.function.arguments.is_empty() {
            opcodes.push(VmOpCode::InitArguments.into());
            opcodes.push(self.function.arguments.len() as u8);
        }
    }

    fn dump(&self, index: Rc<AtomicUsize>, opcodes: &Vec<u8>, buffer: &mut String) {

    }
}
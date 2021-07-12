use crate::compiler::{KaramelCompilerContext, VmOpCode};

use super::OpcodeGeneratorTrait;


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct OpcodeItem {
    pub opcode: VmOpCode
}

impl OpcodeGeneratorTrait for OpcodeItem {
    fn generate(&self, context: &mut KaramelCompilerContext) {
        context.opcodes.push(self.opcode.into());
    }
}
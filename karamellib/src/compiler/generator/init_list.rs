use crate::compiler::{KaramelCompilerContext, VmOpCode};

use super::OpcodeGeneratorTrait;

#[derive(Debug)]
#[derive(Clone)]
pub struct InitListGenerator {
    pub argument_size: usize
}

impl OpcodeGeneratorTrait for InitListGenerator {
    fn generate(&self, context: &mut KaramelCompilerContext) {
        context.opcodes.push(VmOpCode::InitList.into());
        context.opcodes.push(self.argument_size as u8);
    }
}

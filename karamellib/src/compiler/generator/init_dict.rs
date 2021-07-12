use crate::compiler::{KaramelCompilerContext, VmOpCode};

use super::OpcodeGeneratorTrait;

#[derive(Debug)]
#[derive(Clone)]
pub struct InitDictGenerator {
    pub argument_size: usize
}

impl OpcodeGeneratorTrait for InitDictGenerator {
    fn generate(&self, context: &mut KaramelCompilerContext) {
        context.opcodes.push(VmOpCode::InitDict.into());
        context.opcodes.push(self.argument_size as u8);
    }
}

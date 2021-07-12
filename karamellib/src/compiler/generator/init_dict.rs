use crate::compiler::VmOpCode;

use super::OpcodeGeneratorTrait;

#[derive(Debug)]
#[derive(Clone)]
pub struct InitDictGenerator {
    pub argument_size: usize
}

impl OpcodeGeneratorTrait for InitDictGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        opcodes.push(VmOpCode::InitDict.into());
        opcodes.push(self.argument_size as u8);
    }
}

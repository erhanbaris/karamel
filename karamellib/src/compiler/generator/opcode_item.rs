use crate::compiler::VmOpCode;

use super::OpcodeGeneratorTrait;


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct OpcodeItem {
    pub opcode: VmOpCode
}

impl OpcodeGeneratorTrait for OpcodeItem {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        opcodes.push(self.opcode.into());
    }
}
use crate::compiler::{KaramelCompilerContext, VmOpCode};

use super::OpcodeGeneratorTrait;


#[derive(Clone)]
pub struct LoadGenerator { pub location: u8 }
impl OpcodeGeneratorTrait for LoadGenerator {
    fn generate(&self, context: &mut KaramelCompilerContext) {
        context.opcodes.push(VmOpCode::Load as u8);
        context.opcodes.push(self.location);
    }
}
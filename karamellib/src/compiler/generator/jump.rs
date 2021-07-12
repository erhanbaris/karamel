use std::rc::Rc;

use crate::compiler::{KaramelCompilerContext, VmOpCode};

use super::{OpcodeGeneratorTrait, OpcodeLocation};

#[derive(Clone)]
/// Generate jump opcodes. 
pub struct JumpGenerator { pub location:  Rc<OpcodeLocation> }
impl OpcodeGeneratorTrait for JumpGenerator {
    fn generate(&self, context: &mut KaramelCompilerContext) {
        context.opcodes.push(VmOpCode::Jump.into());
        context.opcodes.push(self.location.get() as u8);
        context.opcodes.push((self.location.get() >> 8) as u8);
    }
}
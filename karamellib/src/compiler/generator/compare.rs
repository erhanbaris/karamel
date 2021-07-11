use std::rc::Rc;

use crate::compiler::{KaramelCompilerContext, VmOpCode};

use super::{OpcodeGeneratorTrait, OpcodeLocation};


#[derive(Clone)]
/// Generate compare opcodes
pub struct CompareGenerator { pub location: Rc<OpcodeLocation> }
impl OpcodeGeneratorTrait for CompareGenerator {
    fn generate(&self, context: &mut KaramelCompilerContext) {
        context.opcodes.push(VmOpCode::Compare as u8);
        context.opcodes.push(self.get() as u8);
        context.opcodes.push((self.get() >> 8) as u8);
    }
}

impl CompareGenerator {
    pub fn get(&self) -> usize {
        self.location.get()
    }

    pub fn set(&self, location: usize) {
        self.location.set(location);
    }
}

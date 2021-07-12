use crate::compiler::{KaramelCompilerContext, VmOpCode};

use super::{OpcodeGeneratorTrait};


#[derive(Clone)]
/// Generate function call opcodes
pub struct CallGenerator { 
    pub function_location: u8,
    pub argument_size: u8,
    pub assign_to_temp: bool
}

impl OpcodeGeneratorTrait for CallGenerator {
    fn generate(&self, context: &mut KaramelCompilerContext) {
        context.opcodes.push(VmOpCode::Call.into());
        context.opcodes.push(self.function_location);
        context.opcodes.push(self.argument_size);
        context.opcodes.push(self.assign_to_temp.into());
    }
}

use std::rc::Rc;

use crate::compiler::{KaramelCompilerContext, VmOpCode, function::FunctionReference};

use super::{OpcodeGeneratorTrait};


#[derive(Clone)]
/// Generate jump opcodes. 
pub struct FunctionGenerator(Rc<FunctionReference>);
impl OpcodeGeneratorTrait for FunctionGenerator {
    fn generate(&self, context: &mut KaramelCompilerContext) {
        context.opcodes.push(VmOpCode::Func.into());
        (*self.0).opcode_location.set(context.opcodes.len());
        context.opcodes.push(self.0.arguments.len() as u8);

        if !self.0.arguments.is_empty() {
            context.opcodes.push(VmOpCode::InitArguments.into());
            context.opcodes.push(self.0.arguments.len() as u8);
        }
    }
}
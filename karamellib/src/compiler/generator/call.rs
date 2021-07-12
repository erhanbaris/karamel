use crate::compiler::VmOpCode;

use super::{OpcodeGeneratorTrait};

#[derive(Clone)]
pub enum CallType {
    Call { location: u8 },
    CallStack
}

#[derive(Clone)]
/// Generate function call opcodes
pub struct CallGenerator { 
    pub call_type: CallType,
    pub argument_size: u8,
    pub assign_to_temp: bool
}

impl OpcodeGeneratorTrait for CallGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        match self.call_type {
            CallType::Call { location } => {
                opcodes.push(VmOpCode::Call.into());
                opcodes.push(location);
            },
            CallType::CallStack => opcodes.push(VmOpCode::CallStack.into())
        };
        opcodes.push(self.argument_size);
        opcodes.push(self.assign_to_temp.into());
    }
}

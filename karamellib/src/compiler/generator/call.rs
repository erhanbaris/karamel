use std::{rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use crate::compiler::VmOpCode;

use super::{DumpBuilder, OpcodeGeneratorTrait};

#[derive(Clone)]

/// Function call type. Karamel is support two type of function call mechanism
pub enum CallType {

    /// Call function from memory location
    Call { constant_location: u8 },

    /// Call function from last stack value
    CallStack
}

#[derive(Clone)]
/// Generate function call opcodes. 
pub struct CallGenerator { 

    /// Type of the function call type.
    pub call_type: CallType,

    /// How many arguments are passed to function
    pub argument_size: u8,

    /// Function return value needs to be assigned to stack location or discarded
    pub assign_to_temp: bool
}

/// Generate function call opcodes based on givin parameters
impl OpcodeGeneratorTrait for CallGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        match self.call_type {
            CallType::Call { constant_location } => {
                opcodes.push(VmOpCode::Call.into());
                opcodes.push(constant_location);
            },
            CallType::CallStack => opcodes.push(VmOpCode::CallStack.into())
        };
        opcodes.push(self.argument_size);
        opcodes.push(self.assign_to_temp.into());
    }

    fn dump<'a>(&self, builder: &'a DumpBuilder, index: Rc<AtomicUsize>, _: &Vec<u8>) {
        let opcode_index = index.fetch_add(3, Ordering::SeqCst);

        match self.call_type {
            CallType::Call { constant_location } => {
                index.fetch_add(1, Ordering::SeqCst);
                builder.add(opcode_index, VmOpCode::Call, constant_location.to_string(), self.argument_size.to_string(), (self.assign_to_temp as u8).to_string());
            },
            CallType::CallStack => {
                builder.add(opcode_index, VmOpCode::CallStack, self.argument_size.to_string(), (self.assign_to_temp as u8).to_string(), "".to_string());
            }
        };
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut opcodes = Vec::new();
        let generator = CallGenerator {
            call_type: CallType::Call { constant_location: 100 },
            argument_size: 1,
            assign_to_temp: false
        };

        generator.generate(&mut opcodes);

        assert_eq!(opcodes.len(), 4);
        assert_eq!(opcodes[0], VmOpCode::Call.into());
        assert_eq!(opcodes[1], 100);
        assert_eq!(opcodes[2], 1);
        assert_eq!(opcodes[3], 0);
    }

    #[test]
    fn test_2() {
        let mut opcodes = Vec::new();
        let generator = CallGenerator {
            call_type: CallType::CallStack,
            argument_size: 5,
            assign_to_temp: true
        };

        generator.generate(&mut opcodes);

        assert_eq!(opcodes.len(), 3);
        assert_eq!(opcodes[0], VmOpCode::CallStack.into());
        assert_eq!(opcodes[1], 5);
        assert_eq!(opcodes[2], 1);
    }

    #[test]
    fn test_3() {
        let mut opcodes = Vec::new();
        let generator = CallGenerator {
            call_type: CallType::Call { constant_location: 100 },
            argument_size: 5,
            assign_to_temp: true
        };

        generator.generate(&mut opcodes);

        assert_eq!(opcodes.len(), 4);
        assert_eq!(opcodes[0], VmOpCode::Call.into());
        assert_eq!(opcodes[1], 100);
        assert_eq!(opcodes[2], 5);
        assert_eq!(opcodes[3], 1);
    }
}
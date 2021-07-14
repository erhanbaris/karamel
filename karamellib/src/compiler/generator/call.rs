use crate::compiler::VmOpCode;

use super::{OpcodeGeneratorTrait};

#[derive(Clone)]

/// Function call type. Karamel is support two type of function call mechanism
pub enum CallType {

    /// Call function from memory location
    Call { location: u8 },

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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut opcodes = Vec::new();
        let generator = CallGenerator {
            call_type: CallType::Call { location: 100 },
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
            call_type: CallType::Call { location: 100 },
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
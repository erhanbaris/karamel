use std::rc::Rc;

use crate::compiler::VmOpCode;

use super::{OpcodeGeneratorTrait, OpcodeLocation};


#[derive(Clone)]
/// Generate compare opcodes
pub struct CompareGenerator { pub location: Rc<OpcodeLocation> }
impl OpcodeGeneratorTrait for CompareGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        opcodes.push(VmOpCode::Compare.into());
        self.location.apply(opcodes);
    }
}

impl CompareGenerator {
    pub fn get(&self) -> usize {
        self.location.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut opcodes = Vec::new();
        let location =  Rc::new(OpcodeLocation::new(123));
        let generator = CompareGenerator {
            location: location.clone()
        };

        generator.generate(&mut opcodes);

        assert_eq!(opcodes.len(), 3);
        assert_eq!(opcodes[0], VmOpCode::Compare.into());
        assert_eq!(opcodes[1], 123);
        assert_eq!(opcodes[2], 0);
    }

    #[test]
    fn test_2() {
        let mut opcodes = Vec::new();
        let location =  Rc::new(OpcodeLocation::new(123456789));
        let generator = CompareGenerator {
            location: location.clone()
        };

        generator.generate(&mut opcodes);

        assert_eq!(opcodes.len(), 3);
        assert_eq!(opcodes[0], VmOpCode::Compare.into());
        assert_eq!(opcodes[1], 21);
        assert_eq!(opcodes[2], 205);
    }
}
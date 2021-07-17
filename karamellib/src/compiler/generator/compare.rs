use std::{rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use crate::compiler::VmOpCode;

use super::{DumpBuilder, OpcodeGeneratorTrait, OpcodeLocation, opcode_to_location};


#[derive(Clone)]
/// Generate compare opcodes
pub struct CompareGenerator { pub location: Rc<OpcodeLocation> }
impl OpcodeGeneratorTrait for CompareGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        opcodes.push(VmOpCode::Compare.into());
        self.location.apply(opcodes);
    }

    fn dump<'a>(&self, builder: &'a DumpBuilder, index: Rc<AtomicUsize>, opcodes: &Vec<u8>) {
        let opcode_index = index.fetch_add(1, Ordering::SeqCst);
        let location = opcode_to_location(index, opcodes);
        builder.add(opcode_index, VmOpCode::Compare, location.to_string(), "".to_string(), "".to_string());
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
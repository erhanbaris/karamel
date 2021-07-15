use std::{rc::Rc, sync::atomic::AtomicUsize};

use crate::compiler::VmOpCode;

use super::OpcodeGeneratorTrait;


#[derive(Clone)]
pub struct LoadGenerator { pub location: u8 }
impl OpcodeGeneratorTrait for LoadGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        opcodes.push(VmOpCode::Load.into());
        opcodes.push(self.location);
    }

    fn dump(&self, index: Rc<AtomicUsize>, opcodes: &Vec<u8>, buffer: &mut String) {

    }
}
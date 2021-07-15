use std::{rc::Rc, sync::atomic::AtomicUsize};

use crate::compiler::VmOpCode;

use super::{OpcodeGeneratorTrait, OpcodeLocation};

#[derive(Clone)]
/// Generate jump opcodes. 
pub struct JumpGenerator { pub location:  Rc<OpcodeLocation> }
impl OpcodeGeneratorTrait for JumpGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        opcodes.push(VmOpCode::Jump.into());
        self.location.apply(opcodes);
    }

    fn dump(&self, index: Rc<AtomicUsize>, opcodes: &Vec<u8>, buffer: &mut String) {

    }
}
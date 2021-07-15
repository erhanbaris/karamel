use std::{rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use crate::compiler::VmOpCode;

use super::{OpcodeGeneratorTrait, OpcodeLocation, dump_default, opcode_to_location};

#[derive(Clone)]
/// Generate jump opcodes. 
pub struct JumpGenerator { pub location:  Rc<OpcodeLocation> }
impl OpcodeGeneratorTrait for JumpGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        opcodes.push(VmOpCode::Jump.into());
        self.location.apply(opcodes);
    }

    fn dump(&self, index: Rc<AtomicUsize>, opcodes: &Vec<u8>, buffer: &mut String) {
        let opcode_index = index.fetch_add(1, Ordering::SeqCst);
        let location = opcode_to_location(index, opcodes);
        dump_default(opcode_index, VmOpCode::Jump.to_string(), buffer, location.to_string(), "", "");
    }
}
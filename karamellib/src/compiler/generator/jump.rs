use std::{rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use crate::compiler::VmOpCode;

use super::{DumpBuilder, OpcodeGeneratorTrait, OpcodeLocation, opcode_to_location};

#[derive(Clone)]
/// Generate jump opcodes. 
pub struct JumpGenerator { pub location:  Rc<OpcodeLocation> }
impl OpcodeGeneratorTrait for JumpGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        opcodes.push(VmOpCode::Jump.into());
        self.location.apply(opcodes);
    }

    fn dump<'a>(&self, builder: &'a DumpBuilder, index: Rc<AtomicUsize>, opcodes: &Vec<u8>) {
        let opcode_index = index.fetch_add(1, Ordering::SeqCst);
        let location = opcode_to_location(index, opcodes);
        builder.add(opcode_index, VmOpCode::Jump, location.to_string(), "".to_string(), "".to_string());
    }
}
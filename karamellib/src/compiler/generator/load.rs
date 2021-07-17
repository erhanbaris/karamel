use std::{rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use crate::compiler::VmOpCode;

use super::{DumpBuilder, OpcodeGeneratorTrait};


#[derive(Clone)]
pub struct LoadGenerator { pub location: u8 }
impl OpcodeGeneratorTrait for LoadGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        opcodes.push(VmOpCode::Load.into());
        opcodes.push(self.location);
    }

    fn dump<'a>(&self, builder: &'a DumpBuilder, index: Rc<AtomicUsize>, _: &Vec<u8>) {
        let opcode_index = index.fetch_add(2, Ordering::SeqCst);
        builder.add(opcode_index, VmOpCode::Load, self.location.to_string(), "".to_string(), "".to_string());
    }
}
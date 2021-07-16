use std::{rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use crate::compiler::VmOpCode;

use super::{OpcodeGeneratorTrait, dump_default};


#[derive(Clone)]
pub struct LoadGenerator { pub location: u8 }
impl OpcodeGeneratorTrait for LoadGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        opcodes.push(VmOpCode::Load.into());
        opcodes.push(self.location);
    }

    fn dump(&self, index: Rc<AtomicUsize>, _: &Vec<u8>, buffer: &mut String) {
        let opcode_index = index.fetch_add(1, Ordering::SeqCst);
        dump_default(opcode_index, VmOpCode::Load.to_string(), buffer, self.location.to_string(), "", "");
    }
}
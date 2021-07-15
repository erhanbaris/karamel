use std::{rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use crate::{compiler::VmOpCode};

use super::{OpcodeGeneratorTrait, dump_single_opcode};


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct OpcodeItem {
    pub opcode: VmOpCode
}

impl OpcodeGeneratorTrait for OpcodeItem {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        opcodes.push(self.opcode.into());
    }

    fn dump(&self, index: Rc<AtomicUsize>, _: &Vec<u8>, buffer: &mut String) {
        let opcode_index = index.fetch_add(1, Ordering::SeqCst);
        dump_single_opcode(opcode_index, self.opcode.to_string(), buffer);
    }
}

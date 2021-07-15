use std::{borrow::Borrow, rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use crate::{compiler::VmOpCode, constants::{DUMP_INDEX_WIDTH, DUMP_OPCODE_WIDTH}};

use super::{OpcodeGeneratorTrait, print_opcode};


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
        print_opcode(opcode_index, self.opcode, buffer);
    }
}

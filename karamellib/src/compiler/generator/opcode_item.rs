use std::{rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use crate::{compiler::VmOpCode};

use super::{DumpBuilder, OpcodeGeneratorTrait};


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

    fn dump<'a>(&self, builder: &'a DumpBuilder, index: Rc<AtomicUsize>, _: &Vec<u8>) {
        let opcode_index = index.fetch_add(1, Ordering::SeqCst);
        builder.add(opcode_index, self.opcode, "".to_string(), "".to_string(), "".to_string());
    }
}

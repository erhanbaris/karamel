use std::{rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use crate::compiler::VmOpCode;

use super::{DumpBuilder, OpcodeGeneratorTrait};

#[derive(Debug)]
#[derive(Clone)]
pub struct InitListGenerator {
    pub argument_size: usize
}

impl OpcodeGeneratorTrait for InitListGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        opcodes.push(VmOpCode::Init.into());
        opcodes.push(1);
        opcodes.push(self.argument_size as u8);
    }

    fn dump<'a>(&self, builder: &'a DumpBuilder, index: Rc<AtomicUsize>, _: &Vec<u8>) {
        let opcode_index = index.fetch_add(3, Ordering::SeqCst);
        builder.add(opcode_index, VmOpCode::Init, "1".to_string(), self.argument_size.to_string(), "".to_string());
    }
}

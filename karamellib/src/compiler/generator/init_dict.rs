use std::{rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use crate::compiler::VmOpCode;

use super::{DumpBuilder, OpcodeGeneratorTrait};

#[derive(Debug)]
#[derive(Clone)]
pub struct InitDictGenerator {
    pub argument_size: usize
}

impl<'a> OpcodeGeneratorTrait<'a> for InitDictGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        opcodes.push(VmOpCode::Init.into());
        opcodes.push(0);
        opcodes.push(self.argument_size as u8);
    }

    fn dump(&self, builder: &DumpBuilder, index: Rc<AtomicUsize>, _: &Vec<u8>) {
        let opcode_index = index.fetch_add(3, Ordering::SeqCst);
        builder.add(opcode_index, VmOpCode::Init, "0".to_string(), self.argument_size.to_string(), "".to_string());
    }
}

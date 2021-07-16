use std::{rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use crate::compiler::VmOpCode;

use super::{OpcodeGeneratorTrait, dump_default};

#[derive(Debug)]
#[derive(Clone)]
pub struct InitDictGenerator {
    pub argument_size: usize
}

impl OpcodeGeneratorTrait for InitDictGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        opcodes.push(VmOpCode::InitDict.into());
        opcodes.push(self.argument_size as u8);
    }

    fn dump(&self, index: Rc<AtomicUsize>, _: &Vec<u8>, buffer: &mut String) {
        let opcode_index = index.fetch_add(1, Ordering::SeqCst);
        dump_default(opcode_index, VmOpCode::InitDict.to_string(), buffer, self.argument_size.to_string(), "", "");
    }
}

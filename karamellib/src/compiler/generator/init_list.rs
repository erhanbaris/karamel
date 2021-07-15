use std::{rc::Rc, sync::atomic::AtomicUsize};

use crate::compiler::VmOpCode;

use super::OpcodeGeneratorTrait;

#[derive(Debug)]
#[derive(Clone)]
pub struct InitListGenerator {
    pub argument_size: usize
}

impl OpcodeGeneratorTrait for InitListGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        opcodes.push(VmOpCode::InitList.into());
        opcodes.push(self.argument_size as u8);
    }

    fn dump(&self, index: Rc<AtomicUsize>, opcodes: &Vec<u8>, buffer: &mut String) {

    }
}

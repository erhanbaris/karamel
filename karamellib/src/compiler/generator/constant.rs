use std::{
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::compiler::VmOpCode;

use super::{DumpBuilder, OpcodeGeneratorTrait};

#[derive(Clone)]
pub struct ConstantGenerator {
    pub location: u8,
}
impl OpcodeGeneratorTrait for ConstantGenerator {
    fn generate(&self, opcodes: &mut Vec<u8>) {
        opcodes.push(VmOpCode::Constant.into());
        opcodes.push(self.location);
    }

    fn dump(&self, builder: &DumpBuilder, index: Rc<AtomicUsize>, _: &Vec<u8>) {
        let opcode_index = index.fetch_add(2, Ordering::SeqCst);
        builder.add(opcode_index, VmOpCode::Constant, self.location.to_string(), "".to_string(), "".to_string());
    }
}

use crate::compiler::{KaramelCompilerContext, VmOpCode};

use super::OpcodeGeneratorTrait;

#[derive(Debug)]
#[derive(Clone)]
pub enum StoreType {
    Store(u8),
    FastStore {
        destination: u8,
        source: u8
    },
    CopyToStore(u8)
}

#[derive(Debug)]
#[derive(Clone)]
pub struct StoreGenerator {
    pub store_type: StoreType 
}

impl OpcodeGeneratorTrait for StoreGenerator {
    fn generate(&self, context: &mut KaramelCompilerContext) {
        match self.store_type {
            StoreType::Store(destination) => {
                context.opcodes.push(VmOpCode::Store.into());
                context.opcodes.push(destination);
            },
            StoreType::CopyToStore(destination) => {
                context.opcodes.push(VmOpCode::CopyToStore.into());
                context.opcodes.push(destination);
            },
            StoreType::FastStore { destination, source} => {
                context.opcodes.push(VmOpCode::FastStore.into());
                context.opcodes.push(destination);
                context.opcodes.push(source);
            }
        };
    }
}

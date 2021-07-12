use crate::compiler::VmOpCode;

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
    fn generate(&self, opcodes: &mut Vec<u8>) {
        match self.store_type {
            StoreType::Store(destination) => {
                opcodes.push(VmOpCode::Store.into());
                opcodes.push(destination);
            },
            StoreType::CopyToStore(destination) => {
                opcodes.push(VmOpCode::CopyToStore.into());
                opcodes.push(destination);
            },
            StoreType::FastStore { destination, source} => {
                opcodes.push(VmOpCode::FastStore.into());
                opcodes.push(destination);
                opcodes.push(source);
            }
        };
    }
}

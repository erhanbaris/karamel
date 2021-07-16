use std::{rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use crate::compiler::VmOpCode;

use super::{OpcodeGeneratorTrait, dump_default};

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

    fn dump(&self, index: Rc<AtomicUsize>, _: &Vec<u8>, buffer: &mut String) {
        let opcode_index = index.fetch_add(2, Ordering::SeqCst);
        
        match self.store_type {
            StoreType::Store(destination) => {
                dump_default(opcode_index, VmOpCode::Store.to_string(), buffer, destination.to_string(), "", "");
            },
            StoreType::CopyToStore(destination) => {
                dump_default(opcode_index, VmOpCode::CopyToStore.to_string(), buffer, destination.to_string(), "", "");

            },
            StoreType::FastStore { destination, source} => {
                dump_default(opcode_index, VmOpCode::FastStore.to_string(), buffer, destination.to_string(), source.to_string(), "");
                index.fetch_add(1, Ordering::SeqCst);
            }
        };
    }
}

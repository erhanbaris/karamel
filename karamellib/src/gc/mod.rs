use crate::compiler::BramaPrimative;
use crate::compiler::Scope;
use std::sync::Arc;

use broom::prelude::*;

pub enum HeapItem {
    Number(f64),
    Text(Arc<String>),
    Primative(Arc<BramaPrimative>),
    Scope(Scope)
}

impl Trace<Self> for HeapItem {
    fn trace(&self, _: &mut Tracer<Self>) { }
}

pub struct HeapAllocator {
    heap: Heap<HeapItem>
}

impl HeapAllocator {
    pub fn new() -> HeapAllocator {
        HeapAllocator {
            heap: Heap::default()
        }
    }

    pub fn add_scope(&mut self, scope: Scope) -> *mut Scope {
        let address = &scope as *const Scope as *mut Scope;
        self.heap.insert(HeapItem::Scope(scope));
        address
    }

    pub fn add_string(&mut self, text: Arc<String>) {
        self.heap.insert(HeapItem::Text(text));
    }

    pub fn add_primative(&mut self, primative: Arc<BramaPrimative>) {
        self.heap.insert(HeapItem::Primative(primative));
    }

    pub fn clean(&mut self) {
        self.heap.clean();
    }
}
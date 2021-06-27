use std::ptr;

use crate::types::VmObject;


#[derive(Clone)]
pub struct Scope {
    pub memory: Vec<VmObject>, 
    pub stack: Vec<VmObject>, 
    pub location: *mut u8,
    pub call_return_assign_to_temp: bool,
    pub const_size: u8,
    pub stack_ptr: *mut VmObject ,
    pub memory_ptr: *mut VmObject ,
    pub storage_index: isize
}

impl Scope {
    pub fn empty() -> Scope {
        let mut stack = Vec::new();
        let mut memory = Vec::new();
        
        let stack_ptr = stack.as_mut_ptr();
        let memory_ptr = memory.as_mut_ptr();

        Scope { 
            const_size: 0, 
            call_return_assign_to_temp: false, 
            location: ptr::null_mut(), 
            memory: memory,
            memory_ptr: memory_ptr,
            stack: stack,
            stack_ptr: stack_ptr,
            storage_index: -1
        }
    }
}
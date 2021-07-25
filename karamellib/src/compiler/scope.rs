use std::ptr;

use crate::types::VmObject;


#[derive(Clone)]
pub struct Scope {
    pub memory: Vec<VmObject>, 
    pub location: *mut u8,
    pub call_return_assign_to_temp: bool,
    pub memory_ptr: *mut VmObject,
    pub constant_ptr: *const VmObject ,
    pub storage_index: isize
}

impl Scope {
    pub fn empty() -> Scope {
        let mut memory = Vec::new();
        
        let memory_ptr = memory.as_mut_ptr();

        Scope {
            call_return_assign_to_temp: false, 
            location: ptr::null_mut(), 
            memory: memory,
            memory_ptr: memory_ptr,
            storage_index: -1,
            constant_ptr: ptr::null()
        }
    }
}
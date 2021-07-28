use std::ptr;

use crate::types::VmObject;


#[derive(Clone)]
pub struct Scope {
    pub location: *mut u8,
    pub call_return_assign_to_temp: bool,
    pub stack_ptr: *mut VmObject,
    pub constant_ptr: *const VmObject ,
    pub storage_index: isize
}

impl Scope {
    pub fn empty() -> Scope {
        Scope {
            call_return_assign_to_temp: false, 
            location: ptr::null_mut(), 
            stack_ptr: ptr::null_mut(), 
            storage_index: -1,
            constant_ptr: ptr::null()
        }
    }
}
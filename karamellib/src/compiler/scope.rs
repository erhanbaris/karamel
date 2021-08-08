use std::ptr;

use crate::types::VmObject;


#[derive(Clone)]
pub struct Scope {
    pub location: *mut u8,
    pub call_return_assign_to_temp: bool,
    pub top_stack: *mut VmObject,
    pub constant_ptr: *const VmObject
}

impl Scope {
    pub fn empty() -> Scope {
        Scope {
            call_return_assign_to_temp: false, 
            location: ptr::null_mut(), 
            top_stack: ptr::null_mut(), 
            constant_ptr: ptr::null()
        }
    }
}
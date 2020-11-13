use std::vec::Vec;

pub type BramaByte  = i32;
pub type BramaValue = u64;
pub const SIGN_BIT: u64  = (1 as u64) << 63;

// The bits that must be set to indicate a quiet NaN.
pub const QNAN : u64 = 0x7ffc000000000000;

struct Strorage {
    pub id: u16,
    pub temp_count: u16,
    pub constant_count: u16,
    pub variable_count: u16,
    pub loop_counter: u16,
    pub temp_counter: u16,
    pub local_define: Vec<bool>
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaVmOpCode {
    Halt,
    Addition(u8, u8, u8),
    Subraction(u8, u8, u8),
    Multiply(u8, u8, u8),
    Divition(u8, u8, u8),
    Modulo(u8, u8, u8)
}


pub trait BramaValueTrait {
    fn is_integer(&self) -> bool;
    fn is_object(&self) -> bool;
}

impl BramaValueTrait for BramaValue {
    fn is_integer(&self) -> bool {
        *self & QNAN != QNAN
    }

    fn is_object(&self) -> bool {
        *self & (QNAN | SIGN_BIT) == QNAN | SIGN_BIT
    }
}

pub fn run_vm(opcodes: &Vec<BramaVmOpCode>)
{
    for op in opcodes {

    }
}
use std::vec::Vec;
use std::mem::replace;

use crate::types::*;
use crate::compiler::*;

struct Strorage {
    pub id: u16,
    pub temp_count: u16,
    pub constant_count: u16,
    pub variable_count: u16,
    pub loop_counter: u16,
    pub temp_counter: u16,
    pub local_define: Vec<bool>
}

pub fn run_vm(options: &mut BramaCompilerOption)
{
    let memory = &mut options.storages[0].memory;
    for op in &options.opcodes {
        match op {
            BramaVmOpCode::And { target, left, right } => {
                //std::mem::replace(memory[*target as usize], 1);
                memory[*target as usize] = match (&memory[*left as usize], &memory[*right as usize]) {
                    (VmObjectType::Empty, _) | (_, VmObjectType::Empty)=> VmObjectType::Empty,
                    (VmObjectType::Integer(l_integer), VmObjectType::Integer(r_integer)) => VmObjectType::Bool(*l_integer > 0 && *r_integer > 0),
                    _ => VmObjectType::Empty
                };
            },
            BramaVmOpCode::Addition { target, left, right } => {
                //std::mem::replace(memory[*target as usize], 1);
                memory[*target as usize] = match (&memory[*left as usize], &memory[*right as usize]) {
                    (VmObjectType::Empty, _) | (_, VmObjectType::Empty)                  => VmObjectType::Empty,
                    (VmObjectType::Integer(l_integer), VmObjectType::Integer(r_integer)) => VmObjectType::Integer(*l_integer + *r_integer),
                    (VmObjectType::Double (l_double),  VmObjectType::Double(r_double))   => VmObjectType::Double(*l_double + *r_double),
                    (VmObjectType::Integer(l_integer), VmObjectType::Double(r_double))   => VmObjectType::Double(*l_integer as f64 + *r_double),
                    (VmObjectType::Double(l_double),   VmObjectType::Integer(r_integer)) => VmObjectType::Double(*l_double + *r_integer as f64),
                    (VmObjectType::Bool(l_bool),       VmObjectType::Bool(r_bool))       => VmObjectType::Bool(*l_bool && *r_bool),
                    (VmObjectType::Text(l_text),       VmObjectType::Text(r_text))       => VmObjectType::Text(l_text.to_owned() + r_text),
                    _ => VmObjectType::Empty
                };
            },
            _ => ()
        }
    }

    println!("{:?}", memory);
}
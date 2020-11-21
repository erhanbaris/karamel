use crate::types::*;
use std::rc::Rc;

pub fn run_vm(options: &mut BramaCompilerOption)
{
    let memory = &mut options.storages[0].memory;
    for op in &options.opcodes {
        match op {
            BramaVmOpCode::And { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value), BramaPrimative::Number(r_value)) => VmObject::convert(BramaPrimative::Bool(*l_value > 0.0 && *r_value > 0.0)),
                    (BramaPrimative::Bool(l_value),   BramaPrimative::Bool(r_value))   => VmObject::convert(BramaPrimative::Bool(*l_value && *r_value)),
                    _ => VmObject::convert(BramaPrimative::Empty)
                };
            },
            BramaVmOpCode::Or { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),   BramaPrimative::Number(r_value))   => VmObject::convert(BramaPrimative::Bool(*l_value > 0.0 || *r_value > 0.0)),
                    (BramaPrimative::Bool(l_value),     BramaPrimative::Bool(r_value))     => VmObject::convert(BramaPrimative::Bool(*l_value && *r_value)),
                    _ => VmObject::convert(BramaPrimative::Empty)
                };
            },
            BramaVmOpCode::Addition { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value)) => VmObject::convert(BramaPrimative::Number(*l_value + *r_value)),
                    (BramaPrimative::Text(l_value),    BramaPrimative::Text(r_value))   => {
                        VmObject::convert(BramaPrimative::Text(Rc::new((&**l_value).to_owned() + &**r_value)))
                    },
                    _ => VmObject::convert(BramaPrimative::Empty)
                };
            },
            BramaVmOpCode::Multiply { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::convert(BramaPrimative::Number(*l_value * *r_value)),
                    _ => VmObject::convert(BramaPrimative::Empty)
                };
            },
            BramaVmOpCode::Division { target, left, right } => {
                let calculation = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => (*l_value / *r_value),
                    _ => std::f64::NAN
                };

                memory[*target as usize] = if calculation.is_nan() {
                    VmObject::convert(BramaPrimative::Empty)
                }
                else {
                    VmObject::convert(BramaPrimative::Number(calculation))
                }
            },
            BramaVmOpCode::Subraction { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::convert(BramaPrimative::Number(*l_value - *r_value)),
                    _ => VmObject::convert(BramaPrimative::Empty)
                };
            },
            BramaVmOpCode::Equal { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Empty,               BramaPrimative::Empty)               => VmObject::convert(BramaPrimative::Bool(true)),
                    (BramaPrimative::Atom(l_value),       BramaPrimative::Atom(r_value))       => VmObject::convert(BramaPrimative::Bool(*l_value == *r_value)),
                    (BramaPrimative::Bool(l_value),       BramaPrimative::Bool(r_value))       => VmObject::convert(BramaPrimative::Bool(*l_value == *r_value)),
                    (BramaPrimative::Number(l_value),     BramaPrimative::Number(r_value))     => VmObject::convert(BramaPrimative::Bool(*l_value == *r_value)),
                    (BramaPrimative::Text(l_value),       BramaPrimative::Text(r_value))       => VmObject::convert(BramaPrimative::Bool(*l_value == *r_value)),
                    _ => VmObject::convert(BramaPrimative::Empty)
                };
            },
            BramaVmOpCode::NotEqual { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Empty,               BramaPrimative::Empty)               => VmObject::convert(BramaPrimative::Bool(false)),
                    (BramaPrimative::Atom(l_value),       BramaPrimative::Atom(r_value))       => VmObject::convert(BramaPrimative::Bool(*l_value != *r_value)),
                    (BramaPrimative::Bool(l_value),       BramaPrimative::Bool(r_value))       => VmObject::convert(BramaPrimative::Bool(*l_value != *r_value)),
                    (BramaPrimative::Number(l_value),     BramaPrimative::Number(r_value))     => VmObject::convert(BramaPrimative::Bool(*l_value != *r_value)),
                    (BramaPrimative::Text(l_value),       BramaPrimative::Text(r_value))       => VmObject::convert(BramaPrimative::Bool(*l_value != *r_value)),
                    _ => VmObject::convert(BramaPrimative::Empty)
                };
            },
            BramaVmOpCode::GreaterThan { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::convert(BramaPrimative::Bool(*l_value > *r_value)),
                    _ => VmObject::convert(BramaPrimative::Empty)
                };
            },
            BramaVmOpCode::GreaterEqualThan { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::convert(BramaPrimative::Bool(*l_value >= *r_value)),
                    _ => VmObject::convert(BramaPrimative::Empty)
                };
            },
            BramaVmOpCode::LessThan { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::convert(BramaPrimative::Bool(*l_value < *r_value)),
                    _ => VmObject::convert(BramaPrimative::Empty)
                };
            },
            BramaVmOpCode::LessEqualThan { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::convert(BramaPrimative::Bool(*l_value <= *r_value)),
                    _ => VmObject::convert(BramaPrimative::Empty)
                };
            },
            _ => ()
        }
    }

    options.storages[0].dump();
}
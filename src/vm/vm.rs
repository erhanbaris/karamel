use crate::types::*;
use crate::compiler::*;
use std::rc::Rc;

pub fn run_vm(options: &mut BramaCompilerOption)
{
    let empty_primative: VmObject  = VmObject::convert(Rc::new(BramaPrimative::Empty));
    let false_primative : VmObject = VmObject::convert(Rc::new(BramaPrimative::Bool(false)));
    let true_primative : VmObject  = VmObject::convert(Rc::new(BramaPrimative::Bool(true)));
    
    let memory = &mut options.storages[0].memory;
    for op in &options.opcodes {
        match op {
            BramaVmOpCode::And { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value), BramaPrimative::Number(r_value)) => if *l_value > 0.0 && *r_value > 0.0 { true_primative } else { false_primative },
                    (BramaPrimative::Bool(l_value),   BramaPrimative::Bool(r_value))   => if *l_value && *r_value { true_primative } else { false_primative },
                    _ => empty_primative
                };
            },
            BramaVmOpCode::Or { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),   BramaPrimative::Number(r_value))   => if *l_value > 0.0 || *r_value > 0.0 { true_primative } else { false_primative },
                    (BramaPrimative::Bool(l_value),     BramaPrimative::Bool(r_value))     => if *l_value && *r_value { true_primative } else { false_primative },
                    _ => empty_primative
                };
            },
            BramaVmOpCode::Addition { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value)) => VmObject::convert(Rc::new(BramaPrimative::Number(*l_value + *r_value))),
                    (BramaPrimative::Text(l_value),    BramaPrimative::Text(r_value))   => {
                        VmObject::convert(Rc::new(BramaPrimative::Text(Rc::new((&**l_value).to_owned() + &**r_value))))
                    },
                    (BramaPrimative::Text(l_value),    BramaPrimative::Bool(r_value))   => {
                        let left_string = (&**l_value).to_owned();
                        let right_bool  = if *r_value { 
                            "doğru" 
                        } 
                        else { 
                            "yanlış" 
                        };

                        VmObject::convert(Rc::new(BramaPrimative::Text(Rc::new(left_string + right_bool))))
                    },
                    (BramaPrimative::Text(l_value),    BramaPrimative::Number(r_value))   => {
                        let left_string  = (&**l_value).to_owned();
                        let right_number = (*r_value).to_string();

                        VmObject::convert(Rc::new(BramaPrimative::Text(Rc::new(left_string + &right_number))))
                    },
                    _ => empty_primative
                };
            },
            BramaVmOpCode::Multiply { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::convert(Rc::new(BramaPrimative::Number(*l_value * *r_value))),
                    (BramaPrimative::Text(l_value),    BramaPrimative::Number(r_value))   => VmObject::convert(Rc::new(BramaPrimative::Text(Rc::new((*l_value).repeat((*r_value) as usize))))),
                    _ => empty_primative
                };
            },
            BramaVmOpCode::Division { target, left, right } => {
                let calculation = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => (*l_value / *r_value),
                    _ => std::f64::NAN
                };

                memory[*target as usize] = if calculation.is_nan() {
                    empty_primative
                }
                else {
                    VmObject::convert(Rc::new(BramaPrimative::Number(calculation)))
                }
            },
            BramaVmOpCode::Subraction { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::convert(Rc::new(BramaPrimative::Number(*l_value - *r_value))),
                    _ => empty_primative
                };
            },
            BramaVmOpCode::Equal { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Empty,               BramaPrimative::Empty)               => true_primative,
                    (BramaPrimative::Atom(l_value),       BramaPrimative::Atom(r_value))       => if *l_value == *r_value { true_primative } else { false_primative },
                    (BramaPrimative::Bool(l_value),       BramaPrimative::Bool(r_value))       => if *l_value == *r_value { true_primative } else { false_primative },
                    (BramaPrimative::Number(l_value),     BramaPrimative::Number(r_value))     => if *l_value == *r_value { true_primative } else { false_primative },
                    (BramaPrimative::Text(l_value),       BramaPrimative::Text(r_value))       => if *l_value == *r_value { true_primative } else { false_primative },
                    _ => empty_primative
                };
            },
            BramaVmOpCode::NotEqual { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Empty,               BramaPrimative::Empty)               => false_primative,
                    (BramaPrimative::Atom(l_value),       BramaPrimative::Atom(r_value))       => if *l_value != *r_value { true_primative } else { false_primative },
                    (BramaPrimative::Bool(l_value),       BramaPrimative::Bool(r_value))       => if *l_value != *r_value { true_primative } else { false_primative },
                    (BramaPrimative::Number(l_value),     BramaPrimative::Number(r_value))     => if *l_value != *r_value { true_primative } else { false_primative },
                    (BramaPrimative::Text(l_value),       BramaPrimative::Text(r_value))       => if *l_value != *r_value { true_primative } else { false_primative },
                    _ => empty_primative
                };
            },
            BramaVmOpCode::GreaterThan { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => if *l_value > *r_value { true_primative } else { false_primative },
                    _ => empty_primative
                };
            },
            BramaVmOpCode::GreaterEqualThan { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => if *l_value >= *r_value { true_primative } else { false_primative },
                    _ => empty_primative
                };
            },
            BramaVmOpCode::LessThan { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => if *l_value < *r_value { true_primative } else { false_primative },
                    _ => empty_primative
                };
            },
            BramaVmOpCode::LessEqualThan { target, left, right } => {
                memory[*target as usize] = match (&*memory[*left as usize].deref(), &*memory[*right as usize].deref()) {
                    (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => if *l_value <= *r_value { true_primative } else { false_primative },
                    _ => empty_primative
                };
            },
            BramaVmOpCode::Assign { target, expression } => {
                memory[*target as usize] = memory[*expression as usize];
            },
            _ => ()
        }
    }
    
    options.storages[0].dump();
}
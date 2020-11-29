use crate::types::{VmObject};
use crate::compiler::*;
use std::rc::Rc;


pub fn run_vm<S>(options: &mut BramaCompilerOption<S>) where S: Storage
{ 
    {
        let empty_primative: VmObject  = VmObject::convert(Rc::new(BramaPrimative::Empty));
        let memory_ref  = &mut options.storages[0].get_memory();
        let mut memory  = memory_ref.borrow_mut();
        let opcodes = options.opcodes.iter().map(|byte| byte.decode_as_tuple()).collect::<Vec<_>>();

        for op in &opcodes {
            let &(opcode, target, left, right) = op;

            match opcode {
                VmOpCode::And => {
                    let left_expression = match  &*memory[left].deref() {
                        BramaPrimative::Text(value)       => value.len() > 0,
                        BramaPrimative::Number(value)     => *value > 0.0,
                        BramaPrimative::Bool(value)       => *value,
                        BramaPrimative::Atom(_)           => true,
                        BramaPrimative::List(items)       => items.len() > 0,
                        BramaPrimative::FuncNativeCall(_) => true,
                        BramaPrimative::Empty             => false
                    };

                    let right_expression = match  &*memory[right].deref() {
                        BramaPrimative::Text(value)       => value.len() > 0,
                        BramaPrimative::Number(value)     => *value > 0.0,
                        BramaPrimative::Bool(value)       => *value,
                        BramaPrimative::Atom(_)           => true,
                        BramaPrimative::List(items)       => items.len() > 0,
                        BramaPrimative::FuncNativeCall(_) => true,
                        BramaPrimative::Empty             => false
                    };

                    memory[target] = VmObject::from(left_expression && right_expression);
                },

                VmOpCode::Or => {
                    let left_expression = match  &*memory[left].deref() {
                        BramaPrimative::Text(value)       => value.len() > 0,
                        BramaPrimative::Number(value)     => *value > 0.0,
                        BramaPrimative::Bool(value)       => *value,
                        BramaPrimative::Atom(_)           => true,
                        BramaPrimative::List(items)       => items.len() > 0,
                        BramaPrimative::FuncNativeCall(_) => true,
                        BramaPrimative::Empty             => false
                    };

                    let right_expression = match  &*memory[right].deref() {
                        BramaPrimative::Text(value)       => value.len() > 0,
                        BramaPrimative::Number(value)     => *value > 0.0,
                        BramaPrimative::Bool(value)       => *value,
                        BramaPrimative::Atom(_)           => true,
                        BramaPrimative::List(items)       => items.len() > 0,
                        BramaPrimative::FuncNativeCall(_) => true,
                        BramaPrimative::Empty             => false
                    };

                    memory[target] = VmObject::from(left_expression || right_expression);
                },

                VmOpCode::Addition => {
                    memory[target] = match (&*memory[left].deref(), &*memory[right].deref()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value)) => VmObject::from(*l_value + *r_value),
                        (BramaPrimative::Text(l_value),    BramaPrimative::Text(r_value))   => VmObject::from(Rc::new((&**l_value).to_owned() + &**r_value)),
                        _ => empty_primative
                    };
                },

                VmOpCode::Multiply => {
                    memory[target] = match (&*memory[left].deref(), &*memory[right].deref()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value * *r_value),
                        (BramaPrimative::Text(l_value),    BramaPrimative::Number(r_value))   => VmObject::from((*l_value).repeat((*r_value) as usize)),
                        _ => empty_primative
                    };
                },

                VmOpCode::Division => {
                    let calculation = match (&*memory[left].deref(), &*memory[right].deref()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => (*l_value / *r_value),
                        _ => std::f64::NAN
                    };

                    memory[target] = if calculation.is_nan() {
                        empty_primative
                    }
                    else {
                        VmObject::from(calculation)
                    }
                },

                VmOpCode::Subraction => {
                    memory[target] = match (&*memory[left].deref(), &*memory[right].deref()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value - *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::Equal => {
                    memory[target] = match (&*memory[left].deref(), &*memory[right].deref()) {
                        (BramaPrimative::Empty,               BramaPrimative::Empty)               => VmObject::from(true),
                        (BramaPrimative::Atom(l_value),       BramaPrimative::Atom(r_value))       => VmObject::from(*l_value == *r_value),
                        (BramaPrimative::Bool(l_value),       BramaPrimative::Bool(r_value))       => VmObject::from(*l_value == *r_value),
                        (BramaPrimative::Number(l_value),     BramaPrimative::Number(r_value))     => VmObject::from(*l_value == *r_value),
                        (BramaPrimative::Text(l_value),       BramaPrimative::Text(r_value))       => VmObject::from(*l_value == *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::NotEqual => {
                    memory[target] = match (&*memory[left].deref(), &*memory[right].deref()) {
                        (BramaPrimative::Empty,               BramaPrimative::Empty)               => VmObject::from(false),
                        (BramaPrimative::Atom(l_value),       BramaPrimative::Atom(r_value))       => VmObject::from(*l_value != *r_value),
                        (BramaPrimative::Bool(l_value),       BramaPrimative::Bool(r_value))       => VmObject::from(*l_value != *r_value),
                        (BramaPrimative::Number(l_value),     BramaPrimative::Number(r_value))     => VmObject::from(*l_value != *r_value),
                        (BramaPrimative::Text(l_value),       BramaPrimative::Text(r_value))       => VmObject::from(*l_value != *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::GreaterThan => {
                    memory[target] = match (&*memory[left].deref(), &*memory[right].deref()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value > *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::GreaterEqualThan => {
                    memory[target] = match (&*memory[left].deref(), &*memory[right].deref()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value >= *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::LessThan => {
                    memory[target] = match (&*memory[left].deref(), &*memory[right].deref()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value < *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::LessEqualThan => {
                    memory[target] = match (&*memory[left].deref(), &*memory[right].deref()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value <= *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::Move => {
                    memory[target] = memory[left];
                },

                VmOpCode::NativeCall => {
                    if let BramaPrimative::FuncNativeCall(func) = &*memory[right].deref() {
                        let start   = ((options.storages[0].get_constant_size() + options.storages[0].get_variable_size() + options.storages[0].get_temp_size())  - left as u8) as usize;
                        let end     = left as usize;
                        let mut args  = Vec::new();

                        for arg in memory.iter().skip(start - 1).take(end) {
                            args.push(*arg);
                        }

                        func(&args);
                    }
                },

                VmOpCode::AssignAddition => {
                    memory[target] = match (&*memory[target].deref(), &*memory[left].deref()) {
                        (BramaPrimative::Number(l_value),     BramaPrimative::Number(r_value))     => VmObject::from(*l_value + *r_value),
                        (BramaPrimative::Text(l_value),       BramaPrimative::Text(r_value))       => VmObject::from(Rc::new((&**l_value).to_owned() + &**r_value)),
                        _ => empty_primative
                    };
                },

                VmOpCode::AssignSubtraction => {
                    memory[target] = match (&*memory[target].deref(), &*memory[left].deref()) {
                        (BramaPrimative::Number(l_value), BramaPrimative::Number(r_value)) => VmObject::from(*l_value - *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::AssignMultiplication => {
                    memory[target] = match (&*memory[target].deref(), &*memory[left].deref()) {
                        (BramaPrimative::Number(l_value), BramaPrimative::Number(r_value)) => VmObject::from(*l_value * *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::AssignDivision => {
                    memory[target] = match (&*memory[target].deref(), &*memory[left].deref()) {
                        (BramaPrimative::Number(l_value), BramaPrimative::Number(r_value)) => VmObject::from(*l_value / *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::Increment => {
                    memory[target] = match &*memory[target].deref() {
                        BramaPrimative::Number(value) => VmObject::from(*value + 1 as f64),
                        _ => empty_primative
                    };
                },

                VmOpCode::Decrement => {
                    memory[target] = match &*memory[target].deref() {
                        BramaPrimative::Number(value) => VmObject::from(*value - 1 as f64),
                        _ => empty_primative
                    };
                },

                VmOpCode::None => ()
            }
        }
    }

    options.storages[0].dump();
}
use crate::types::{VmObject};
use crate::compiler::*;
use crate::vm::debug_helpers::{dump_opcode_header, dump_opcode};
use std::rc::Rc;
use std::mem;
use std::ops::Deref;

pub fn run_vm<S>(options: &mut BramaCompilerOption<S>) where S: Storage
{
    #[cfg(feature = "dumpOpcodes")] {
        /*println!("            OPCODE");
        println!("---------------------------------------------");
        for (index, item) in options.opcodes.iter().enumerate() {
            let (opcode, target, left, right) = item.decode_as_tuple();
            println!("| {:?} | {:15} | {:^5?} | {:^5?} | {:^3?}", index, format!("{:?}", opcode), target, left, right);
        }*/
    }

    //dump_opcode_header();
    {
        let empty_primative: VmObject  = VmObject::convert(Rc::new(BramaPrimative::Empty));
        let memory_ref    = &mut options.storages[0].get_memory();
        let mut memory    = memory_ref.borrow_mut();
        let mut index     = 0;
        let opcode_size   = options.opcodes.len();
        let mut mem_index = 0;
        let mut stack: Vec<VmObject> = Vec::with_capacity(options.storages[0].get_temp_size() as usize);

        for i in 0..10 {
            stack.push(VmObject::from(0.0));
        }

        while opcode_size > index {
            let opcode = unsafe { mem::transmute::<u8, VmOpCode>(options.opcodes[index]) };
            
            match opcode {
                VmOpCode::Addition => {
                    mem_index -= 1;
                    let right = stack[mem_index].deref();

                    mem_index -= 1;
                    let left = stack[mem_index].deref();

                    stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value)) => VmObject::from(l_value + r_value),
                        (BramaPrimative::Text(l_value),    BramaPrimative::Text(r_value))   => VmObject::from(Rc::new((&**l_value).to_owned() + &**r_value)),
                        _ => empty_primative
                    };

                    println!("{:?} = {:?} + {:?}", stack[mem_index].deref(), left.deref(), right.deref());
                    mem_index += 1;
                },

                VmOpCode::Load => {
                    let tmp = options.opcodes[index + 1] as usize;
                    stack[mem_index] = memory[tmp];
                    println!("-- {:?}, {}", stack[mem_index].deref(), mem_index);
                    index     += 1;
                    mem_index += 1;
                },

                /*VmOpCode::And => {
                    
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
                    
                    memory[target] = match (&*memory[tmp_index-2].deref(), &*memory[tmp_index-1].deref()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value - *r_value),
                        _ => empty_primative
                    };

                    tmp_index -= 2;
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

                VmOpCode::Store => {
                    
                    memory[target] = memory[tmp_index + 1];
                },

                VmOpCode::NativeCall => {
                    
                    if let BramaPrimative::FuncNativeCall(func) = &*memory[right].deref() {
                        let end      = left as usize;
                        let mut args = Vec::new();

                        for arg in memory.iter().skip(tmp_index - 1).take(end) {
                            args.push(*arg);
                        }

                        match func(&args) {
                            Ok(result) => {
                                tmp_index        -= end;
                                memory[target] = result;
                                tmp_index        += 1;
                            },
                            Err((error, _, _)) => {
                                println!("{:?}", error);
                            }
                        };
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

                VmOpCode::Not => {
                    
                    memory[target] = match &*memory[left].deref() {
                        BramaPrimative::Number(value) => VmObject::from(!(*value > 0.0)),
                        BramaPrimative::Bool(value)   => VmObject::from(!*value),
                        _ => empty_primative
                    };
                },
*/
                _ => ()
            }

            index += 1;
        }


        for i in 0..10 {
            println!("{:?}", stack[i].deref());
        }
    }

    #[cfg(feature = "dumpMemory")] {
        options.storages[0].dump();
    }
}
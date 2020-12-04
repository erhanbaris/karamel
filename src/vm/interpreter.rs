use crate::types::{VmObject};
use crate::compiler::*;
use std::rc::Rc;
use std::mem;


macro_rules! pop {
    ($mem_index: expr, $stack: expr) => {{
        $mem_index -= 1;
        $stack[$mem_index as usize].deref()
    }}
}

pub fn run_vm(options: &mut BramaCompilerOption)
{
    #[cfg(feature = "dumpOpcodes")] {
        println!("            OPCODE");
        println!("---------------------------------------------");
        let opcode_size   = options.opcodes.len();
        let mut opcode_index = 0;

        while opcode_size > opcode_index {
            let opcode = unsafe { mem::transmute::<u8, VmOpCode>(options.opcodes[opcode_index]) };
            match opcode {
                VmOpCode::Addition | 
                VmOpCode::And | 
                VmOpCode::Or |
                VmOpCode::Subraction | 
                VmOpCode::Multiply => {
                    println!("| {:4} | {:15} | {:^5}", opcode_index, format!("{:?}", opcode), "");
                    opcode_index += 2;
                },

                VmOpCode::Load |
                VmOpCode::Store => {
                    println!("| {:4} | {:15} | {:^5?}", opcode_index, format!("{:?}", opcode), options.opcodes[opcode_index + 1]);
                    opcode_index += 1;
                },

                _ => {
                    println!("| {:4} | {:15} | {:^5?}", opcode_index, format!("{:?}", opcode), "");
                }
            }

            opcode_index += 1;
        }
    }

    //dump_opcode_header();
    {
        let empty_primative: VmObject  = VmObject::convert(Rc::new(BramaPrimative::Empty));
        let memory_ref    = &mut options.storages[0].get_memory();
        let mut memory    = memory_ref.borrow_mut();
        let mut index     = options.opcode_index;
        let opcode_size   = options.opcodes.len();
        let mut mem_index: i16 = 0;
        let opcode_index = 0;
        let mut stack: Vec<VmObject> = Vec::with_capacity(options.storages[0].get_temp_size() as usize);

        for _i in 0..options.storages[0].get_temp_size() {
            stack.push(VmObject::from(0.0));
        }

        while opcode_size > index {
            let opcode = unsafe { mem::transmute::<u8, VmOpCode>(options.opcodes[index]) };
            
            match opcode {
                VmOpCode::Addition => {
                    let right = pop!(mem_index, stack);
                    let left  = pop!(mem_index, stack);

                    stack[mem_index as usize] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value)) => VmObject::from(l_value + r_value),
                        (BramaPrimative::Text(l_value),    BramaPrimative::Text(r_value))   => VmObject::from(Rc::new((&**l_value).to_owned() + &**r_value)),
                        _ => empty_primative
                    };
                    index += 2;
                    mem_index += 1;
                },

                VmOpCode::Load => {
                    let tmp = options.opcodes[index + 1] as usize;
                    stack[mem_index as usize] = memory[tmp];
                    index     += 1;
                    mem_index += 1;
                },

                VmOpCode::Store => {
                    let tmp = options.opcodes[index + 1] as usize;
                    mem_index -= 1;
                    memory[tmp] = stack[mem_index as usize];
                    index     += 1;
                },

                VmOpCode::And => {
                    let left = pop!(mem_index, stack);
                    let right = pop!(mem_index, stack);

                    let left_expression = match &*left {
                        BramaPrimative::Text(value)       => value.len() > 0,
                        BramaPrimative::Number(value)     => *value > 0.0,
                        BramaPrimative::Bool(value)       => *value,
                        BramaPrimative::Atom(_)           => true,
                        BramaPrimative::List(items)       => items.len() > 0,
                        BramaPrimative::FuncNativeCall(_) => true,
                        BramaPrimative::Empty             => false
                    };

                    let right_expression = match &*right {
                        BramaPrimative::Text(value)       => value.len() > 0,
                        BramaPrimative::Number(value)     => *value > 0.0,
                        BramaPrimative::Bool(value)       => *value,
                        BramaPrimative::Atom(_)           => true,
                        BramaPrimative::List(items)       => items.len() > 0,
                        BramaPrimative::FuncNativeCall(_) => true,
                        BramaPrimative::Empty             => false
                    };

                    stack[mem_index as usize] = VmObject::from(left_expression && right_expression);
                    index += 2;
                    mem_index += 1;
                },

                VmOpCode::Or => {
                    let left = pop!(mem_index, stack);
                    let right = pop!(mem_index, stack);

                    let left_expression = match &*left {
                        BramaPrimative::Text(value)       => value.len() > 0,
                        BramaPrimative::Number(value)     => *value > 0.0,
                        BramaPrimative::Bool(value)       => *value,
                        BramaPrimative::Atom(_)           => true,
                        BramaPrimative::List(items)       => items.len() > 0,
                        BramaPrimative::FuncNativeCall(_) => true,
                        BramaPrimative::Empty             => false
                    };

                    let right_expression = match &*right {
                        BramaPrimative::Text(value)       => value.len() > 0,
                        BramaPrimative::Number(value)     => *value > 0.0,
                        BramaPrimative::Bool(value)       => *value,
                        BramaPrimative::Atom(_)           => true,
                        BramaPrimative::List(items)       => items.len() > 0,
                        BramaPrimative::FuncNativeCall(_) => true,
                        BramaPrimative::Empty             => false
                    };

                    stack[mem_index as usize] = VmObject::from(left_expression || right_expression);
                    index += 2;
                    mem_index += 1;
                },

                VmOpCode::Multiply => {
                    let right = pop!(mem_index, stack);
                    let left  = pop!(mem_index, stack);
                    stack[mem_index as usize] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value * *r_value),
                        (BramaPrimative::Text(l_value),    BramaPrimative::Number(r_value))   => VmObject::from((*l_value).repeat((*r_value) as usize)),
                        _ => empty_primative
                    };
                    index += 2;
                    mem_index += 1;
                },


                /*
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


        for i in 0..mem_index {
            println!("{:?}", stack[i as usize].deref());
        }

        options.opcode_index = index;
    }

    #[cfg(feature = "dumpMemory")] {
        options.storages[0].dump();
    }
}
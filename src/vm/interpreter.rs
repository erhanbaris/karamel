use crate::types::{VmObject};
use crate::compiler::*;
use std::rc::Rc;
use std::mem;


macro_rules! pop {
    ($mem_index: expr, $stack: expr) => {{
        $mem_index -= 1;
        $stack[$mem_index].deref()
    }}
}

pub fn run_vm(options: &mut BramaCompilerOption)
{
    #[cfg(feature = "dumpOpcodes")] {

        println!("╔════════════════════════════════════════╗");
        println!("║                  OPCODE                ║");
        println!("╠══════╦═════════════════╦═══════╦═══════╣");
        let opcode_size   = options.opcodes.len();
        let mut opcode_index = 0;

        while opcode_size > opcode_index {
            let opcode = unsafe { mem::transmute::<u8, VmOpCode>(options.opcodes[opcode_index]) };
            match opcode {
                VmOpCode::Division |
                VmOpCode::Not |
                VmOpCode::Equal |
                VmOpCode::NotEqual |
                VmOpCode::Dublicate |
                VmOpCode::Increment |
                VmOpCode::Decrement | 
                VmOpCode::Addition | 
                VmOpCode::And | 
                VmOpCode::Or |
                VmOpCode::Subraction | 
                VmOpCode::GreaterEqualThan |
                VmOpCode::GreaterThan | 
                VmOpCode::LessEqualThan | 
                VmOpCode::LessThan | 
                VmOpCode::Multiply => {
                    println!("║ {:4} ║ {:15} ║ {:^5} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), "", "");
                },

                VmOpCode::Compare |
                VmOpCode::Jump => {
                    let location = ((options.opcodes[opcode_index+2] as u16 * 256) + options.opcodes[opcode_index+1] as u16) as usize;

                    println!("║ {:4} ║ {:15} ║ {:^5?} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), location + opcode_index + 1, "");
                    opcode_index += 2;
                },

                VmOpCode::CopyToStore |
                VmOpCode::Load |
                VmOpCode::Store => {
                    println!("║ {:4} ║ {:15} ║ {:^5?} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), options.opcodes[opcode_index + 1], "");
                    opcode_index += 1;
                },

                VmOpCode::None => {
                    println!("║ {:4} ║ {:15} ║ {:^5?} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), "", "");
                    opcode_index += 1;
                },

                VmOpCode::NativeCall |
                VmOpCode::FastStore => {
                    println!("║ {:4} ║ {:15} ║ {:^5?} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), options.opcodes[opcode_index + 1], options.opcodes[opcode_index + 2]);
                    opcode_index += 2;
                }
            }

            opcode_index += 1;
        }
        println!("╚══════╩═════════════════╩═══════╩═══════╝");
    }

    //dump_opcode_header();
    {
        let empty_primative: VmObject  = VmObject::convert(Rc::new(BramaPrimative::Empty));
        let memory_ref    = &mut options.storages[0].get_memory();
        let stack_ref    = &mut options.storages[0].get_stack();
        let mut memory    = memory_ref.borrow_mut();
        let mut stack     = stack_ref.borrow_mut();
        let mut index     = options.opcode_index;
        let opcode_size   = options.opcodes.len();
        let mut mem_index: usize = 0;

        while opcode_size > index {
            let opcode = unsafe { mem::transmute::<u8, VmOpCode>(options.opcodes[index]) };
            
            match opcode {
                VmOpCode::Subraction => {
                    let right = pop!(mem_index, stack);
                    let left  = pop!(mem_index, stack);

                    stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value - *r_value),
                        _ => empty_primative
                    };
                    mem_index += 1;
                },

                VmOpCode::Addition => {
                    let right = pop!(mem_index, stack);
                    let left  = pop!(mem_index, stack);

                    stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value)) => VmObject::from(l_value + r_value),
                        (BramaPrimative::Text(l_value),    BramaPrimative::Text(r_value))   => VmObject::from(Rc::new((&**l_value).to_owned() + &**r_value)),
                        _ => empty_primative
                    };
                    mem_index += 1;
                },

                VmOpCode::Load => {
                    let tmp = options.opcodes[index + 1] as usize;
                    stack[mem_index] = memory[tmp];
                    index     += 1;
                    mem_index += 1;
                },

                VmOpCode::Store => {
                    let tmp = options.opcodes[index + 1] as usize;
                    mem_index -= 1;
                    memory[tmp] = stack[mem_index];
                    index     += 1;
                },

                VmOpCode::CopyToStore => {
                    let tmp = options.opcodes[index + 1] as usize;
                    memory[tmp] = stack[mem_index - 1];
                    index     += 1;
                },

                VmOpCode::FastStore => {
                    let destination = options.opcodes[index + 1] as usize;
                    let source      = options.opcodes[index + 2] as usize;
                    memory[destination as usize] = memory[source];
                    index     += 2;
                },

                VmOpCode::Not => {
                    stack[mem_index - 1] = match &*stack[mem_index - 1].deref() {
                        BramaPrimative::Bool(value) => VmObject::from(!*value),
                        BramaPrimative::Number(value) => VmObject::from(*value <= 0.0),
                        _ => empty_primative
                    };
                },

                VmOpCode::Dublicate => {
                    stack[mem_index] = stack[mem_index - 1];
                    mem_index += 1;
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

                    stack[mem_index] = VmObject::from(left_expression && right_expression);
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

                    stack[mem_index] = VmObject::from(left_expression || right_expression);
                    mem_index += 1;
                },

                VmOpCode::Multiply => {
                    let right = pop!(mem_index, stack);
                    let left  = pop!(mem_index, stack);
                    stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value * *r_value),
                        (BramaPrimative::Text(l_value),    BramaPrimative::Number(r_value))   => VmObject::from((*l_value).repeat((*r_value) as usize)),
                        _ => empty_primative
                    };
                    mem_index += 1;
                },

                VmOpCode::Division => {
                    let right = pop!(mem_index, stack);
                    let left  = pop!(mem_index, stack);

                    let calculation = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => (*l_value / *r_value),
                        _ => std::f64::NAN
                    };

                    stack[mem_index] = if calculation.is_nan() {
                        empty_primative
                    }
                    else {
                        VmObject::from(calculation)
                    };

                    mem_index += 1;
                },

                VmOpCode::Equal => {
                    let right = pop!(mem_index, stack);
                    let left  = pop!(mem_index, stack);
                    
                    stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Empty,               BramaPrimative::Empty)               => VmObject::from(true),
                        (BramaPrimative::Atom(l_value),       BramaPrimative::Atom(r_value))       => VmObject::from(*l_value == *r_value),
                        (BramaPrimative::Bool(l_value),       BramaPrimative::Bool(r_value))       => VmObject::from(*l_value == *r_value),
                        (BramaPrimative::Number(l_value),     BramaPrimative::Number(r_value))     => VmObject::from(*l_value == *r_value),
                        (BramaPrimative::Text(l_value),       BramaPrimative::Text(r_value))       => VmObject::from(*l_value == *r_value),
                        _ => empty_primative
                    };
                    mem_index += 1;
                },


                VmOpCode::NotEqual => {
                    let right = pop!(mem_index, stack);
                    let left  = pop!(mem_index, stack);
                    
                    stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Empty,               BramaPrimative::Empty)               => VmObject::from(false),
                        (BramaPrimative::Atom(l_value),       BramaPrimative::Atom(r_value))       => VmObject::from(*l_value != *r_value),
                        (BramaPrimative::Bool(l_value),       BramaPrimative::Bool(r_value))       => VmObject::from(*l_value != *r_value),
                        (BramaPrimative::Number(l_value),     BramaPrimative::Number(r_value))     => VmObject::from(*l_value != *r_value),
                        (BramaPrimative::Text(l_value),       BramaPrimative::Text(r_value))       => VmObject::from(*l_value != *r_value),
                        _ => empty_primative
                    };
                    mem_index += 1;
                },

                VmOpCode::GreaterThan => {
                    let right = pop!(mem_index, stack);
                    let left  = pop!(mem_index, stack);
                    
                    stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value > *r_value),
                        _ => empty_primative
                    };
                    mem_index += 1;
                },

                VmOpCode::GreaterEqualThan => {
                    let right = pop!(mem_index, stack);
                    let left  = pop!(mem_index, stack);
                    
                    stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value >= *r_value),
                        _ => empty_primative
                    };
                    mem_index += 1;
                },

                VmOpCode::LessThan => {
                    let right = pop!(mem_index, stack);
                    let left  = pop!(mem_index, stack);
                    
                    stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value < *r_value),
                        _ => empty_primative
                    };
                    mem_index += 1;
                },

                VmOpCode::LessEqualThan => {
                    let right = pop!(mem_index, stack);
                    let left  = pop!(mem_index, stack);
                    
                    stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value <= *r_value),
                        _ => empty_primative
                    };
                    mem_index += 1;
                },

                VmOpCode::NativeCall => {
                    let func_location = options.opcodes[index + 1] as usize;
                    
                    if let BramaPrimative::FuncNativeCall(func) = *memory[func_location].deref() {
                        let total_args = options.opcodes[index + 2];
                        
                        match func(&stack, (mem_index-1) as usize, total_args) {
                            Ok(result) => {
                                mem_index        -= total_args as usize;
                                stack[mem_index] = result;
                                mem_index        += 1;
                            },
                            Err((error, _, _)) => {
                                println!("{:?}", error);
                            }
                        };

                        index += 2;
                    }
                },

                VmOpCode::Increment => {
                    stack[mem_index - 1] = match &*stack[mem_index - 1].deref() {
                        BramaPrimative::Number(value) => VmObject::from(*value + 1 as f64),
                        _ => empty_primative
                    };
                },

                VmOpCode::Decrement => {
                    stack[mem_index - 1] = match &*stack[mem_index - 1].deref() {
                        BramaPrimative::Number(value) => VmObject::from(*value - 1 as f64),
                        _ => empty_primative
                    };
                },

                VmOpCode::Compare => {
                    let condition = pop!(mem_index, stack);

                    let status = match &*condition {
                        BramaPrimative::Empty => false,
                        BramaPrimative::Atom(_) => true,
                        BramaPrimative::Bool(l_value) => *l_value,
                        BramaPrimative::Number(l_value) => *l_value > 0.0,
                        BramaPrimative::Text(l_value) => (*l_value).len() > 0,
                        _ => false
                    };

                    if status {
                        index += 2 as usize;
                    }
                    else {
                        let location = ((options.opcodes[index + 2] as u16 * 256) + options.opcodes[index + 1] as u16) as usize;
                        index += location as usize;
                    }
                },

                VmOpCode::Jump => {
                    let location = ((options.opcodes[index + 2] as u16 * 256) + options.opcodes[index + 1] as u16) as usize;
                    index += location as usize;
                },
                _ => ()
            }

            index += 1;
        }

        if stack.len() > 0 {
            println!("╔════════════════════════════════════════╗");
            println!("║                  STACK                 ║");
            println!("╠════════════════════════════════════════╣");
            println!("║ Stack size: {:<10}                 ║", stack.len());
            println!("╠════════════════════════════════════════╣");
            for i in 0..stack.len() {
                println!("║ {:38} ║", format!("{:?}", stack[i as usize].deref()));
            }
            println!("╚════════════════════════════════════════╝");
        }

        options.opcode_index = index;
    }

    #[cfg(feature = "dumpMemory")] {
        options.storages[0].dump();
    }
}
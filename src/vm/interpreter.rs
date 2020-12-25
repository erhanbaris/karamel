use crate::types::{VmObject};
use crate::compiler::*;
use std::rc::Rc;
use std::mem;
use std::collections::HashMap;

macro_rules! pop {
    ($mem_index: expr, $stack: expr) => {{
        $mem_index -= 1;
        $stack[$mem_index].deref()
    }}
}

#[derive(Clone)]
struct Scope {
    memory: Vec<VmObject>, 
    stack: Vec<VmObject>, 
    location: usize,
    mem_index: usize
}

pub fn run_vm(options: &mut BramaCompilerOption) -> Result<(), &'static str>
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
                VmOpCode::GetItem | 
                VmOpCode::Multiply => {
                    println!("║ {:4} ║ {:15} ║ {:^5} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), "", "");
                },

                VmOpCode::Compare |
                VmOpCode::Jump => {
                    let location = ((options.opcodes[opcode_index+2] as u16 * 256) + options.opcodes[opcode_index+1] as u16) as usize;

                    println!("║ {:4} ║ {:15} ║ {:^5?} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), location + opcode_index + 1, "");
                    opcode_index += 2;
                },

                VmOpCode::Func => {
                    let location = ((options.opcodes[opcode_index+2] as u16 * 256) + options.opcodes[opcode_index+1] as u16) as usize;

                    println!("║ {:4} ║ {:15} ║ {:^5?} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), location + opcode_index + 1, "");
                    opcode_index += 2;
                },

                VmOpCode::CopyToStore |
                VmOpCode::Load |
                VmOpCode::InitList |
                VmOpCode::InitDict |
                VmOpCode::Store => {
                    println!("║ {:4} ║ {:15} ║ {:^5?} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), options.opcodes[opcode_index + 1], "");
                    opcode_index += 1;
                },

                VmOpCode::None |
                VmOpCode::Return => {
                    println!("║ {:4} ║ {:15} ║ {:^5} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), "", "");
                },

                VmOpCode::Call |
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
        let mut storages: Vec<Scope> = Vec::with_capacity(32);
        let empty_primative: VmObject  = VmObject::convert(Rc::new(BramaPrimative::Empty));
        let mut index     = options.opcode_index;
        let opcode_size   = options.opcodes.len();
        let mut mem_index: usize = 0;
        let mut storages_index: usize = 0;

        storages.resize(32, Scope { mem_index: 0, location: 0, memory: Vec::new(), stack: Vec::new()});
        storages[storages_index] = Scope {
            memory: options.storages[0].get_memory().borrow().to_vec(),
            stack: options.storages[0].get_stack().borrow().to_vec(),
            location: 0,
            mem_index: 0
        };

        let mut scope = &mut storages[storages_index];

        while opcode_size > index {
            let opcode = unsafe { mem::transmute::<u8, VmOpCode>(options.opcodes[index]) };
            
            match opcode {
                VmOpCode::Subraction => {
                    let right = pop!(mem_index, scope.stack);
                    let left  = pop!(mem_index, scope.stack);

                    scope.stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value - *r_value),
                        _ => empty_primative
                    };
                    mem_index += 1;
                },

                VmOpCode::Addition => {
                    let right = pop!(mem_index, scope.stack);
                    let left  = pop!(mem_index, scope.stack);

                    scope.stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value)) => VmObject::from(l_value + r_value),
                        (BramaPrimative::Text(l_value),    BramaPrimative::Text(r_value))   => VmObject::from(Rc::new((&**l_value).to_owned() + &**r_value)),
                        _ => empty_primative
                    };
                    mem_index += 1;
                },

                VmOpCode::Load => {
                    let tmp = options.opcodes[index + 1] as usize;
                    scope.stack[mem_index] = scope.memory[tmp];
                    index     += 1;
                    mem_index += 1;
                },

                VmOpCode::Store => {
                    let tmp = options.opcodes[index + 1] as usize;
                    mem_index -= 1;
                    scope.memory[tmp] = scope.stack[mem_index];
                    index     += 1;
                },

                VmOpCode::CopyToStore => {
                    let tmp = options.opcodes[index + 1] as usize;
                    scope.memory[tmp] = scope.stack[mem_index - 1];
                    index     += 1;
                },

                VmOpCode::FastStore => {
                    let destination = options.opcodes[index + 1] as usize;
                    let source      = options.opcodes[index + 2] as usize;
                    scope.memory[destination as usize] = scope.memory[source];
                    index     += 2;
                },

                VmOpCode::Not => {
                    scope.stack[mem_index - 1] = VmObject::from(!scope.stack[mem_index - 1].deref().is_true());
                },

                VmOpCode::Dublicate => {
                    scope.stack[mem_index] = scope.stack[mem_index - 1];
                    mem_index += 1;
                },

                VmOpCode::And => {
                    let left = pop!(mem_index, scope.stack);
                    let right = pop!(mem_index, scope.stack);

                    scope.stack[mem_index] = VmObject::from(left.is_true() && right.is_true());
                    mem_index += 1;
                },

                VmOpCode::Or => {
                    let left = pop!(mem_index, scope.stack);
                    let right = pop!(mem_index, scope.stack);
                    scope.stack[mem_index] = VmObject::from(left.is_true() || right.is_true());
                    mem_index += 1;
                },

                VmOpCode::Multiply => {
                    let right = pop!(mem_index, scope.stack);
                    let left  = pop!(mem_index, scope.stack);
                    scope.stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value * *r_value),
                        (BramaPrimative::Text(l_value),    BramaPrimative::Number(r_value))   => VmObject::from((*l_value).repeat((*r_value) as usize)),
                        _ => empty_primative
                    };
                    mem_index += 1;
                },

                VmOpCode::Division => {
                    let right = pop!(mem_index, scope.stack);
                    let left  = pop!(mem_index, scope.stack);

                    let calculation = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => (*l_value / *r_value),
                        _ => std::f64::NAN
                    };

                    scope.stack[mem_index] = if calculation.is_nan() {
                        empty_primative
                    }
                    else {
                        VmObject::from(calculation)
                    };

                    mem_index += 1;
                },

                VmOpCode::Equal => {                    
                    let right = pop!(mem_index, scope.stack);
                    let left  = pop!(mem_index, scope.stack);
                    
                    scope.stack[mem_index] = VmObject::from(left == right);
                    mem_index += 1;
                },


                VmOpCode::NotEqual => {
                    let right = pop!(mem_index, scope.stack);
                    let left  = pop!(mem_index, scope.stack);
                    
                    scope.stack[mem_index] = VmObject::from(left != right);
                    mem_index += 1;
                },

                VmOpCode::GreaterThan => {
                    let right = pop!(mem_index, scope.stack);
                    let left  = pop!(mem_index, scope.stack);
                    
                    scope.stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value > *r_value),
                        _ => empty_primative
                    };
                    mem_index += 1;
                },

                VmOpCode::GreaterEqualThan => {
                    let right = pop!(mem_index, scope.stack);
                    let left  = pop!(mem_index, scope.stack);
                    
                    scope.stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value >= *r_value),
                        _ => empty_primative
                    };
                    mem_index += 1;
                },

                VmOpCode::LessThan => {
                    let right = pop!(mem_index, scope.stack);
                    let left  = pop!(mem_index, scope.stack);
                    
                    scope.stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value < *r_value),
                        _ => empty_primative
                    };
                    mem_index += 1;
                },

                VmOpCode::LessEqualThan => {
                    let right = pop!(mem_index, scope.stack);
                    let left  = pop!(mem_index, scope.stack);
                    
                    scope.stack[mem_index] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value <= *r_value),
                        _ => empty_primative
                    };
                    mem_index += 1;
                },

                VmOpCode::Call => {
                    let location = ((options.opcodes[index + 2] as u16 * 256) + options.opcodes[index + 1] as u16) as usize;
                    let old_index = index + 1;
                    index = location as usize;
                    storages_index += 1;
                    let storage_location = ((options.opcodes[index + 1] as u16 * 256) + options.opcodes[index] as u16) as usize;

                    if storages.len() <= storage_location {
                        storages.resize(storages.len() * 2, Scope { mem_index: 0, location: 0, memory: Vec::new(), stack: Vec::new()});
                    }

                    storages[storage_location] = Scope {
                        memory: options.storages[storage_location].get_memory().borrow().to_vec(),
                        stack: options.storages[storage_location].get_stack().borrow().to_vec(),
                        location: old_index,
                        mem_index: mem_index
                    };

                    scope = &mut storages[storages_index];
                    mem_index = 0;
                    
                    index += 1;
                },

                VmOpCode::Return => {
                    index     = scope.location;
                    mem_index = scope.mem_index;
                    storages_index -= 1;
                    scope = &mut storages[storages_index];
                    
                },

                VmOpCode::NativeCall => {
                    let func_location = options.opcodes[index + 1] as usize;
                    
                    if let BramaPrimative::FuncNativeCall(func) = *scope.memory[func_location].deref() {
                        let total_args = options.opcodes[index + 2];
                        
                        match func(&scope.stack, mem_index, total_args) {
                            Ok(result) => {
                                mem_index        -= total_args as usize;
                                scope.stack[mem_index] = result;
                                mem_index        += 1;
                            },
                            Err((error, _, _)) => {
                                println!("{:?}", error);
                                return Err(error);
                            }
                        };

                        index += 2;
                    }
                },

                VmOpCode::Increment => {
                    scope.stack[mem_index - 1] = match &*scope.stack[mem_index - 1].deref() {
                        BramaPrimative::Number(value) => VmObject::from(*value + 1 as f64),
                        _ => empty_primative
                    };
                },

                VmOpCode::Decrement => {
                    scope.stack[mem_index - 1] = match &*scope.stack[mem_index - 1].deref() {
                        BramaPrimative::Number(value) => VmObject::from(*value - 1 as f64),
                        _ => empty_primative
                    };
                },

                VmOpCode::InitList => {
                    let total_item = options.opcodes[index + 1] as usize;
                    let mut list = Vec::with_capacity(total_item);

                    for _ in 0..total_item {
                        list.push(pop!(mem_index, scope.stack));
                    }
                    
                    scope.stack[mem_index] = VmObject::from(list);
                    mem_index += 1;
                    index     += 1;
                },

                VmOpCode::InitDict => {
                    let total_item = options.opcodes[index + 1] as usize;
                    let mut dict = HashMap::new();

                    for _ in 0..total_item {
                        let value = pop!(mem_index, scope.stack);
                        let key = pop!(mem_index, scope.stack);
                        
                        dict.insert(key.get_text(), value);
                    }
                    
                    scope.stack[mem_index] = VmObject::from(dict);
                    mem_index += 1;
                    index     += 1;
                },

                VmOpCode::Compare => {
                    let condition = pop!(mem_index, scope.stack);

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

                VmOpCode::GetItem => {
                    let indexer = pop!(mem_index, scope.stack);
                    let object  = pop!(mem_index, scope.stack);
                    
                    scope.stack[mem_index] = match &*object {
                        BramaPrimative::List(value) => {
                            let indexer_value = match &*indexer {
                                BramaPrimative::Number(number) => *number as u64,
                                _ => return Err("Indexer must be number")
                            };

                            match (*value).get(indexer_value as usize) {
                                Some(data) => VmObject::from(data.clone()),
                                _ => empty_primative
                            }
                        },
                        BramaPrimative::Dict(value) => {
                            let indexer_value = match &*indexer {
                                BramaPrimative::Text(text) => &*text,
                                _ => return Err("Indexer must be string")
                            };

                            match (*value).get(&indexer_value.to_string()) {
                                Some(data) => VmObject::from(data.clone()),
                                _ => empty_primative
                            }
                        }
                        _ => empty_primative
                    };
                    mem_index += 1;
                },

                VmOpCode::Func => (),
                VmOpCode::None => (),
            }

            index += 1;
        }

        #[cfg(feature = "dumpMemory")] {
            if scope.stack.len() > 0 {
                println!("╔════════════════════════════════════════╗");
                println!("║                  STACK                 ║");
                println!("╠════════════════════════════════════════╣");
                println!("║ Stack size: {:<10}                 ║", scope.stack.len());
                println!("╠════════════════════════════════════════╣");
                for i in 0..scope.stack.len() {
                    println!("║ {:38} ║", format!("{:?}", scope.stack[i as usize].deref()));
                }
                println!("╚════════════════════════════════════════╝");
            }
        }

        options.opcode_index = index;
    }

    #[cfg(feature = "dumpMemory")] {
        options.storages[0].dump();
    }

    Ok(())
}
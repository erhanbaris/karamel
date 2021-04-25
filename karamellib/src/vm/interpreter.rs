use crate::{pop, inc_memory_index, dec_memory_index, get_memory_index};
use crate::types::{VmObject};
use crate::compiler::*;
use std::sync::Arc;
use std::mem;
use std::collections::HashMap;
use std::io::stdout;
use log_update::LogUpdate;
use colored::*;
use std::io::{self, Write};
use crate::buildin::class::PRIMATIVE_CLASSES;
use crate::buildin::ClassProperty;

#[cfg(all(feature = "dumpOpcodes"))]
pub unsafe fn dump_opcode<W: Write>(index: usize, options: &mut BramaCompiler, log_update: &mut LogUpdate<W>) {
    #[cfg(feature = "liveOpcodeView")] {
        use std::{thread, time};
    }

    let mut buffer = String::new();

    fn build_arrow(index: usize, opcode_index: usize, opcode_length: usize, buffer: &mut String, data: &String) { 
        if index >= opcode_index && index <= opcode_index + opcode_length {
            buffer.push_str(&format!("║{:3}{}\r\n", " > ".green().bold() , data));
        } else {
            buffer.push_str(&format!("║{:3}{}\r\n", "", data));
        }
    }

    buffer.push_str("╔════════════════════════════════════════════╗\r\n");
    buffer.push_str("║                    OPCODE                  ║\r\n");
    buffer.push_str("╠═══╦══════╦═════════════════╦═══════╦═══════╣\r\n");
    let opcode_size   = options.opcodes.len();
    let mut opcode_index = 0;

    while opcode_size > opcode_index {
        let opcode =  mem::transmute::<u8, VmOpCode>(options.opcodes[opcode_index]);
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
            VmOpCode::SetItem |
            VmOpCode::Multiply => {
                let data = format!("║ {:4} ║ {:15} ║ {:^5} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), "", "").to_string();
                build_arrow(index, opcode_index, 0, &mut buffer, &data);
            },

            
            VmOpCode::Compare => {
                let location = opcode_index + ((options.opcodes[opcode_index+2] as u16 * 256) + options.opcodes[opcode_index+1] as u16) as usize;
                let data = format!("║ {:4} ║ {:15} ║ {:^5?} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), location, "");
                build_arrow(index, opcode_index, 0, &mut buffer, &data);
                opcode_index += 2;
            },

            VmOpCode::Jump => {
                let location = ((options.opcodes[opcode_index+2] as u16 * 256) + options.opcodes[opcode_index+1] as u16) as usize;
                let data = format!("║ {:4} ║ {:15} ║ {:^5?} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), location + 1, "");
                build_arrow(index, opcode_index, 0, &mut buffer, &data);
                opcode_index += 2;
            },

            VmOpCode::Func => {
                let data = format!("║ {:4} ║ {:15} ║ {:^5?} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), opcode_index + 1, "");
                build_arrow(index, opcode_index, 1, &mut buffer, &data);
                opcode_index += 1;
            },

            VmOpCode::InitArguments |
            VmOpCode::CopyToStore |
            VmOpCode::Load |
            VmOpCode::InitList |
            VmOpCode::InitDict |
            VmOpCode::Store => {
                let data = format!("║ {:4} ║ {:15} ║ {:^5?} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), options.opcodes[opcode_index + 1], "");
                build_arrow(index, opcode_index, 1, &mut buffer, &data);
                opcode_index += 1;
            },

            VmOpCode::None |
            VmOpCode::Return => {
                let data = format!("║ {:4} ║ {:15} ║ {:^5} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), "", "").to_string();
                build_arrow(index, opcode_index, 0, &mut buffer, &data);
            },

            VmOpCode::CallStack |
            VmOpCode::Call => {
                let location = (options.opcodes[opcode_index+1] as u16) as usize;
                let data = format!("║ {:4} ║ {:15} ║ {:^5?} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), location, options.opcodes[opcode_index + 2]);
                build_arrow(index, opcode_index, 2, &mut buffer, &data);
                opcode_index += 2;
            },
            
            VmOpCode::FastStore => {
                let data = format!("║ {:4} ║ {:15} ║ {:^5?} ║ {:^5} ║", opcode_index, format!("{:?}", opcode), options.opcodes[opcode_index + 1], options.opcodes[opcode_index + 2]);
                build_arrow(index, opcode_index, 2, &mut buffer, &data);
                opcode_index += 2;
            }
        }

        opcode_index += 1;
    }
    buffer.push_str("╚═══╩══════╩═════════════════╩═══════╩═══════╝\r\n");
    #[cfg(not(feature = "test"))] {
        log_update.render(&format!("{}", buffer)).unwrap();
        io::stdout().flush().unwrap();
    }
    #[cfg(feature = "liveOpcodeView")] {
        thread::sleep(time::Duration::from_millis(500));
    }
}

pub unsafe fn run_vm(options: &mut BramaCompiler) -> Result<Vec<VmObject>, String>
{
    let mut log_update = LogUpdate::new(stdout()).unwrap();
    
    #[cfg(feature = "dumpMemory")] {
        options.storages[0].dump();
    }
    
    #[cfg(all(feature = "dumpOpcodes"))] {
        dump_opcode(0, options, &mut log_update);
    }
    {
        let empty_primative: VmObject  = VmObject::convert(Arc::new(BramaPrimative::Empty));
        let opcode_size   = options.opcodes.len();

        options.scopes[options.scope_index] = Scope {
            memory: options.storages[0].get_memory().borrow().to_vec(),
            stack: options.storages[0].get_stack().borrow().to_vec(),
            location: 0,
            memory_index: 0,
            const_size: 0,
            call_return_assign_to_temp: false
        };

        while opcode_size > options.opcode_index {
            let opcode = mem::transmute::<u8, VmOpCode>(options.opcodes[options.opcode_index]);
            #[cfg(all(feature = "liveOpcodeView"))] {
                dump_opcode(options.opcode_index, options, &mut log_update);
            }
            
            match opcode {
                VmOpCode::Subraction => {
                    let right = pop!(options);
                    let left  = pop!(options);

                    (*options.current_scope).stack[get_memory_index!(options)] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value - *r_value),
                        _ => empty_primative
                    };
                    inc_memory_index!(options, 1);
                },

                VmOpCode::Addition => {
                    let right = pop!(options);
                    let left  = pop!(options);

                    (*options.current_scope).stack[get_memory_index!(options)] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value)) => VmObject::from(l_value + r_value),
                        (BramaPrimative::Text(l_value),    BramaPrimative::Text(r_value))   => VmObject::from(Arc::new((&**l_value).to_owned() + &**r_value)),
                        _ => empty_primative
                    };
                    inc_memory_index!(options, 1);
                },

                VmOpCode::Load => {
                    let tmp = options.opcodes[options.opcode_index + 1] as usize;
                    (*options.current_scope).stack[get_memory_index!(options)] = (*options.current_scope).memory[tmp];
                    options.opcode_index += 1;
                    inc_memory_index!(options, 1);
                },

                VmOpCode::Store => {
                    let tmp = options.opcodes[options.opcode_index + 1] as usize;
                    dec_memory_index!(options, 1);
                    (*options.current_scope).memory[tmp] = (*options.current_scope).stack[get_memory_index!(options)];
                    options.opcode_index += 1;
                },

                VmOpCode::CopyToStore => {
                    let tmp = options.opcodes[options.opcode_index + 1] as usize;
                    (*options.current_scope).memory[tmp] = (*options.current_scope).stack[get_memory_index!(options) - 1];
                    options.opcode_index += 1;
                },

                VmOpCode::FastStore => {
                    let destination = options.opcodes[options.opcode_index + 1] as usize;
                    let source      = options.opcodes[options.opcode_index + 2] as usize;
                    (*options.current_scope).memory[destination as usize] = (*options.current_scope).memory[source];
                    options.opcode_index += 2;
                },

                VmOpCode::Not => {
                    (*options.current_scope).stack[get_memory_index!(options) - 1] = VmObject::from(!(*options.current_scope).stack[get_memory_index!(options) - 1].deref().is_true());
                },

                VmOpCode::Dublicate => {
                    (*options.current_scope).stack[get_memory_index!(options)] = (*options.current_scope).stack[get_memory_index!(options) - 1];
                    inc_memory_index!(options, 1);
                },

                VmOpCode::And => {
                    let left = pop!(options);
                    let right = pop!(options);

                    (*options.current_scope).stack[get_memory_index!(options)] = VmObject::from(left.is_true() && right.is_true());
                    inc_memory_index!(options, 1);
                },

                VmOpCode::Or => {
                    let left = pop!(options);
                    let right = pop!(options);
                    (*options.current_scope).stack[get_memory_index!(options)] = VmObject::from(left.is_true() || right.is_true());
                    inc_memory_index!(options, 1);
                },

                VmOpCode::Multiply => {
                    let right = pop!(options);
                    let left  = pop!(options);
                    (*options.current_scope).stack[get_memory_index!(options)] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value * *r_value),
                        (BramaPrimative::Text(l_value),    BramaPrimative::Number(r_value))   => VmObject::from((*l_value).repeat((*r_value) as usize)),
                        _ => empty_primative
                    };
                    inc_memory_index!(options, 1);
                },

                VmOpCode::Division => {
                    let right = pop!(options);
                    let left  = pop!(options);

                    let calculation = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => (*l_value / *r_value),
                        _ => std::f64::NAN
                    };

                    (*options.current_scope).stack[get_memory_index!(options)] = if calculation.is_nan() {
                        empty_primative
                    }
                    else {
                        VmObject::from(calculation)
                    };

                    inc_memory_index!(options, 1);
                },

                VmOpCode::Equal => {                    
                    let right = pop!(options);
                    let left  = pop!(options);
                    
                    (*options.current_scope).stack[get_memory_index!(options)] = VmObject::from(left == right);
                    inc_memory_index!(options, 1);
                },


                VmOpCode::NotEqual => {
                    let right = pop!(options);
                    let left  = pop!(options);
                    
                    (*options.current_scope).stack[get_memory_index!(options)] = VmObject::from(left != right);
                    inc_memory_index!(options, 1);
                },

                VmOpCode::GreaterThan => {
                    let right = pop!(options);
                    let left  = pop!(options);
                    
                    (*options.current_scope).stack[get_memory_index!(options)] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value > *r_value),
                        _ => empty_primative
                    };
                    inc_memory_index!(options, 1);
                },

                VmOpCode::GreaterEqualThan => {
                    let right = pop!(options);
                    let left  = pop!(options);
                    
                    (*options.current_scope).stack[get_memory_index!(options)] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value >= *r_value),
                        _ => empty_primative
                    };
                    inc_memory_index!(options, 1);
                },

                VmOpCode::LessThan => {
                    let right = pop!(options);
                    let left  = pop!(options);
                    
                    (*options.current_scope).stack[get_memory_index!(options)] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value < *r_value),
                        _ => empty_primative
                    };
                    inc_memory_index!(options, 1);
                },

                VmOpCode::LessEqualThan => {
                    let right = pop!(options);
                    let left  = pop!(options);
                    
                    (*options.current_scope).stack[get_memory_index!(options)] = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value <= *r_value),
                        _ => empty_primative
                    };
                    inc_memory_index!(options, 1);
                },

                VmOpCode::Call => {
                    let func_location = options.opcodes[options.opcode_index + 1] as usize;
                    options.opcode_index += 1;
                    
                    if let BramaPrimative::Function(reference) = &*(*options.current_scope).memory[func_location].deref() {
                        reference.execute(options)?;
                    }
                    else {
                        return Err("Not callable".to_string());
                    }
                },

                VmOpCode::CallStack => {
                    let function = pop_raw!(options);
                    match &*function.deref() {
                        BramaPrimative::Function(reference) => reference.execute(options)?,
                        _ => {
                            log::debug!("{:?} not callable", &*function.deref());
                        return Err("Not callable".to_string());
                        }
                    };
                },

                VmOpCode::Return => {
                    let return_value = (*options.current_scope).stack[get_memory_index!(options)-1];
                    options.opcode_index = (*options.current_scope).location;
                    let call_return_assign_to_temp = (*options.current_scope).call_return_assign_to_temp;
                    options.scope_index -= 1;
                    options.current_scope = &mut options.scopes[options.scope_index] as *mut Scope;

                    if call_return_assign_to_temp {
                        (*options.current_scope).stack[get_memory_index!(options)] = return_value;
                        inc_memory_index!(options, 1);
                    }
                },

                VmOpCode::Increment => {
                    (*options.current_scope).stack[get_memory_index!(options) - 1] = match &*(*options.current_scope).stack[get_memory_index!(options) - 1].deref() {
                        BramaPrimative::Number(value) => VmObject::from(*value + 1 as f64),
                        _ => empty_primative
                    };
                },

                VmOpCode::Decrement => {
                    (*options.current_scope).stack[get_memory_index!(options) - 1] = match &*(*options.current_scope).stack[get_memory_index!(options) - 1].deref() {
                        BramaPrimative::Number(value) => VmObject::from(*value - 1 as f64),
                        _ => empty_primative
                    };
                },

                VmOpCode::InitList => {
                    let total_item = options.opcodes[options.opcode_index + 1] as usize;
                    let mut list = Vec::with_capacity(total_item);

                    for _ in 0..total_item {
                        list.push(pop_raw!(options));
                    }
                    
                    (*options.current_scope).stack[get_memory_index!(options)] = VmObject::from(list);
                    inc_memory_index!(options, 1);
                    options.opcode_index += 1;
                },

                VmOpCode::InitDict => {
                    let total_item = options.opcodes[options.opcode_index + 1] as usize;
                    let mut dict = HashMap::new();

                    for _ in 0..total_item {
                        let value = pop_raw!(options);
                        let key   = pop!(options);
                        
                        dict.insert(key.get_text(), value);
                    }
                    
                    (*options.current_scope).stack[get_memory_index!(options)] = VmObject::from(dict);
                    inc_memory_index!(options, 1);
                    options.opcode_index += 1;
                },

                VmOpCode::Compare => {
                    let condition = pop!(options);

                    let status = match &*condition {
                        BramaPrimative::Empty => false,
                        BramaPrimative::Bool(l_value) => *l_value,
                        BramaPrimative::Number(l_value) => *l_value > 0.0,
                        BramaPrimative::Text(l_value) => !(*l_value).is_empty(),
                        _ => false
                    };

                    if status {
                        options.opcode_index += 2 as usize;
                    }
                    else {
                        let location = ((options.opcodes[options.opcode_index + 2] as u16 * 256) + options.opcodes[options.opcode_index + 1] as u16) as usize;
                        options.opcode_index += location as usize;
                    }
                },

                VmOpCode::Jump => {
                    let location = ((options.opcodes[options.opcode_index + 2] as u16 * 256) + options.opcodes[options.opcode_index + 1] as u16) as usize;
                    options.opcode_index = location as usize;
                    continue;
                },
                
                VmOpCode::SetItem => {
                    let assign_item  = pop_raw!(options);
                    let indexer = pop!(options);
                    let object  = pop!(options);

                    match &*object {
                        BramaPrimative::List(value) => {
                            let indexer_value = match &*indexer {
                                BramaPrimative::Number(number) => *number as usize,
                                _ => return Err("Indexer must be number".to_string())
                            };

                            value.borrow_mut()[indexer_value] = assign_item;
                        },
                        BramaPrimative::Dict(value) => {
                            let indexer_value = match &*indexer {
                                BramaPrimative::Text(text) => &*text,
                                _ => return Err("Indexer must be string".to_string())
                            };

                            value.borrow_mut().insert(indexer_value.to_string(), assign_item);
                        }
                        _ => ()
                    };
                },

                VmOpCode::GetItem => {
                    let indexer = pop!(options);
                    let raw_object  = pop_raw!(options);
                    let object = &*raw_object.deref();

                    (*options.current_scope).stack[get_memory_index!(options)] = match &*indexer {
                        BramaPrimative::Text(text) => {
                             match object.get_class().get_element(Some(raw_object), text.clone()) {
                                Some(element) => match element {
                                    ClassProperty::Function(function) => VmObject::from(Arc::new(BramaPrimative::Function(function.clone()))),
                                    ClassProperty::Field(field) => VmObject::from(field.clone())
                                },
                                _ => empty_primative
                            }
                        },
                        BramaPrimative::Number(index) => match object.get_class().get_getter() {
                            Some(function) => function(raw_object, *index as usize)?,
                            _ => empty_primative
                        }
                        _ => empty_primative
                    };

                    inc_memory_index!(options, 1);
                },

                VmOpCode::InitArguments => {
                    let size = options.opcodes[options.opcode_index + 1] as usize;
                    let const_size = (*options.current_scope).const_size as usize;
                    for i in 0..size {
                        dec_memory_index!(options, 1);
                        (*options.current_scope).memory[i + const_size] = (*options.current_scope).stack[get_memory_index!(options)];
                    }

                    options.opcode_index += 1;
                },
                VmOpCode::Func => (),
                VmOpCode::None => (),
            }

            options.opcode_index += 1;
        }

        
        for (index, item) in options.scopes[0].stack.iter().enumerate() {
            options.storages[0].get_stack().borrow_mut()[index] = *item;
        }

        for (index, item) in options.scopes[0].memory.iter().enumerate() {
            options.storages[0].get_memory().borrow_mut()[index] = *item;
        }
        
        #[cfg(feature = "dumpMemory")] {
            options.storages[0].dump();
        }
    }
    
    let mut result = Vec::with_capacity((*options.current_scope).memory_index);
    for index in 0..(*options.current_scope).memory_index {
        result.push((*options.current_scope).stack[index]);
    }

    Ok(result)
}
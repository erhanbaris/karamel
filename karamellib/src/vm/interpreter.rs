use crate::{pop, inc_memory_index, dec_memory_index, get_memory_index};
use crate::types::{VmObject};
use crate::compiler::*;
use std::rc::Rc;
use std::mem;
use std::collections::HashMap;
use std::io::stdout;
use log_update::LogUpdate;
use std::ptr;
use colored::*;
use std::io::{self, Write};
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
            VmOpCode::Module |
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
            VmOpCode::Halt |
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
        let mut stack = options.storages[0].get_stack();
        let stack_ptr = stack.as_mut_ptr();

        let mut memory = options.storages[0].get_memory();
        let memory_ptr = memory.as_mut_ptr();

        options.scopes[options.scope_index] = Scope {
            memory: memory,
            stack: stack,
            location: ptr::null_mut(),
            const_size: 0,
            call_return_assign_to_temp: false,
            stack_ptr: stack_ptr,
            memory_ptr: memory_ptr,
            storage_index: 0
        };

        loop {
            let opcode = mem::transmute::<u8, VmOpCode>(*options.opcodes_ptr);
            #[cfg(all(feature = "liveOpcodeView"))] {
                dump_opcode(options.opcode_index, options, &mut log_update);
            }
            
            match opcode {
                VmOpCode::Subraction => {
                    let right = pop_raw!(options);
                    let left  = pop_raw!(options);

                    *(*options.current_scope).stack_ptr = match (left.as_number(), right.as_number()) {
                        (Some(l_value),  Some(r_value))   => VmObject::from(l_value - r_value),
                        _ => EMPTY_OBJECT
                    };
                    inc_memory_index!(options, 1);
                },

                VmOpCode::Addition => {
                    let right = pop_raw!(options);
                    let left  = pop_raw!(options);
                    *(*options.current_scope).stack_ptr = match (&left.deref_clean(), &right.deref_clean()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value)) => VmObject::from(l_value + r_value),
                        (BramaPrimative::Text(l_value),    BramaPrimative::Text(r_value))   => VmObject::from(Rc::new((&**l_value).to_owned() + &**r_value)),
                        _ => EMPTY_OBJECT
                    };
                    inc_memory_index!(options, 1);
                },

                VmOpCode::Load => {
                    let tmp   = *options.opcodes_ptr.offset(1) as usize;
                    let scope = &mut *options.current_scope;
                    *scope.stack_ptr = *scope.memory_ptr.offset(tmp as isize);
                    options.opcodes_ptr = options.opcodes_ptr.offset(1);
                    inc_memory_index!(options, 1);
                },

                VmOpCode::Store => {
                    let tmp = *options.opcodes_ptr.offset(1) as usize;
                    dec_memory_index!(options, 1);
                    *(*options.current_scope).memory_ptr.offset(tmp as isize) = *(*options.current_scope).stack_ptr;
                    options.opcodes_ptr = options.opcodes_ptr.offset(1);
                },

                VmOpCode::CopyToStore => {
                    let tmp = *options.opcodes_ptr.offset(1) as usize;
                    *(*options.current_scope).memory_ptr.offset(tmp as isize) = *(*options.current_scope).stack_ptr.sub(1);
                    options.opcodes_ptr = options.opcodes_ptr.offset(1);
                },

                VmOpCode::FastStore => {
                    let destination = *options.opcodes_ptr.offset(1) as usize;
                    let source      = *options.opcodes_ptr.offset(2) as usize;
                    *(*options.current_scope).memory_ptr.offset(destination as isize) = *(*options.current_scope).memory_ptr.offset(source as isize);
                    options.opcodes_ptr = options.opcodes_ptr.offset(2);
                },

                VmOpCode::Not => {
                    *(*options.current_scope).stack_ptr.sub(1) = VmObject::from(!(*(*options.current_scope).stack_ptr.sub(1)).deref().is_true());
                },

                VmOpCode::Dublicate => {
                    *(*options.current_scope).stack_ptr = *(*options.current_scope).stack_ptr.sub(1);
                    inc_memory_index!(options, 1);
                },

                VmOpCode::And => {
                    let left = pop!(options);
                    let right = pop!(options);

                    *(*options.current_scope).stack_ptr = VmObject::from(left.is_true() && right.is_true());
                    inc_memory_index!(options, 1);
                },

                VmOpCode::Or => {
                    let left = pop!(options);
                    let right = pop!(options);
                    *(*options.current_scope).stack_ptr = VmObject::from(left.is_true() || right.is_true());
                    inc_memory_index!(options, 1);
                },

                VmOpCode::Multiply => {
                    let right = pop!(options);
                    let left  = pop!(options);
                    *(*options.current_scope).stack_ptr = match (&*left, &*right) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value * *r_value),
                        (BramaPrimative::Text(l_value),    BramaPrimative::Number(r_value))   => VmObject::from((*l_value).repeat((*r_value) as usize)),
                        _ => EMPTY_OBJECT
                    };
                    inc_memory_index!(options, 1);
                },

                VmOpCode::Division => {
                    let right = pop_raw!(options);
                    let left  = pop_raw!(options);

                    let calculation = match (left.as_number(), right.as_number()) {
                        (Some(l_value),  Some(r_value))   => (l_value / r_value),
                        _ => std::f64::NAN
                    };

                    *(*options.current_scope).stack_ptr = if calculation.is_nan() {
                        EMPTY_OBJECT
                    }
                    else {
                        VmObject::from(calculation)
                    };

                    inc_memory_index!(options, 1);
                },

                VmOpCode::Module => {
                    let right = pop_raw!(options);
                    let left  = pop_raw!(options);

                    *(*options.current_scope).stack_ptr = match (left.as_number(), right.as_number()) {
                        (Some(l_value),  Some(r_value))   => VmObject::from(l_value % r_value),
                        _ => EMPTY_OBJECT
                    };

                    inc_memory_index!(options, 1);
                },

                VmOpCode::Equal => {                    
                    let right = pop!(options);
                    let left  = pop!(options);
                    
                    *(*options.current_scope).stack_ptr = VmObject::from(left == right);
                    inc_memory_index!(options, 1);
                },


                VmOpCode::NotEqual => {
                    let right = pop!(options);
                    let left  = pop!(options);
                    
                    *(*options.current_scope).stack_ptr = VmObject::from(left != right);
                    inc_memory_index!(options, 1);
                },

                VmOpCode::GreaterThan => {
                    let right = pop_raw!(options);
                    let left  = pop_raw!(options);
                    
                    *(*options.current_scope).stack_ptr = match (left.as_number(), right.as_number()) {
                        (Some(l_value),  Some(r_value))   => VmObject::from(l_value > r_value),
                        _ => EMPTY_OBJECT
                    };
                    inc_memory_index!(options, 1);
                },

                VmOpCode::GreaterEqualThan => {
                    let right = pop_raw!(options);
                    let left  = pop_raw!(options);
                    
                    *(*options.current_scope).stack_ptr = match (left.as_number(), right.as_number()) {
                        (Some(l_value),  Some(r_value))   => VmObject::from(l_value >= r_value),
                        _ => EMPTY_OBJECT
                    };
                    inc_memory_index!(options, 1);
                },

                VmOpCode::LessThan => {
                    let right = pop_raw!(options);
                    let left  = pop_raw!(options);
                    
                    *(*options.current_scope).stack_ptr = match (left.as_number(), right.as_number()) {
                        (Some(l_value),  Some(r_value))   => VmObject::from(l_value < r_value),
                        _ => EMPTY_OBJECT
                    };
                    inc_memory_index!(options, 1);
                },

                VmOpCode::LessEqualThan => {
                    let right = pop_raw!(options);
                    let left  = pop_raw!(options);
                    
                    *(*options.current_scope).stack_ptr = match (left.as_number(), right.as_number()) {
                        (Some(l_value),  Some(r_value))   => VmObject::from(l_value <= r_value),
                        _ => EMPTY_OBJECT
                    };
                    inc_memory_index!(options, 1);
                },

                VmOpCode::Call => {
                    let func_location   = *options.opcodes_ptr.offset(1) as usize;
                    options.opcodes_ptr = options.opcodes_ptr.offset(1);
                    
                    if let BramaPrimative::Function(reference, _) = &(*(*options.current_scope).memory_ptr.offset(func_location as isize)).deref_clean() {
                        reference.execute(options, None)?;
                    }
                    else {
                        return Err("Not callable".to_string());
                    }
                },

                VmOpCode::CallStack => {
                    let function = pop_raw!(options);
                    match &*function.deref() {
                        BramaPrimative::Function(reference, base) => reference.execute(options, *base)?,
                        _ => {
                            log::debug!("{:?} not callable", &*function.deref());
                        return Err("Not callable".to_string());
                        }
                    };
                },

                VmOpCode::Return => {
                    let return_value               = *(*options.current_scope).stack_ptr.sub(1);
                    options.opcodes_ptr            = (*options.current_scope).location;
                    let call_return_assign_to_temp = (*options.current_scope).call_return_assign_to_temp;
                    options.scope_index           -= 1;
                    options.current_scope          = &mut options.scopes[options.scope_index] as *mut Scope;

                    if call_return_assign_to_temp {
                        *(*options.current_scope).stack_ptr = return_value;
                        inc_memory_index!(options, 1);
                    }
                },

                VmOpCode::Increment => {
                    *(*options.current_scope).stack_ptr.sub(1) = match (*(*options.current_scope).stack_ptr.sub(1)).as_number() {
                        Some(value) => VmObject::from(value + 1 as f64),
                        _ => EMPTY_OBJECT
                    };
                },

                VmOpCode::Decrement => {
                    *(*options.current_scope).stack_ptr.sub(1) = match (*(*options.current_scope).stack_ptr.sub(1)).as_number() {
                        Some(value) => VmObject::from(value - 1 as f64),
                        _ => EMPTY_OBJECT
                    };
                },

                VmOpCode::InitList => {
                    let total_item = *options.opcodes_ptr.offset(1);
                    let mut list = Vec::with_capacity(total_item.into());

                    for _ in 0..total_item {
                        list.push(pop_raw!(options));
                    }
                    
                    *(*options.current_scope).stack_ptr = VmObject::from(list);
                    inc_memory_index!(options, 1);
                    options.opcodes_ptr = options.opcodes_ptr.offset(1);
                },

                VmOpCode::InitDict => {
                    let total_item = *options.opcodes_ptr.offset(1) as usize;
                    let mut dict   = HashMap::new();

                    for _ in 0..total_item {
                        let value = pop_raw!(options);
                        let key   = pop!(options);
                        
                        dict.insert(key.get_text(), value);
                    }
                    
                    *(*options.current_scope).stack_ptr = VmObject::from(dict);
                    inc_memory_index!(options, 1);
                    options.opcodes_ptr = options.opcodes_ptr.offset(1);
                },

                VmOpCode::Compare => {
                    let condition = pop_raw!(options);

                    let status = match &condition.deref_clean() {
                        BramaPrimative::Empty => false,
                        BramaPrimative::Bool(l_value) => *l_value,
                        BramaPrimative::Number(l_value) => *l_value > 0.0,
                        BramaPrimative::Text(l_value) => !(*l_value).is_empty(),
                        _ => false
                    };

                    if status {
                        options.opcodes_ptr = options.opcodes_ptr.offset(2);
                    }
                    else {
                        let location = ((*options.opcodes_ptr.offset(2) as u16 * 256) + *options.opcodes_ptr.offset(1) as u16) as usize;
                        options.opcodes_ptr = options.opcodes_ptr.offset(location as isize);
                    }
                },

                VmOpCode::Jump => {
                    let location = ((*options.opcodes_ptr.offset(2)  as u16 * 256) + *options.opcodes_ptr.offset(1)  as u16) as usize;
                    options.opcodes_ptr = options.opcodes.as_mut_ptr().offset(location as isize);
                    continue;
                },
                
                VmOpCode::SetItem => {
                    let assign_item  = pop_raw!(options);
                    let indexer = pop!(options);
                    let raw_object = pop_raw!(options);
                    let object  = raw_object.deref();

                    // todo: change all those codes with setter implementation

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
                        },
                        BramaPrimative::Text(_) => {
                            let indexer_value = match &*indexer {
                                BramaPrimative::Number(number) => *number,
                                _ => return Err("Indexer must be number".to_string())
                            };

                            match options.get_class(&object).get_setter() {
                                Some(function) => function(raw_object, indexer_value, assign_item)?,
                                _ => EMPTY_OBJECT
                            };
                        },
                        
                        _ => ()
                    };
                },

                VmOpCode::GetItem => {
                    let indexer = pop!(options);
                    let raw_object  = pop_raw!(options);
                    let object = &*raw_object.deref();

                    *(*options.current_scope).stack_ptr = match &*indexer {
                        BramaPrimative::Text(text) => {
                             match options.get_class(object).get_element(Some(raw_object), text.clone()) {
                                Some(element) => match element {
                                    ClassProperty::Function(function) => VmObject::from(Rc::new(BramaPrimative::Function(function.clone(), Some(raw_object)))),
                                    ClassProperty::Field(field) => VmObject::from(field.clone())
                                },
                                _ => EMPTY_OBJECT
                            }
                        },
                        BramaPrimative::Number(index) => match options.get_class(object).get_getter() {
                            Some(function) => function(raw_object, *index)?,
                            _ => EMPTY_OBJECT
                        }
                        _ => EMPTY_OBJECT
                    };

                    inc_memory_index!(options, 1);
                },

                VmOpCode::InitArguments => {
                    let size = *options.opcodes_ptr.offset(1) as usize;
                    let const_size = (*options.current_scope).const_size as usize;
                    for i in 0..size {
                        dec_memory_index!(options, 1);
                        *(*options.current_scope).memory_ptr.offset((i + const_size) as isize) = *(*options.current_scope).stack_ptr;
                    }

                    options.opcodes_ptr = options.opcodes_ptr.offset(1);
                },
                VmOpCode::Func => (),
                VmOpCode::None => (),
                VmOpCode::Halt => {
                    break;
                },
            }

            options.opcodes_ptr = options.opcodes_ptr.offset(1);
        }

        
        for (index, item) in options.scopes[0].stack.iter().enumerate() {
            options.storages[0].get_mut_stack()[index] = *item;
        }

        for (index, item) in options.scopes[0].memory.iter().enumerate() {
            options.storages[0].get_mut_memory()[index] = *item;
        }
        
        #[cfg(feature = "dumpMemory")] {
            options.storages[0].dump();
        }
    }
    
    let mut result = Vec::with_capacity(get_memory_index!(options) as usize);
    for index in 0..get_memory_index!(options) {
        result.push(*(*options.current_scope).stack_ptr.add(index as usize));
    }

    Ok(result)
}
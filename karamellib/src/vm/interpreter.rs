use crate::compiler::context::KaramelCompilerContext;
use crate::compiler::scope::Scope;
use crate::error::KaramelErrorType;
use crate::{pop, inc_memory_index, dec_memory_index, get_memory_index, karamel_dbg};
use crate::types::{VmObject};
use crate::compiler::*;
use std::rc::Rc;
use std::mem;
use std::collections::HashMap;
use std::io::stdout;
use std::sync::atomic::AtomicUsize;
use log_update::LogUpdate;
use std::io::{self, Write};
use std::ptr;
use colored::*;
use crate::buildin::ClassProperty;

#[cfg(all(feature = "NONONO"))]
pub unsafe fn dump_opcode<W: Write>(index: usize, context: &mut KaramelCompilerContext, log_update: &mut LogUpdate<W>) {
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

    #[cfg(not(feature = "test"))] {
        log_update.render(&format!("{}", buffer)).unwrap();
        io::stdout().flush().unwrap();
    }
    #[cfg(feature = "liveOpcodeView")] {
        thread::sleep(time::Duration::from_millis(500));
    }
}

pub unsafe fn run_vm(context: &mut KaramelCompilerContext) -> Result<Vec<VmObject>, KaramelErrorType>
{
    #[cfg(any(feature = "liveOpcodeView", feature = "dumpOpcodes"))]
    let mut log_update = LogUpdate::new(stdout()).unwrap();
    
    #[cfg(feature = "dumpMemory")] {
        context.storages[0].dump();
    }
    
    #[cfg(all(feature = "dumpOpcodes"))] {    
        let generated = context.opcode_generator.dump(&context.opcodes);
        log_update.render(&generated[..]);
        return Ok(Vec::new());
    }

    // Save top stack for main storage
    let top_stack = context.stack.as_mut_ptr();

    // Move stack pointer to forward. First slots are reserved for variable memories.
    context.stack_ptr = top_stack.add(context.storages[0].variables.len());
    context.storages_ptr = context.storages.as_mut_ptr();
    {
        context.scopes[context.scope_index] = Scope {
            location: ptr::null_mut(),
            call_return_assign_to_temp: false,
            top_stack: top_stack,
            constant_ptr: context.storages[0].constants.as_ptr()
        };

        loop {
            let opcode = mem::transmute::<u8, VmOpCode>(*context.opcodes_ptr);
            #[cfg(all(feature = "liveOpcodeView"))] {
                dump_opcode(context.opcode_index, context, &mut log_update);
            }
            
            match karamel_dbg_any!(opcode) {
                VmOpCode::Subraction => {
                    let right = pop_raw!(context, "right");
                    let left = pop_raw!(context, "left");

                    karamel_print_level2!("Subraction: {:?} - {:?}", left, right);

                    *context.stack_ptr = match (left.as_number(), right.as_number()) {
                        (Some(l_value),  Some(r_value))   => VmObject::from(karamel_dbg!(l_value) - karamel_dbg!(r_value)),
                        _ => EMPTY_OBJECT
                    };
                    inc_memory_index!(context, 1);
                    dump_data!(context, "result");
                },

                VmOpCode::Addition => {
                    let right = pop_raw!(context, "right");
                    let left = pop_raw!(context, "left");
                    karamel_print_level2!("Addition: {:?} + {:?}", left, right);

                    *context.stack_ptr = match (&left.deref_clean(), &right.deref_clean()) {
                        (KaramelPrimative::Number(l_value),  KaramelPrimative::Number(r_value)) => VmObject::from(karamel_dbg!(l_value) + karamel_dbg!(r_value)),
                        (KaramelPrimative::Text(l_value),    KaramelPrimative::Text(r_value))   => VmObject::from(Rc::new((&**l_value).to_owned() + &**r_value)),
                        _ => EMPTY_OBJECT
                    };
                    dump_data!(context, "result");
                    inc_memory_index!(context, 1);
                },

                VmOpCode::Load => {
                    let tmp   = *context.opcodes_ptr.offset(1) as usize;
                    let scope = &mut *context.current_scope;
                    *context.stack_ptr = karamel_dbg!(*scope.top_stack.offset(tmp as isize));
                    context.opcodes_ptr = context.opcodes_ptr.offset(1);
                    karamel_print_level2!("Load: [{:?}]: {:?}", tmp, *context.stack_ptr);
                    dump_data!(context, "loaded");
                    inc_memory_index!(context, 1);
                },

                VmOpCode::Constant => {
                    let tmp   = *context.opcodes_ptr.offset(1) as usize;
                    let scope = &mut *context.current_scope;        
                    *context.stack_ptr = karamel_dbg!(*scope.constant_ptr.offset(tmp as isize));        
                    context.opcodes_ptr = context.opcodes_ptr.offset(1);
                    karamel_print_level2!("Constant: [{:?}]: {:?}", tmp, *context.stack_ptr);
                    dump_data!(context, "constant loaded");
                    inc_memory_index!(context, 1);
                },

                VmOpCode::Store => {
                    let tmp = *context.opcodes_ptr.offset(1) as usize;
                    dec_memory_index!(context, 1);
                    *(*context.current_scope).top_stack.offset(tmp as isize) = karamel_dbg!(*context.stack_ptr);
                    context.opcodes_ptr = context.opcodes_ptr.offset(1);
                    karamel_print_level2!("Store: [{:?}]: {:?}", tmp, *context.stack_ptr);
                },

                VmOpCode::CopyToStore => {
                    let tmp = *context.opcodes_ptr.offset(1) as usize;
                    *(*context.current_scope).top_stack.offset(tmp as isize) = karamel_dbg!(*context.stack_ptr.sub(1));
                    context.opcodes_ptr = context.opcodes_ptr.offset(1);
                    karamel_print_level2!("CopyToStore: [{:?}]: {:?}", tmp, *context.stack_ptr);
                },

                VmOpCode::FastStore => {
                    let destination = *context.opcodes_ptr.offset(1) as usize;
                    let source      = *context.opcodes_ptr.offset(2) as usize;
                    *(*context.current_scope).top_stack.offset(destination as isize) = karamel_dbg!(*(*context.current_scope).constant_ptr.offset(source as isize));
                    context.opcodes_ptr = context.opcodes_ptr.offset(2);
                    karamel_print_level2!("FastStore: {:?}: {:?} => {:?}", *(*context.current_scope).top_stack.offset(destination as isize), source, destination);
                },

                VmOpCode::Not => {
                    *context.stack_ptr.sub(1) = VmObject::from(!(*context.stack_ptr.sub(1)).deref_clean().is_true());
                    dump_data!(context, "result");
                    karamel_print_level2!("Not: {:?}", *context.stack_ptr.sub(1));
                },

                VmOpCode::Dublicate => {
                    *context.stack_ptr = karamel_dbg!(*context.stack_ptr.sub(1));
                    karamel_print_level2!("Dublicate: {:?}", *context.stack_ptr);
                    inc_memory_index!(context, 1);
                },

                VmOpCode::And => {
                    let left  = pop!(context, "left");
                    let right = pop!(context, "right");
                    karamel_print_level2!("And: {:?} && {:?}", left, right);

                    *context.stack_ptr = VmObject::from(karamel_dbg!(left.is_true()) && karamel_dbg!(right.is_true()));
                    dump_data!(context, "result");
                    inc_memory_index!(context, 1);
                },

                VmOpCode::Or => {
                    let left  = pop!(context, "left");
                    let right = pop!(context, "right");
                    karamel_print_level2!("Or: {:?} || {:?}", left, right);

                    *context.stack_ptr = VmObject::from(karamel_dbg!(left.is_true()) || karamel_dbg!(right.is_true()));
                    dump_data!(context, "result");
                    inc_memory_index!(context, 1);
                },

                VmOpCode::Multiply => {
                    let right = pop!(context, "right");
                    let left  = pop!(context, "left");
                    karamel_print_level2!("Multiply: {:?} * {:?}", left, right);

                    *context.stack_ptr = match (&*left, &*right) {
                        (KaramelPrimative::Number(l_value),  KaramelPrimative::Number(r_value))   => VmObject::from(*l_value * *r_value),
                        (KaramelPrimative::Text(l_value),    KaramelPrimative::Number(r_value))   => VmObject::from((*l_value).repeat((*r_value) as usize)),
                        _ => EMPTY_OBJECT
                    };
                    dump_data!(context, "result");
                    inc_memory_index!(context, 1);
                },

                VmOpCode::Division => {
                    let right = pop_raw!(context, "right");
                    let left = pop_raw!(context, "left");
                    karamel_print_level2!("Division: {:?} / {:?}", left, right);

                    let calculation = match (left.as_number(), right.as_number()) {
                        (Some(l_value),  Some(r_value))   => (l_value / r_value),
                        _ => std::f64::NAN
                    };

                    *context.stack_ptr = if calculation.is_nan() {
                        EMPTY_OBJECT
                    }
                    else {
                        VmObject::from(calculation)
                    };

                    inc_memory_index!(context, 1);
                },

                VmOpCode::Module => {
                    let right = pop_raw!(context, "right");
                    let left = pop_raw!(context, "left");
                    karamel_print_level2!("Module: {:?} / {:?}", left, right);

                    *context.stack_ptr = match (left.as_number(), right.as_number()) {
                        (Some(l_value),  Some(r_value))   => VmObject::from(karamel_dbg!(l_value) % karamel_dbg!(r_value)),
                        _ => EMPTY_OBJECT
                    };
                    dump_data!(context, "result");
                    inc_memory_index!(context, 1);
                },

                VmOpCode::Equal => {
                    let right = pop!(context, "right");
                    let left  = pop!(context, "left");
                    karamel_print_level2!("Equal: {:?} == {:?}", left, right);
                    
                    *context.stack_ptr = VmObject::from(karamel_dbg!(left) == karamel_dbg!(right));
                    dump_data!(context, "result");
                    inc_memory_index!(context, 1);
                },


                VmOpCode::NotEqual => {
                    let right = pop!(context, "right");
                    let left  = pop!(context, "left");
                    karamel_print_level2!("NotEqual: {:?} != {:?}", left, right);
                    
                    *context.stack_ptr = VmObject::from(karamel_dbg!(left) != karamel_dbg!(right));
                    dump_data!(context, "result");
                    inc_memory_index!(context, 1);
                },

                VmOpCode::GreaterThan => {
                    let right = pop_raw!(context, "right");
                    let left = pop_raw!(context, "left");
                    karamel_print_level2!("GreaterThan: {:?} > {:?}", left, right);
                    
                    *context.stack_ptr = match (left.as_number(), right.as_number()) {
                        (Some(l_value),  Some(r_value))   => VmObject::from(karamel_dbg!(l_value) > karamel_dbg!(r_value)),
                        _ => EMPTY_OBJECT
                    };
                    dump_data!(context, "result");
                    inc_memory_index!(context, 1);
                },

                VmOpCode::GreaterEqualThan => {
                    let right = pop_raw!(context, "right");
                    let left = pop_raw!(context, "left");
                    karamel_print_level2!("GreaterEqualThan {:?} >= {:?}", left, right);
                    
                    *context.stack_ptr = match (left.as_number(), right.as_number()) {
                        (Some(l_value),  Some(r_value))   => VmObject::from(karamel_dbg!(l_value) >= karamel_dbg!(r_value)),
                        _ => EMPTY_OBJECT
                    };
                    dump_data!(context, "result");
                    inc_memory_index!(context, 1);
                },

                VmOpCode::Call => {
                    let func_location   = *context.opcodes_ptr.offset(1) as usize;
                    context.opcodes_ptr = context.opcodes_ptr.offset(1);
                    
                    let value = (*(*context.current_scope).constant_ptr.offset(func_location as isize)).deref();

                    karamel_print_level2!("Call: {:?}", value);
                    if let KaramelPrimative::Function(reference, _) = karamel_dbg!(&*value) {
                        reference.execute(context, None)?;
                    }
                    else {
                        return Err(KaramelErrorType::NotCallable(value.clone()));
                    }
                },

                VmOpCode::CallStack => {
                    let function = pop_raw!(context, "function");
                    let value =  function.deref();
                    karamel_print_level2!("CallStack {:?}", value);
                    
                    match &*value {
                        KaramelPrimative::Function(reference, base) => reference.execute(context, *base)?,
                        _ => {
                            log::debug!("{:?} not callable", &*function.deref());
                        return Err(KaramelErrorType::NotCallable(value.clone()));
                        }
                    };
                },

                VmOpCode::Return => {
                    let return_value               = *context.stack_ptr.sub(1);
                    context.opcodes_ptr            = (*context.current_scope).location;
                    let call_return_assign_to_temp = (*context.current_scope).call_return_assign_to_temp;
                    context.scope_index           -= 1;

                    context.stack_ptr = (*context.current_scope).top_stack;
                    context.current_scope          = context.scopes_ptr.add(context.scope_index);              

                    if call_return_assign_to_temp {
                        *context.stack_ptr = return_value;
                        karamel_print_level2!("Return [{:?}] {:?}", get_memory_index!(context), *context.stack_ptr);
                        inc_memory_index!(context, 1);
                    } else {
                        karamel_print_level2!("Return");
                    }
                },

                VmOpCode::Increment => {
                    karamel_print_level2!("Increment");
                    *context.stack_ptr.sub(1) = match (*context.stack_ptr.sub(1)).as_number() {
                        Some(value) => VmObject::from(karamel_dbg!(value + 1 as f64)),
                        _ => EMPTY_OBJECT
                    };
                },

                VmOpCode::Decrement => {
                    karamel_print_level2!("Increment");
                    *context.stack_ptr.sub(1) = match (*context.stack_ptr.sub(1)).as_number() {
                        Some(value) => VmObject::from(value - 1 as f64),
                        _ => EMPTY_OBJECT
                    };
                },

                VmOpCode::Init => {
                    let init_type = *context.opcodes_ptr.offset(1) as usize;
                    let total_item = *context.opcodes_ptr.offset(2) as usize;
                    karamel_print_level2!("Init: {:?} {:?}", init_type, total_item);

                    *context.stack_ptr = match init_type {
                        // Dict
                        0 => {
                            let mut dict   = HashMap::new();
        
                            for _ in 0..total_item {
                                let value = pop_raw!(context, "value");
                                let key   = pop!(context, "key");
                                
                                dict.insert(key.get_text(), value);
                            }

                            VmObject::from(dict)
                        },

                        // List
                        1 => {
                            let mut list = Vec::with_capacity(total_item.into());

                            for i in 0..total_item {
                                list.push(pop_raw!(context, i));
                            }
                            
                            VmObject::from(list)
                        },
                         _ => return Err(KaramelErrorType::GeneralError("Geçersiz yükleme tipi".to_string()))
                    };
                    
                    inc_memory_index!(context, 1);
                    context.opcodes_ptr = context.opcodes_ptr.offset(2);
                },

                VmOpCode::Compare => {
                    let condition = pop_raw!(context, "condition");
                    karamel_print_level2!("Compare: {:?}", condition);

                    let status = match &condition.deref_clean() {
                        KaramelPrimative::Empty => false,
                        KaramelPrimative::Bool(l_value) => *l_value,
                        KaramelPrimative::Number(l_value) => *l_value > 0.0,
                        KaramelPrimative::Text(l_value) => !(*l_value).is_empty(),
                        _ => false
                    };

                    if status {
                        context.opcodes_ptr = context.opcodes_ptr.offset(2);
                    }
                    else {
                        let location = ((*context.opcodes_ptr.offset(2) as u16 * 256) + *context.opcodes_ptr.offset(1) as u16) as usize;
                        context.opcodes_ptr = context.opcodes_ptr.offset(location as isize);
                        continue;
                    }
                },

                VmOpCode::Jump => {
                    let location = ((*context.opcodes_ptr.offset(2)  as u16 * 256) + *context.opcodes_ptr.offset(1)  as u16) as usize;
                    karamel_print_level2!("Jump: {:?}", location);
                    context.opcodes_ptr = context.opcodes.as_mut_ptr().offset(location as isize);
                    continue;
                },
                
                VmOpCode::SetItem => {
                    let assign_item  = pop_raw!(context, "assign_item");
                    let indexer = pop!(context, "indexer");
                    let raw_object = pop_raw!(context, "raw_object");
                    let object  = raw_object.deref();
                    karamel_print_level2!("GetItem: object={:?}, indexer={:?}, item={:?}", object, indexer, assign_item);

                    // todo: change all those codes with setter implementation
                    match &*object {
                        KaramelPrimative::List(value) => {
                            let indexer_value = match &*indexer {
                                KaramelPrimative::Number(number) => *number as usize,
                                _ => return Err(KaramelErrorType::IndexerMustBeNumber(indexer.clone()))
                            };

                            value.borrow_mut()[indexer_value] = assign_item;
                        },
                        KaramelPrimative::Dict(value) => {
                            let indexer_value = match &*indexer {
                                KaramelPrimative::Text(text) => &*text,
                                _ => return Err(KaramelErrorType::IndexerMustBeString(indexer.clone()))
                            };

                            value.borrow_mut().insert(indexer_value.to_string(), assign_item);
                        },
                        KaramelPrimative::Text(_) => {
                            let indexer_value = match &*indexer {
                                KaramelPrimative::Number(number) => *number,
                                _ => return Err(KaramelErrorType::IndexerMustBeNumber(indexer.clone()))
                            };

                            match context.get_class(&object).get_setter() {
                                Some(function) => function(raw_object, indexer_value, assign_item)?,
                                _ => EMPTY_OBJECT
                            };
                        },
                        
                        _ => ()
                    };
                },

                VmOpCode::GetItem => {
                    let indexer = pop!(context, "indexer");
                    let raw_object  = pop_raw!(context, "raw_object");
                    let object = &*raw_object.deref();
                    karamel_print_level2!("GetItem: object={:?}, indexer={:?}", object, indexer);

                    *context.stack_ptr = match &*indexer {
                        KaramelPrimative::Text(text) => {
                             match context.get_class(object).get_element(Some(raw_object), text.clone()) {
                                Some(element) => match element {
                                    ClassProperty::Function(function) => VmObject::from(Rc::new(KaramelPrimative::Function(function.clone(), Some(raw_object)))),
                                    ClassProperty::Field(field) => VmObject::from(field.clone())
                                },
                                _ => EMPTY_OBJECT
                            }
                        },
                        KaramelPrimative::Number(index) => match context.get_class(object).get_getter() {
                            Some(function) => function(raw_object, *index)?,
                            _ => EMPTY_OBJECT
                        }
                        _ => EMPTY_OBJECT
                    };

                    inc_memory_index!(context, 1);
                },

                VmOpCode::Halt => {
                    karamel_print_level2!("Halt");
                    break;
                },
            }

            context.opcodes_ptr = context.opcodes_ptr.offset(1);
        }
        
        #[cfg(feature = "dumpMemory")] {
            context.storages[0].dump();
        }
    }
    
    let mut result = Vec::with_capacity(get_memory_index!(context) as usize);
    for index in 0..get_memory_index!(context) {
        result.push(*top_stack.add(context.storages[0].variables.len() + index as usize));
    }

    Ok(result)
}
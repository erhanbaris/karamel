use crate::types::{VmObject};
use crate::compiler::*;
use std::rc::Rc;

#[cfg(feature = "dumpExecutionOpcode")]
fn dump_opcode(opcode: VmOpCode, target: VmObject, left: VmObject, right: VmObject) {
    println!("| # {:20} | {:20} | {:20} | {:20} |", format!("{:?}", opcode), format!("{:?}", target.deref()), format!("{:?}", left.deref()), format!("{:?}", right.deref()));
}

#[cfg(not(feature = "dumpExecutionOpcode"))]
fn dump_opcode(opcode: VmOpCode, target: VmObject, left: VmObject, right: VmObject) { }


pub fn run_vm<S>(options: &mut BramaCompilerOption<S>) where S: Storage
{ 
    {
        let empty_primative: VmObject  = VmObject::convert(Rc::new(BramaPrimative::Empty));
        let memory_ref    = &mut options.storages[0].get_memory();
        let mut memory    = memory_ref.borrow_mut();
        let opcodes       = options.opcodes.iter().map(|byte| byte.decode_as_tuple()).collect::<Vec<_>>();
        let mut tmp_index = (options.storages[0].get_constant_size() + options.storages[0].get_variable_size()) as usize;

        for op in &opcodes {
            let &(opcode, target, left, right) = op;

            match opcode {
                VmOpCode::And => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
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
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
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
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[target] = match (&*memory[left].deref(), &*memory[right].deref()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value)) => VmObject::from(*l_value + *r_value),
                        (BramaPrimative::Text(l_value),    BramaPrimative::Text(r_value))   => VmObject::from(Rc::new((&**l_value).to_owned() + &**r_value)),
                        _ => empty_primative
                    };
                },

                VmOpCode::Multiply => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[target] = match (&*memory[left].deref(), &*memory[right].deref()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value * *r_value),
                        (BramaPrimative::Text(l_value),    BramaPrimative::Number(r_value))   => VmObject::from((*l_value).repeat((*r_value) as usize)),
                        _ => empty_primative
                    };
                },

                VmOpCode::Division => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
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
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[target] = match (&*memory[tmp_index-2].deref(), &*memory[tmp_index-1].deref()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value - *r_value),
                        _ => empty_primative
                    };

                    tmp_index -= 2;
                },

                VmOpCode::Equal => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
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
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
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
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[target] = match (&*memory[left].deref(), &*memory[right].deref()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value > *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::GreaterEqualThan => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[target] = match (&*memory[left].deref(), &*memory[right].deref()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value >= *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::LessThan => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[target] = match (&*memory[left].deref(), &*memory[right].deref()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value < *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::LessEqualThan => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[target] = match (&*memory[left].deref(), &*memory[right].deref()) {
                        (BramaPrimative::Number(l_value),  BramaPrimative::Number(r_value))   => VmObject::from(*l_value <= *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::Move => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[target] = memory[left];
                },

                VmOpCode::Load => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[tmp_index] = memory[target];
                    tmp_index += 1;
                },

                VmOpCode::Store => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[target] = memory[tmp_index + 1];
                },

                VmOpCode::NativeCall => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
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
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[target] = match (&*memory[target].deref(), &*memory[left].deref()) {
                        (BramaPrimative::Number(l_value),     BramaPrimative::Number(r_value))     => VmObject::from(*l_value + *r_value),
                        (BramaPrimative::Text(l_value),       BramaPrimative::Text(r_value))       => VmObject::from(Rc::new((&**l_value).to_owned() + &**r_value)),
                        _ => empty_primative
                    };
                },

                VmOpCode::AssignSubtraction => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[target] = match (&*memory[target].deref(), &*memory[left].deref()) {
                        (BramaPrimative::Number(l_value), BramaPrimative::Number(r_value)) => VmObject::from(*l_value - *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::AssignMultiplication => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[target] = match (&*memory[target].deref(), &*memory[left].deref()) {
                        (BramaPrimative::Number(l_value), BramaPrimative::Number(r_value)) => VmObject::from(*l_value * *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::AssignDivision => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[target] = match (&*memory[target].deref(), &*memory[left].deref()) {
                        (BramaPrimative::Number(l_value), BramaPrimative::Number(r_value)) => VmObject::from(*l_value / *r_value),
                        _ => empty_primative
                    };
                },

                VmOpCode::Increment => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[target] = match &*memory[target].deref() {
                        BramaPrimative::Number(value) => VmObject::from(*value + 1 as f64),
                        _ => empty_primative
                    };
                },

                VmOpCode::Decrement => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[target] = match &*memory[target].deref() {
                        BramaPrimative::Number(value) => VmObject::from(*value - 1 as f64),
                        _ => empty_primative
                    };
                },

                VmOpCode::Not => {
                    dump_opcode(opcode, memory[target], memory[left], memory[right]);
                    memory[target] = match &*memory[left].deref() {
                        BramaPrimative::Number(value) => VmObject::from(!(*value > 0.0)),
                        BramaPrimative::Bool(value)   => VmObject::from(!*value),
                        _ => empty_primative
                    };
                },

                VmOpCode::None => ()
            }
        }
    }

    options.storages[0].dump();

    println!("            OPCODE");
    println!("-------------------------------");
    for (index, item) in options.opcodes.iter().enumerate() {
        println!("| {:?} | {:?}", index, item);
    }
}
use std::vec::Vec;
use std::rc::Rc;
use std::mem::ManuallyDrop;
use std::fmt;
use std::collections::HashMap;

use crate::{inc_memory_index, dec_memory_index, get_memory_index};
use crate::types::*;
use crate::compiler::{Storage, BramaCompiler, Scope};

#[derive(Clone)]
pub struct FunctionReference {
    pub callback: FunctionType,
    pub framework: String,
    pub module_path: Vec<String>,
    pub name: String
}

#[derive(Clone)]
pub enum FunctionType {
    Native(NativeCall),
    Opcode(u16)
}

impl FunctionReference {
    pub fn execute(&self, compiler: &mut BramaCompiler) -> NativeCallResult{
        unsafe {
            match self.callback {
                FunctionType::Native(func) => FunctionReference::native_function_call(func, compiler),
                FunctionType::Opcode(location) => FunctionReference::opcode_function_call(location, compiler)
            }
        }
    }

    pub fn native_function(func: NativeCall, name: String, module_path: Vec<String>, framework: String) -> Rc<BramaPrimative> {
        let reference = FunctionReference {
            callback: FunctionType::Native(func),
            framework: framework,
            module_path: module_path,
            name: name
        };
        return Rc::new(BramaPrimative::Function(reference));
    }

    pub fn opcode_function(location: u16, name: String, module_path: Vec<String>, framework: String) -> Rc<BramaPrimative> {
        let reference = FunctionReference {
            callback: FunctionType::Opcode(location),
            framework: framework,
            module_path: module_path,
            name: name
        };
        return Rc::new(BramaPrimative::Function(reference));
    }

    unsafe fn native_function_call(func: NativeCall, compiler: &mut BramaCompiler) -> NativeCallResult {            
        let total_args = compiler.opcodes[compiler.opcode_index + 1];
        let call_return_assign_to_temp = compiler.opcodes[compiler.opcode_index + 2] != 0;
        
        match func(&(*compiler.current_scope).stack, get_memory_index!(compiler), total_args) {
            Ok(result) => {
                dec_memory_index!(compiler, total_args as usize);

                if call_return_assign_to_temp {
                    (*compiler.current_scope).stack[get_memory_index!(compiler)] = result;
                    inc_memory_index!(compiler, 1);
                }

                compiler.opcode_index += 2;
                return Ok(result);
            },
            Err((error, l, c)) => {
                println!("{:?}", error);
                return Err((error, l, c));
            }
        };
    }

    fn opcode_function_call(location: u16, options: &mut BramaCompiler) -> NativeCallResult {
        let argument_size  = options.opcodes[options.opcode_index + 1];
        let call_return_assign_to_temp = options.opcodes[options.opcode_index + 2] != 0;
        let old_index = options.opcode_index + 2;
        options.opcode_index = location as usize;
        options.scope_index += 1;
        let storage_location = ((options.opcodes[options.opcode_index + 1] as u16 * 256) + options.opcodes[options.opcode_index] as u16) as usize;

        if argument_size != options.opcodes[options.opcode_index + 1] {
            return Err(("Function argument error".to_string(), 0, 0));
        }
        let mut args: Vec<VmObject> = Vec::with_capacity(argument_size as usize);

        if argument_size > 0 {
            for _ in 0..argument_size {
                unsafe {
                    dec_memory_index!(options, 1);
                    args.push((*options.current_scope).stack[get_memory_index!(options)]);
                }
            }
        }

        if options.scopes.len() <= options.scope_index {
            options.scopes.resize(options.scopes.len() * 2, Scope::empty());
        }

        unsafe {
            options.scopes[options.scope_index] = Scope {
                memory: options.storages[storage_location].get_memory().borrow().to_vec(),
                stack: options.storages[storage_location].get_stack().borrow().to_vec(),
                location: old_index,
                memory_index: get_memory_index!(options),
                const_size: options.storages[storage_location].get_constant_size(),
                call_return_assign_to_temp: call_return_assign_to_temp
            };

            options.current_scope = &mut options.scopes[options.scope_index] as *mut Scope;
            (*options.current_scope).memory_index = 0;
        }

        if argument_size > 0 {
            for _ in 0..argument_size {
                unsafe {
                    (*options.current_scope).stack[get_memory_index!(options)] = args[get_memory_index!(options)];
                    inc_memory_index!(options, 1);
                }
            }
        }

        options.opcode_index += 2;
        return Ok(VmObject::convert(Rc::new(BramaPrimative::Empty)));
    }
}

pub type NativeCallResult = Result<VmObject, (String, u32, u32)>;
pub type NativeCall       = fn(stack: &Vec<VmObject>, last_position: usize, arg_size: u8) -> NativeCallResult;

pub const EMPTY_OBJECT: VmObject = VmObject(QNAN | EMPTY_FLAG);
pub const TRUE_OBJECT: VmObject  = VmObject(QNAN | TRUE_FLAG);
pub const FALSE_OBJECT: VmObject = VmObject(QNAN | FALSE_FLAG);

#[repr(C)]
#[derive(Clone)]
pub enum BramaPrimative {
    Empty,
    Number(f64),
    Bool(bool),
    List(Vec<Rc<BramaPrimative>>),
    Dict(HashMap<String, Rc<BramaPrimative>>),
    Atom(u64),
    Text(Rc<String>),
    Function(FunctionReference)
}

impl BramaPrimative {
    pub fn is_true(&self) -> bool {
        match self {
            BramaPrimative::Text(value)       => value.len() > 0,
            BramaPrimative::Number(value)     => *value > 0.0,
            BramaPrimative::Bool(value)       => *value,
            BramaPrimative::Atom(_)           => true,
            BramaPrimative::List(items)       => items.len() > 0,
            BramaPrimative::Dict(items) => items.len() > 0,
            BramaPrimative::Empty             => false,
            BramaPrimative::Function(_) => true
        }
    }

    pub fn get_text(&self) -> String {
        match self {
            BramaPrimative::Text(value) => value.to_string(),
            _ => "".to_string()
        }
    }
}

impl From<f64> for VmObject {
    fn from(source: f64) -> Self {
        VmObject::convert(Rc::new(BramaPrimative::Number(source)))
    }
}

impl From<bool> for VmObject {
    fn from(source: bool) -> Self {
        VmObject::convert(Rc::new(BramaPrimative::Bool(source)))
    }
}

impl From<Rc<String>> for VmObject {
    fn from(source: Rc<String>) -> Self {
        VmObject::convert(Rc::new(BramaPrimative::Text(source)))
    }
}

impl From<String> for VmObject {
    fn from(source: String) -> Self {
        VmObject::convert(Rc::new(BramaPrimative::Text(Rc::new(source))))
    }
}

impl From<Vec<Rc<BramaPrimative>>> for VmObject {
    fn from(source: Vec<Rc<BramaPrimative>>) -> Self {
        VmObject::convert(Rc::new(BramaPrimative::List(source)))
    }
}

impl From<Rc<BramaPrimative>> for VmObject {
    fn from(source: Rc<BramaPrimative>) -> Self {
        VmObject::convert(source)
    }
}

impl From<HashMap<String, Rc<BramaPrimative>>> for VmObject {
    fn from(source: HashMap<String, Rc<BramaPrimative>>) -> Self {
        VmObject::convert(Rc::new(BramaPrimative::Dict(source)))
    }
}

impl fmt::Debug for BramaPrimative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BramaPrimative::Empty => write!(f, "Empty"),
            BramaPrimative::Number(number) => write!(f, "{:?}", number),
            BramaPrimative::Bool(b) => write!(f, "{:?}", b),
            BramaPrimative::List(b) => write!(f, "{:?}", b),
            BramaPrimative::Dict(b) => write!(f, "{:?}", b),
            BramaPrimative::Atom(b) => write!(f, "{:?}", b),
            BramaPrimative::Text(b) => write!(f, "{:?}", b),
            BramaPrimative::Function(func) => write!(f, "<function='{}'>", func.name)
        }
    }
}

impl Drop for BramaPrimative {
    fn drop(&mut self) {
        //println!("> {:?}", self);
    }
}

impl PartialEq for BramaPrimative {
    fn eq(&self, other: &Self) -> bool {
        match (self, &other) {
            (BramaPrimative::Bool(lvalue),            BramaPrimative::Bool(rvalue)) => lvalue == rvalue,
            (BramaPrimative::Atom(lvalue),            BramaPrimative::Atom(rvalue)) => lvalue == rvalue,
            (BramaPrimative::Empty,                   BramaPrimative::Empty)        => true,
            (BramaPrimative::Number(n),               BramaPrimative::Number(m))    => if n.is_nan() && m.is_nan() { true } else { n == m },
            (BramaPrimative::Text(lvalue),            BramaPrimative::Text(rvalue)) => lvalue == rvalue,
            (BramaPrimative::List(l_value),           BramaPrimative::List(r_value))       => {
                if (*l_value).len() != (*r_value).len() {
                    return false;
                }

                for i in 0..(*l_value).len() {
                    if (*l_value)[i].clone() != (*r_value)[i].clone() {
                        return false;
                    }
                }
                return true;
            },
            (BramaPrimative::Function(l_value), BramaPrimative::Function(r_value)) => {
                if l_value.name != r_value.name ||
                   l_value.framework != r_value.framework ||
                   l_value.module_path != r_value.module_path {
                    return false;
                }
                return true;
            },
            (BramaPrimative::Dict(l_value),           BramaPrimative::Dict(r_value))       => {
                if (*l_value).len() != (*r_value).len() {
                    return false;
                }

                for (key, l_item) in l_value {
                    match r_value.get(key) {
                        Some(r_item) => {
                            if l_item != r_item {
                                return false;
                            }
                        },
                        None => return false
                    }
                }
                return true;
            },
            _ => false
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl VmObject {
    pub fn convert(primative: Rc<BramaPrimative>) -> VmObject {
        match *primative {
            BramaPrimative::Empty            => VmObject(QNAN | EMPTY_FLAG),
            BramaPrimative::Number(number)   => VmObject(number.to_bits()),
            BramaPrimative::Bool(true)       => TRUE_OBJECT,
            BramaPrimative::Bool(false)      => FALSE_OBJECT,
            _                                => {
                VmObject(QNAN | POINTER_FLAG | (POINTER_MASK & (Rc::into_raw(primative)) as u64))
            }
        }
    }

    pub fn native_convert(primative: BramaPrimative) -> VmObject {
        match primative {
            BramaPrimative::Empty            => VmObject(QNAN | EMPTY_FLAG),
            BramaPrimative::Number(number)   => VmObject(number.to_bits()),
            BramaPrimative::Bool(true)       => TRUE_OBJECT,
            BramaPrimative::Bool(false)      => FALSE_OBJECT,
            _                                => {
                VmObject(QNAN | POINTER_FLAG | (POINTER_MASK & (Rc::into_raw(Rc::new(primative))) as u64))
            }
        }
    }

    pub fn deref(&self) -> Rc<BramaPrimative> {
        match self.0 {
            n if (n & QNAN) != QNAN       => Rc::new(BramaPrimative::Number(f64::from_bits(n))),
            e if e == (QNAN | EMPTY_FLAG) => Rc::new(BramaPrimative::Empty),
            f if f == (QNAN | FALSE_FLAG) => Rc::new(BramaPrimative::Bool(false)),
            t if t == (QNAN | TRUE_FLAG)  => Rc::new(BramaPrimative::Bool(true)),
            p if (p & POINTER_FLAG) == POINTER_FLAG => {
                let pointer = (self.0 & POINTER_MASK) as *mut BramaPrimative;
                let data = unsafe { ManuallyDrop::new(Rc::from_raw(pointer)) };
                Rc::clone(&data)
            },
            _ => Rc::new(BramaPrimative::Empty)
        }
    }
}
use std::vec::Vec;
use std::rc::Rc;
use std::mem::ManuallyDrop;
use std::fmt;
use std::collections::HashMap;

use crate::types::*;

pub type NativeCallResult = Result<VmObject, (&'static str, u32, u32)>;
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
    FuncNativeCall(NativeCall)
}

impl BramaPrimative {
    pub fn is_true(&self) -> bool {
        match self {
            BramaPrimative::Text(value)       => value.len() > 0,
            BramaPrimative::Number(value)     => *value > 0.0,
            BramaPrimative::Bool(value)       => *value,
            BramaPrimative::Atom(_)           => true,
            BramaPrimative::List(items)       => items.len() > 0,
            BramaPrimative::FuncNativeCall(_) => true,
            BramaPrimative::Dict(items) => items.len() > 0,
            BramaPrimative::Empty             => false
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
            BramaPrimative::FuncNativeCall(func) => write!(f, "func ({:p})", &func),
            BramaPrimative::Bool(b) => write!(f, "{:?}", b),
            BramaPrimative::List(b) => write!(f, "{:?}", b),
            BramaPrimative::Dict(b) => write!(f, "{:?}", b),
            BramaPrimative::Atom(b) => write!(f, "{:?}", b),
            BramaPrimative::Text(b) => write!(f, "{:?}", b)
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
            (BramaPrimative::FuncNativeCall(lvalue),  BramaPrimative::FuncNativeCall(rvalue)) => *lvalue as usize == *rvalue as usize,
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
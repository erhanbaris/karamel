use std::vec::Vec;
use std::rc::Rc;
use std::cell::RefCell;
use std::mem::ManuallyDrop;
use std::fmt;
use std::collections::HashMap;

use crate::types::*;
use crate::compiler::function::FunctionReference;
use crate::compiler::GetType;

pub const EMPTY_OBJECT: VmObject = VmObject(QNAN | EMPTY_FLAG);
pub const TRUE_OBJECT: VmObject  = VmObject(QNAN | TRUE_FLAG);
pub const FALSE_OBJECT: VmObject = VmObject(QNAN | FALSE_FLAG);

#[repr(C)]
#[derive(Clone)]
pub enum BramaPrimative {
    Empty,
    Number(f64),
    Bool(bool),
    List(RefCell<Vec<Rc<BramaPrimative>>>),
    Dict(RefCell<HashMap<String, Rc<BramaPrimative>>>),
    Atom(u64),
    Text(Rc<String>),
    Function(Rc<FunctionReference>)
}

impl BramaPrimative {
    pub fn is_true(&self) -> bool {
        match self {
            BramaPrimative::Text(value)       => !value.is_empty(),
            BramaPrimative::Number(value)     => *value > 0.0,
            BramaPrimative::Bool(value)       => *value,
            BramaPrimative::Atom(_)           => true,
            BramaPrimative::List(items)       => !items.borrow().is_empty(),
            BramaPrimative::Dict(items) => !items.borrow().is_empty(),
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

impl GetType for BramaPrimative {
    fn get_type(&self) -> String {
        match self {
            BramaPrimative::Text(_)     => "yazı".to_string(),
            BramaPrimative::Number(_)   => "sayı".to_string(),
            BramaPrimative::Bool(_)     => "bool".to_string(),
            BramaPrimative::Atom(_)     => "atom".to_string(),
            BramaPrimative::List(_)     => "liste".to_string(),
            BramaPrimative::Dict(_)     => "sözlük".to_string(),
            BramaPrimative::Empty       => "boş".to_string(),
            BramaPrimative::Function(_) => "fonksiyon".to_string()
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
        VmObject::convert(Rc::new(BramaPrimative::List(RefCell::new(source))))
    }
}

impl From<Rc<BramaPrimative>> for VmObject {
    fn from(source: Rc<BramaPrimative>) -> Self {
        VmObject::convert(source)
    }
}

impl From<HashMap<String, Rc<BramaPrimative>>> for VmObject {
    fn from(source: HashMap<String, Rc<BramaPrimative>>) -> Self {
        VmObject::convert(Rc::new(BramaPrimative::Dict(RefCell::new(source))))
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
                if (*l_value).borrow().len() != (*r_value).borrow().len() {
                    return false;
                }

                for i in 0..(*l_value).borrow().len() {
                    if (*l_value).borrow()[i].clone() != (*r_value).borrow()[i].clone() {
                        return false;
                    }
                }
                true
            },
            (BramaPrimative::Function(l_value), BramaPrimative::Function(r_value)) => {
                if l_value.name != r_value.name ||
                   l_value.framework != r_value.framework ||
                   l_value.module_path != r_value.module_path {
                    return false;
                }
                true
            },
            (BramaPrimative::Dict(l_value),           BramaPrimative::Dict(r_value))       => {
                if (*l_value).borrow().len() != (*r_value).borrow().len() {
                    return false;
                }

                for (key, l_item) in l_value.borrow().iter() {
                    match r_value.borrow().get(key) {
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
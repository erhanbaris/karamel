use std::vec::Vec;
use std::rc::Rc;
use std::mem::ManuallyDrop;
use std::fmt;

use crate::types::*;
use crate::compiler::ast::BramaAstType;

pub type NativeCallResult = Result<VmObject, (&'static str, u32, u32)>;
pub type NativeCall       = fn(items: &Vec<VmObject>) -> NativeCallResult;

pub const EMPTY_OBJECT: VmObject = VmObject(QNAN | EMPTY_FLAG);
pub const TRUE_OBJECT: VmObject  = VmObject(QNAN | TRUE_FLAG);
pub const FALSE_OBJECT: VmObject = VmObject(QNAN | FALSE_FLAG);

#[repr(C)]
#[derive(Clone)]
pub enum BramaPrimative {
    Empty,
    Number(f64),
    Bool(bool),
    List(Vec<Box<BramaAstType>>),
    Atom(u64),
    Text(Rc<String>),
    FuncNativeCall(NativeCall)
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

impl fmt::Debug for BramaPrimative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BramaPrimative::Empty => write!(f, "Empty"),
            BramaPrimative::Number(number) => write!(f, "{:?}", number),
            BramaPrimative::FuncNativeCall(func) => write!(f, "func ({:p})", &func),
            BramaPrimative::Bool(b) => write!(f, "{:?}", b),
            BramaPrimative::List(b) => write!(f, "{:?}", b),
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
            (BramaPrimative::Bool(lvalue),  BramaPrimative::Bool(rvalue)) => lvalue == rvalue,
            (BramaPrimative::Atom(lvalue),  BramaPrimative::Atom(rvalue)) => lvalue == rvalue,
            (BramaPrimative::List(lvalue),  BramaPrimative::List(rvalue)) => lvalue == rvalue,
            (BramaPrimative::Empty,         BramaPrimative::Empty)        => true,
            (BramaPrimative::Number(n),     BramaPrimative::Number(m))    => if n.is_nan() && m.is_nan() { true } else { n == m },
            (BramaPrimative::Text(lvalue),  BramaPrimative::Text(rvalue)) => lvalue == rvalue,
            (BramaPrimative::FuncNativeCall(lvalue),  BramaPrimative::FuncNativeCall(rvalue)) => *lvalue as usize == *rvalue as usize,
            _ => false
        }
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
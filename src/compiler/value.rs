use std::vec::Vec;
use std::rc::Rc;
use std::mem::ManuallyDrop;

use crate::types::*;
use crate::compiler::ast::BramaAstType;

#[repr(C)]
#[derive(Clone, Debug)]
pub enum BramaPrimative {
    Empty,
    Number(f64),
    Bool(bool),
    List(Vec<Box<BramaAstType>>),
    Atom(u64),
    Text(Rc<String>)
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
            _ => false
        }
    }
}

impl BramaPrimative {
    pub fn to_object(&self) -> VmObject {
        match self {
            BramaPrimative::Empty            => VmObject(QNAN | EMPTY_FLAG),
            BramaPrimative::Number(number)   => VmObject(number.to_bits()),
            BramaPrimative::Bool(boolean)    => VmObject(QNAN | if *boolean { TRUE_FLAG } else { FALSE_FLAG }),
            _                                => {
                VmObject(QNAN | POINTER_FLAG | (POINTER_MASK & (Rc::into_raw(Rc::new(self))) as u64))
            }
        }
    } 
}

impl VmObject {
    pub fn convert(primative: Rc<BramaPrimative>) -> VmObject {
        match *primative {
            BramaPrimative::Empty            => VmObject(QNAN | EMPTY_FLAG),
            BramaPrimative::Number(number)   => VmObject(number.to_bits()),
            BramaPrimative::Bool(boolean)    => VmObject(QNAN | if boolean { TRUE_FLAG } else { FALSE_FLAG }),
            _                                => {
                VmObject(QNAN | POINTER_FLAG | (POINTER_MASK & (Rc::into_raw(primative)) as u64))
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
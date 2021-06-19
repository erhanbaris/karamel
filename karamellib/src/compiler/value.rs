use std::vec::Vec;
use std::rc::Rc;
use std::cell::RefCell;
use std::mem::ManuallyDrop;
use std::fmt;
use std::collections::HashMap;


use crate::{buildin::Class, types::*};
use crate::compiler::function::FunctionReference;
use crate::compiler::GetType;

pub const EMPTY_OBJECT: VmObject = VmObject(QNAN | EMPTY_FLAG);
pub const TRUE_OBJECT: VmObject  = VmObject(QNAN | TRUE_FLAG);
pub const FALSE_OBJECT: VmObject = VmObject(QNAN | FALSE_FLAG);
pub static EMPTY_PRIMATIVE: BramaPrimative = BramaPrimative::Empty;

#[repr(C)]
#[derive(Clone)]
pub enum BramaPrimative {
    Empty,
    Number(f64),
    Bool(bool),
    List(RefCell<Vec<VmObject>>),
    Dict(RefCell<HashMap<String, VmObject>>),
    Text(Rc<String>),
    Function(Rc<FunctionReference>, Option<VmObject>),
    Class(Rc<dyn Class>)
}

unsafe impl Send for BramaPrimative {}
unsafe impl Sync for BramaPrimative {}

unsafe impl Send for VmObject {}
unsafe impl Sync for VmObject {}

impl BramaPrimative {

    pub fn format(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BramaPrimative::Empty => write!(f, "boş"),
            BramaPrimative::Number(number) => write!(f, "{:?}", number),
            BramaPrimative::Bool(b) => match b {
                true => write!(f, "doğru"),
                false => write!(f, "yanlış")
            },
            BramaPrimative::List(b) => write!(f, "{:?}", b.borrow()),
            BramaPrimative::Dict(b) => write!(f, "{:?}", b.borrow()),
            BramaPrimative::Text(b) => write!(f, "{:?}", b),
            BramaPrimative::Function(func, _) => write!(f, "<Fonksiyon='{}'>", func.name),
            BramaPrimative::Class(class) => write!(f, "<Sınıf='{}'>", class.get_type())
        }
    }

    pub fn is_true(&self) -> bool {
        match self {
            BramaPrimative::Text(value)       => !value.is_empty(),
            BramaPrimative::Number(value)     => *value > 0.0,
            BramaPrimative::Bool(value)       => *value,
            BramaPrimative::List(items)       => !items.borrow().is_empty(),
            BramaPrimative::Dict(items) => !items.borrow().is_empty(),
            BramaPrimative::Empty             => false,
            BramaPrimative::Function(_, _) => true,
            BramaPrimative::Class(_) => true
        }
    }

    pub fn get_text(&self) -> String {
        match self {
            BramaPrimative::Text(value) => value.to_string(),
            _ => "".to_string()
        }
    }

    pub fn discriminant(&self) -> usize {
        match self {
            BramaPrimative::Number(_) => 0,
            BramaPrimative::Text(_) => 1,
            BramaPrimative::List(_) => 2,
            BramaPrimative::Dict(_) => 3,
            
            BramaPrimative::Empty => 4,
            BramaPrimative::Bool(_) => 5,
            BramaPrimative::Function(_, _) => 6,
            BramaPrimative::Class(_) => 7
        }
    }
}

impl GetType for BramaPrimative {
    fn get_type(&self) -> String {
        match self {
            BramaPrimative::Text(_)     => "yazı".to_string(),
            BramaPrimative::Number(_)   => "sayı".to_string(),
            BramaPrimative::Bool(_)     => "bool".to_string(),
            BramaPrimative::List(_)     => "liste".to_string(),
            BramaPrimative::Dict(_)     => "sözlük".to_string(),
            BramaPrimative::Empty       => "boş".to_string(),
            BramaPrimative::Function(_, _) => "fonksiyon".to_string(),
            BramaPrimative::Class(_)    => "Sınıf".to_string()
        }
    }
}

impl From<f64> for VmObject {
    fn from(number: f64) -> Self {
        VmObject(number.to_bits())
    }
}

impl From<i64> for VmObject {
    fn from(number: i64) -> Self {
        VmObject((number as f64).to_bits())
    }
}

impl From<usize> for VmObject {
    fn from(number: usize) -> Self {
        VmObject((number as f64).to_bits())
    }
}

impl From<bool> for VmObject {
    fn from(source: bool) -> Self {
        match source {
            true => TRUE_OBJECT,
            false => FALSE_OBJECT,
        }
    }
}

impl From<Rc<String>> for VmObject {
    fn from(source: Rc<String>) -> Self {
        VmObject::native_convert(BramaPrimative::Text(source))
    }
}

impl From<String> for VmObject {
    fn from(source: String) -> Self {
        VmObject::native_convert(BramaPrimative::Text(Rc::new(source)))
    }
}

impl From<Vec<VmObject>> for VmObject {
    fn from(source: Vec<VmObject>) -> Self {
        VmObject::native_convert(BramaPrimative::List(RefCell::new(source)))
    }
}

impl From<Rc<BramaPrimative>> for VmObject {
    fn from(source: Rc<BramaPrimative>) -> Self {
        VmObject::convert(source)
    }
}

impl From<HashMap<String, VmObject>> for VmObject {
    fn from(source: HashMap<String, VmObject>) -> Self {
        VmObject::convert(Rc::new(BramaPrimative::Dict(RefCell::new(source))))
    }
}

impl fmt::Debug for BramaPrimative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Display for BramaPrimative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for VmObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &*self.deref())
    }
}

impl fmt::Display for VmObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &*self.deref())
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
            (BramaPrimative::Empty,                   BramaPrimative::Empty)        => true,
            (BramaPrimative::Number(n),               BramaPrimative::Number(m))    => if n.is_nan() && m.is_nan() { true } else { n == m },
            (BramaPrimative::Text(lvalue),            BramaPrimative::Text(rvalue)) => lvalue == rvalue,
            (BramaPrimative::List(l_value),           BramaPrimative::List(r_value))       => {
                if (*l_value).borrow().len() != (*r_value).borrow().len() {
                    return false;
                }

                for i in 0..(*l_value).borrow().len() {
                    if (*l_value).borrow()[i].deref() != (*r_value).borrow()[i].deref() {
                        return false;
                    }
                }
                true
            },
            (BramaPrimative::Function(l_value, _), BramaPrimative::Function(r_value, _)) => {
                if l_value.name != r_value.name ||
                   l_value.framework != r_value.framework ||
                   l_value.module_path != r_value.module_path {
                    return false;
                }
                true
            },
            (BramaPrimative::Class(l_value), BramaPrimative::Class(r_value)) => {
                l_value.get_type() == r_value.get_type()
            },
            (BramaPrimative::Dict(l_value),           BramaPrimative::Dict(r_value))       => {
                if (*l_value).borrow().len() != (*r_value).borrow().len() {
                    return false;
                }

                for (key, l_item) in l_value.borrow().iter() {
                    match r_value.borrow().get(key) {
                        Some(r_item) => {
                            if l_item.deref() != r_item.deref() {
                                return false;
                            }
                        },
                        None => return false
                    }
                }
                true
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

    pub fn native_convert_by_ref(primative: Rc<BramaPrimative>) -> VmObject {
        match &*primative {
            BramaPrimative::Empty            => VmObject(QNAN | EMPTY_FLAG),
            BramaPrimative::Number(number)   => VmObject(number.to_bits()),
            BramaPrimative::Bool(true)       => TRUE_OBJECT,
            BramaPrimative::Bool(false)      => FALSE_OBJECT,
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

    pub fn deref_clean(&self) -> BramaPrimative {
        match self.0 {
            n if (n & QNAN) != QNAN       => BramaPrimative::Number(f64::from_bits(n)),
            e if e == (QNAN | EMPTY_FLAG) => BramaPrimative::Empty,
            f if f == (QNAN | FALSE_FLAG) => BramaPrimative::Bool(false),
            t if t == (QNAN | TRUE_FLAG)  => BramaPrimative::Bool(true),
            p if (p & POINTER_FLAG) == POINTER_FLAG => {
                let pointer = (self.0 & POINTER_MASK) as *mut BramaPrimative;
                let data = unsafe { ManuallyDrop::new(Rc::from_raw(pointer)) };
                match &**data {
                    BramaPrimative::Text(text) => BramaPrimative::Text(text.clone()),
                    BramaPrimative::List(list) => BramaPrimative::List(list.clone()),
                    BramaPrimative::Dict(dict) => BramaPrimative::Dict(dict.clone()),
                    BramaPrimative::Function(func, base) => BramaPrimative::Function(func.clone(), *base),
                    BramaPrimative::Class(klass) => BramaPrimative::Class(klass.clone()),
                    _ => BramaPrimative::Empty
                }
            },
            _ => BramaPrimative::Empty
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match (self.0 & QNAN) != QNAN {
            true => Some(f64::from_bits(self.0)),
            false => None
        }
    }
}
use std::borrow::Borrow;
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
pub static EMPTY_PRIMATIVE: KaramelPrimative = KaramelPrimative::Empty;

#[repr(C)]
#[derive(Clone)]
pub enum KaramelPrimative {
    Empty,
    Number(f64),
    Bool(bool),
    List(RefCell<Vec<VmObject>>),
    Dict(RefCell<HashMap<String, VmObject>>),
    Text(Rc<String>),
    Function(Rc<FunctionReference>, Option<VmObject>),
    Class(Rc<dyn Class>)
}

unsafe impl Send for KaramelPrimative {}
unsafe impl Sync for KaramelPrimative {}

unsafe impl Send for VmObject {}
unsafe impl Sync for VmObject {}

impl Default for KaramelPrimative {
    fn default() -> Self { KaramelPrimative::Empty }
}

impl KaramelPrimative {

    pub fn format(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KaramelPrimative::Empty => write!(f, "boş"),
            KaramelPrimative::Number(number) => {
                if *number == (*number as u64) as f64 {
                    write!(f, "{:?}", (*number as u64))
                } else {
                    write!(f, "{:?}", number)
                }
            },
            KaramelPrimative::Bool(b) => match b {
                true => write!(f, "doğru"),
                false => write!(f, "yanlış")
            },
            KaramelPrimative::List(b) => write!(f, "{:?}", b.borrow()),
            KaramelPrimative::Dict(b) => write!(f, "{:?}", b.borrow()),
            KaramelPrimative::Text(b) => write!(f, "\"{}\"", b),
            KaramelPrimative::Function(func, _) => write!(f, "<Fonksiyon='{}'>", func.name),
            KaramelPrimative::Class(class) => write!(f, "<Sınıf='{}'>", class.get_type())
        }
    }

    pub fn is_true(&self) -> bool {
        match self {
            KaramelPrimative::Text(value)       => !value.is_empty(),
            KaramelPrimative::Number(value)     => *value > 0.0,
            KaramelPrimative::Bool(value)       => *value,
            KaramelPrimative::List(items)       => !items.borrow().is_empty(),
            KaramelPrimative::Dict(items) => !items.borrow().is_empty(),
            KaramelPrimative::Empty             => false,
            KaramelPrimative::Function(_, _) => true,
            KaramelPrimative::Class(_) => true
        }
    }

    pub fn get_text(&self) -> String {
        match self {
            KaramelPrimative::Text(value) => value.to_string(),
            _ => "".to_string()
        }
    }

    pub fn discriminant(&self) -> usize {
        match self {
            KaramelPrimative::Number(_) => 0,
            KaramelPrimative::Text(_) => 1,
            KaramelPrimative::List(_) => 2,
            KaramelPrimative::Dict(_) => 3,
            
            KaramelPrimative::Empty => 4,
            KaramelPrimative::Bool(_) => 5,
            KaramelPrimative::Function(_, _) => 6,
            KaramelPrimative::Class(_) => 7
        }
    }
}

impl GetType for KaramelPrimative {
    fn get_type(&self) -> String {
        match self {
            KaramelPrimative::Text(_)     => "yazı".to_string(),
            KaramelPrimative::Number(_)   => "sayı".to_string(),
            KaramelPrimative::Bool(_)     => "bool".to_string(),
            KaramelPrimative::List(_)     => "liste".to_string(),
            KaramelPrimative::Dict(_)     => "sözlük".to_string(),
            KaramelPrimative::Empty       => "boş".to_string(),
            KaramelPrimative::Function(_, _) => "fonksiyon".to_string(),
            KaramelPrimative::Class(_)    => "sınıf".to_string()
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
        VmObject::native_convert(KaramelPrimative::Text(source))
    }
}

impl From<String> for VmObject {
    fn from(source: String) -> Self {
        VmObject::native_convert(KaramelPrimative::Text(Rc::new(source)))
    }
}

impl From<Vec<VmObject>> for VmObject {
    fn from(source: Vec<VmObject>) -> Self {
        VmObject::native_convert(KaramelPrimative::List(RefCell::new(source)))
    }
}

impl From<Rc<KaramelPrimative>> for VmObject {
    fn from(source: Rc<KaramelPrimative>) -> Self {
        VmObject::convert(source)
    }
}

impl From<HashMap<String, VmObject>> for VmObject {
    fn from(source: HashMap<String, VmObject>) -> Self {
        VmObject::convert(Rc::new(KaramelPrimative::Dict(RefCell::new(source))))
    }
}

impl fmt::Debug for KaramelPrimative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Display for KaramelPrimative {
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

impl Drop for KaramelPrimative {
    fn drop(&mut self) {
        //println!("> {:?}", self);
    }
}

impl PartialEq for KaramelPrimative {
    fn eq(&self, other: &Self) -> bool {
        match (self, &other) {
            (KaramelPrimative::Bool(lvalue),            KaramelPrimative::Bool(rvalue)) => lvalue == rvalue,
            (KaramelPrimative::Empty,                   KaramelPrimative::Empty)        => true,
            (KaramelPrimative::Number(n),               KaramelPrimative::Number(m))    => if n.is_nan() && m.is_nan() { true } else { n == m },
            (KaramelPrimative::Text(lvalue),            KaramelPrimative::Text(rvalue)) => lvalue == rvalue,
            (KaramelPrimative::List(l_value),           KaramelPrimative::List(r_value))       => {
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
            (KaramelPrimative::Function(l_value, _), KaramelPrimative::Function(r_value, _)) => {
                if l_value.name != r_value.name ||
                   l_value.module.get_path() != r_value.module.get_path() {
                    return false;
                }
                true
            },
            (KaramelPrimative::Class(l_value), KaramelPrimative::Class(r_value)) => {
                l_value.get_type() == r_value.get_type()
            },
            (KaramelPrimative::Dict(l_value),           KaramelPrimative::Dict(r_value))       => {
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

    #[inline]
    pub fn convert(primative: Rc<KaramelPrimative>) -> VmObject {
        match *primative {
            KaramelPrimative::Empty            => VmObject(QNAN | EMPTY_FLAG),
            KaramelPrimative::Number(number)   => VmObject(number.to_bits()),
            KaramelPrimative::Bool(true)       => TRUE_OBJECT,
            KaramelPrimative::Bool(false)      => FALSE_OBJECT,
            _                                => {
                VmObject(QNAN | POINTER_FLAG | (POINTER_MASK & (Rc::into_raw(primative)) as u64))
            }
        }
    }

    #[inline]
    pub fn native_convert<T: Borrow<KaramelPrimative>>(primative: T) -> VmObject {
        match primative.borrow() {
            KaramelPrimative::Empty            => VmObject(QNAN | EMPTY_FLAG),
            KaramelPrimative::Number(number)   => VmObject(number.to_bits()),
            KaramelPrimative::Bool(true)       => TRUE_OBJECT,
            KaramelPrimative::Bool(false)      => FALSE_OBJECT,
            _                                => {
                VmObject(QNAN | POINTER_FLAG | (POINTER_MASK & (Rc::into_raw(Rc::new(primative))) as u64))
            }
        }
    }

    #[inline]
    pub fn native_convert_by_ref(primative: Rc<KaramelPrimative>) -> VmObject {
        match &*primative {
            KaramelPrimative::Empty            => VmObject(QNAN | EMPTY_FLAG),
            KaramelPrimative::Number(number)   => VmObject(number.to_bits()),
            KaramelPrimative::Bool(true)       => TRUE_OBJECT,
            KaramelPrimative::Bool(false)      => FALSE_OBJECT,
            _                                => {
                VmObject(QNAN | POINTER_FLAG | (POINTER_MASK & (Rc::into_raw(primative)) as u64))
            }
        }
    }

    #[inline]
    pub fn deref(&self) -> Rc<KaramelPrimative> {
        match self.0 {
            n if (n & QNAN) != QNAN       => Rc::new(KaramelPrimative::Number(f64::from_bits(n))),
            e if e == (QNAN | EMPTY_FLAG) => Rc::new(KaramelPrimative::Empty),
            f if f == (QNAN | FALSE_FLAG) => Rc::new(KaramelPrimative::Bool(false)),
            t if t == (QNAN | TRUE_FLAG)  => Rc::new(KaramelPrimative::Bool(true)),
            p if (p & POINTER_FLAG) == POINTER_FLAG => {
                let pointer = (self.0 & POINTER_MASK) as *mut KaramelPrimative;
                let data = unsafe { ManuallyDrop::new(Rc::from_raw(pointer)) };
                Rc::clone(&data)
            },
            _ => Rc::new(KaramelPrimative::Empty)
        }
    }

    #[inline]
    pub fn deref_clean(&self) -> KaramelPrimative {
        match self.0 {
            n if (n & QNAN) != QNAN       => KaramelPrimative::Number(f64::from_bits(n)),
            e if e == (QNAN | EMPTY_FLAG) => KaramelPrimative::Empty,
            f if f == (QNAN | FALSE_FLAG) => KaramelPrimative::Bool(false),
            t if t == (QNAN | TRUE_FLAG)  => KaramelPrimative::Bool(true),
            p if (p & POINTER_FLAG) == POINTER_FLAG => {
                let pointer = (self.0 & POINTER_MASK) as *mut KaramelPrimative;
                let data = unsafe { ManuallyDrop::new(Rc::from_raw(pointer)) };
                match &**data {
                    KaramelPrimative::Text(text) => KaramelPrimative::Text(text.clone()),
                    KaramelPrimative::List(list) => KaramelPrimative::List(list.clone()),
                    KaramelPrimative::Dict(dict) => KaramelPrimative::Dict(dict.clone()),
                    KaramelPrimative::Function(func, base) => KaramelPrimative::Function(func.clone(), *base),
                    KaramelPrimative::Class(klass) => KaramelPrimative::Class(klass.clone()),
                    _ => KaramelPrimative::Empty
                }
            },
            _ => KaramelPrimative::Empty
        }
    }

    #[inline]
    pub fn as_number(&self) -> Option<f64> {
        match (self.0 & QNAN) != QNAN {
            true => Some(f64::from_bits(self.0)),
            false => None
        }
    }
}
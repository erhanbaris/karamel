use std::vec::Vec;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::cmp;


use crate::types::*;

#[derive(PartialEq, Debug)]
pub enum VmObjectType {
    Empty,
    Atom(u64),
    Integer(i64),
    Double(f64),
    Text(String),
    Bool(bool),
    List(Vec<Box<BramaAstType>>)
}

#[derive(PartialEq, Debug)]
pub struct VmObject {
    pub marked: bool,
    pub data: VmObjectType
}

impl VmObject {
    pub fn new(data: VmObjectType) -> VmObject {
        VmObject {
            marked: false,
            data: data
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct InnerStrorage {
    pub return_back_address: u32,
    pub return_back_variable: u32,
    pub const_variables: Vec<VmObject>,
    pub temporary_ariables: u8,
    pub variables: HashMap<String, VmObject>
}

pub struct StorageFeeder;

impl InnerStrorage {
    pub fn new() -> InnerStrorage {
        InnerStrorage {
            return_back_address: 0,
            return_back_variable: 0,
            const_variables: Vec::new(),
            temporary_ariables: 0,
            variables: HashMap::new()
        }
    }
    
    pub fn add_bool(&mut self, value: bool) {
        self.add_const_variable(VmObjectType::Bool(value));
    }

    pub fn add_list(&mut self) {
        self.const_variables.push(VmObject::new(VmObjectType::List(Vec::new())));
    }

    pub fn add_text(&mut self, value: String) {
        self.add_const_variable(VmObjectType::Text(value));
    }
    
    pub fn add_integer(&mut self, value: i64) {
        self.add_const_variable(VmObjectType::Integer(value));
    }
    
    pub fn add_double(&mut self, value: f64) {
        self.add_const_variable(VmObjectType::Double(value));
    }
    
    pub fn add_empty(&mut self) {
        self.add_const_variable(VmObjectType::Empty);
    }

    pub fn add_symbol(&mut self, symbol: &String) {
        if !self.variables.contains_key(&symbol[..]) {
            self.variables.insert(symbol.to_string(), VmObject::new(VmObjectType::Empty));
        }
    }

    pub fn add_atom(&mut self, atom: &String) {
        let mut hasher = DefaultHasher::new();
        atom.hash(&mut hasher);
        let item = VmObjectType::Atom(hasher.finish());

        self.add_const_variable(item);        
    }

    fn add_const_variable(&mut self, obj: VmObjectType) {
        let position = self.const_variables.iter().position(|x| {
            return x.data == obj;
        });
        
        match position {
            None => self.const_variables.push(VmObject::new(obj)),
            _ => ()
        };
    }
}


impl StorageFeeder {
    pub fn add_ast(ast: &BramaAstType, upper_ast: &BramaAstType, storage: &mut InnerStrorage) -> u8 {
        let mut temp_count = 0;

        match ast {
            BramaAstType::Binary {
                left, 
                operator: _, 
                right
            } => {
                let left_temp_count  = Self::add_ast(left, ast, storage);
                let right_temp_count = Self::add_ast(right, ast, storage);

                temp_count = match &upper_ast {
                    BramaAstType::None => left_temp_count + right_temp_count + 1,
                    _ => cmp::max(left_temp_count, right_temp_count)
                };
            },
            BramaAstType::Control {
                left, 
                operator: _, 
                right
            } => {
                let left_temp_count  = Self::add_ast(left, ast, storage);
                let right_temp_count = Self::add_ast(right, ast, storage);

                temp_count = match &upper_ast {
                    BramaAstType::None => left_temp_count + right_temp_count + 1,
                    _ => cmp::max(left_temp_count, right_temp_count)
                };
            },
            BramaAstType::Symbol(string) => storage.add_symbol(string),
            BramaAstType::Primative(primative) => {
                match primative {
                    BramaPrimative::Empty => storage.add_empty(),
                    BramaPrimative::Atom(atom) => storage.add_atom(atom),
                    BramaPrimative::Double(double) => storage.add_double(*double),
                    BramaPrimative::Integer(integer) => storage.add_integer(*integer),
                    BramaPrimative::Text(string) => storage.add_text(string.to_string()),
                    BramaPrimative::Bool(boolean) => storage.add_bool(*boolean),
                    BramaPrimative::List(list) => {
                        storage.add_list();
                        let mut list_temp_count = 0;
                        for array_item in list {
                            list_temp_count = cmp::max(Self::add_ast(array_item, ast, storage), list_temp_count);
                        }
                        temp_count = list_temp_count;
                    },
                    _ => ()
                }
            }
            _ => ()
        };

        return temp_count;
    }
}
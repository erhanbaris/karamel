use std::vec::Vec;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::cmp;


use crate::types::*;

impl BramaCompilerOption {
    pub fn new() -> BramaCompilerOption {
        BramaCompilerOption {
            opcodes: Vec::new(),
            storages: vec![InnerStorage::new()]
        }
    }
}

impl Storage for InnerStorage {
    fn build(&mut self) {
        self.memory.reserve((self.get_constant_size() + self.get_temp_size()).into());
        self.total_const_variables = self.get_constant_size() as i16;
        self.memory.append(&mut self.constants);

        for _ in 0..self.get_temp_size() {
            self.memory.push(VmObjectType::Empty);
        }
    }

    fn get_memory(&self) -> &Vec<VmObjectType> { &self.memory }
    fn get_constant_size(&self) -> u16 { self.constants.len() as u16 }
    fn get_variable_size(&self) -> u16 { self.variables.len() as u16 }
    fn get_temp_size(&self) -> u16     { self.temp_size }
    fn inc_temp_size(&mut self)        { self.temp_size += 1; }
    fn get_free_temp_slot(&mut self) -> u16 { 
        let index = self.temp_counter;
        self.temp_counter += 1;
        return self.get_variable_size() + index;
    }

    fn get_temp_counter(&self) -> u16 { self.temp_counter }
    fn inc_temp_counter(&mut self)    { self.temp_counter += 1; }
    fn reset_temp_counter(&mut self)  { self.temp_counter = 0; }

    fn add_variable        (&mut self, name: &str, variable: &VmObjectType);
    fn add_constant_atom   (&mut self, atom: &String);
    fn add_constant_empty  (&mut self);
    fn add_constant_double (&mut self, value: f64);
    fn add_constant_integer(&mut self, value: i64);
    fn add_constant_text   (&mut self, value: String);
    fn add_constant_list   (&mut self);
    fn add_constant_bool   (&mut self, value: bool);
    
    fn get_total_const_variables(&self) -> i16 { self.total_const_variables }

    fn get_temp_counter(&self) -> i16 { self.temp_counter }
    fn inc_temp_counter(&mut self) { self.temp_counter += 1; }
    fn reset_temp_counter(&mut self) { self.temp_counter = 0; }

    fn set_temp_variable_size(&mut self, size: u16) { self.temporary_variables = size; }
    fn add_variable(&mut self, name: &str, variable: &VmObjectType) { ; }
}

impl InnerStorage {
    fn new() -> InnerStorage {
        InnerStorage {
            constants: Vec::new(),
            temp_size: 0,
            temp_counter: 0,
            memory: Vec::new(),
            variables: HashMap::new()
        }
    }

    pub fn get_temp_variable_index(&mut self) -> i16 {
        let index = self.temp_counter;
        self.temp_counter += 1;
        return self.total_const_variables + index;
    }
    
    pub fn add_bool(&mut self, value: bool) {
        self.add_const_variable(VmObjectType::Bool(value));
    }

    pub fn add_list(&mut self) {
        self.constants.push(VmObjectType::List(Vec::new()));
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
            self.variables.insert(symbol.to_string(), VmObjectType::Empty);
        }
    }

    pub fn add_atom(&mut self, atom: &String) {
        let mut hasher = DefaultHasher::new();
        atom.hash(&mut hasher);
        let item = VmObjectType::Atom(hasher.finish());

        self.add_const_variable(item);        
    }

    fn add_const_variable(&mut self, obj: VmObjectType) {
        let position = self.constants.iter().position(|x| {
            return *x == obj;
        });
        
        match position {
            None => self.const_variables.push(obj),
            _ => ()
        };
    }

    fn find_const_variable(&self, obj: VmObjectType) -> Option<usize> {
        self.memory.iter().position(|x| {
            return *x == obj;
        })
    }
}

struct CompileInfo {
    index: i16
}

pub trait Compiler {
    fn prepare_variable_store(&self, ast: &BramaAstType, options: &mut BramaCompilerOption);
    fn compile(&self, ast: &BramaAstType, options: &mut BramaCompilerOption) -> CompilerResult;
}


pub struct InterpreterCompiler;
impl Compiler for InterpreterCompiler {
    fn prepare_variable_store(&self, ast: &BramaAstType, options: &mut BramaCompilerOption) {
        let max_temps = self.add_ast(ast, &BramaAstType::None, options, 0);
        options.storages[0].temporary_variables = max_temps;
        options.storages[0].build();
    }
    
    fn compile(&self, ast: &BramaAstType, options: &mut BramaCompilerOption) -> CompilerResult {
        let mut main_compile_info = CompileInfo {index: 0};

        self.inner_compile(ast, &BramaAstType::None, &mut main_compile_info, options, 0)?;
        Ok(())
    }
}
impl InterpreterCompiler {
    fn inner_compile(&self, ast: &BramaAstType, upper_ast: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult {
        match ast {
            BramaAstType::Control { left, operator, right } => self.control(left, operator, right, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Binary { left, operator, right } => self.binary(left, operator, right, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Primative(primative) => self.primative(primative, compiler_info, options, storage_index),
            _ => {
                println!("{:?}", ast);
                Err(("Not implemented", 0, 0))
            }
        }
    }

    fn add_ast(&self, ast: &BramaAstType, _: &BramaAstType, options: &mut BramaCompilerOption, storage_index: usize) -> u16 {
        let temp_count = match ast {
            BramaAstType::Binary {
                left, 
                operator: _, 
                right
            } => self.add_ast(left, ast, options, storage_index) + self.add_ast(right, ast, options, storage_index) + 1,
            BramaAstType::Control {
                left, 
                operator: _, 
                right
            } => self.add_ast(left, ast, options, storage_index) + self.add_ast(right, ast, options, storage_index) + 1,
            BramaAstType::PrefixUnary(_, inner_ast) => self.add_ast(inner_ast, ast, options, storage_index),
            BramaAstType::SuffixUnary(_, inner_ast) => self.add_ast(inner_ast, ast, options, storage_index),
            BramaAstType::Symbol(string) => {
                options.storages.get_mut(storage_index).unwrap().add_symbol(string);
                0
            },
            BramaAstType::Primative(primative) => {
                match primative {
                    BramaPrimative::Empty => options.storages.get_mut(storage_index).unwrap().add_empty(),
                    BramaPrimative::Atom(atom) => options.storages.get_mut(storage_index).unwrap().add_atom(atom),
                    BramaPrimative::Double(double) => options.storages.get_mut(storage_index).unwrap().add_double(*double),
                    BramaPrimative::Integer(integer) => options.storages.get_mut(storage_index).unwrap().add_integer(*integer),
                    BramaPrimative::Text(string) => options.storages.get_mut(storage_index).unwrap().add_text(string.to_string()),
                    BramaPrimative::Bool(boolean) => options.storages.get_mut(storage_index).unwrap().add_bool(*boolean),
                    BramaPrimative::List(list) => {
                        options.storages.get_mut(storage_index).unwrap().add_list();
                        let mut list_temp_count = 0;
                        for array_item in list {
                            list_temp_count = cmp::max(self.add_ast(array_item, ast, options, storage_index), list_temp_count);
                        }
                    }
                };
                0
            }
            _ => 0
        };

        return temp_count;
    }

    fn primative(&self, primative: &BramaPrimative, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult {
        let storage :&InnerStorage = &options.storages[storage_index];

        let result = match primative {
            BramaPrimative::Bool(boolean) => storage.find_const_variable(VmObjectType::Bool(*boolean)),
            BramaPrimative::Integer(integer) => storage.find_const_variable(VmObjectType::Integer(*integer)),
            BramaPrimative::Double(double) => storage.find_const_variable(VmObjectType::Double(*double)),
            BramaPrimative::Atom(atom) =>  {
                let mut hasher = DefaultHasher::new();
                atom.hash(&mut hasher);
                storage.find_const_variable(VmObjectType::Atom(hasher.finish()))
            },
            BramaPrimative::Text(text) => storage.find_const_variable(VmObjectType::Text(text.to_string())),
            BramaPrimative::Empty => storage.find_const_variable(VmObjectType::Empty),
            _ => Some(0)
        };

        match result {
            Some(index) => {
                compiler_info.index = index as i16;
                Ok(())
            },
            _ => Err(("Value not found in storage", 0, 0))
        }
    }

    fn control(&self, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult {
        compiler_info.index = -1;

        self.inner_compile(left_ast, &BramaAstType::None, compiler_info, options, storage_index)?;

        let left = compiler_info.index;
        compiler_info.index = -1;
        self.inner_compile(right_ast, &BramaAstType::None, compiler_info, options, storage_index)?;
        let right = compiler_info.index;

        let target = options.storages[storage_index].get_temp_variable_index();

        let opcode = match operator {
            BramaOperatorType::Or               => BramaVmOpCode::Or               { target, left, right },
            BramaOperatorType::And              => BramaVmOpCode::And              { target, left, right },
            BramaOperatorType::Equal            => BramaVmOpCode::Equal            { target, left, right },
            BramaOperatorType::NotEqual         => BramaVmOpCode::NotEqual         { target, left, right },
            BramaOperatorType::GreaterThan      => BramaVmOpCode::GreaterThan      { target, left, right },
            BramaOperatorType::LessThan         => BramaVmOpCode::LessThan         { target, left, right },
            BramaOperatorType::GreaterEqualThan => BramaVmOpCode::GreaterEqualThan { target, left, right },
            BramaOperatorType::LessEqualThan    => BramaVmOpCode::LessEqualThan    { target, left, right },
            _ => BramaVmOpCode::None
        };

        options.opcodes.push(opcode);
        compiler_info.index = target;

        Ok(())
    }

    fn binary(&self, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult {
        compiler_info.index = -1;

        self.inner_compile(left_ast, &BramaAstType::None, compiler_info, options, storage_index)?;

        let left = compiler_info.index;
        compiler_info.index = -1;
        self.inner_compile(right_ast, &BramaAstType::None, compiler_info, options, storage_index)?;
        let right = compiler_info.index;

        let target = options.storages[storage_index].get_temp_variable_index();

        let opcode = match operator {
            BramaOperatorType::Addition       => BramaVmOpCode::Addition { target, left, right },
            BramaOperatorType::Subtraction    => BramaVmOpCode::Subraction { target, left, right },
            BramaOperatorType::Multiplication => BramaVmOpCode::Multiply { target, left, right },
            BramaOperatorType::Division       => BramaVmOpCode::Division { target, left, right },
            _ => BramaVmOpCode::None
        };

        options.opcodes.push(opcode);
        compiler_info.index = target;

        Ok(())
    }
}
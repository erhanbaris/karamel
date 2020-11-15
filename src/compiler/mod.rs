use std::vec::Vec;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::cmp;


use crate::types::*;

impl VmObject {
    pub fn new(data: VmObjectType) -> VmObject {
        VmObject {
            marked: false,
            data: data
        }
    }
}

impl BramaCompilerOption {
    pub fn new() -> BramaCompilerOption {
        BramaCompilerOption {
            opcodes: Vec::new(),
            storages: vec![InnerStrorage::new()]
        }
    }
}

impl InnerStrorage {
    fn new() -> InnerStrorage {
        InnerStrorage {
            return_back_address: 0,
            return_back_variable: 0,
            const_variables: Vec::new(),
            temporary_variables: 0,
            total_const_variables: 0,
            temp_counter: 0,
            memory: Vec::new(),
            variables: HashMap::new()
        }
    }

    pub fn build(&mut self) {
        self.memory.reserve(self.const_variables.len() + self.temporary_variables as usize);
        self.total_const_variables = self.const_variables.len() as i16;
        self.memory.append(&mut self.const_variables);

        for _ in 0..self.temporary_variables {
            self.memory.push(VmObjectType::Empty);
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
        self.const_variables.push(VmObjectType::List(Vec::new()));
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
        let position = self.const_variables.iter().position(|x| {
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

    fn add_ast(&self, ast: &BramaAstType, upper_ast: &BramaAstType, options: &mut BramaCompilerOption, storage_index: usize) -> u16 {
        let mut temp_count = 0;

        match ast {
            BramaAstType::Binary {
                left, 
                operator: _, 
                right
            } => {
                let left_temp_count  = self.add_ast(left, ast, options, storage_index);
                let right_temp_count = self.add_ast(right, ast, options, storage_index);

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
                let left_temp_count  = self.add_ast(left, ast, options, storage_index);
                let right_temp_count = self.add_ast(right, ast, options, storage_index);

                temp_count = match &upper_ast {
                    BramaAstType::None => left_temp_count + right_temp_count + 1,
                    _ => cmp::max(left_temp_count, right_temp_count)
                };
            },
            BramaAstType::PrefixUnary(_, inner_ast) => {
                temp_count = cmp::max(temp_count, self.add_ast(inner_ast, ast, options, storage_index));
            },
            BramaAstType::SuffixUnary(_, inner_ast) => {
                temp_count = cmp::max(temp_count, self.add_ast(inner_ast, ast, options, storage_index));
            },
            BramaAstType::Symbol(string) => options.storages.get_mut(storage_index).unwrap().add_symbol(string),
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
                        temp_count = list_temp_count;
                    },
                    _ => {
                        println!("{:?}", primative);
                    }
                }
            }
            _ => ()
        };

        return temp_count;
    }

    fn primative(&self, primative: &BramaPrimative, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult {
        let storage :&InnerStrorage = &options.storages[storage_index];

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
            BramaOperatorType::Or => BramaVmOpCode::Or { target, left, right },
            BramaOperatorType::And => BramaVmOpCode::And { target, left, right },
            _ => BramaVmOpCode::None
        };

        options.opcodes.push(opcode);

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
            BramaOperatorType::Addition => BramaVmOpCode::Addition { target, left, right },
            BramaOperatorType::Subtraction => BramaVmOpCode::Subraction { target, left, right },
            _ => BramaVmOpCode::None
        };

        options.opcodes.push(opcode);

        Ok(())
    }
}
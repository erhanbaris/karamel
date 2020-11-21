use std::vec::Vec;
use std::collections::HashMap;
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
        self.constant_size = self.constants.len() as u16;

        /* Allocate memory */
        self.memory.reserve((self.get_constant_size() + self.get_variable_size() + self.get_temp_size()).into());

        /* Move all constants informations to memory location */
        self.memory.append(&mut self.constants);

        /*  Allocate variable memory and update referances */
        let mut index = self.get_constant_size();
        for (_, value) in self.variables.iter_mut() {
            self.memory.push(VmObject::convert(BramaPrimative::Empty));
            *value = index;
            index += 1;
        }

        let start_index = self.get_temp_size();
        for _ in 0..start_index {
            self.memory.push(VmObject::convert(BramaPrimative::Empty));
        }
    }

    fn get_memory(&self) -> &Vec<VmObject> { &self.memory }
    fn get_constant_size(&self) -> u16 { self.constant_size }
    fn get_variable_size(&self) -> u16 { self.variables.len() as u16 }
    fn get_temp_size(&self) -> u16     { self.temp_size }
    fn set_temp_size(&mut self, value: u16) { self.temp_size = value; }
    fn get_free_temp_slot(&mut self) -> u16 { 
        let index = self.temp_counter;
        self.temp_counter += 1;
        return self.get_constant_size() + self.get_variable_size() + index;
    }

    fn get_temp_counter(&self) -> u16 { self.temp_counter }
    fn inc_temp_counter(&mut self)    { self.temp_counter += 1; }
    fn reset_temp_counter(&mut self)  { self.temp_counter = 0; }

    fn add_constant(&mut self, value: &BramaPrimative) {
        let position = self.constants.iter().position(|x| {
            return *x.deref() == *value;
        });
        
        match position {
            None => self.constants.push(VmObject::convert(value.clone())),
            _ => ()
        };
    }

    fn add_variable(&mut self, name: &String) { 
        if !self.variables.contains_key(&name[..]) {
            self.variables.insert(name.to_string(), 0);
        }
    }

    fn set_variable_value(&mut self, name: &String, object: VmObject) {
        match self.get_variable_location(name) {
            Some(location) => {
                self.memory[location as usize] = object;
            },
            _ => ()
        };
    }

    fn get_variable_location(&mut self, name: &String) -> Option<u16> {
        if self.variables.contains_key(name) {
            return Some(*self.variables.get(name).unwrap());
        }
        return None;
    }

    fn get_constant_location(&mut self, value: &BramaPrimative) -> Option<u16> {
        return match self.memory.iter().position(|x| { return *x.deref() == *value; }) {
            Some(number) => Some(number as u16),
            _ => None
        };
    }

    fn dump(&self) {
        println!("-------------------------------");
        println!("        MEMORY DUMP");
        println!("-------------------------------");
        for (index, item) in self.memory.iter().enumerate() {
            println!("| {:?} | {:?}", index, *item.deref());
        }
        println!("-------------------------------");
        println!("-------------------------------");
        println!("        VARIABLE DUMP");
        println!("-------------------------------");
        for (variable, value) in &self.variables {
            println!("| {:?}  [{:?}]", variable, value);
        }
        println!("-------------------------------");
    }
}

impl InnerStorage {
    fn new() -> InnerStorage {
        InnerStorage {
            constants: Vec::new(),
            constant_size: 0,
            temp_size: 0,
            temp_counter: 0,
            total_const_variables: 0,
            memory: Vec::new(),
            variables: HashMap::new()
        }
    }

    fn find_const_variable(&self, obj: &BramaPrimative) -> Option<usize> {
        self.memory.iter().position(|x| {
            return *x.deref() == *obj;
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
        options.storages[0].set_temp_size(max_temps);
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
            BramaAstType::Symbol(variable) => self.symbol(variable, upper_ast, compiler_info, options, storage_index),
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
                options.storages.get_mut(storage_index).unwrap().add_variable(string);
                0
            },
            BramaAstType::Primative(primative) => {
                options.storages.get_mut(storage_index).unwrap().add_constant(primative);
                if let BramaPrimative::List(list) = primative {
                    let mut list_temp_count = 0;
                    for array_item in list {
                        list_temp_count = cmp::max(self.add_ast(array_item, ast, options, storage_index), list_temp_count);
                    }
                }
                0
            }
            _ => 0
        };
        return temp_count;
    }

    fn primative(&self, primative: &BramaPrimative, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult {
        let storage :&InnerStorage = &options.storages[storage_index];

        let result = storage.find_const_variable(primative);
        match result {
            Some(index) => {
                compiler_info.index = index as i16;
                Ok(())
            },
            _ => Err(("Value not found in storage", 0, 0))
        }
    }

    fn symbol(&self, variable: &String, _: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult {
        compiler_info.index = match options.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
            Some(location) => location as i16,
            _ => return Err(("Variable not found in storage", 0, 0))
        };

        Ok(())
    }

    fn control(&self, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult {
        compiler_info.index = -1;

        self.inner_compile(left_ast, &BramaAstType::None, compiler_info, options, storage_index)?;

        let left = compiler_info.index;
        compiler_info.index = -1;
        self.inner_compile(right_ast, &BramaAstType::None, compiler_info, options, storage_index)?;
        let right = compiler_info.index;

        let target = options.storages[storage_index].get_free_temp_slot() as i16;

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

        let target = options.storages[storage_index].get_free_temp_slot() as i16;

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
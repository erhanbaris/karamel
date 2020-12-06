use std::vec::Vec;
use std::rc::Rc;

use crate::types::*;
use crate::compiler::*;
use crate::core::*;
use crate::compiler::value::BramaPrimative;
use crate::compiler::ast::BramaAstType;
use crate::compiler::storage_builder::StorageBuilder;

pub struct BramaCompilerOption {
    pub opcodes : Vec<u8>,
    pub storages: Vec<StaticStorage>,
    pub modules: ModuleCollection,
    pub opcode_index: usize
}

impl  BramaCompilerOption {
    pub fn new() -> BramaCompilerOption {
        BramaCompilerOption {
            opcodes: Vec::new(),
            storages: vec![StaticStorage::new()],
            modules: ModuleCollection::new(),
            opcode_index: 0
        }
    }

    pub fn reset(&mut self) {
        self.opcodes = Vec::new();
    }
}


struct CompileInfo {
    /*index: Option<u8>*/
}

pub trait Compiler
{
    fn compile(&self, ast: &BramaAstType, options: &mut BramaCompilerOption) -> CompilerResult;
}


pub struct InterpreterCompiler;
impl Compiler for InterpreterCompiler {   
    fn compile(&self, ast: &BramaAstType, options: &mut BramaCompilerOption) -> CompilerResult {
        let storage_builder: StorageBuilder = StorageBuilder::new();
        storage_builder.prepare_variable_store(ast, options);
        
        let mut main_compile_info = CompileInfo { };

        self.generate_opcode(ast, &BramaAstType::None, &mut main_compile_info, options, 0)?;
        Ok(0)
    }
}

impl InterpreterCompiler {
    fn generate_opcode(&self, ast: &BramaAstType, upper_ast: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult {
        match ast {
            BramaAstType::Assignment { variable, operator, expression } => self.generate_assignment(variable.clone(), operator, expression, compiler_info, options, storage_index),
            BramaAstType::Symbol(variable)                              => self.generate_symbol(variable, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Control { left, operator, right }             => self.generate_control(left, operator, right, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Binary { left, operator, right }              => self.generate_binary(left, operator, right, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Block(asts)                                   => self.generate_block(asts, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Primative(primative)                          => self.generate_primative(primative.clone(), compiler_info, options, storage_index),
            BramaAstType::FuncCall { names, arguments }                 => self.generate_func_call(names, arguments, upper_ast, compiler_info, options, storage_index),
            BramaAstType::PrefixUnary (operator, expression)            => self.generate_prefix_unary(operator, expression, upper_ast, compiler_info, options, storage_index),
            BramaAstType::SuffixUnary (operator, expression)            => self.generate_suffix_unary(operator, expression, upper_ast, compiler_info, options, storage_index),
            BramaAstType::NewLine => Ok(0),
            BramaAstType::None => {
                println!("{:?}", ast);
                Err("Not implemented")
            }
        }
    }

    fn generate_primative(&self, primative: Rc<BramaPrimative>, _: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult {
        let storage = &options.storages[storage_index];

        let result = storage.get_constant_location(primative);
        match result {
            Some(index) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(index as u8);
                Ok(index)
            },
            _ => Err("Value not found in storage")
        }
    }

    fn generate_func_call(&self, names: &Vec<String>, arguments: &Vec<Box<BramaAstType>>,  upper_ast: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult {
        /* Build arguments */
        for argument in arguments {
            self.generate_opcode(argument, upper_ast, compiler_info, options, storage_index)?;
        }

        let func = options.modules.find_method(names);
        return match func {
            Some(function) => {
                if let Some(location) = options.storages[storage_index].get_constant_location(Rc::new(BramaPrimative::FuncNativeCall(function))) {
                    options.opcodes.push(VmOpCode::NativeCall as u8);
                    options.opcodes.push(location as u8);
                    options.opcodes.push((arguments.len() as u8) as u8);
                    Ok(0 as u8)
                } else {
                    Err("Function not found")
                }
            },
            None => Err("Function not found")
        };
    }

    fn generate_symbol(&self, variable: &String, _: &BramaAstType, _: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult {
        match options.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
            Some(location) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(location as u8);
                Ok(location)
            },
            _ => return Err("Variable not found in storage")
        }
    }

    fn generate_control(&self, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult {
        self.generate_opcode(left_ast, &BramaAstType::None, compiler_info, options, storage_index)?;
        self.generate_opcode(right_ast, &BramaAstType::None, compiler_info, options, storage_index)?;

        let opcode = match operator {
            BramaOperatorType::Or               => VmOpCode::Or as u8,
            BramaOperatorType::And              => VmOpCode::And as u8,
            BramaOperatorType::Equal            => VmOpCode::Equal as u8,
            BramaOperatorType::NotEqual         => VmOpCode::NotEqual as u8,
            BramaOperatorType::GreaterThan      => VmOpCode::GreaterThan as u8,
            BramaOperatorType::LessThan         => VmOpCode::LessThan as u8,
            BramaOperatorType::GreaterEqualThan => VmOpCode::GreaterEqualThan as u8,
            BramaOperatorType::LessEqualThan    => VmOpCode::LessEqualThan as u8,
            _ => VmOpCode::None as u8
        };

        options.opcodes.push(opcode);
        Ok(0)
    }

    fn generate_assignment(&self, variable: Rc<String>, operator: &BramaOperatorType, expression_ast: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult {
        let location = options.storages.get_mut(storage_index).unwrap().add_variable(&*variable);
        let storage = &options.storages[storage_index];
        
        if let BramaAstType::Primative(primative) = expression_ast {
            if *operator == BramaOperatorType::Assign {
                let result = storage.get_constant_location(primative.clone());
                let primative_location = match result {
                    Some(index) => index as u8,
                    _ => return Err("Value not found in storage")
                };

                options.opcodes.push(VmOpCode::FastStore as u8);
                options.opcodes.push(location);
                options.opcodes.push(primative_location);
                return Ok(0);
            }
        }

        if *operator != BramaOperatorType::Assign {

            /* Load variable data to stack */
            options.opcodes.push(VmOpCode::Load as u8);
            options.opcodes.push(location);

            self.generate_opcode(expression_ast, &BramaAstType::None, compiler_info, options, storage_index)?;

            let opcode = match operator {
                BramaOperatorType::AssignAddition       => VmOpCode::Addition as u8,
                BramaOperatorType::AssignDivision       => VmOpCode::Division as u8,
                BramaOperatorType::AssignMultiplication => VmOpCode::Multiply as u8,
                BramaOperatorType::AssignSubtraction    => VmOpCode::Subraction as u8,
                _ => BramaOperatorType::None as u8
            };

            options.opcodes.push(opcode);
        } else {
            self.generate_opcode(expression_ast, &BramaAstType::None, compiler_info, options, storage_index)?;
        }
        
        options.opcodes.push(VmOpCode::Store as u8);
        options.opcodes.push(location);
        
        Ok(0)
    }

    fn generate_binary(&self, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult { 
        self.generate_opcode(left_ast, &BramaAstType::None, compiler_info, options, storage_index)?;
        self.generate_opcode(right_ast, &BramaAstType::None, compiler_info, options, storage_index)?;
        let opcode = match operator {
            BramaOperatorType::Addition       => VmOpCode::Addition as u8,
            BramaOperatorType::Subtraction    => VmOpCode::Subraction as u8,
            BramaOperatorType::Multiplication => VmOpCode::Multiply as u8,
            BramaOperatorType::Division       => VmOpCode::Division as u8,
            _ => VmOpCode::None as u8
        };

        options.opcodes.push(opcode);
        Ok(0)
    }

    fn generate_prefix_unary(&self, operator: &BramaOperatorType, expression: &BramaAstType, _: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult { 
        
        if *operator == BramaOperatorType::Not { 
            return self.generate_not(expression, compiler_info, options, storage_index);
        }

        if let BramaAstType::Symbol(variable) = expression {
            let location = match options.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
                Some(location) => location,
                _ => return Err("Variable not found in storage")
            };

            /* Load data from memory */
            options.opcodes.push(VmOpCode::Load as u8);
            options.opcodes.push(location);
        
            let opcode = match operator {
                BramaOperatorType::Increment  => VmOpCode::Increment as u8,
                BramaOperatorType::Deccrement => VmOpCode::Decrement as u8,
                _ => return Err("Unary operator not found")
            };
    
            options.opcodes.push(opcode);
            options.opcodes.push(VmOpCode::CopyToStore as u8);
            options.opcodes.push(location);
            return Ok(0);
        }

        Err("Unary expression not valid")
    }

    fn generate_not(&self, expression: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult { 
        self.generate_opcode(expression, &BramaAstType::None, compiler_info, options, storage_index)?;
        options.opcodes.push(VmOpCode::Not as u8);
        return Ok(0);
    }

    fn generate_suffix_unary(&self, operator: &BramaOperatorType, expression: &BramaAstType, _: &BramaAstType, _: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult { 
        if let BramaAstType::Symbol(variable) = expression {
            let location = match options.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
                Some(location) => location,
                _ => return Err("Variable not found in storage")
            };

            options.opcodes.push(VmOpCode::Load as u8);
            options.opcodes.push(location);
            options.opcodes.push(VmOpCode::Dublicate as u8);

            let opcode = match operator {
                BramaOperatorType::Increment  => VmOpCode::Increment as u8,
                BramaOperatorType::Deccrement => VmOpCode::Decrement as u8,
                BramaOperatorType::Not        => VmOpCode::Not as u8,
                _ => return Err("Unary operator not found")
            };
    
            options.opcodes.push(opcode);
            options.opcodes.push(VmOpCode::Store as u8);
            options.opcodes.push(location);
            return Ok(0);
        }

        Err("Unary expression not valid")
    }

    fn generate_block(&self, asts: &Vec<BramaAstType>, upper_ast: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption, storage_index: usize) -> CompilerResult {
        for ast in asts {
            self.generate_opcode(&ast, upper_ast, compiler_info, options, storage_index)?;
        }
        Ok(0)
    }
}
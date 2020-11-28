use std::vec::Vec;
use std::rc::Rc;

use crate::types::*;
use crate::compiler::*;
use crate::core::*;
use crate::compiler::value::BramaPrimative;
use crate::compiler::ast::BramaAstType;
use crate::compiler::storage_builder::StorageBuilder;

pub struct BramaCompilerOption<S>
where S: Storage {
    pub opcodes : Vec<VmByte>,
    pub storages: Vec<S>,
    pub modules: ModuleCollection
}

impl<S>  BramaCompilerOption<S>
where S: Storage
{
    pub fn new() -> BramaCompilerOption<S> {
        BramaCompilerOption {
            opcodes: Vec::new(),
            storages: vec![S::new()],
            modules: ModuleCollection::new()
        }
    }
}


struct CompileInfo {
    /*index: Option<u8>*/
}

pub trait Compiler<S> where S: Storage
{
    fn compile(&self, ast: &BramaAstType, options: &mut BramaCompilerOption<S>) -> CompilerResult;
}


pub struct InterpreterCompiler;
impl<S> Compiler<S> for InterpreterCompiler where S: Storage {   
    fn compile(&self, ast: &BramaAstType, options: &mut BramaCompilerOption<S>) -> CompilerResult {
        let storage_builder: StorageBuilder<S> = StorageBuilder::new();
        storage_builder.prepare_variable_store(ast, options);
        
        let mut main_compile_info = CompileInfo { };

        self.generate_opcode(ast, &BramaAstType::None, &mut main_compile_info, options, 0)?;
        Ok(0)
    }
}

impl InterpreterCompiler {
    fn generate_opcode<S>(&self, ast: &BramaAstType, upper_ast: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        match ast {
            BramaAstType::Assignment { variable, operator, expression } => self.generate_assignment(variable.clone(), operator, expression, compiler_info, options, storage_index),
            BramaAstType::Symbol(variable)                              => self.generate_symbol(variable, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Control { left, operator, right }             => self.generate_control(left, operator, right, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Binary { left, operator, right }              => self.generate_binary(left, operator, right, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Block(asts)                                   => self.generate_block(asts, upper_ast, compiler_info, options, storage_index),
            BramaAstType::FuncCall { name, arguments }                  => self.generate_func_call(name, arguments, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Primative(primative)                          => self.generate_primative(primative.clone(), compiler_info, options, storage_index),
            _ => {
                println!("{:?}", ast);
                Err("Not implemented")
            }
        }
    }

    fn generate_primative<S>(&self, primative: Rc<BramaPrimative>, _: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        let storage = &options.storages[storage_index];

        let result = storage.get_constant_location(primative);
        match result {
            Some(index) => Ok(index),
            _ => Err("Value not found in storage")
        }
    }

    fn generate_func_call<S>(&self, name: &String, arguments: &Vec<Box<BramaAstType>>,  upper_ast: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        /* Save temp counter to restore back */
        let temp_index = options.storages[storage_index].get_temp_counter();

        /* Build arguments */
        for argument in arguments {
            self.generate_opcode(argument, upper_ast, compiler_info, options, storage_index)?;
        }

        let func = options.modules.get_function("buildin".to_string(), name.to_string());
        match func {
            Some(function) => {
                /*BramaVmOpCode::NativeFuncCall {
                    target: target,
                    function
                };*/
            },
            None => ()
        };

        /* Restore temp counter */
        options.storages[storage_index].set_temp_counter(temp_index);
        Ok(0)
    }

    fn generate_symbol<S>(&self, variable: &String, _: &BramaAstType, _: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        match options.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
            Some(location) => Ok(location),
            _ => return Err("Variable not found in storage")
        }
    }

    fn generate_control<S>(&self, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        let left   = self.generate_opcode(left_ast, &BramaAstType::None, compiler_info, options, storage_index)?;
        let right  = self.generate_opcode(right_ast, &BramaAstType::None, compiler_info, options, storage_index)?;
        let target = options.storages[storage_index].get_free_temp_slot() as u8;

        let opcode = match operator {
            BramaOperatorType::Or               => VmByte::new(VmOpCode::Or, target, left, right),
            BramaOperatorType::And              => VmByte::new(VmOpCode::And, target, left, right),
            BramaOperatorType::Equal            => VmByte::new(VmOpCode::Equal, target, left, right),
            BramaOperatorType::NotEqual         => VmByte::new(VmOpCode::NotEqual, target, left, right),
            BramaOperatorType::GreaterThan      => VmByte::new(VmOpCode::GreaterThan, target, left, right),
            BramaOperatorType::LessThan         => VmByte::new(VmOpCode::LessThan, target, left, right),
            BramaOperatorType::GreaterEqualThan => VmByte::new(VmOpCode::GreaterEqualThan, target, left, right),
            BramaOperatorType::LessEqualThan    => VmByte::new(VmOpCode::LessEqualThan, target, left, right),
            _ => VmByte::none()
        };

        options.opcodes.push(opcode);
        Ok(target)
    }

    fn generate_assignment<S>(&self, variable: Rc<String>, operator: &BramaOperatorType, expression_ast: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        let target = options.storages.get_mut(storage_index).unwrap().add_variable(&*variable);
        let source = self.generate_opcode(expression_ast, &BramaAstType::None, compiler_info, options, storage_index)?;

        let opcode = match operator {
            BramaOperatorType::Assign               => VmByte::new(VmOpCode::Assign,               target, source, 0),
            BramaOperatorType::AssignAddition       => VmByte::new(VmOpCode::AssignAddition,       target, source, 0),
            BramaOperatorType::AssignDivision       => VmByte::new(VmOpCode::AssignDivision,       target, source, 0),
            BramaOperatorType::AssignMultiplication => VmByte::new(VmOpCode::AssignMultiplication, target, source, 0),
            BramaOperatorType::AssignSubtraction    => VmByte::new(VmOpCode::AssignSubtraction,    target, source, 0),
            _ => VmByte::none()
        };
        
        options.opcodes.push(opcode);
        Ok(0)
    }

    fn generate_binary<S>(&self, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage { let left = self.generate_opcode(left_ast, &BramaAstType::None, compiler_info, options, storage_index)?;
        let right  = self.generate_opcode(right_ast, &BramaAstType::None, compiler_info, options, storage_index)?;
        let target = options.storages[storage_index].get_free_temp_slot();

        let opcode = match operator {
            BramaOperatorType::Addition       => VmByte::new(VmOpCode::Addition, target, left, right),
            BramaOperatorType::Subtraction    => VmByte::new(VmOpCode::Subraction, target, left, right),
            BramaOperatorType::Multiplication => VmByte::new(VmOpCode::Multiply, target, left, right),
            BramaOperatorType::Division       => VmByte::new(VmOpCode::Division, target, left, right),
            _ => VmByte::none()
        };

        options.opcodes.push(opcode);
        Ok(target)
    }

    fn generate_block<S>(&self, asts: &Vec<BramaAstType>, upper_ast: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        for ast in asts {
            self.generate_opcode(&ast, upper_ast, compiler_info, options, storage_index)?;
            options.storages[storage_index].reset_temp_counter();
        }
        Ok(0)
    }
}
use std::vec::Vec;
use std::rc::Rc;
use std::cmp::max;

use crate::types::*;
use crate::compiler::*;

pub struct BramaCompilerOption<S>
where S: Storage {
    pub opcodes : Vec<BramaVmOpCode>,
    pub storages: Vec<S>
}

impl<S>  BramaCompilerOption<S>
where S: Storage
{
    pub fn new() -> BramaCompilerOption<S> {
        BramaCompilerOption {
            opcodes: Vec::new(),
            storages: vec![S::new()]
        }
    }
}


struct CompileInfo {
    index: i16
}

pub trait Compiler<S> where S: Storage
{
    fn prepare_variable_store(&self, ast: &BramaAstType, options: &mut BramaCompilerOption<S>);
    fn compile(&self, ast: &BramaAstType, options: &mut BramaCompilerOption<S>) -> CompilerResult;
}


pub struct InterpreterCompiler;
impl<S> Compiler<S> for InterpreterCompiler where S: Storage {
    fn prepare_variable_store(&self, ast: &BramaAstType, options: &mut BramaCompilerOption<S>) {
        let max_temps = self.get_temp_count_from_ast(ast, &BramaAstType::None, options, 0);
        options.storages[0].set_temp_size(max_temps);
        options.storages[0].build();
    }
    
    fn compile(&self, ast: &BramaAstType, options: &mut BramaCompilerOption<S>) -> CompilerResult {
        let mut main_compile_info = CompileInfo {index: 0};

        self.generate_opcode(ast, &BramaAstType::None, &mut main_compile_info, options, 0)?;
        Ok(())
    }
}

impl InterpreterCompiler {
    fn generate_opcode<S>(&self, ast: &BramaAstType, upper_ast: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        match ast {
            BramaAstType::Assignment { variable, operator, expression } => self.generate_assignment(variable.clone(), operator, expression, compiler_info, options, storage_index),
            BramaAstType::Symbol(variable)                              => self.generate_symbol(variable, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Control { left, operator, right }             => self.generate_control(left, operator, right, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Binary { left, operator, right }              => self.generate_binary(left, operator, right, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Block(asts) => self.generate_block(asts, upper_ast, compiler_info, options, storage_index),
            BramaAstType::Primative(primative)                          => self.generate_primative(primative.clone(), compiler_info, options, storage_index),
            _ => {
                println!("{:?}", ast);
                Err(("Not implemented", 0, 0))
            }
        }
    }

    fn get_temp_count_from_ast<S>(&self, ast: &BramaAstType, _: &BramaAstType, options: &mut BramaCompilerOption<S>, storage_index: usize) -> u16 where S: Storage {
        let temp_count = match ast {
            BramaAstType::Binary {
                left,
                operator: _,
                right} => self.get_temp_count_from_ast(left, ast, options, storage_index) + self.get_temp_count_from_ast(right, ast, options, storage_index) + 1,
            
            BramaAstType::Control {
                left,
                operator: _,
                right} => self.get_temp_count_from_ast(left, ast, options, storage_index) + self.get_temp_count_from_ast(right, ast, options, storage_index) + 1,
            
            BramaAstType::PrefixUnary(_, inner_ast) => self.get_temp_count_from_ast(inner_ast, ast, options, storage_index),
            BramaAstType::SuffixUnary(_, inner_ast) => self.get_temp_count_from_ast(inner_ast, ast, options, storage_index),
            BramaAstType::Symbol(string) => {
                options.storages.get_mut(storage_index).unwrap().add_variable(&string);
                0
            },
            
            BramaAstType::Assignment {
                variable,
                operator: _,
                expression} =>  {
                options.storages.get_mut(storage_index).unwrap().add_variable(&*variable);
                self.get_temp_count_from_ast(expression, ast, options, storage_index)
            },
            
            BramaAstType::Block(asts) => {
                let mut list_temp_count = 0;
                for array_item in asts {
                    list_temp_count = max(self.get_temp_count_from_ast(array_item, ast, options, storage_index), list_temp_count);
                }
                list_temp_count
            },

            BramaAstType::Primative(primative) => {
                options.storages.get_mut(storage_index).unwrap().add_constant(Rc::clone(primative));
                /*if let BramaPrimative::List(list) = primative {
                    let mut list_temp_count = 0;
                    for array_item in list {
                        list_temp_count = cmp::max(self.get_temp_count_from_ast(array_item, ast, options, storage_index), list_temp_count);
                    }
                }*/
                0
            }
            _ => 0
        };
        return temp_count;
    }

    fn generate_primative<S>(&self, primative: Rc<BramaPrimative>, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        let storage = &options.storages[storage_index];

        let result = storage.get_constant_location(primative);
        match result {
            Some(index) => {
                compiler_info.index = index as i16;
                Ok(())
            },
            _ => Err(("Value not found in storage", 0, 0))
        }
    }

    fn generate_symbol<S>(&self, variable: &String, _: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        compiler_info.index = match options.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
            Some(location) => location as i16,
            _ => return Err(("Variable not found in storage", 0, 0))
        };

        Ok(())
    }

    fn generate_control<S>(&self, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        compiler_info.index = -1;

        self.generate_opcode(left_ast, &BramaAstType::None, compiler_info, options, storage_index)?;

        let left = compiler_info.index;
        compiler_info.index = -1;
        self.generate_opcode(right_ast, &BramaAstType::None, compiler_info, options, storage_index)?;
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

    fn generate_assignment<S>(&self, variable: Rc<String>, operator: &BramaOperatorType, expression_ast: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        let target = options.storages.get_mut(storage_index).unwrap().add_variable(&*variable) as i16;
        compiler_info.index = -1;
        self.generate_opcode(expression_ast, &BramaAstType::None, compiler_info, options, storage_index)?;

        let opcode = match operator {
            BramaOperatorType::Assign               => BramaVmOpCode::Assign               { target: target, expression: compiler_info.index },
            BramaOperatorType::AssignAddition       => BramaVmOpCode::AssignAddition       { target: target, expression: compiler_info.index },
            BramaOperatorType::AssignDivision       => BramaVmOpCode::AssignDivision       { target: target, expression: compiler_info.index },
            BramaOperatorType::AssignMultiplication => BramaVmOpCode::AssignMultiplication { target: target, expression: compiler_info.index },
            BramaOperatorType::AssignSubtraction    => BramaVmOpCode::AssignSubtraction    { target: target, expression: compiler_info.index },
            _ => BramaVmOpCode::None
        };
        
        options.opcodes.push(opcode);
        Ok(())
    }

    fn generate_binary<S>(&self, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        compiler_info.index = -1;

        self.generate_opcode(left_ast, &BramaAstType::None, compiler_info, options, storage_index)?;

        let left = compiler_info.index;
        compiler_info.index = -1;
        self.generate_opcode(right_ast, &BramaAstType::None, compiler_info, options, storage_index)?;
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

    fn generate_block<S>(&self, asts: &Vec<BramaAstType>, upper_ast: &BramaAstType, compiler_info: &mut CompileInfo, options: &mut BramaCompilerOption<S>, storage_index: usize) -> CompilerResult where S: Storage {
        for ast in asts {
            self.generate_opcode(&ast, upper_ast, compiler_info, options, storage_index)?;
            options.storages[storage_index].reset_temp_counter();
        }
        Ok(())
    }
}
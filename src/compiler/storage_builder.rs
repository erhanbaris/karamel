use std::rc::Rc;
use std::cmp::max;

use crate::compiler::Storage;
use crate::compiler::ast::BramaAstType;
use crate::compiler::value::BramaPrimative;
use crate::compiler::BramaCompilerOption;
use crate::types::BramaOperatorType;
pub struct StorageBuilder;
pub struct CompilerOption {
    pub max_stack: u8
}

impl StorageBuilder {
    pub fn new() -> StorageBuilder {
        StorageBuilder {
        }
    }

    pub fn prepare_variable_store(&self, ast: &BramaAstType, options: &mut BramaCompilerOption) {
        let mut compiler_option = CompilerOption { max_stack: 0 };
        self.get_temp_count_from_ast(ast, &BramaAstType::None, options, 0, &mut compiler_option);
        options.storages[0].set_temp_size(compiler_option.max_stack);
        options.storages[0].build();
    }

    fn get_temp_count_from_ast(&self, ast: &BramaAstType, _: &BramaAstType, options: &mut BramaCompilerOption, storage_index: usize, compiler_option: &mut CompilerOption) -> u8 {
        let temp_count = match ast {
            BramaAstType::Binary {
                left,
                operator: _,
                right} => {
                    let total =  self.get_temp_count_from_ast(left, ast, options, storage_index, compiler_option) + self.get_temp_count_from_ast(right, ast, options, storage_index, compiler_option);
                    compiler_option.max_stack = max(total, compiler_option.max_stack);
                    total
                },
            
            BramaAstType::Control {
                left,
                operator: _,
                right} => {
                    let total =  self.get_temp_count_from_ast(left, ast, options, storage_index, compiler_option) + self.get_temp_count_from_ast(right, ast, options, storage_index, compiler_option);
                    compiler_option.max_stack = max(total, compiler_option.max_stack);
                    total
                },
            
            BramaAstType::PrefixUnary(_, inner_ast) => {
                let total = self.get_temp_count_from_ast(inner_ast, ast, options, storage_index, compiler_option);
                compiler_option.max_stack = max(total, compiler_option.max_stack);
                total
            },

            BramaAstType::SuffixUnary(_, inner_ast) => {
                let total = self.get_temp_count_from_ast(inner_ast, ast, options, storage_index, compiler_option);
                compiler_option.max_stack = max(total, compiler_option.max_stack);
                total
            },
            
            BramaAstType::Symbol(string) => {
                options.storages.get_mut(storage_index).unwrap().add_variable(&string);
                compiler_option.max_stack = max(1, compiler_option.max_stack);
                1
            },
            
            BramaAstType::Assignment {
                variable,
                operator,
                expression} =>  {
                options.storages.get_mut(storage_index).unwrap().add_variable(&*variable);
                self.get_temp_count_from_ast(expression, ast, options, storage_index, compiler_option);
                
                let size = match *operator {
                    BramaOperatorType::Assign => 0,
                    _ => 2
                };
                compiler_option.max_stack = max(size, compiler_option.max_stack);
                0
            },
            
            BramaAstType::Block(asts) => {
                let mut list_temp_count = 0;
                for array_item in asts {
                    list_temp_count = max(self.get_temp_count_from_ast(array_item, ast, options, storage_index, compiler_option), list_temp_count);
                }
                compiler_option.max_stack = max(list_temp_count, compiler_option.max_stack);
                list_temp_count
            },

/*
╔══════════════════════╗
║# Function call ast  #║
╚══════════════════════╝
╔══════════════════════╗
║         Arg 1        ║
╠══════════════════════╣
║         Arg 2        ║
╠══════════════════════╣
║         Arg 3        ║
╠══════════════════════╣
║   Function Pointer   ║
╚══════════════════════╝
 */
            BramaAstType::FuncCall{ names, arguments } => {

                /* Need to allocate space for function arguments */
                let mut max_temp = arguments.len() as u8;

                /* Native function call */
                if let Some(function) = options.modules.find_method(&names) {
                    for arg in arguments {
                        max_temp = max(self.get_temp_count_from_ast(arg, ast, options, storage_index, compiler_option), max_temp);
                    }

                    /* Add function pointer to consts */
                    options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(BramaPrimative::FuncNativeCall(function)));
                }

                max_temp /* Variables */
            },

            BramaAstType::Primative(primative) => {
                options.storages.get_mut(storage_index).unwrap().add_constant(Rc::clone(primative));
                /*if let BramaPrimative::List(list) = primative {
                    let mut list_temp_count = 0;
                    for array_item in list {
                        list_temp_count = cmp::max(self.get_temp_count_from_ast(array_item, ast, options, storage_index), list_temp_count);
                    }
                }*/
                compiler_option.max_stack = max(1, compiler_option.max_stack);
                1
            }
            _ => 0
        };
        return temp_count;
    }
}
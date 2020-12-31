use std::rc::Rc;
use std::cmp::max;
use std::cell::Cell;
use std::cell::RefCell;

use crate::compiler::Storage;
use crate::compiler::FunctionInformation;
use crate::compiler::ast::BramaAstType;
use crate::compiler::value::BramaPrimative;
use crate::compiler::value::FunctionReference;
use crate::compiler::BramaCompiler;
use crate::compiler::static_storage::StaticStorage;
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

    pub fn prepare_variable_store(&self, ast: &BramaAstType, options: &mut BramaCompiler) {
        let mut compiler_option = CompilerOption { max_stack: 0 };
        self.get_temp_count_from_ast(ast, &BramaAstType::None, options, 0, &mut compiler_option);
        options.storages[0].set_temp_size(compiler_option.max_stack);
        options.storages[0].set_parent_index(usize::MAX);
        options.storages[0].build();
    }

    fn get_temp_count_from_ast(&self, ast: &BramaAstType, _: &BramaAstType, options: &mut BramaCompiler, storage_index: usize, compiler_option: &mut CompilerOption) -> u8 {
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
                let total = self.get_temp_count_from_ast(inner_ast, ast, options, storage_index, compiler_option) + 1;
                compiler_option.max_stack = max(total, compiler_option.max_stack);
                total
            },
            
            BramaAstType::Symbol(string) => {
                if let Some(function) = options.modules.find_method(&[string.to_string()].to_vec()) {
                    let reference = FunctionReference::native_function(function, string.to_string(), [].to_vec(), "".to_string());
                    options.storages.get_mut(storage_index).unwrap().add_constant(reference);
                } else if let Some(function) = options.storages[storage_index].get_function(string) {
                    let reference = FunctionReference::opcode_function(0, function.name.to_string(), [].to_vec(), "".to_string());
                    options.storages.get_mut(storage_index).unwrap().add_constant(reference);
                } else {
                    options.storages.get_mut(storage_index).unwrap().add_variable(&string);
                    compiler_option.max_stack = max(1, compiler_option.max_stack);
                }
                1
            },
            
            BramaAstType::Assignment {
                variable,
                operator,
                expression} =>  {
                options.storages.get_mut(storage_index).unwrap().add_variable(&*variable);
                
                let stack_size = self.get_temp_count_from_ast(expression, ast, options, storage_index, compiler_option);
                compiler_option.max_stack = max(stack_size, compiler_option.max_stack);
                
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
                    list_temp_count += self.get_temp_count_from_ast(array_item, ast, options, storage_index, compiler_option);
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
            BramaAstType::FuncCall { names, arguments, assign_to_temp } => {

                /* Need to allocate space for function arguments */
                let mut max_temp = 0 as u8;

                /* Native function call */
                if let Some(function) = options.modules.find_method(&names) {
                    /* Add function pointer to consts */
                    let reference = FunctionReference::native_function(function, names[names.len() - 1].to_string(), names[0..(names.len()-1)].to_vec(), "".to_string());
                    options.storages.get_mut(storage_index).unwrap().add_constant(reference);
                
                }

                /* Build arguments */
                for arg in arguments {
                    max_temp += self.get_temp_count_from_ast(arg, ast, options, storage_index, compiler_option);
                }

                compiler_option.max_stack = max(max_temp, compiler_option.max_stack);


                /* Variables */
                if *assign_to_temp {
                    max_temp + 1
                }
                else {
                    max_temp
                }
            },

            BramaAstType::Return(expression) => {
                self.get_temp_count_from_ast(expression, ast, options, storage_index, compiler_option);
                compiler_option.max_stack = max(1, compiler_option.max_stack);
                1
            },

            BramaAstType::EndlessLoop(expression) => {
                self.get_temp_count_from_ast(expression, ast, options, storage_index, compiler_option);
                0
            },

            BramaAstType::WhileLoop { control, body } => {
                self.get_temp_count_from_ast(control, ast, options, storage_index, compiler_option);
                self.get_temp_count_from_ast(body, ast, options, storage_index, compiler_option);
                compiler_option.max_stack = max(1, compiler_option.max_stack);
                1
            },

            BramaAstType::Primative(primative) => {
                options.storages.get_mut(storage_index).unwrap().add_constant(Rc::clone(primative));
                compiler_option.max_stack = max(1, compiler_option.max_stack);
                1
            },

            BramaAstType::List(list) => {
                let mut total_size = 1;
                for array_item in list {
                    total_size += self.get_temp_count_from_ast(&*array_item, ast, options, storage_index, compiler_option);
                }
                compiler_option.max_stack = max(total_size, compiler_option.max_stack);
                return total_size
            },

            BramaAstType::Dict(dict) => {
                let mut total_size = 1;
                for dict_item in dict {
                    options.storages.get_mut(storage_index).unwrap().add_constant(dict_item.key.clone());
                    total_size += self.get_temp_count_from_ast(&dict_item.value, ast, options, storage_index, compiler_option);
                    total_size += 1;
                }
                compiler_option.max_stack = max(total_size, compiler_option.max_stack);
                return total_size
            },

            BramaAstType::Indexer { body, indexer } => {
                let body_size = self.get_temp_count_from_ast(body, ast, options, storage_index, compiler_option);
                let indexer_size = self.get_temp_count_from_ast(indexer, ast, options, storage_index, compiler_option);

                compiler_option.max_stack = max(indexer_size + body_size, compiler_option.max_stack);
                indexer_size + body_size
            },

            BramaAstType::FunctionDefination { name, arguments, body } => {
                /* Create new storage for new function */
                let mut function_compiler_option = CompilerOption { max_stack: 0 };
                let new_storage_index = options.storages.len();
                let function_information = Rc::new(FunctionInformation {
                    name: name.to_string(),
                    opcode_location: Cell::new(0),
                    used_locations: RefCell::new(Vec::new()),
                    storage_index: new_storage_index as u16
                });

                options.storages[storage_index].add_function(name, function_information);
                options.storages.push(StaticStorage::new());

                for argument in arguments {
                    options.storages[new_storage_index].add_variable(argument);
                }
                
                self.get_temp_count_from_ast(body, ast, options, new_storage_index, &mut function_compiler_option);
                
                function_compiler_option.max_stack = max(arguments.len() as u8, function_compiler_option.max_stack);
                options.storages[new_storage_index].set_parent_index(storage_index);
                options.storages[new_storage_index].set_temp_size(function_compiler_option.max_stack as u8);
                options.storages[new_storage_index].build();
                0
            },

            BramaAstType::IfStatement {
                condition, body, else_body, else_if} => {
                    let mut total = self.get_temp_count_from_ast(condition, ast, options, storage_index, compiler_option);
                    total = max(total, self.get_temp_count_from_ast(body, ast, options, storage_index, compiler_option));

                    if let Some(else_) = else_body {
                        total = max(total, self.get_temp_count_from_ast(else_, ast, options, storage_index, compiler_option));
                    }

                    for else_if_item in else_if {
                        total = max(total, self.get_temp_count_from_ast(&else_if_item.condition, ast, options, storage_index, compiler_option));
                        total = max(total, self.get_temp_count_from_ast(&else_if_item.body, ast, options, storage_index, compiler_option));
                    }
                    
                    compiler_option.max_stack = max(0, total);
                    total
                },

                BramaAstType::None => {
                    options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(BramaPrimative::Empty));
                    1
                },
            _ => 0
        };
        return temp_count;
    }
}
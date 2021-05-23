use std::rc::Rc;
use std::cmp::max;

use crate::{compiler::ast::BramaAstType};
use crate::compiler::value::BramaPrimative;
use crate::compiler::function::FunctionReference;
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
                let function_search = options.find_function(string.to_string(), Vec::new(), "".to_string(), storage_index);
                match function_search {
                    Some(reference) => {
                        options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(BramaPrimative::Function(reference, None)));
                    },
                    None => ()
                };

                let class_search = options.find_class(string.to_string(), Vec::new(), "".to_string(), storage_index);
                match class_search {
                    Some(reference) => {
                        options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(BramaPrimative::Class(reference)));
                    },
                    None => ()
                };

                options.storages.get_mut(storage_index).unwrap().add_variable(&string);
                compiler_option.max_stack = max(1, compiler_option.max_stack);
                1
            },

            BramaAstType::FunctionMap(params) => {
                let name = params[params.len() - 1].to_string();
                let module_path = params[0..(params.len() - 1)].to_vec();

                let function_search = options.find_function(name, module_path, "".to_string(), storage_index);
                if let Some(reference) = function_search {
                    options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(BramaPrimative::Function(reference, None)));
                };
                compiler_option.max_stack = max(1, compiler_option.max_stack);
                1
            },
            
            BramaAstType::Assignment {
                variable,
                operator,
                expression} =>  {
                let var_stack_size = self.get_temp_count_from_ast(variable, ast, options, storage_index, compiler_option);                
                let stack_size = self.get_temp_count_from_ast(expression, ast, options, storage_index, compiler_option);
                compiler_option.max_stack = max(stack_size + var_stack_size, compiler_option.max_stack);
                
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
            
            BramaAstType::AccessorFuncCall {
                source,
                indexer,
                assign_to_temp: _
            } => {
                let source_tmp  = self.get_temp_count_from_ast(source, ast, options, storage_index, compiler_option);
                let indexer_tmp = self.get_temp_count_from_ast(indexer, ast, options, storage_index, compiler_option);

                compiler_option.max_stack = max(source_tmp + indexer_tmp, compiler_option.max_stack);
                source_tmp + indexer_tmp
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
            BramaAstType::FuncCall { func_name_expression, arguments, assign_to_temp } => {

                /* Need to allocate space for function arguments */
                let mut max_temp = 0_u8;

                /* Build arguments */
                for arg in arguments {
                    max_temp += self.get_temp_count_from_ast(arg, ast, options, storage_index, compiler_option);
                }

                //compiler_option.max_stack = max(max_temp, compiler_option.max_stack);

                match &**func_name_expression {
                    BramaAstType::Symbol(function_name) => {
                        let function_search = options.find_function(function_name.to_string(), Vec::new(), "".to_string(), storage_index);
                        if let Some(reference) = function_search {
                            options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(BramaPrimative::Function(reference, None)));
                        }
                        else {
                            options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(BramaPrimative::Text(Rc::new(function_name.to_string()))));
                        }
                    },
                    BramaAstType::FunctionMap(names) => {
                        let function_search = options.find_function(names[names.len() - 1].to_string(), names[0..(names.len()-1)].to_vec(), "".to_string(), storage_index);
                        if let Some(reference) = function_search {
                            options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(BramaPrimative::Function(reference, None)));
                        };
                    },
                    _ => {
                        log::debug!("{:?}", func_name_expression);
                        let name_expression_count = self.get_temp_count_from_ast(func_name_expression, ast, options, storage_index, compiler_option);
                        compiler_option.max_stack = max(name_expression_count, compiler_option.max_stack);
                        compiler_option.max_stack += max_temp;
                    }
                };

                /* Variables */
                let size = if *assign_to_temp {
                    max_temp + 2
                }
                else {
                    max_temp + 1
                };
                compiler_option.max_stack = max(size, compiler_option.max_stack);
                size
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
                let function = FunctionReference::opcode_function(name.to_string(), arguments.to_vec(), Vec::new(), "".to_string(), new_storage_index, storage_index);
                
                //options.storages[storage_index].add_constant(function);
                options.add_function(function.clone());
                options.storages.push(StaticStorage::new());
                options.storages[new_storage_index].set_parent_location(storage_index);
                options.storages[storage_index].add_constant(Rc::new(BramaPrimative::Function(function.clone(), None)));

                for argument in arguments {
                    options.storages[new_storage_index].add_variable(argument);
                }
                
                self.get_temp_count_from_ast(body, ast, options, new_storage_index, &mut function_compiler_option);
                
                function_compiler_option.max_stack = max(arguments.len() as u8, function_compiler_option.max_stack);
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
                    
                    compiler_option.max_stack = max(total, compiler_option.max_stack);
                    total
                },

                BramaAstType::None => {
                    options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(BramaPrimative::Empty));
                    compiler_option.max_stack = max(1, compiler_option.max_stack);
                    1
                },
            _ => 0
        };
        return temp_count;
    }
}
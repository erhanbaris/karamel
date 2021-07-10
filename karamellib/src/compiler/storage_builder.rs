use std::rc::Rc;
use std::cmp::max;

use crate::error::KaramelErrorType;
use crate::compiler::ast::KaramelAstType;
use crate::compiler::value::KaramelPrimative;
use crate::compiler::context::KaramelCompilerContext;
use crate::types::KaramelOperatorType;
use crate::syntax::loops::LoopType;

use super::module::OpcodeModule;
pub struct StorageBuilder;
pub struct StorageBuilderOption {
    pub max_stack: u8
}

impl StorageBuilder {
    pub fn new() -> StorageBuilder {
        StorageBuilder {
        }
    }

    pub fn prepare(&self, module: Rc<OpcodeModule>, ast: &KaramelAstType, storage_index: usize, options: &mut KaramelCompilerContext, compiler_options: &mut StorageBuilderOption) -> Result<(), KaramelErrorType> {
        self.get_temp_count_from_ast(module.clone(),ast, &KaramelAstType::None, options, storage_index, compiler_options)?;
        options.storages[storage_index].set_temp_size(compiler_options.max_stack);
        options.storages[storage_index].build();
        Ok(())
    }

    fn get_temp_count_from_ast(&self, module: Rc<OpcodeModule>, ast: &KaramelAstType, _: &KaramelAstType, options: &mut KaramelCompilerContext, storage_index: usize, compiler_option: &mut StorageBuilderOption) -> Result<u8, KaramelErrorType> {
        use crate::buildin::Module;
        
        let temp_count = match ast {
            KaramelAstType::Binary {
                left,
                operator: _,
                right} => {
                    let total =  self.get_temp_count_from_ast(module.clone(),left, ast, options, storage_index, compiler_option)? + self.get_temp_count_from_ast(module.clone(),right, ast, options, storage_index, compiler_option)?;
                    compiler_option.max_stack = max(total, compiler_option.max_stack);
                    total
                },
            
            KaramelAstType::Control {
                left,
                operator: _,
                right} => {
                    let total =  self.get_temp_count_from_ast(module.clone(),left, ast, options, storage_index, compiler_option)? + self.get_temp_count_from_ast(module.clone(),right, ast, options, storage_index, compiler_option)?;
                    compiler_option.max_stack = max(total, compiler_option.max_stack);
                    total
                },
            
            KaramelAstType::PrefixUnary { operator: _, expression, assign_to_temp } => {
                let total = self.get_temp_count_from_ast(module.clone(),expression, ast, options, storage_index, compiler_option)?;
                let total = match assign_to_temp.get() {
                    true => total + 1,
                    false => total
                };
                compiler_option.max_stack = max(total, compiler_option.max_stack);
                total
            },

            KaramelAstType::SuffixUnary(_, expression) => {
                let total = self.get_temp_count_from_ast(module.clone(),expression, ast, options, storage_index, compiler_option)? + 1;
                compiler_option.max_stack = max(total, compiler_option.max_stack);
                total
            },
            
            KaramelAstType::Symbol(string) => {
                match module.get_method(&string[..]) {
                    Some(reference) => {
                        options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(KaramelPrimative::Function(reference, None)));
                    },
                    None => ()
                };

                let function_search = options.get_function(string.to_string(), module.get_path(), storage_index);
                match function_search {
                    Some(reference) => {
                        options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(KaramelPrimative::Function(reference, None)));
                    },
                    None => ()
                };

                let class_search = options.find_class(string.to_string(), module.get_path(), storage_index);
                match class_search {
                    Some(reference) => {
                        options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(KaramelPrimative::Class(reference)));
                    },
                    None => ()
                };

                options.storages.get_mut(storage_index).unwrap().add_variable(&string);
                compiler_option.max_stack = max(1, compiler_option.max_stack);
                1
            },

            KaramelAstType::ModulePath(params) => {
                let name = params[params.len() - 1].to_string();
                let module_path = params[0..(params.len() - 1)].to_vec();

                let function_search = options.get_function(&name, &module_path, storage_index);
                match function_search {
                    Some(reference) => options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(KaramelPrimative::Function(reference, None))),
                    None => return Err(KaramelErrorType::FunctionNotFound(name.to_string()))
                };

                compiler_option.max_stack = max(1, compiler_option.max_stack);
                1
            },
            
            KaramelAstType::Assignment {
                variable,
                operator,
                expression} =>  {
                let var_stack_size = self.get_temp_count_from_ast(module.clone(),variable, ast, options, storage_index, compiler_option)?;                
                let stack_size = self.get_temp_count_from_ast(module.clone(),expression, ast, options, storage_index, compiler_option)?;
                compiler_option.max_stack = max(stack_size + var_stack_size, compiler_option.max_stack);
                
                let size = match *operator {
                    KaramelOperatorType::Assign => 0,
                    _ => 2
                };
                compiler_option.max_stack = max(size, compiler_option.max_stack);
                0
            },
            
            KaramelAstType::Block(asts) => {
                let mut list_temp_count = 0;
                for array_item in asts {
                    list_temp_count += self.get_temp_count_from_ast(module.clone(),array_item, ast, options, storage_index, compiler_option)?;
                }

                compiler_option.max_stack = max(list_temp_count, compiler_option.max_stack);
                list_temp_count
            },
            
            KaramelAstType::AccessorFuncCall {
                source,
                indexer,
                assign_to_temp: _
            } => {
                let source_tmp  = self.get_temp_count_from_ast(module.clone(),source, ast, options, storage_index, compiler_option)?;
                let indexer_tmp = self.get_temp_count_from_ast(module.clone(),indexer, ast, options, storage_index, compiler_option)?;

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
            KaramelAstType::FuncCall { func_name_expression, arguments, assign_to_temp } => {

                /* Need to allocate space for function arguments */
                let mut max_temp = 0_u8;

                /* Build arguments */
                for arg in arguments {
                    max_temp += self.get_temp_count_from_ast(module.clone(),arg, ast, options, storage_index, compiler_option)?;
                }

                //compiler_option.max_stack = max(max_temp, compiler_option.max_stack);

                match &**func_name_expression {
                    KaramelAstType::Symbol(function_name) => {
                        let function_search = options.get_function(function_name.to_string(), module.get_path(), storage_index);
                        if let Some(reference) = function_search {
                            options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(KaramelPrimative::Function(reference, None)));
                        }
                        else {
                            options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(KaramelPrimative::Text(Rc::new(function_name.to_string()))));
                        }
                    },
                    KaramelAstType::ModulePath(names) => {
                        let function_search = options.get_function(names[names.len() - 1].to_string(), &names[0..(names.len()-1)].to_vec(), storage_index);
                        match function_search {
                            Some(reference) => options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(KaramelPrimative::Function(reference, None))),
                            None => return Err(KaramelErrorType::FunctionNotFound(names[names.len() - 1].to_string()))
                        };
                    },
                    _ => {
                        let name_expression_count = self.get_temp_count_from_ast(module.clone(),func_name_expression, ast, options, storage_index, compiler_option)?;
                        compiler_option.max_stack = max(name_expression_count, compiler_option.max_stack);
                        compiler_option.max_stack += max_temp;
                    }
                };

                /* Variables */
                let size = if assign_to_temp.get() {
                    max_temp + 2
                }
                else {
                    max_temp + 1
                };
                compiler_option.max_stack = max(size, compiler_option.max_stack);
                size
            },

            KaramelAstType::Return(expression) => {
                self.get_temp_count_from_ast(module.clone(),expression, ast, options, storage_index, compiler_option)?;
                compiler_option.max_stack = max(1, compiler_option.max_stack);
                1
            },

            KaramelAstType::Loop {
                loop_type,
                body
            } => {
                let mut total = 0;
                match loop_type {
                    LoopType::Scalar { variable, control, increment } => {
                        total = max(total, self.get_temp_count_from_ast(module.clone(),&*variable, ast, options, storage_index, compiler_option)?);
                        total = max(total, self.get_temp_count_from_ast(module.clone(),&*control, ast, options, storage_index, compiler_option)?);
                        total = max(total, self.get_temp_count_from_ast(module.clone(),&*increment, ast, options, storage_index, compiler_option)?);
                    },
                    LoopType::Simple(control) => {
                        total = self.get_temp_count_from_ast(module.clone(),&*control, ast, options, storage_index, compiler_option)?
                    },
                    LoopType::Endless => {}
                };
                total = max(total, self.get_temp_count_from_ast(module.clone(),&*body, ast, options, storage_index, compiler_option)?);
                compiler_option.max_stack = max(total, compiler_option.max_stack);
                total
            },

            KaramelAstType::Primative(primative) => {
                options.storages.get_mut(storage_index).unwrap().add_constant(Rc::clone(primative));
                compiler_option.max_stack = max(1, compiler_option.max_stack);
                1
            },

            KaramelAstType::List(list) => {
                let mut total_size = 1;
                for array_item in list {
                    total_size += self.get_temp_count_from_ast(module.clone(),&*array_item, ast, options, storage_index, compiler_option)?;
                }
                compiler_option.max_stack = max(total_size, compiler_option.max_stack);
                return Ok(total_size)
            },

            KaramelAstType::Dict(dict) => {
                let mut total_size = 1;
                for dict_item in dict {
                    options.storages.get_mut(storage_index).unwrap().add_constant(dict_item.key.clone());
                    total_size += self.get_temp_count_from_ast(module.clone(),&dict_item.value, ast, options, storage_index, compiler_option)?;
                    total_size += 1;
                }
                compiler_option.max_stack = max(total_size, compiler_option.max_stack);
                return Ok(total_size)
            },

            KaramelAstType::Indexer { body, indexer } => {
                let body_size = self.get_temp_count_from_ast(module.clone(),body, ast, options, storage_index, compiler_option)?;
                let indexer_size = self.get_temp_count_from_ast(module.clone(),indexer, ast, options, storage_index, compiler_option)?;

                compiler_option.max_stack = max(indexer_size + body_size, compiler_option.max_stack);
                indexer_size + body_size
            },

            KaramelAstType::FunctionDefination { name: _, arguments, body } => {
                self.get_temp_count_from_ast(module.clone(),body, ast, options, storage_index, compiler_option)?;
                compiler_option.max_stack = max(arguments.len() as u8, compiler_option.max_stack);
                0
            },

            KaramelAstType::IfStatement {
                condition, body, else_body, else_if} => {
                    let mut total = self.get_temp_count_from_ast(module.clone(),condition, ast, options, storage_index, compiler_option)?;
                    total = max(total, self.get_temp_count_from_ast(module.clone(),body, ast, options, storage_index, compiler_option)?);

                    if let Some(else_) = else_body {
                        total = max(total, self.get_temp_count_from_ast(module.clone(),else_, ast, options, storage_index, compiler_option)?);
                    }

                    for else_if_item in else_if {
                        total = max(total, self.get_temp_count_from_ast(module.clone(),&else_if_item.condition, ast, options, storage_index, compiler_option)?);
                        total = max(total, self.get_temp_count_from_ast(module.clone(),&else_if_item.body, ast, options, storage_index, compiler_option)?);
                    }
                    
                    compiler_option.max_stack = max(total, compiler_option.max_stack);
                    total
                },

                KaramelAstType::None => {
                    options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(KaramelPrimative::Empty));
                    compiler_option.max_stack = max(1, compiler_option.max_stack);
                    1
                },
            _ => 0
        };
        return Ok(temp_count);
    }
}
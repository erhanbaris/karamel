use std::rc::Rc;

use crate::error::KaramelErrorType;
use crate::compiler::ast::KaramelAstType;
use crate::compiler::value::KaramelPrimative;
use crate::compiler::context::KaramelCompilerContext;
use crate::types::KaramelOperatorType;
use crate::syntax::loops::LoopType;

use super::module::OpcodeModule;
pub struct StorageBuilder;

impl StorageBuilder {
    pub fn new() -> Self {
        StorageBuilder { }
    }

    pub fn prepare(&self, module: Rc<OpcodeModule>, ast: &KaramelAstType, storage_index: usize, options: &mut KaramelCompilerContext) -> Result<(), KaramelErrorType> {
        self.get_temp_count_from_ast(module.clone(),ast, &KaramelAstType::None, options, storage_index)?;
        Ok(())
    }

    fn get_temp_count_from_ast(&self, module: Rc<OpcodeModule>, ast: &KaramelAstType, _: &KaramelAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> Result<(), KaramelErrorType> {
        use crate::buildin::Module;
        
        match ast {
            KaramelAstType::Binary {
                left,
                operator: _,
                right} => {
                    self.get_temp_count_from_ast(module.clone(),left, ast, options, storage_index)?;
                    self.get_temp_count_from_ast(module.clone(),right, ast, options, storage_index)?;
                },
            
            KaramelAstType::Control {
                left,
                operator: _,
                right} => {
                    self.get_temp_count_from_ast(module.clone(),left, ast, options, storage_index)?;
                    self.get_temp_count_from_ast(module.clone(),right, ast, options, storage_index)?;
                },
            
            KaramelAstType::PrefixUnary { operator: _, expression, assign_to_temp } => {
                self.get_temp_count_from_ast(module.clone(),expression, ast, options, storage_index)?;
            },

            KaramelAstType::SuffixUnary(_, expression) => {
                self.get_temp_count_from_ast(module.clone(),expression, ast, options, storage_index)?;
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
            },

            KaramelAstType::ModulePath(params) => {
                let name = params[params.len() - 1].to_string();
                let module_path = params[0..(params.len() - 1)].to_vec();

                let function_search = options.get_function(&name, &module_path, storage_index);
                match function_search {
                    Some(reference) => options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(KaramelPrimative::Function(reference, None))),
                    None => return Err(KaramelErrorType::FunctionNotFound(name.to_string()))
                };
            },
            
            KaramelAstType::Assignment {
                variable,
                operator,
                expression} =>  {
                let var_stack_size = self.get_temp_count_from_ast(module.clone(),variable, ast, options, storage_index)?;                
                let stack_size = self.get_temp_count_from_ast(module.clone(),expression, ast, options, storage_index)?;
                
                let size = match *operator {
                    KaramelOperatorType::Assign => 0,
                    _ => 2
                };
            },
            
            KaramelAstType::Block(asts) => {
                for array_item in asts {
                    self.get_temp_count_from_ast(module.clone(),array_item, ast, options, storage_index)?;
                }
            },
            
            KaramelAstType::AccessorFuncCall {
                source,
                indexer,
                assign_to_temp: _
            } => {
                self.get_temp_count_from_ast(module.clone(),source, ast, options, storage_index)?;
                self.get_temp_count_from_ast(module.clone(),indexer, ast, options, storage_index)?;
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
                /* Build arguments */
                for arg in arguments {
                    self.get_temp_count_from_ast(module.clone(),arg, ast, options, storage_index)?;
                }

                //compiler_option.max_stack = max(max_temp.max_stack);

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
                        self.get_temp_count_from_ast(module.clone(),func_name_expression, ast, options, storage_index)?;
                    }
                };
            },

            KaramelAstType::Return(expression) => {
                self.get_temp_count_from_ast(module.clone(),expression, ast, options, storage_index)?;
            },

            KaramelAstType::Loop {
                loop_type,
                body
            } => {
                match loop_type {
                    LoopType::Scalar { variable, control, increment } => {
                        self.get_temp_count_from_ast(module.clone(),&*variable, ast, options, storage_index)?;
                        self.get_temp_count_from_ast(module.clone(),&*control, ast, options, storage_index)?;
                        self.get_temp_count_from_ast(module.clone(),&*increment, ast, options, storage_index)?;
                    },
                    LoopType::Simple(control) => {
                        self.get_temp_count_from_ast(module.clone(),&*control, ast, options, storage_index)?
                    },
                    LoopType::Endless => {}
                };
                self.get_temp_count_from_ast(module.clone(),&*body, ast, options, storage_index)?;
            },

            KaramelAstType::Primative(primative) => {
                options.storages.get_mut(storage_index).unwrap().add_constant(Rc::clone(primative));
            },

            KaramelAstType::List(list) => {
                for array_item in list {
                    self.get_temp_count_from_ast(module.clone(),&*array_item, ast, options, storage_index)?;
                }
                return Ok(())
            },

            KaramelAstType::Dict(dict) => {
                for dict_item in dict {
                    options.storages.get_mut(storage_index).unwrap().add_constant(dict_item.key.clone());
                    self.get_temp_count_from_ast(module.clone(),&dict_item.value, ast, options, storage_index)?;
                }
                return Ok(())
            },

            KaramelAstType::Indexer { body, indexer } => {
                self.get_temp_count_from_ast(module.clone(),body, ast, options, storage_index)?;
                self.get_temp_count_from_ast(module.clone(),indexer, ast, options, storage_index)?;
            },

            KaramelAstType::FunctionDefination { name: _, arguments, body } => {
                self.get_temp_count_from_ast(module.clone(),body, ast, options, storage_index)?;
            },

            KaramelAstType::IfStatement {
                condition, body, else_body, else_if} => {
                    self.get_temp_count_from_ast(module.clone(),condition, ast, options, storage_index)?;
                    self.get_temp_count_from_ast(module.clone(),body, ast, options, storage_index)?;

                    if let Some(else_) = else_body {
                        self.get_temp_count_from_ast(module.clone(),else_, ast, options, storage_index)?;
                    }

                    for else_if_item in else_if {
                        self.get_temp_count_from_ast(module.clone(),&else_if_item.condition, ast, options, storage_index)?;
                        self.get_temp_count_from_ast(module.clone(),&else_if_item.body, ast, options, storage_index)?;
                    }
                },

                KaramelAstType::None => {
                    options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(KaramelPrimative::Empty));
                },
            _ => ()
        };
        return Ok(());
    }
}
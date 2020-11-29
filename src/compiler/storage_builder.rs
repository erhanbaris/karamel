use std::{marker::PhantomData, rc::Rc};
use std::cmp::max;

use crate::compiler::Storage;
use crate::compiler::ast::BramaAstType;
use crate::compiler::value::BramaPrimative;
use crate::compiler::BramaCompilerOption;
pub struct StorageBuilder<S> where S: Storage { _marker: PhantomData<S> }

impl<S> StorageBuilder<S> where S: Storage {
    pub fn new() -> StorageBuilder<S> {
        StorageBuilder {
            _marker: PhantomData
        }
    }

    pub fn prepare_variable_store(&self, ast: &BramaAstType, options: &mut BramaCompilerOption<S>) {
        let max_temps = self.get_temp_count_from_ast(ast, &BramaAstType::None, options, 0);
        options.storages[0].set_temp_size(max_temps);
        options.storages[0].build();
    }

    fn get_temp_count_from_ast(&self, ast: &BramaAstType, _: &BramaAstType, options: &mut BramaCompilerOption<S>, storage_index: usize) -> u8 {
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
                        max_temp = max(self.get_temp_count_from_ast(arg, ast, options, storage_index), max_temp);
                    }

                    /* Add function pointer to consts */
                    options.storages.get_mut(storage_index).unwrap().add_constant(Rc::new(BramaPrimative::FuncNativeCall(function)));
                }
                max_temp + 1 /* Variables + function */
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
}
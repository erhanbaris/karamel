use std::borrow::Borrow;
use std::cell::Cell;
use std::vec::Vec;
use std::rc::Rc;
use std::path::PathBuf;
use std::cell::RefCell;

use ast::KaramelDictItem;
use crate::buildin::Module;
use crate::file::read_module_or_script;
use crate::syntax::loops::LoopType;
use crate::types::*;
use crate::error::*;
use crate::compiler::*;
use crate::parser::*;
use crate::syntax::SyntaxParser;
use crate::compiler::value::KaramelPrimative;
use crate::compiler::ast::{KaramelAstType, KaramelIfStatementElseItem};
use crate::compiler::storage_builder::StorageBuilder;
use crate::compiler::function::{FunctionReference};
use crate::buildin::class::PRIMATIVE_CLASS_NAMES;

use log;

use super::context::KaramelCompilerContext;
use super::function::find_function_definition_type;
use super::module::{OpcodeModule, get_modules};
use super::storage_builder::StorageBuilderOption;


pub struct InterpreterCompiler;
impl InterpreterCompiler {   
    pub fn compile(&self, main_ast: Rc<KaramelAstType>, context: &mut KaramelCompilerContext) -> CompilerResult {
        let storage_builder: StorageBuilder = StorageBuilder::new();
        let mut compiler_options = StorageBuilderOption { max_stack: 0 };

        self.add_initial_jump(context);
        
        /* Save all function information */
        let modules = self.detect_modules(main_ast.clone(), context)?;
        let main_module = self.prepare_main_module(main_ast.clone(), context)?;
        //self.prepare_modules(context)?;

        storage_builder.prepare(main_module.clone(), &*main_ast, 0, context, &mut compiler_options)?;

        /* First part of the codes are functions */
        let mut functions = Vec::new();
        for module in modules.iter() {
            self.get_function_definations(module.clone(), module.main_ast.clone(), &mut functions, context, module.storage_index)?;
        }

        self.get_function_definations(main_module.clone(), main_ast.clone(), &mut functions, context, 0)?;

        self.generate_functions(main_module.clone(), &mut functions, context)?;

        /* Prepare jump code for main function */
        let current_location = context.opcodes.len();
        context.opcodes[1] = current_location as u8;
        context.opcodes[2] = (current_location >> 8) as u8;

        /* Generate main function code */
        self.generate_opcode(main_module.clone(), &*main_ast, &KaramelAstType::None, context, 0)?;
        context.opcodes.push(VmOpCode::Halt as u8);
        context.opcodes_ptr = context.opcodes.as_mut_ptr();

        Ok(())
    }

    pub fn add_initial_jump(&self, context: &mut KaramelCompilerContext) {
        /* Jump over all function definations to main function */
        context.opcodes.push(VmOpCode::Jump as u8);
        context.opcodes.push(0_u8);
        context.opcodes.push(0_u8);
    }

    pub fn detect_modules(&self, main_ast: Rc<KaramelAstType>, context: &mut KaramelCompilerContext) -> Result<Vec<Rc<OpcodeModule>>, KaramelErrorType> {
        Ok(get_modules(main_ast.clone(), context)?)
    }

    pub fn prepare_main_module(&self, main_ast: Rc<KaramelAstType>, context: &mut KaramelCompilerContext) -> Result<Rc<OpcodeModule>, KaramelErrorType> {
        let module = OpcodeModule::new("!baz".to_string(), String::new(), main_ast.clone());
        let module = Rc::new(module);
        context.main_module = module.as_ref() as *const OpcodeModule as *mut OpcodeModule;
        context.add_module(module.clone());

        find_function_definition_type(module.clone(), main_ast.clone(), context, 0, true)?;
        Ok(module.clone())
    }
    
    pub fn check_prohibited_names<T: Borrow<String>>(&self, variable: T) -> Result<(), KaramelErrorType> {
        if KEYWORDS.iter().any(|(key, _)| variable.borrow() == *key) {
            return Err(KaramelErrorType::ReservedName(variable.borrow().to_string()));
        }

        if PRIMATIVE_CLASS_NAMES.lock().unwrap().contains(variable.borrow()) {
            return Err(KaramelErrorType::ReservedName(variable.borrow().to_string()));
        }
        
        Ok(())   
    }

    pub fn prepare_modules(&self, context: &mut KaramelCompilerContext) -> CompilerResult {
        let mut functions = Vec::new();

        for (_, module) in context.modules.iter() {
            for function_pointer in module.get_methods() {
                functions.push(function_pointer.clone());
            }
        }

        for reference in functions {
            context.add_function(reference);
        }
        Ok(())
    }

    fn get_function_definations(&self, module: Rc<OpcodeModule>, ast: Rc<KaramelAstType>, functions: &mut Vec<Rc<FunctionReference>>, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult{
        match &*ast {
            KaramelAstType::FunctionDefination { name, arguments: _, body  } => {
                let search = context.get_function(name.to_string(), module.get_path(), storage_index);
                match search {
                    Some(reference) => {
                        functions.push(reference.clone());
                        self.get_function_definations(module.clone(), body.clone(), functions, context, reference.storage_index)?;
                    },

                    None => return Err(KaramelErrorType::FunctionNotFound(name.to_string()))
                };
            },
            KaramelAstType::Block(blocks) => {
                for block in blocks {
                    self.get_function_definations(module.clone(), block.clone(), functions, context, storage_index)?;
                }
            },
            _ => ()
        };

        Ok(())
    }

    fn generate_functions(&self, module: Rc<OpcodeModule>, functions: &mut Vec<Rc<FunctionReference>>, context: &mut KaramelCompilerContext) -> CompilerResult {
        for function in functions {

            /* Validate function name and parameters */
            self.check_prohibited_names(&function.name)?;
            for argument in &function.arguments {
                self.check_prohibited_names(argument)?;
            }

            self.check_prohibited_names(&function.name)?;

            context.opcodes.push(VmOpCode::Func as u8);
            (*function).opcode_location.set(context.opcodes.len());
            context.opcodes.push(function.arguments.len() as u8);

            if !function.arguments.is_empty() {
                context.opcodes.push(VmOpCode::InitArguments as u8);
                context.opcodes.push(function.arguments.len() as u8);
            }

            self.generate_opcode(module.clone(), &function.opcode_body.as_ref().unwrap(), &function.opcode_body.as_ref().unwrap(), context, function.storage_index as usize)?;
        }

        Ok(())
    }

    fn generate_opcode(&self, module: Rc<OpcodeModule>, ast: &KaramelAstType, upper_ast: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        match ast {
            KaramelAstType::Assignment { variable, operator, expression } => self.generate_assignment(module.clone(), variable, operator, expression, context, storage_index),
            KaramelAstType::Symbol(variable) => self.generate_symbol(module.clone(), variable, upper_ast, context, storage_index),
            KaramelAstType::Control { left, operator, right } => self.generate_control(module.clone(), left, operator, right, upper_ast, context, storage_index),
            KaramelAstType::Binary { left, operator, right } => self.generate_binary(module.clone(), left, operator, right, upper_ast, context, storage_index),
            KaramelAstType::Block(asts) => self.generate_block(module.clone(), asts, upper_ast, context, storage_index),
            KaramelAstType::Primative(primative) => self.generate_primative(primative.clone(), upper_ast, context, storage_index),
            KaramelAstType::List(list) => self.generate_list(module.clone(), list, upper_ast, context, storage_index),
            KaramelAstType::Dict(dict) => self.generate_dict(module.clone(), dict, upper_ast, context, storage_index),
            KaramelAstType::FuncCall { func_name_expression, arguments, assign_to_temp } => self.generate_func_call(module.clone(), func_name_expression, arguments, assign_to_temp.get(), upper_ast, context, storage_index),
            KaramelAstType::AccessorFuncCall { source, indexer, assign_to_temp } => self.generate_accessor_func_call(module.clone(), source, indexer, assign_to_temp.get(), upper_ast, context, storage_index),
            KaramelAstType::PrefixUnary { operator, expression, assign_to_temp } => self.generate_prefix_unary(module.clone(), operator, expression, assign_to_temp, upper_ast, context, storage_index),
            KaramelAstType::SuffixUnary(operator, expression) => self.generate_suffix_unary(operator, expression, upper_ast, context, storage_index),
            KaramelAstType::NewLine => Ok(()),
            KaramelAstType::Loop { loop_type, body } => self.generate_loop(module.clone(), loop_type, body, upper_ast, context, storage_index),
            KaramelAstType::Break => self.generate_break(upper_ast, context, storage_index),
            KaramelAstType::Continue => self.generate_continue(upper_ast, context, storage_index),
            KaramelAstType::Return(expression) => self.generate_return(module.clone(), expression, upper_ast, context, storage_index),
            KaramelAstType::IfStatement {condition, body, else_body, else_if} => self.generate_if_condition(module.clone(),condition, body, else_body, else_if, upper_ast, context, storage_index),
            KaramelAstType::Indexer {body, indexer} => self.generate_indexer(module.clone(), body, indexer, upper_ast, context, storage_index),
            KaramelAstType::None => self.generate_none(context, storage_index),
            KaramelAstType::FunctionDefination{name: _, arguments: _, body: _} => Ok(()),
            KaramelAstType::ModulePath(name) => self.generate_function_map(name, context, storage_index),
            KaramelAstType::Load(names) => self.generate_load_module(names, context),
        }
    }

    fn generate_primative(&self, primative: Rc<KaramelPrimative>, _: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        let storage = &context.storages[storage_index];

        let result = storage.get_constant_location(primative);
        match result {
            Some(index) => {
                context.opcodes.push(VmOpCode::Load as u8);
                context.opcodes.push(index as u8);
                Ok(())
            },
            _ => Err(KaramelErrorType::ValueNotFoundInStorage)
        }
    }

    fn generate_load_module(&self, params: &[String], context: &mut KaramelCompilerContext) -> CompilerResult {
        let mut path = PathBuf::from(&context.execution_path.path[..]);
        let module = &params[(params.len() - 1)];

        for item in params.iter().take(params.len() - 1) {
            path.push(item);
        }

        path.push(module);
        match read_module_or_script(path.to_str().unwrap(), context) {
            Ok(content) => {
                let mut parser = Parser::new(&content);
                match parser.parse() {
                    Err(error) => return Err(KaramelErrorType::from(error)),
                    _ => ()
                };
    
                let syntax = SyntaxParser::new(parser.tokens().to_vec());
                match syntax.parse() {
                    Ok(ast) => ast,
                    Err(error) => return Err(KaramelErrorType::from(error))
                };
                Ok(())
            },
            Err(error) => Err(KaramelErrorType::ModuleParseError{
                name: "<Bilinmeyen>".to_string(),
                error: error.to_string()
            })
        }
    }

    fn generate_function_map(&self, params: &[String], context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        let storage = &context.storages[storage_index];

        let name = params[params.len() - 1].to_string();
        let module_path = params[0..(params.len() - 1)].to_vec();

        let function_search = context.get_function(&name, &module_path, storage_index);
        match function_search {
            Some(reference) => {
                let result = storage.get_constant_location(Rc::new(KaramelPrimative::Function(reference, None)));
                match result {
                    Some(index) => {
                        context.opcodes.push(VmOpCode::Load as u8);
                        context.opcodes.push(index as u8);
                        Ok(())
                    },
                    _ => Err(KaramelErrorType::FunctionNotFoundInStorage(name.to_string()))
                }
            },
            None => Err(KaramelErrorType::FunctionNotFoundInStorage(name.to_string()))
        }
    }

    fn generate_none(&self, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        let storage = &context.storages[storage_index];

        let result = storage.get_constant_location(Rc::new(KaramelPrimative::Empty));
        match result {
            Some(index) => {
                context.opcodes.push(VmOpCode::Load as u8);
                context.opcodes.push(index as u8);
                Ok(())
            },
            _ => Err(KaramelErrorType::ValueNotFoundInStorage)
        }
    }

    fn generate_list(&self, module: Rc<OpcodeModule>, list: &Vec<Rc<KaramelAstType>>, upper_ast: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        for item in list.iter().rev() {
            self.generate_opcode(module.clone(), item, upper_ast, context, storage_index)?;
        }
        context.opcodes.push(VmOpCode::InitList as u8);
        context.opcodes.push(list.len() as u8);
        Ok(())
    }

    fn generate_dict(&self, module: Rc<OpcodeModule>, dict: &Vec<Rc<KaramelDictItem>>, upper_ast: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        for item in dict.iter().rev() {
            self.generate_primative(item.key.clone(), upper_ast, context, storage_index)?;
            self.generate_opcode(module.clone(), &item.value, upper_ast, context, storage_index)?;
        }
        context.opcodes.push(VmOpCode::InitDict as u8);
        context.opcodes.push(dict.len() as u8);
        Ok(())
    }

    fn generate_func_call_by_name(&self, name :&String, module_path: &Vec<String>, arguments: &Vec<Rc<KaramelAstType>>, assign_to_temp: bool, context: &mut KaramelCompilerContext, storage_index: usize) -> Result<bool, KaramelErrorType> {
        let function_search = context.get_function(name.to_string(), module_path, storage_index);

        match function_search {
            Some(function_ref) => {
                let search_location = context.storages[storage_index].get_constant_location(Rc::new(KaramelPrimative::Function(function_ref.clone(), None)));
                match search_location {
                    Some(location) => {
                        context.opcodes.push(VmOpCode::Call as u8);
                        context.opcodes.push(location as u8);
                        context.opcodes.push(arguments.len() as u8);
                        context.opcodes.push(assign_to_temp as u8);
                        return Ok(true);
                    },
                    _ => return Err(KaramelErrorType::FunctionNotFound(function_ref.name.to_string()))
                }
            },
            _ => ()
        };

        match context.storages[storage_index].get_variable_location(&name) {
            /* Variable found */
            Some(location) => {
                context.opcodes.push(VmOpCode::Load as u8);
                context.opcodes.push(location as u8);

                context.opcodes.push(VmOpCode::CallStack as u8);
                context.opcodes.push(arguments.len() as u8);
                context.opcodes.push(true as u8);
                return Ok(true);
            },
            /* Variable not found, lets check for function */
            None => ()
        };

        Ok(false)
    }

    fn generate_accessor_func_call(&self, module: Rc<OpcodeModule>, source: &KaramelAstType, indexer: &KaramelAstType, _assign_to_temp: bool,  upper_ast: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {

        if let KaramelAstType::FuncCall { func_name_expression, arguments, assign_to_temp: _ } = indexer {
            match &**func_name_expression {
                KaramelAstType::Symbol(function_name) => {
                            /* Build arguments */
                    for argument in arguments {
                        self.generate_opcode(module.clone(), argument, upper_ast, context, storage_index)?;
                    }
                    
                    self.generate_opcode(module.clone(), source, &KaramelAstType::None, context, storage_index)?;
                    //todo: Pass real object to function as a parameter.
                    
                    let search_location = context.storages[storage_index].get_constant_location(Rc::new(KaramelPrimative::Text(Rc::new(function_name.to_string()))));
                    match search_location {
                        Some(location) => {
                            context.opcodes.push(VmOpCode::Load as u8);
                            context.opcodes.push(location as u8);
                            context.opcodes.push(VmOpCode::GetItem as u8);
                            
                            context.opcodes.push(VmOpCode::CallStack as u8);
                            context.opcodes.push(arguments.len() as u8);
                            context.opcodes.push(true as u8);
                            /*context.opcodes.push(arguments.len() as u8);
                            context.opcodes.push(assign_to_temp as u8);*/
                            return Ok(());
                        },
                        _ => return Err(KaramelErrorType::FunctionNotFound(function_name.to_string()))
                    }
                },
                
                KaramelAstType::FuncCall {
                    func_name_expression,
                    arguments,
                    assign_to_temp: _
                }=> {
                    return self.generate_func_call(module.clone(), func_name_expression, arguments, true, upper_ast, context, storage_index);
                },
                _ => {
                    return Err(KaramelErrorType::FunctionNotFound("<Bilinmeyen>".to_string()));
                }
            }   
        }
        else {
            return Err(KaramelErrorType::FunctionNotFound("<Bilinmeyen>".to_string()));
        }
    }

    fn generate_func_call(&self, module: Rc<OpcodeModule>, func_name_expression: &KaramelAstType, arguments: &Vec<Rc<KaramelAstType>>, assign_to_temp: bool,  upper_ast: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        /* Build arguments */
        for argument in arguments {
            self.generate_opcode(module.clone(), argument, upper_ast, context, storage_index)?;
        }

        match &func_name_expression {
            KaramelAstType::Symbol(function_name) => {
                let result = self.generate_func_call_by_name(&function_name, module.get_path(), &arguments, assign_to_temp, context, storage_index)?;
                match result {
                    true => return Ok(()),
                    false => {
                        log::debug!("{:?}", function_name);
                        return Err(KaramelErrorType::FunctionNotFound(function_name.to_string()));
                    }
                }
            },

            KaramelAstType::FuncCall {func_name_expression, arguments: inner_arguments, assign_to_temp: _} => {
                self.generate_func_call(module.clone(), func_name_expression, inner_arguments, true, upper_ast, context, storage_index)?;
                context.opcodes.push(VmOpCode::CallStack as u8);
                context.opcodes.push(arguments.len() as u8);
                context.opcodes.push(true as u8);

                return Ok(());
            },

            KaramelAstType::ModulePath(names) => {
                let result = self.generate_func_call_by_name(&names[names.len() - 1].to_string(), &names[0..(names.len()-1)].to_vec(), &arguments, assign_to_temp, context, storage_index)?;
                match result {
                    true => return Ok(()),
                    false =>  return Err(KaramelErrorType::FunctionNotFound(names[names.len() - 1].to_string()))
                }
            },
            _ => {
                self.generate_opcode(module.clone(), func_name_expression, upper_ast, context, storage_index)?;
                context.opcodes.push(VmOpCode::CallStack as u8);
                context.opcodes.push(arguments.len() as u8);
                context.opcodes.push(true as u8);
                return Ok(());
            }
        }
    }

    fn generate_break(&self, _: &KaramelAstType, context: &mut KaramelCompilerContext, _: usize) -> CompilerResult {       
        context.opcodes.push(VmOpCode::Jump as u8);
        context.loop_breaks.push(context.opcodes.len());
        context.opcodes.push(0);
        context.opcodes.push(0);
        Ok(())
    }

    fn generate_continue(&self, _: &KaramelAstType, context: &mut KaramelCompilerContext, _: usize) -> CompilerResult {       
        context.opcodes.push(VmOpCode::Jump as u8);
        context.loop_continues.push(context.opcodes.len());
        context.opcodes.push(0);
        context.opcodes.push(0);
        Ok(())
    }

    fn generate_return(&self, module: Rc<OpcodeModule>, expression: &KaramelAstType, upper_ast: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        self.generate_opcode(module.clone(), expression, upper_ast, context, storage_index)?;
        context.opcodes.push(VmOpCode::Return as u8);
        Ok(())
    }

    fn generate_loop(&self, module: Rc<OpcodeModule>, loop_type: &LoopType, body: &KaramelAstType, upper_ast: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        /* Backup loop informations */
        let loop_breaks = context.loop_breaks.to_vec();
        let loop_continues = context.loop_continues.to_vec();
        let mut compare_location = 0;

        let (variable, control, increment) = match loop_type {
            LoopType::Endless => {
                (None, None, None)
            },

            LoopType::Simple(control) => {
                (None, Some(control.clone()), None)
            },

            LoopType::Scalar {
                variable, 
                control,
                increment
            } => {
                (Some(variable.clone()), Some(control.clone()), Some(increment.clone()))
            }
        };

        if let Some(variable) = &variable {
            self.generate_opcode(module.clone(), &*&variable, upper_ast, context, storage_index)?;
        }

        let start_location = context.opcodes.len();
        
        if let Some(control) = &control {
            self.generate_opcode(module.clone(), &*control, upper_ast, context, storage_index)?;
            context.opcodes.push(VmOpCode::Compare as u8);
            compare_location = context.opcodes.len();

            context.opcodes.push(0_u8);
            context.opcodes.push(0_u8);
        }

        self.generate_opcode(module.clone(), body, upper_ast, context, storage_index)?;

        if let Some(increment) = &increment {
            self.generate_opcode(module.clone(), &*&increment, upper_ast, context, storage_index)?;
        }

        context.opcodes.push(VmOpCode::Jump as u8);
        context.opcodes.push(start_location as u8);
        context.opcodes.push((start_location >> 8) as u8);

        let current_location = context.opcodes.len(); 

        for break_info in &context.loop_breaks {
            context.opcodes[*break_info]     = current_location as u8;
            context.opcodes[*break_info + 1] = (current_location >> 8) as u8;
        } 

        for continue_info in &context.loop_continues {
            context.opcodes[*continue_info]     = start_location as u8;
            context.opcodes[*continue_info + 1] = (start_location >> 8) as u8;
        } 

        context.loop_breaks    = loop_breaks.to_vec();
        context.loop_continues = loop_continues.to_vec();

        if control.is_some() {
            let end_location = current_location - compare_location;
            context.opcodes[compare_location]     = end_location as u8;
            context.opcodes[compare_location + 1] = (end_location >> 8) as u8;
        }

        Ok(())
    }

    fn generate_symbol(&self, module: Rc<OpcodeModule>, variable: &String, _: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        let storage = &context.storages[storage_index];                
        let result = storage.get_function_constant(variable.to_string(), module.clone());
        match result {
            Some(index) => {
                context.opcodes.push(VmOpCode::Load as u8);
                context.opcodes.push(index as u8);
                return Ok(())
            },
            _ => ()
        };

        let result = storage.get_class_constant(variable.to_string(), module.clone());
        match result {
            Some(index) => {
                context.opcodes.push(VmOpCode::Load as u8);
                context.opcodes.push(index as u8);
                return Ok(())
            },
            _ => ()
        };

        match storage.get_variable_location(variable) {
            /* Variable found */
            Some(location) => {
                context.opcodes.push(VmOpCode::Load as u8);
                context.opcodes.push(location as u8);
                Ok(())
            },
            /* Variable not found, lets check for function */
            None => return Err(KaramelErrorType::ValueNotFoundInStorage)
        }
    }

    fn generate_control(&self, module: Rc<OpcodeModule>, left_ast: &KaramelAstType, operator: &KaramelOperatorType, right_ast: &KaramelAstType, _: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        self.generate_opcode(module.clone(), left_ast, &KaramelAstType::None, context, storage_index)?;
        self.generate_opcode(module.clone(), right_ast, &KaramelAstType::None, context, storage_index)?;

        let opcode = match operator {
            KaramelOperatorType::Or               => VmOpCode::Or as u8,
            KaramelOperatorType::And              => VmOpCode::And as u8,
            KaramelOperatorType::Equal            => VmOpCode::Equal as u8,
            KaramelOperatorType::NotEqual         => VmOpCode::NotEqual as u8,
            KaramelOperatorType::GreaterThan      => VmOpCode::GreaterThan as u8,
            KaramelOperatorType::LessThan         => VmOpCode::LessThan as u8,
            KaramelOperatorType::GreaterEqualThan => VmOpCode::GreaterEqualThan as u8,
            KaramelOperatorType::LessEqualThan    => VmOpCode::LessEqualThan as u8,
            _ => VmOpCode::None as u8
        };

        context.opcodes.push(opcode);
        Ok(())
    }

    fn generate_assignment(&self, module: Rc<OpcodeModule>, variable: &KaramelAstType, operator: &KaramelOperatorType, expression_ast: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        match variable {
            KaramelAstType::Symbol(symbol) => {
                
                /* Validate function name and parameters */
                if let KaramelAstType::Symbol(variable_name) = variable {
                    self.check_prohibited_names(variable_name)?;    
                }
                
                let location = context.storages.get_mut(storage_index).unwrap().add_variable(&*symbol);
                let storage = &context.storages[storage_index];
                
                if let KaramelAstType::Primative(primative) = expression_ast {
                    if mem::discriminant(&**primative) != mem::discriminant(&KaramelPrimative::List(RefCell::new(Vec::new()))) && 
                    *operator == KaramelOperatorType::Assign {
                        let result = storage.get_constant_location(primative.clone());
                        let primative_location = match result {
                            Some(index) => index as u8,
                            _ => return Err(KaramelErrorType::ValueNotFoundInStorage)
                        };

                        context.opcodes.push(VmOpCode::FastStore as u8);
                        context.opcodes.push(location);
                        context.opcodes.push(primative_location);
                        return Ok(());
                    }
                }

                if *operator != KaramelOperatorType::Assign {

                    /* Load variable data to stack */
                    context.opcodes.push(VmOpCode::Load as u8);
                    context.opcodes.push(location);

                    self.generate_opcode(module.clone(), expression_ast, &KaramelAstType::None, context, storage_index)?;

                    let opcode = match operator {
                        KaramelOperatorType::AssignAddition       => VmOpCode::Addition as u8,
                        KaramelOperatorType::AssignDivision       => VmOpCode::Division as u8,
                        KaramelOperatorType::AssignMultiplication => VmOpCode::Multiply as u8,
                        KaramelOperatorType::AssignSubtraction    => VmOpCode::Subraction as u8,
                        _ => KaramelOperatorType::None as u8
                    };

                    context.opcodes.push(opcode);
                } else {
                    self.generate_opcode(module.clone(), expression_ast, &KaramelAstType::None, context, storage_index)?;
                }
                
                context.opcodes.push(VmOpCode::Store as u8);
                context.opcodes.push(location);
                Ok(())
            },

            KaramelAstType::Indexer {body, indexer} => {
                self.generate_opcode(module.clone(), body, &KaramelAstType::None, context, storage_index)?;
                self.generate_opcode(module.clone(), indexer, &KaramelAstType::None, context, storage_index)?;

                self.generate_opcode(module.clone(), expression_ast, &KaramelAstType::None, context, storage_index)?;
                
                context.opcodes.push(VmOpCode::SetItem as u8);
                Ok(())
            },
            _ => Ok(())
        }
    }

    fn generate_binary(&self, module: Rc<OpcodeModule>, left_ast: &KaramelAstType, operator: &KaramelOperatorType, right_ast: &KaramelAstType, _: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult { 
        self.generate_opcode(module.clone(), left_ast, &KaramelAstType::None, context, storage_index)?;
        self.generate_opcode(module.clone(), right_ast, &KaramelAstType::None, context, storage_index)?;
        let opcode = match operator {
            KaramelOperatorType::Addition       => VmOpCode::Addition as u8,
            KaramelOperatorType::Subtraction    => VmOpCode::Subraction as u8,
            KaramelOperatorType::Multiplication => VmOpCode::Multiply as u8,
            KaramelOperatorType::Division       => VmOpCode::Division as u8,
            KaramelOperatorType::Modulo         => VmOpCode::Module as u8,
            _ => VmOpCode::None as u8
        };

        context.opcodes.push(opcode);
        Ok(())
    }

    fn generate_prefix_unary(&self, module: Rc<OpcodeModule>, operator: &KaramelOperatorType, expression: &KaramelAstType, assign_to_temp: &Cell<bool>, _: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult { 
        
        if *operator == KaramelOperatorType::Not { 
            return self.generate_not(module.clone(), expression, context, storage_index);
        }

        if let KaramelAstType::Symbol(variable) = expression {
            let location = match context.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
                Some(location) => location,
                _ => return Err(KaramelErrorType::ValueNotFoundInStorage)
            };

            /* Load data from memory */
            context.opcodes.push(VmOpCode::Load as u8);
            context.opcodes.push(location);
        
            let opcode = match operator {
                KaramelOperatorType::Increment  => VmOpCode::Increment as u8,
                KaramelOperatorType::Deccrement => VmOpCode::Decrement as u8,
                _ => return Err(KaramelErrorType::UnaryOperatorNotFound)
            };
    
            context.opcodes.push(opcode);

            // Keep value at the stack if assign_to_temp is true
            match assign_to_temp.get() {
                true => context.opcodes.push(VmOpCode::CopyToStore as u8),
                false => context.opcodes.push(VmOpCode::Store as u8),
            }
            
            context.opcodes.push(location);
            return Ok(());
        }

        Err(KaramelErrorType::UnaryExpressionNotValid)
    }

    fn generate_not(&self, module: Rc<OpcodeModule>, expression: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult { 
        self.generate_opcode(module.clone(), expression, &KaramelAstType::None, context, storage_index)?;
        context.opcodes.push(VmOpCode::Not as u8);
        Ok(())
    }

    fn create_exit_jump(&self, context: &mut KaramelCompilerContext, exit_locations: &mut Vec<usize>) {
        context.opcodes.push(VmOpCode::Jump as u8);
        exit_locations.push(context.opcodes.len());
        context.opcodes.push(0_u8);
        context.opcodes.push(0_u8);
    }

    fn create_compare(&self, context: &mut KaramelCompilerContext) -> usize {
        context.opcodes.push(VmOpCode::Compare as u8);
        let compare_location = context.opcodes.len();

        context.opcodes.push(0_u8);
        context.opcodes.push(0_u8);

        compare_location
    }

    fn build_jump_location(&self, context: &mut KaramelCompilerContext, jump_location: usize) {
        let current_location = context.opcodes.len();
        context.opcodes[jump_location]     = current_location as u8;
        context.opcodes[jump_location + 1] = (current_location >> 8) as u8;
    }

    fn build_compare_location(&self, context: &mut KaramelCompilerContext, jump_location: usize) {
        let current_location = context.opcodes.len() - jump_location;
        context.opcodes[jump_location]     = current_location as u8;
        context.opcodes[jump_location + 1] = (current_location >> 8) as u8;
    }

    fn generate_if_condition(&self, module: Rc<OpcodeModule>, condition: &KaramelAstType, body: &KaramelAstType, else_body: &Option<Rc<KaramelAstType>>, else_if: &Vec<Rc<KaramelIfStatementElseItem>>, upper_ast: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        /*
        ╔════════════════════╗
        ║   IF CONDITION     ║
        ╠════════════════════╣
        ║   JUMP TO NEXT     ║
        ║   CASE LOCATION    ║
        ╠════════════════════╣
        ║   TRUE BODY        ║
        ╠════════════════════╣
        ║   JUMP TO OUT OF   ║
        ║   IF CONDITION     ║
        ╠════════════════════╣
        ║   IF CONDITION     ║
        ╠════════════════════╣
        ║   IF CONDITION     ║
        ╚════════════════════╝
        */
        let mut exit_locations: Vec<usize> = Vec::new();
        
        self.generate_opcode(module.clone(), condition, upper_ast, context, storage_index)?;
        let mut if_failed_location = self.create_compare(context);
        self.generate_opcode(module.clone(), body, upper_ast, context, storage_index)?;

        if !else_if.is_empty() || else_body.is_some() {
            /* After executing body, need to exit from 'if condition'.
               Jump to out of if condition */
            self.create_exit_jump(context, &mut exit_locations);
        }

        for else_if_item in else_if {
            /* Previous conditon should jump to this location */
            self.build_compare_location(context, if_failed_location);

            /* Build condition */
            self.generate_opcode(module.clone(), &else_if_item.condition, upper_ast, context, storage_index)?;

            if_failed_location = self.create_compare(context);

            self.generate_opcode(module.clone(), &else_if_item.body, upper_ast, context, storage_index)?;

            /* Jump to out of if condition */
            self.create_exit_jump(context, &mut exit_locations);
        }

        if let Some(_else_body) = else_body {
            self.build_compare_location(context, if_failed_location);
            self.generate_opcode(module.clone(), _else_body, upper_ast, context, storage_index)?;
        }
        else {
            self.build_compare_location(context, if_failed_location);
        }

        for exit_location in exit_locations {
            self.build_jump_location(context, exit_location);
        }

        Ok(())
    }

    fn generate_indexer(&self, module: Rc<OpcodeModule>, body: &KaramelAstType, indexer: &KaramelAstType, upper_ast: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        self.generate_opcode(module.clone(), body, upper_ast, context, storage_index)?;
        self.generate_opcode(module.clone(), indexer, upper_ast, context, storage_index)?;
        context.opcodes.push(VmOpCode::GetItem as u8);

        Ok(())
    }

    fn generate_suffix_unary(&self, operator: &KaramelOperatorType, expression: &KaramelAstType, _: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult { 
        if let KaramelAstType::Symbol(variable) = expression {
            let location = match context.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
                Some(location) => location,
                _ => return Err(KaramelErrorType::ValueNotFoundInStorage)
            };

            context.opcodes.push(VmOpCode::Load as u8);
            context.opcodes.push(location);
            context.opcodes.push(VmOpCode::Dublicate as u8);

            let opcode = match operator {
                KaramelOperatorType::Increment  => VmOpCode::Increment as u8,
                KaramelOperatorType::Deccrement => VmOpCode::Decrement as u8,
                KaramelOperatorType::Not        => VmOpCode::Not as u8,
                _ => return Err(KaramelErrorType::UnaryOperatorNotFound)
            };
    
            context.opcodes.push(opcode);
            context.opcodes.push(VmOpCode::Store as u8);
            context.opcodes.push(location);
            return Ok(());
        }

        Err(KaramelErrorType::UnaryExpressionNotValid)
    }

    fn generate_block(&self, module: Rc<OpcodeModule>, asts: &[Rc<KaramelAstType>], upper_ast: &KaramelAstType, context: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        for ast in asts {
            self.generate_opcode(module.clone(), &ast, upper_ast, context, storage_index)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::buildin::DummyModule;
    use crate::compiler::*;
    use std::vec::Vec;
    use std::rc::Rc;
    use crate::compiler::ast::{KaramelAstType};
    use crate::compiler::storage_builder::StorageBuilder;
    use crate::compiler::function::{FunctionReference};
    use super::module::{OpcodeModule};

    use super::storage_builder::StorageBuilderOption;
    use crate::error::KaramelErrorType;

    #[test]
    fn test_1() -> Result<(), KaramelErrorType> {
        let mut context = KaramelCompilerContext::new();
        let compiler = InterpreterCompiler {};
        let storage_builder: StorageBuilder = StorageBuilder::new();
        let mut compiler_options = StorageBuilderOption { max_stack: 0 };

        let function_define = FunctionReference::opcode_function("test".to_string(), Vec::new(), Rc::new(KaramelAstType::None), Rc::new(DummyModule::new()), 0, 0, true);

        let mut functions = Vec::new();
        functions.push(function_define);

        let module = Rc::new(OpcodeModule::new("".to_string(), "".to_string(), Rc::new(KaramelAstType::None)));
        storage_builder.prepare(module.clone(), &KaramelAstType::None, 0, &mut context, &mut compiler_options)?;
        compiler.generate_functions(module.clone(), &mut functions, &mut context)?;
        Ok(())
    }

    #[test]
    fn test_2() -> Result<(), KaramelErrorType> {
        let mut context = KaramelCompilerContext::new();
        let compiler = InterpreterCompiler {};
        let storage_builder: StorageBuilder = StorageBuilder::new();
        let mut compiler_options = StorageBuilderOption { max_stack: 0 };

        let function_define = FunctionReference::opcode_function("yazı".to_string(), Vec::new(), Rc::new(KaramelAstType::None), Rc::new(DummyModule::new()), 0, 0, true);

        let mut functions = Vec::new();
        functions.push(function_define);

        let module = Rc::new(OpcodeModule::new("".to_string(), "".to_string(), Rc::new(KaramelAstType::None)));
        storage_builder.prepare(module.clone(), &KaramelAstType::None, 0, &mut context, &mut compiler_options)?;
        let result = compiler.generate_functions(module.clone(), &mut functions, &mut context);

        match result {
            Ok(_) => Err(KaramelErrorType::GeneralError("yazı geçerli bir fonksiyon ismi değil".to_string())),
            Err(_) => Ok(())
        }
    }

    #[test]
    fn test_3() -> Result<(), KaramelErrorType> {
        let mut context = KaramelCompilerContext::new();
        let compiler = InterpreterCompiler {};
        let storage_builder: StorageBuilder = StorageBuilder::new();
        let mut compiler_options = StorageBuilderOption { max_stack: 0 };

        let function_define = FunctionReference::opcode_function("döndür".to_string(), Vec::new(), Rc::new(KaramelAstType::None), Rc::new(DummyModule::new()), 0, 0, true);

        let mut functions = Vec::new();
        functions.push(function_define);

        let module = Rc::new(OpcodeModule::new("".to_string(), "".to_string(), Rc::new(KaramelAstType::None)));
        storage_builder.prepare(module.clone(), &KaramelAstType::None, 0, &mut context, &mut compiler_options)?;
        let result = compiler.generate_functions(module.clone(), &mut functions, &mut context);

        match result {
            Ok(_) => Err(KaramelErrorType::GeneralError("yazı geçerli bir fonksiyon ismi değil".to_string())),
            Err(_) => Ok(())
        }
    }

    #[test]
    fn test_4() -> Result<(), KaramelErrorType> {
        let mut context = KaramelCompilerContext::new();
        let compiler = InterpreterCompiler {};
        let storage_builder: StorageBuilder = StorageBuilder::new();
        let mut compiler_options = StorageBuilderOption { max_stack: 0 };

        let function_define = FunctionReference::opcode_function("sayı".to_string(), Vec::new(), Rc::new(KaramelAstType::None), Rc::new(DummyModule::new()), 0, 0, true);

        let mut functions = Vec::new();
        functions.push(function_define);

        let module = Rc::new(OpcodeModule::new("".to_string(), "".to_string(), Rc::new(KaramelAstType::None)));
        storage_builder.prepare(module.clone(), &KaramelAstType::None, 0, &mut context, &mut compiler_options)?;
        let result = compiler.generate_functions(module.clone(), &mut functions, &mut context);

        match result {
            Ok(_) => Err(KaramelErrorType::GeneralError("sayı geçerli bir fonksiyon ismi değil".to_string())),
            Err(_) => Ok(())
        }
    }

    #[test]
    fn test_5() -> Result<(), KaramelErrorType> {
        
        let mut context = KaramelCompilerContext::new();
        let compiler = InterpreterCompiler {};
        let storage_builder: StorageBuilder = StorageBuilder::new();
        let mut compiler_options = StorageBuilderOption { max_stack: 0 };

        let function_define = FunctionReference::opcode_function("test".to_string(), vec!["test".to_string()], Rc::new(KaramelAstType::None), Rc::new(DummyModule::new()), 0, 0, true);

        let mut functions = Vec::new();
        functions.push(function_define);

        let module = Rc::new(OpcodeModule::new("".to_string(), "".to_string(), Rc::new(KaramelAstType::None)));
        storage_builder.prepare(module.clone(), &KaramelAstType::None, 0, &mut context, &mut compiler_options)?;
        compiler.generate_functions(module.clone(), &mut functions, &mut context)?;
        Ok(())
    }

    #[test]
    fn test_6() -> Result<(), KaramelErrorType> {
        let mut context = KaramelCompilerContext::new();
        let compiler = InterpreterCompiler {};
        let storage_builder: StorageBuilder = StorageBuilder::new();
        let mut compiler_options = StorageBuilderOption { max_stack: 0 };

        let function_define = FunctionReference::opcode_function("test".to_string(), vec!["sayı".to_string()], Rc::new(KaramelAstType::None), Rc::new(DummyModule::new()), 0, 0, true);

        let mut functions = Vec::new();
        functions.push(function_define);

        let module = Rc::new(OpcodeModule::new("".to_string(), "".to_string(), Rc::new(KaramelAstType::None)));
        storage_builder.prepare(module.clone(), &KaramelAstType::None, 0, &mut context, &mut compiler_options)?;
        let result = compiler.generate_functions(module.clone(), &mut functions, &mut context);

        match result {
            Ok(_) => Err(KaramelErrorType::GeneralError("sayı geçerli bir fonksiyon parametresi değil".to_string())),
            Err(_) => Ok(())
        }
    }

    #[test]
    fn test_7() -> Result<(), KaramelErrorType> {
        let mut context = KaramelCompilerContext::new();
        let compiler = InterpreterCompiler {};
        let storage_builder: StorageBuilder = StorageBuilder::new();
        let mut compiler_options = StorageBuilderOption { max_stack: 0 };

        let function_define = FunctionReference::opcode_function("döndür".to_string(), vec!["sayı".to_string()], Rc::new(KaramelAstType::None), Rc::new(DummyModule::new()), 0, 0, true);

        let mut functions = Vec::new();
        functions.push(function_define);

        let module = Rc::new(OpcodeModule::new("".to_string(), "".to_string(), Rc::new(KaramelAstType::None)));
        storage_builder.prepare(module.clone(), &KaramelAstType::None, 0, &mut context, &mut compiler_options)?;
        let result = compiler.generate_functions(module.clone(), &mut functions, &mut context);

        match result {
            Ok(_) => Err(KaramelErrorType::GeneralError("döndür geçerli bir fonksiyon parametresi değil".to_string())),
            Err(_) => Ok(())
        }
    }

    #[test]
    fn test_8() -> Result<(), KaramelErrorType> {
        let compiler = InterpreterCompiler {};
        KaramelCompilerContext::new();
        compiler.check_prohibited_names("test".to_string())
    }

    #[test]
    fn test_9() -> Result<(), KaramelErrorType> {
        let compiler = InterpreterCompiler {};
        KaramelCompilerContext::new();
        compiler.check_prohibited_names("abc".to_string())
    }

    #[test]
    fn test_10() -> Result<(), KaramelErrorType> {
        let compiler = InterpreterCompiler {};
        KaramelCompilerContext::new();
        match compiler.check_prohibited_names("sayı".to_string()) {
            Ok(_) => Err(KaramelErrorType::GeneralError("sayı tipi kullanılamaz".to_string())),
            _ => Ok(())
        }
    }

    #[test]
    fn test_11() -> Result<(), KaramelErrorType> {
        let compiler = InterpreterCompiler {};
        KaramelCompilerContext::new();
        match compiler.check_prohibited_names("döndür".to_string()) {
            Ok(_) => Err(KaramelErrorType::GeneralError("sayı tipi kullanılamaz".to_string())),
            _ => Ok(())
        }
    }
}
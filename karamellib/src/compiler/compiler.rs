use std::vec::Vec;
use std::rc::Rc;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::cell::RefCell;

use ast::BramaDictItem;
use crate::types::*;
use crate::error::*;
use crate::compiler::*;
use crate::parser::*;
use crate::syntax::SyntaxParser;
use crate::compiler::value::BramaPrimative;
use crate::compiler::ast::{BramaAstType, BramaIfStatementElseItem};
use crate::compiler::storage_builder::StorageBuilder;
use crate::compiler::function::{FunctionReference};

use log;

use super::context::KaramelCompilerContext;
use super::function::find_function_definition_type;
use super::module::{OpcodeModule, get_modules};
use super::storage_builder::StorageBuilderOption;

pub struct FunctionDefine {
    arguments: Vec<String>,
    body: Rc<BramaAstType>,
    reference: Rc<FunctionReference>
}

pub struct InterpreterCompiler;
impl InterpreterCompiler {   
    pub fn compile(&self, main_ast: Rc<BramaAstType>, options: &mut KaramelCompilerContext) -> CompilerResult {
        let storage_builder: StorageBuilder = StorageBuilder::new();
        let mut compiler_options = StorageBuilderOption { max_stack: 0 };

        self.add_initial_jump(options);
        
        /* Save all function information */
        let modules = self.detect_modules(main_ast.clone(), options)?;
        let main_module = self.prepare_main_module(main_ast.clone(), options)?;
        //self.prepare_modules(options)?;

        storage_builder.prepare(&*main_module, &*main_ast, 0, options, &mut compiler_options);

        /* First part of the codes are functions */
        let mut functions = Vec::new();
        for module in modules.iter() {
            self.get_function_definations(module.clone(), module.main_ast.clone(), &mut functions, options, module.storage_index)?;
        }

        self.get_function_definations(main_module.clone(), main_ast.clone(), &mut functions, options, 0)?;

        self.generate_functions(main_module.clone(), &mut functions, options)?;

        /* Prepare jump code for main function */
        let current_location = options.opcodes.len();
        options.opcodes[1] = current_location as u8;
        options.opcodes[2] = (current_location >> 8) as u8;

        /* Generate main function code */
        self.generate_opcode(main_module.clone(), &*main_ast, &BramaAstType::None, options, 0)?;
        options.opcodes.push(VmOpCode::Halt as u8);
        options.opcodes_ptr = options.opcodes.as_mut_ptr();

        Ok(())
    }

    pub fn add_initial_jump(&self, options: &mut KaramelCompilerContext) {
        /* Jump over all function definations to main function */
        options.opcodes.push(VmOpCode::Jump as u8);
        options.opcodes.push(0_u8);
        options.opcodes.push(0_u8);
    }

    pub fn detect_modules(&self, main_ast: Rc<BramaAstType>, options: &mut KaramelCompilerContext) -> Result<Vec<Rc<OpcodeModule>>, String> {
        let modules = get_modules(main_ast.clone(), options)?;

        for module in modules.iter() {
            options.add_module(module.clone());
        }
        Ok(modules)
    }

    pub fn prepare_main_module(&self, main_ast: Rc<BramaAstType>, options: &mut KaramelCompilerContext) -> Result<Rc<OpcodeModule>, String> {
        let module = OpcodeModule::new("!baz".to_string(), String::new(), main_ast.clone());
        let module = Rc::new(module);
        options.main_module = module.as_ref() as *const OpcodeModule as *mut OpcodeModule;
        options.add_module(module.clone());

        unsafe { find_function_definition_type(options.main_module.as_mut().unwrap(), main_ast.clone(), options, 0, true)?; }
        Ok(module.clone())
    }

    pub fn prepare_modules(&self, options: &mut KaramelCompilerContext) -> CompilerResult {
        let mut functions = Vec::new();

        for (_, module) in options.modules.iter() {
            for (_, function_pointer) in module.get_methods() {
                functions.push(function_pointer.clone());
            }
        }

        for reference in functions {
            options.add_function(reference);
        }
        Ok(())
    }

    fn get_function_definations(&self, module: Rc<OpcodeModule>, ast: Rc<BramaAstType>, functions: &mut Vec<FunctionDefine>, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult{
        match &*ast {
            BramaAstType::FunctionDefination { name, arguments, body  } => {
                let search = options.get_function(name.to_string(), [module.name.clone()].to_vec(), storage_index);
                match search {
                    Some(reference) => {
                        println!("Function: {}", name);

                        functions.push(FunctionDefine {
                            arguments: arguments.to_vec(),
                            body: body.clone(),
                            reference: reference.clone()
                        });

                        self.get_function_definations(module.clone(), body.clone(), functions, options, reference.storage_index)?;
                    },

                    None => return Err(format!("'{}' fonksiyonu bulunamadı", name))
                };
            },
            BramaAstType::Block(blocks) => {
                for block in blocks {
                    self.get_function_definations(module.clone(), block.clone(), functions, options, storage_index)?;
                }
            },
            _ => ()
        };

        Ok(())
    }

    fn generate_functions(&self, module: Rc<OpcodeModule>, functions: &mut Vec<FunctionDefine>, options: &mut KaramelCompilerContext) -> CompilerResult {
        for function in functions {
            options.opcodes.push(VmOpCode::Func as u8);
            (*function.reference).opcode_location.set(options.opcodes.len());
            options.opcodes.push(function.arguments.len() as u8);

            if !function.arguments.is_empty() {
                options.opcodes.push(VmOpCode::InitArguments as u8);
                options.opcodes.push(function.arguments.len() as u8);
            }

            self.generate_opcode(module.clone(), &function.body, &function.body, options, function.reference.storage_index as usize)?;
        }

        Ok(())
    }

    fn generate_opcode(&self, module: Rc<OpcodeModule>, ast: &BramaAstType, upper_ast: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        match ast {
            BramaAstType::Assignment { variable, operator, expression } => self.generate_assignment(module.clone(), variable, operator, expression, options, storage_index),
            BramaAstType::Symbol(variable) => self.generate_symbol(module.clone(), variable, upper_ast, options, storage_index),
            BramaAstType::Control { left, operator, right } => self.generate_control(module.clone(), left, operator, right, upper_ast, options, storage_index),
            BramaAstType::Binary { left, operator, right } => self.generate_binary(module.clone(), left, operator, right, upper_ast, options, storage_index),
            BramaAstType::Block(asts) => self.generate_block(module.clone(), asts, upper_ast, options, storage_index),
            BramaAstType::Primative(primative) => self.generate_primative(primative.clone(), upper_ast, options, storage_index),
            BramaAstType::List(list) => self.generate_list(module.clone(), list, upper_ast, options, storage_index),
            BramaAstType::Dict(dict) => self.generate_dict(module.clone(), dict, upper_ast, options, storage_index),
            BramaAstType::FuncCall { func_name_expression, arguments, assign_to_temp } => self.generate_func_call(module.clone(), func_name_expression, arguments, assign_to_temp.get(), upper_ast, options, storage_index),
            BramaAstType::AccessorFuncCall { source, indexer, assign_to_temp } => self.generate_accessor_func_call(module.clone(), source, indexer, assign_to_temp.get(), upper_ast, options, storage_index),
            BramaAstType::PrefixUnary (operator, expression) => self.generate_prefix_unary(module.clone(), operator, expression, upper_ast, options, storage_index),
            BramaAstType::SuffixUnary (operator, expression) => self.generate_suffix_unary(operator, expression, upper_ast, options, storage_index),
            BramaAstType::NewLine => Ok(()),
            BramaAstType::WhileLoop { control, body } => self.generate_whileloop(module.clone(), control, body, upper_ast, options, storage_index),
            BramaAstType::EndlessLoop(expression) => self.generate_endlessloop(module.clone(), expression, upper_ast, options, storage_index),
            BramaAstType::Break => self.generate_break(upper_ast, options, storage_index),
            BramaAstType::Continue => self.generate_continue(upper_ast, options, storage_index),
            BramaAstType::Return(expression) => self.generate_return(module.clone(), expression, upper_ast, options, storage_index),
            BramaAstType::IfStatement {condition, body, else_body, else_if} => self.generate_if_condition(module.clone(),condition, body, else_body, else_if, upper_ast, options, storage_index),
            BramaAstType::Indexer {body, indexer} => self.generate_indexer(module.clone(), body, indexer, upper_ast, options, storage_index),
            BramaAstType::None => self.generate_none(options, storage_index),
            BramaAstType::FunctionDefination{name: _, arguments: _, body: _} => Ok(()),
            BramaAstType::FunctionMap(name) => self.generate_function_map(name, options, storage_index),
            BramaAstType::Load(names) => self.generate_load_module(names, options),
        }
    }

    fn generate_primative(&self, primative: Rc<BramaPrimative>, _: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        let storage = &options.storages[storage_index];

        let result = storage.get_constant_location(primative);
        match result {
            Some(index) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(index as u8);
                Ok(())
            },
            _ => Err("Value not found in storage".to_string())
        }
    }

    fn generate_load_module(&self, params: &[String], options: &mut KaramelCompilerContext) -> CompilerResult {
        let mut path = PathBuf::from(&options.script_path[..]);
        let module = &params[(params.len() - 1)];

        for item in params.iter().take(params.len() - 1) {
            path.push(item);
        }

        path.push(format!("{}.tpd", module));

        if path.is_file() {
            let content = match File::open(path.to_str().unwrap()) {
                Ok(mut file) => {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    contents
                },
                Err(error) => return Err(format!("Dosya okuma hatası oldu. Hata : {:?}", error))
            };


            let mut parser = Parser::new(&content);
            match parser.parse() {
                Err(error) => return Err(generate_error_message(&content, &error)),
                _ => ()
            };

            let syntax = SyntaxParser::new(parser.tokens().to_vec());
            let ast = match syntax.parse() {
                Ok(ast) => ast,
                Err(error) => return Err(generate_error_message(&content, &error))
            };

            println!("{:?}", ast);
        }
        return Ok(());
    }

    fn generate_function_map(&self, params: &[String], options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        let storage = &options.storages[storage_index];

        let name = params[params.len() - 1].to_string();
        let module_path = params[0..(params.len() - 1)].to_vec();

        let function_search = options.get_function(name, module_path, storage_index);
        match function_search {
            Some(reference) => {
                let result = storage.get_constant_location(Rc::new(BramaPrimative::Function(reference, None)));
                match result {
                    Some(index) => {
                        options.opcodes.push(VmOpCode::Load as u8);
                        options.opcodes.push(index as u8);
                        Ok(())
                    },
                    _ => Err("Function not found in storage".to_string())
                }
            },
            None => Err("Function not found in storage".to_string())
        }
    }

    fn generate_none(&self, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        let storage = &options.storages[storage_index];

        let result = storage.get_constant_location(Rc::new(BramaPrimative::Empty));
        match result {
            Some(index) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(index as u8);
                Ok(())
            },
            _ => Err("Value not found in storage".to_string())
        }
    }

    fn generate_list(&self, module: Rc<OpcodeModule>, list: &Vec<Box<BramaAstType>>, upper_ast: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        for item in list.iter().rev() {
            self.generate_opcode(module.clone(), item, upper_ast, options, storage_index)?;
        }
        options.opcodes.push(VmOpCode::InitList as u8);
        options.opcodes.push(list.len() as u8);
        Ok(())
    }

    fn generate_dict(&self, module: Rc<OpcodeModule>, dict: &Vec<Box<BramaDictItem>>, upper_ast: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        for item in dict.iter().rev() {
            self.generate_primative(item.key.clone(), upper_ast, options, storage_index)?;
            self.generate_opcode(module.clone(), &item.value, upper_ast, options, storage_index)?;
        }
        options.opcodes.push(VmOpCode::InitDict as u8);
        options.opcodes.push(dict.len() as u8);
        Ok(())
    }

    fn generate_func_call_by_name(&self, name :&String, module_path: Vec<String>, arguments: &Vec<Box<BramaAstType>>, assign_to_temp: bool, options: &mut KaramelCompilerContext, storage_index: usize) -> Result<bool, String> {
        let function_search = options.get_function(name.to_string(), module_path, storage_index);

        match function_search {
            Some(function_ref) => {
                let search_location = options.storages[storage_index].get_constant_location(Rc::new(BramaPrimative::Function(function_ref, None)));
                match search_location {
                    Some(location) => {
                        options.opcodes.push(VmOpCode::Call as u8);
                        options.opcodes.push(location as u8);
                        options.opcodes.push(arguments.len() as u8);
                        options.opcodes.push(assign_to_temp as u8);
                        return Ok(true);
                    },
                    _ => return Err("Function not found".to_string())
                }
            },
            _ => ()
        };

        match options.storages[storage_index].get_variable_location(&name) {
            /* Variable found */
            Some(location) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(location as u8);

                options.opcodes.push(VmOpCode::CallStack as u8);
                options.opcodes.push(arguments.len() as u8);
                options.opcodes.push(true as u8);
                return Ok(true);
            },
            /* Variable not found, lets check for function */
            None => ()
        };

        Ok(false)
    }

    fn generate_accessor_func_call(&self, module: Rc<OpcodeModule>, source: &BramaAstType, indexer: &BramaAstType, _assign_to_temp: bool,  upper_ast: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {

        if let BramaAstType::FuncCall { func_name_expression, arguments, assign_to_temp: _ } = indexer {
            match &**func_name_expression {
                BramaAstType::Symbol(function_name) => {
                            /* Build arguments */
                    for argument in arguments {
                        self.generate_opcode(module.clone(), argument, upper_ast, options, storage_index)?;
                    }
                    
                    self.generate_opcode(module.clone(), source, &BramaAstType::None, options, storage_index)?;
                    //todo: Pass real object to function as a parameter.
                    
                    let search_location = options.storages[storage_index].get_constant_location(Rc::new(BramaPrimative::Text(Rc::new(function_name.to_string()))));
                    match search_location {
                        Some(location) => {
                            options.opcodes.push(VmOpCode::Load as u8);
                            options.opcodes.push(location as u8);
                            options.opcodes.push(VmOpCode::GetItem as u8);
                            
                            options.opcodes.push(VmOpCode::CallStack as u8);
                            options.opcodes.push(arguments.len() as u8);
                            options.opcodes.push(true as u8);
                            /*options.opcodes.push(arguments.len() as u8);
                            options.opcodes.push(assign_to_temp as u8);*/
                            return Ok(());
                        },
                        _ => return Err("Function not found".to_string())
                    }
                },
                
                BramaAstType::FuncCall {
                    func_name_expression,
                    arguments,
                    assign_to_temp: _
                }=> {
                    return self.generate_func_call(module.clone(), func_name_expression, arguments, true, upper_ast, options, storage_index);
                },
                _ => {
                    return Err("Function not found".to_string());
                }
            }   
        }
        else {
            return Err("Function not found".to_string());
        }
    }

    fn generate_func_call(&self, module: Rc<OpcodeModule>, func_name_expression: &BramaAstType, arguments: &Vec<Box<BramaAstType>>, assign_to_temp: bool,  upper_ast: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        /* Build arguments */
        for argument in arguments {
            self.generate_opcode(module.clone(), argument, upper_ast, options, storage_index)?;
        }

        match &func_name_expression {
            BramaAstType::Symbol(function_name) => {
                let result = self.generate_func_call_by_name(&function_name, [module.name.clone()].to_vec(), &arguments, assign_to_temp, options, storage_index)?;
                match result {
                    true => return Ok(()),
                    false => {
                        log::debug!("{:?}", function_name);
                        return Err("Function not found".to_string());
                    }
                }
            },

            BramaAstType::FuncCall {func_name_expression, arguments: inner_arguments, assign_to_temp: _} => {
                self.generate_func_call(module.clone(), func_name_expression, inner_arguments, true, upper_ast, options, storage_index)?;
                options.opcodes.push(VmOpCode::CallStack as u8);
                options.opcodes.push(arguments.len() as u8);
                options.opcodes.push(true as u8);

                return Ok(());
            },

            BramaAstType::FunctionMap(names) => {
                let result = self.generate_func_call_by_name(&names[names.len() - 1].to_string(), names[0..(names.len()-1)].to_vec(), &arguments, assign_to_temp, options, storage_index)?;
                match result {
                    true => return Ok(()),
                    false =>  return Err("Function not found".to_string())
                }
            },
            _ => {
                self.generate_opcode(module.clone(), func_name_expression, upper_ast, options, storage_index)?;
                options.opcodes.push(VmOpCode::CallStack as u8);
                options.opcodes.push(arguments.len() as u8);
                options.opcodes.push(true as u8);
                return Ok(());
            }
        }
    }

    fn generate_break(&self, _: &BramaAstType, options: &mut KaramelCompilerContext, _: usize) -> CompilerResult {       
        options.opcodes.push(VmOpCode::Jump as u8);
        options.loop_breaks.push(options.opcodes.len());
        options.opcodes.push(0);
        options.opcodes.push(0);
        Ok(())
    }

    fn generate_continue(&self, _: &BramaAstType, options: &mut KaramelCompilerContext, _: usize) -> CompilerResult {       
        options.opcodes.push(VmOpCode::Jump as u8);
        options.loop_continues.push(options.opcodes.len());
        options.opcodes.push(0);
        options.opcodes.push(0);
        Ok(())
    }

    fn generate_return(&self, module: Rc<OpcodeModule>, expression: &BramaAstType, upper_ast: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        self.generate_opcode(module.clone(), expression, upper_ast, options, storage_index)?;
        options.opcodes.push(VmOpCode::Return as u8);
        Ok(())
    }

    fn generate_whileloop(&self, module: Rc<OpcodeModule>, control: &BramaAstType, body: &BramaAstType, upper_ast: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        /* Backup loop informations */
        let loop_breaks = options.loop_breaks.to_vec();
        let loop_continues = options.loop_continues.to_vec();
        
        let start_location = options.opcodes.len(); 
        self.generate_opcode(module.clone(), control, upper_ast, options, storage_index)?;
        options.opcodes.push(VmOpCode::Compare as u8);
        let compare_location = options.opcodes.len();

        options.opcodes.push(0_u8);
        options.opcodes.push(0_u8);

        self.generate_opcode(module.clone(), body, upper_ast, options, storage_index)?;

        options.opcodes.push(VmOpCode::Jump as u8);
        options.opcodes.push(start_location as u8);
        options.opcodes.push((start_location >> 8) as u8);

        let current_location = options.opcodes.len(); 

        for break_info in &options.loop_breaks {
            options.opcodes[*break_info]     = current_location as u8;
            options.opcodes[*break_info + 1] = (current_location >> 8) as u8;
        } 

        for continue_info in &options.loop_continues {
            options.opcodes[*continue_info]     = start_location as u8;
            options.opcodes[*continue_info + 1] = (start_location >> 8) as u8;
        } 

        options.loop_breaks    = loop_breaks.to_vec();
        options.loop_continues = loop_continues.to_vec();

        let end_location = current_location - compare_location;
        options.opcodes[compare_location]     = end_location as u8;
        options.opcodes[compare_location + 1] = (end_location >> 8) as u8;

        Ok(())
    }

    fn generate_endlessloop(&self, module: Rc<OpcodeModule>, expression: &BramaAstType, upper_ast: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        /* Backup loop informations */
        let loop_breaks = options.loop_breaks.to_vec();
        let loop_continues = options.loop_continues.to_vec();
        
        let start_location = options.opcodes.len(); 
        self.generate_opcode(module.clone(), expression, upper_ast, options, storage_index)?;

        /* 
        1. ... Endless loop start location
        2. ...
        3. ...
        4. ...
        5. Jump to  1.*/
        options.opcodes.push(VmOpCode::Jump as u8);
        options.opcodes.push(start_location as u8);
        options.opcodes.push((start_location >> 8) as u8);

        let current_location = options.opcodes.len(); 

        for break_info in &options.loop_breaks {
            options.opcodes[*break_info]     = current_location as u8;
            options.opcodes[*break_info + 1] = (current_location >> 8) as u8;
        } 

        for continue_info in &options.loop_continues {
            options.opcodes[*continue_info]     = start_location as u8;
            options.opcodes[*continue_info + 1] = (start_location >> 8) as u8;
        } 

        options.loop_breaks = loop_breaks.to_vec();
        options.loop_continues = loop_continues.to_vec();

        Ok(())
    }

    fn generate_symbol(&self, module: Rc<OpcodeModule>, variable: &String, _: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        let storage = &options.storages[storage_index];                
        let result = storage.get_function_constant(variable.to_string(), [module.name.clone()].to_vec());
        match result {
            Some(index) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(index as u8);
                return Ok(())
            },
            _ => ()
        };

        let result = storage.get_class_constant(variable.to_string(), [module.name.clone()].to_vec());
        match result {
            Some(index) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(index as u8);
                return Ok(())
            },
            _ => ()
        };

        match storage.get_variable_location(variable) {
            /* Variable found */
            Some(location) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(location as u8);
                Ok(())
            },
            /* Variable not found, lets check for function */
            None => return Err("Value not found in storage".to_string())
        }
    }

    fn generate_control(&self, module: Rc<OpcodeModule>, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        self.generate_opcode(module.clone(), left_ast, &BramaAstType::None, options, storage_index)?;
        self.generate_opcode(module.clone(), right_ast, &BramaAstType::None, options, storage_index)?;

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
        Ok(())
    }

    fn generate_assignment(&self, module: Rc<OpcodeModule>, variable: &BramaAstType, operator: &BramaOperatorType, expression_ast: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        match variable {
            BramaAstType::Symbol(symbol) => {
                let location = options.storages.get_mut(storage_index).unwrap().add_variable(&*symbol);
                let storage = &options.storages[storage_index];
                
                if let BramaAstType::Primative(primative) = expression_ast {
                    if mem::discriminant(&**primative) != mem::discriminant(&BramaPrimative::List(RefCell::new(Vec::new()))) && 
                    *operator == BramaOperatorType::Assign {
                        let result = storage.get_constant_location(primative.clone());
                        let primative_location = match result {
                            Some(index) => index as u8,
                            _ => return Err("Value not found in storage".to_string())
                        };

                        options.opcodes.push(VmOpCode::FastStore as u8);
                        options.opcodes.push(location);
                        options.opcodes.push(primative_location);
                        return Ok(());
                    }
                }

                if *operator != BramaOperatorType::Assign {

                    /* Load variable data to stack */
                    options.opcodes.push(VmOpCode::Load as u8);
                    options.opcodes.push(location);

                    self.generate_opcode(module.clone(), expression_ast, &BramaAstType::None, options, storage_index)?;

                    let opcode = match operator {
                        BramaOperatorType::AssignAddition       => VmOpCode::Addition as u8,
                        BramaOperatorType::AssignDivision       => VmOpCode::Division as u8,
                        BramaOperatorType::AssignMultiplication => VmOpCode::Multiply as u8,
                        BramaOperatorType::AssignSubtraction    => VmOpCode::Subraction as u8,
                        _ => BramaOperatorType::None as u8
                    };

                    options.opcodes.push(opcode);
                } else {
                    self.generate_opcode(module.clone(), expression_ast, &BramaAstType::None, options, storage_index)?;
                }
                
                options.opcodes.push(VmOpCode::Store as u8);
                options.opcodes.push(location);
                Ok(())
            },

            BramaAstType::Indexer {body, indexer} => {
                self.generate_opcode(module.clone(), body, &BramaAstType::None, options, storage_index)?;
                self.generate_opcode(module.clone(), indexer, &BramaAstType::None, options, storage_index)?;

                self.generate_opcode(module.clone(), expression_ast, &BramaAstType::None, options, storage_index)?;
                
                options.opcodes.push(VmOpCode::SetItem as u8);
                Ok(())
            },
            _ => Ok(())
        }
    }

    fn generate_binary(&self, module: Rc<OpcodeModule>, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult { 
        self.generate_opcode(module.clone(), left_ast, &BramaAstType::None, options, storage_index)?;
        self.generate_opcode(module.clone(), right_ast, &BramaAstType::None, options, storage_index)?;
        let opcode = match operator {
            BramaOperatorType::Addition       => VmOpCode::Addition as u8,
            BramaOperatorType::Subtraction    => VmOpCode::Subraction as u8,
            BramaOperatorType::Multiplication => VmOpCode::Multiply as u8,
            BramaOperatorType::Division       => VmOpCode::Division as u8,
            BramaOperatorType::Modulo         => VmOpCode::Module as u8,
            _ => VmOpCode::None as u8
        };

        options.opcodes.push(opcode);
        Ok(())
    }

    fn generate_prefix_unary(&self, module: Rc<OpcodeModule>, operator: &BramaOperatorType, expression: &BramaAstType, _: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult { 
        
        if *operator == BramaOperatorType::Not { 
            return self.generate_not(module.clone(), expression, options, storage_index);
        }

        if let BramaAstType::Symbol(variable) = expression {
            let location = match options.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
                Some(location) => location,
                _ => return Err("Variable not found in storage".to_string())
            };

            /* Load data from memory */
            options.opcodes.push(VmOpCode::Load as u8);
            options.opcodes.push(location);
        
            let opcode = match operator {
                BramaOperatorType::Increment  => VmOpCode::Increment as u8,
                BramaOperatorType::Deccrement => VmOpCode::Decrement as u8,
                _ => return Err("Unary operator not found".to_string())
            };
    
            options.opcodes.push(opcode);
            options.opcodes.push(VmOpCode::CopyToStore as u8);
            options.opcodes.push(location);
            return Ok(());
        }

        Err("Unary expression not valid".to_string())
    }

    fn generate_not(&self, module: Rc<OpcodeModule>, expression: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult { 
        self.generate_opcode(module.clone(), expression, &BramaAstType::None, options, storage_index)?;
        options.opcodes.push(VmOpCode::Not as u8);
        Ok(())
    }

    fn create_exit_jump(&self, options: &mut KaramelCompilerContext, exit_locations: &mut Vec<usize>) {
        options.opcodes.push(VmOpCode::Jump as u8);
        exit_locations.push(options.opcodes.len());
        options.opcodes.push(0_u8);
        options.opcodes.push(0_u8);
    }

    fn create_compare(&self, options: &mut KaramelCompilerContext) -> usize {
        options.opcodes.push(VmOpCode::Compare as u8);
        let compare_location = options.opcodes.len();

        options.opcodes.push(0_u8);
        options.opcodes.push(0_u8);

        compare_location
    }

    fn build_jump_location(&self, options: &mut KaramelCompilerContext, jump_location: usize) {
        let current_location = options.opcodes.len();
        options.opcodes[jump_location]     = current_location as u8;
        options.opcodes[jump_location + 1] = (current_location >> 8) as u8;
    }

    fn build_compare_location(&self, options: &mut KaramelCompilerContext, jump_location: usize) {
        let current_location = options.opcodes.len() - jump_location;
        options.opcodes[jump_location]     = current_location as u8;
        options.opcodes[jump_location + 1] = (current_location >> 8) as u8;
    }

    fn generate_if_condition(&self, module: Rc<OpcodeModule>, condition: &BramaAstType, body: &BramaAstType, else_body: &Option<Box<BramaAstType>>, else_if: &Vec<Box<BramaIfStatementElseItem>>, upper_ast: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
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
        
        self.generate_opcode(module.clone(), condition, upper_ast, options, storage_index)?;
        let mut if_failed_location = self.create_compare(options);
        self.generate_opcode(module.clone(), body, upper_ast, options, storage_index)?;

        if !else_if.is_empty() || else_body.is_some() {
            /* After executing body, need to exit from 'if condition'.
               Jump to out of if condition */
            self.create_exit_jump(options, &mut exit_locations);
        }

        for else_if_item in else_if {
            /* Previous conditon should jump to this location */
            self.build_compare_location(options, if_failed_location);

            /* Build condition */
            self.generate_opcode(module.clone(), &else_if_item.condition, upper_ast, options, storage_index)?;

            if_failed_location = self.create_compare(options);

            self.generate_opcode(module.clone(), &else_if_item.body, upper_ast, options, storage_index)?;

            /* Jump to out of if condition */
            self.create_exit_jump(options, &mut exit_locations);
        }

        if let Some(_else_body) = else_body {
            self.build_compare_location(options, if_failed_location);
            self.generate_opcode(module.clone(), _else_body, upper_ast, options, storage_index)?;
        }
        else {
            self.build_compare_location(options, if_failed_location);
        }

        for exit_location in exit_locations {
            self.build_jump_location(options, exit_location);
        }

        Ok(())
    }

    fn generate_indexer(&self, module: Rc<OpcodeModule>, body: &BramaAstType, indexer: &BramaAstType, upper_ast: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        self.generate_opcode(module.clone(), body, upper_ast, options, storage_index)?;
        self.generate_opcode(module.clone(), indexer, upper_ast, options, storage_index)?;
        options.opcodes.push(VmOpCode::GetItem as u8);

        Ok(())
    }

    fn generate_suffix_unary(&self, operator: &BramaOperatorType, expression: &BramaAstType, _: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult { 
        if let BramaAstType::Symbol(variable) = expression {
            let location = match options.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
                Some(location) => location,
                _ => return Err("Variable not found in storage".to_string())
            };

            options.opcodes.push(VmOpCode::Load as u8);
            options.opcodes.push(location);
            options.opcodes.push(VmOpCode::Dublicate as u8);

            let opcode = match operator {
                BramaOperatorType::Increment  => VmOpCode::Increment as u8,
                BramaOperatorType::Deccrement => VmOpCode::Decrement as u8,
                BramaOperatorType::Not        => VmOpCode::Not as u8,
                _ => return Err("Unary operator not found".to_string())
            };
    
            options.opcodes.push(opcode);
            options.opcodes.push(VmOpCode::Store as u8);
            options.opcodes.push(location);
            return Ok(());
        }

        Err("Unary expression not valid".to_string())
    }

    fn generate_block(&self, module: Rc<OpcodeModule>, asts: &[Rc<BramaAstType>], upper_ast: &BramaAstType, options: &mut KaramelCompilerContext, storage_index: usize) -> CompilerResult {
        for ast in asts {
            self.generate_opcode(module.clone(), &ast, upper_ast, options, storage_index)?;
        }
        Ok(())
    }
}
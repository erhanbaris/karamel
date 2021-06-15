use std::{borrow::Borrow, vec::Vec};
use std::rc::Rc;
use std::cell::RefCell;

use std::ptr;

use ast::BramaDictItem;
use crate::{types::*};
use crate::compiler::*;
use crate::buildin::*;
use crate::compiler::value::BramaPrimative;
use crate::compiler::ast::{BramaAstType, BramaIfStatementElseItem};
use crate::compiler::storage_builder::StorageBuilder;
use crate::compiler::function::{FunctionReference, FunctionType};
use crate::buildin::class::*;

use log;

#[derive(Clone)]
pub struct Scope {
    pub memory: Vec<VmObject>, 
    pub stack: Vec<VmObject>, 
    pub location: *mut u8,
    pub call_return_assign_to_temp: bool,
    pub const_size: u8,
    pub stack_ptr: *mut VmObject ,
    pub memory_ptr: *mut VmObject ,
    pub storage_index: isize
}

impl Scope {
    pub fn empty() -> Scope {
        let mut stack = Vec::new();
        let mut memory = Vec::new();
        
        let stack_ptr = stack.as_mut_ptr();
        let memory_ptr = memory.as_mut_ptr();

        Scope { 
            const_size: 0, 
            call_return_assign_to_temp: false, 
            location: ptr::null_mut(), 
            memory: memory,
            memory_ptr: memory_ptr,
            stack: stack,
            stack_ptr: stack_ptr,
            storage_index: -1
        }
    }
}

pub struct FunctionDefine {
    arguments: Vec<String>,
    body: Rc<BramaAstType>,
    reference: Rc<FunctionReference>
}

pub struct BramaCompiler {
    pub opcodes : Vec<u8>,
    pub storages: Vec<StaticStorage>,
    pub modules: ModuleCollection,
    pub loop_breaks: Vec<usize>,
    pub loop_continues: Vec<usize>,
    pub scopes: Vec<Scope>,
    pub current_scope: *mut Scope,
    pub scope_index: usize,
    pub functions : Vec<Rc<FunctionReference>>,
    pub classes : Vec<Rc<dyn Class >>,
    pub stdout: Option<RefCell<String>>,
    pub stderr: Option<RefCell<String>>,
    pub opcodes_ptr: *mut u8,
    pub primative_classes: Vec<Rc<dyn Class>>
}

impl  BramaCompiler {
    pub fn new() -> BramaCompiler {
        let mut compiler = BramaCompiler {
            opcodes: Vec::new(),
            storages: vec![StaticStorage::new()],
            modules: ModuleCollection::new(),
            loop_breaks: Vec::new(),
            loop_continues: Vec::new(),
            scopes: Vec::new(),
            current_scope: ptr::null_mut(),
            scope_index: 0,
            functions: Vec::new(),
            classes: Vec::new(),
            stdout: None,
            stderr: None,
            opcodes_ptr: ptr::null_mut(),
            primative_classes: Vec::new()
        };

        compiler.primative_classes.push(number::get_primative_class());
        compiler.primative_classes.push(text::get_primative_class());
        compiler.primative_classes.push(list::get_primative_class());
        compiler.primative_classes.push(dict::get_primative_class());
        compiler.primative_classes.push(get_empty_class());
        compiler.primative_classes.push(get_empty_class());
        compiler.primative_classes.push(get_empty_class());
        compiler.primative_classes.push(get_empty_class());
        compiler.primative_classes.push(proxy::get_primative_class());
        compiler.primative_classes.push(get_empty_class());

        for _ in 0..32{
            compiler.scopes.push(Scope::empty());
        }
        
        compiler.current_scope = &mut compiler.scopes[0] as *mut Scope;
        compiler
    }

    pub fn prepare_modules(&mut self) {
        let mut functions = Vec::new();

        for module in self.modules.modules.values() {
            let mut module_path = Vec::new();
            if !module.get_module_name().is_empty() {
                module_path = [module.get_module_name()].to_vec();
            }

            for (function_name, function_pointer) in module.get_methods() {
                let reference = FunctionReference::native_function(function_pointer, function_name.to_string(), module_path.to_vec(), "".to_string());
                functions.push(reference);
            }
        }

        for reference in functions {
            self.add_function(reference);
        }
    }

    pub fn add_function(&mut self, information: Rc<FunctionReference>) {
        self.functions.push(information);
    }

    pub fn add_class(&mut self, class_info: Rc<dyn Class + Sync + Send>) {
        self.classes.push(class_info.clone());
    }

    pub fn find_function(&self, name: String, module_path: Vec<String>, framework: String, start_storage_index: usize) -> Option<Rc<FunctionReference>> {
        let mut search_storage = start_storage_index;
        loop {

            /* Search function with storage */
            let function_location = self.functions.iter().position(|function_reference| {
                return match function_reference.callback {
                    FunctionType::Native(_) =>
                    function_reference.module_path   == module_path && 
                        function_reference.name          == name && 
                        function_reference.framework     == framework,
                    FunctionType::Opcode => 
                    function_reference.name          == name && 
                        function_reference.module_path   == module_path && 
                        function_reference.framework     == framework &&
                        function_reference.defined_storage_index == search_storage
                };
            });

            match function_location {
                Some(location) => return Some(self.functions[location].clone()),
                _ => ()
            };
            
            search_storage = match self.storages[search_storage].get_parent_location() {
                Some(location) => location as usize,
                None => return None
            };
        }
    }

    pub fn get_class(&self, value: &BramaPrimative) -> Rc<dyn Class > {
        unsafe {
            self.primative_classes.get_unchecked(value.discriminant()).clone()
        }
    }

    pub fn find_class(&self, name: String, _module_path: Vec<String>, _framework: String, _start_storage_index: usize) -> Option<Rc<dyn Class >> {
        let primative_search = self.primative_classes.iter().find(|&item| item.get_class_name() == name);
        match primative_search {
            Some(class) => Some(class.clone()),
            None => None
        }
    }

    pub fn reset(&mut self) {
        self.opcodes = Vec::new();
    }
}

pub trait Compiler
{
    fn compile(&self, ast: &BramaAstType, options: &mut BramaCompiler) -> CompilerResult;
}


pub struct InterpreterCompiler;
impl Compiler for InterpreterCompiler {   
    fn compile(&self, ast: &BramaAstType, options: &mut BramaCompiler) -> CompilerResult {
        let storage_builder: StorageBuilder = StorageBuilder::new();
        /* Save all function information */
        options.prepare_modules();
        storage_builder.prepare_variable_store(ast, options);

        /* Jump over all function definations to main function */
        options.opcodes.push(VmOpCode::Jump as u8);
        options.opcodes.push(0_u8);
        options.opcodes.push(0_u8);

        /* First part of the codes are functions */
        let mut functions = Vec::new();
        self.find_function_definations(ast, &mut functions, options, 0);
        self.generate_functions(&mut functions, options)?;

        /* If there are no function defination, remove previous opcodes */
        if options.opcodes.len() == 3 {
            options.opcodes.clear();
        }
        else {
            /* Prepare jump code for main function */
            let current_location = options.opcodes.len();
            options.opcodes[1] = current_location as u8;
            options.opcodes[2] = (current_location >> 8) as u8;
        }

        /* Generate main function code */
        self.generate_opcode(ast, &BramaAstType::None, options, 0)?;
        options.opcodes.push(VmOpCode::Halt as u8);
        options.opcodes_ptr = options.opcodes.as_mut_ptr();

        Ok(())
    }
}

impl InterpreterCompiler {
    fn find_function_definations(&self, ast: &BramaAstType, functions: &mut Vec<FunctionDefine>, options: &mut BramaCompiler, storage_index: usize) {
        match ast {
            BramaAstType::FunctionDefination { name, arguments, body  } => {
                let search = options.find_function(name.to_string(), Vec::new(), "".to_string(), storage_index);
                match search {
                    Some(reference) => {
                        functions.push(FunctionDefine {
                            arguments: arguments.to_vec(),
                            body: body.clone(),
                            reference: reference.clone()
                        });

                        self.find_function_definations(body, functions, options, reference.storage_index);
                    },

                    None => log::info!("Nope nope")
                };
            },
            BramaAstType::Block(blocks) => {
                for block in blocks {
                    self.find_function_definations(&block, functions, options, storage_index);
                }
            },
            _ => ()
        };
    }

    fn generate_functions(&self, functions: &mut Vec<FunctionDefine>, options: &mut BramaCompiler) -> CompilerResult {
        for function in functions {
            options.opcodes.push(VmOpCode::Func as u8);
            (*function.reference).opcode_location.set(options.opcodes.len());
            options.opcodes.push(function.arguments.len() as u8);

            if !function.arguments.is_empty() {
                options.opcodes.push(VmOpCode::InitArguments as u8);
                options.opcodes.push(function.arguments.len() as u8);
            }

            self.generate_opcode(&function.body, &function.body, options, function.reference.storage_index as usize)?;
        }

        Ok(())
    }

    fn generate_opcode(&self, ast: &BramaAstType, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        match ast {
            BramaAstType::Assignment { variable, operator, expression } => self.generate_assignment(variable, operator, expression, options, storage_index),
            BramaAstType::Symbol(variable) => self.generate_symbol(variable, upper_ast, options, storage_index),
            BramaAstType::Control { left, operator, right } => self.generate_control(left, operator, right, upper_ast, options, storage_index),
            BramaAstType::Binary { left, operator, right } => self.generate_binary(left, operator, right, upper_ast, options, storage_index),
            BramaAstType::Block(asts) => self.generate_block(asts, upper_ast, options, storage_index),
            BramaAstType::Primative(primative) => self.generate_primative(primative.clone(), upper_ast, options, storage_index),
            BramaAstType::List(list) => self.generate_list(list, upper_ast, options, storage_index),
            BramaAstType::Dict(dict) => self.generate_dict(dict, upper_ast, options, storage_index),
            BramaAstType::FuncCall { func_name_expression, arguments, assign_to_temp } => self.generate_func_call(func_name_expression, arguments, *assign_to_temp, upper_ast, options, storage_index),
            BramaAstType::AccessorFuncCall { source, indexer, assign_to_temp } => self.generate_accessor_func_call(source, indexer, *assign_to_temp, upper_ast, options, storage_index),
            BramaAstType::PrefixUnary (operator, expression) => self.generate_prefix_unary(operator, expression, upper_ast, options, storage_index),
            BramaAstType::SuffixUnary (operator, expression) => self.generate_suffix_unary(operator, expression, upper_ast, options, storage_index),
            BramaAstType::NewLine => Ok(()),
            BramaAstType::WhileLoop { control, body } => self.generate_whileloop(control, body, upper_ast, options, storage_index),
            BramaAstType::EndlessLoop(expression) => self.generate_endlessloop(expression, upper_ast, options, storage_index),
            BramaAstType::Break => self.generate_break(upper_ast, options, storage_index),
            BramaAstType::Continue => self.generate_continue(upper_ast, options, storage_index),
            BramaAstType::Return(expression) => self.generate_return(expression, upper_ast, options, storage_index),
            BramaAstType::IfStatement {condition, body, else_body, else_if} => self.generate_if_condition(condition, body, else_body, else_if, upper_ast, options, storage_index),
            BramaAstType::Indexer {body, indexer} => self.generate_indexer(body, indexer, upper_ast, options, storage_index),
            BramaAstType::None => self.generate_none(options, storage_index),
            BramaAstType::FunctionDefination{name: _, arguments: _, body: _} => Ok(()),
            BramaAstType::FunctionMap(name) => self.generate_function_map(name, options, storage_index)
        }
    }

    fn generate_primative(&self, primative: Rc<BramaPrimative>, _: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        let storage = &options.storages[storage_index];

        let result = storage.get_constant_location(primative);
        match result {
            Some(index) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(index as u8);
                Ok(())
            },
            _ => Err("Value not found in storage")
        }
    }


    fn generate_function_map(&self, params: &[String], options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        let storage = &options.storages[storage_index];

        let name = params[params.len() - 1].to_string();
        let module_path = params[0..(params.len() - 1)].to_vec();

        let function_search = options.find_function(name, module_path, "".to_string(), storage_index);
        match function_search {
            Some(reference) => {
                let result = storage.get_constant_location(Rc::new(BramaPrimative::Function(reference, None)));
                match result {
                    Some(index) => {
                        options.opcodes.push(VmOpCode::Load as u8);
                        options.opcodes.push(index as u8);
                        Ok(())
                    },
                    _ => Err("Function not found in storage")
                }
            },
            None => Err("Function not found in storage")
        }
    }

    fn generate_none(&self, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        let storage = &options.storages[storage_index];

        let result = storage.get_constant_location(Rc::new(BramaPrimative::Empty));
        match result {
            Some(index) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(index as u8);
                Ok(())
            },
            _ => Err("Value not found in storage")
        }
    }

    fn generate_list(&self, list: &Vec<Box<BramaAstType>>, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        for item in list.iter().rev() {
            self.generate_opcode(item, upper_ast, options, storage_index)?;
        }
        options.opcodes.push(VmOpCode::InitList as u8);
        options.opcodes.push(list.len() as u8);
        Ok(())
    }

    fn generate_dict(&self, dict: &Vec<Box<BramaDictItem>>, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        for item in dict.iter().rev() {
            self.generate_primative(item.key.clone(), upper_ast, options, storage_index)?;
            self.generate_opcode(&item.value, upper_ast, options, storage_index)?;
        }
        options.opcodes.push(VmOpCode::InitDict as u8);
        options.opcodes.push(dict.len() as u8);
        Ok(())
    }

    fn generate_func_call_by_name(&self, name :&String, module_path: Vec<String>, framework: String, arguments: &Vec<Box<BramaAstType>>, assign_to_temp: bool, options: &mut BramaCompiler, storage_index: usize) -> Result<bool, &'static str> {
        let function_search = options.find_function(name.to_string(), module_path, framework, storage_index);

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
                    _ => return Err("Function not found")
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

    fn generate_accessor_func_call(&self, source: &BramaAstType, indexer: &BramaAstType, _assign_to_temp: bool,  upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {

        if let BramaAstType::FuncCall { func_name_expression, arguments, assign_to_temp: _ } = indexer {
            match &**func_name_expression {
                BramaAstType::Symbol(function_name) => {
                            /* Build arguments */
                    for argument in arguments {
                        self.generate_opcode(argument, upper_ast, options, storage_index)?;
                    }
                    
                    self.generate_opcode(source, &BramaAstType::None, options, storage_index)?;
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
                        _ => return Err("Function not found")
                    }
                },
                
                BramaAstType::FuncCall {
                    func_name_expression,
                    arguments,
                    assign_to_temp: _
                }=> {
                    return self.generate_func_call(func_name_expression, arguments, true, upper_ast, options, storage_index);
                },
                _ => {
                    return Err("Function not found");
                }
            }   
        }
        else {
            return Err("Function not found");
        }
    }

    fn generate_func_call(&self, func_name_expression: &BramaAstType, arguments: &Vec<Box<BramaAstType>>, assign_to_temp: bool,  upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        /* Build arguments */
        for argument in arguments {
            self.generate_opcode(argument, upper_ast, options, storage_index)?;
        }

        match &func_name_expression {
            BramaAstType::Symbol(function_name) => {
                let result = self.generate_func_call_by_name(&function_name, Vec::new(), "".to_string(), &arguments, assign_to_temp, options, storage_index)?;
                match result {
                    true => return Ok(()),
                    false => {
                        log::debug!("{:?}", function_name);
                        return Err("Function not found");
                    }
                }
            },

            BramaAstType::FuncCall {func_name_expression, arguments: inner_arguments, assign_to_temp: _} => {
                self.generate_func_call(func_name_expression, inner_arguments, true, upper_ast, options, storage_index)?;
                options.opcodes.push(VmOpCode::CallStack as u8);
                options.opcodes.push(arguments.len() as u8);
                options.opcodes.push(true as u8);

                return Ok(());
            },

            BramaAstType::FunctionMap(names) => {
                let result = self.generate_func_call_by_name(&names[names.len() - 1].to_string(), names[0..(names.len()-1)].to_vec(), "".to_string(), &arguments, assign_to_temp, options, storage_index)?;
                match result {
                    true => return Ok(()),
                    false =>  return Err("Function not found")
                }
            },
            _ => {
                self.generate_opcode(func_name_expression, upper_ast, options, storage_index)?;
                options.opcodes.push(VmOpCode::CallStack as u8);
                options.opcodes.push(arguments.len() as u8);
                options.opcodes.push(true as u8);
                return Ok(());
            }
        }
    }

    fn generate_break(&self, _: &BramaAstType, options: &mut BramaCompiler, _: usize) -> CompilerResult {       
        options.opcodes.push(VmOpCode::Jump as u8);
        options.loop_breaks.push(options.opcodes.len());
        options.opcodes.push(0);
        options.opcodes.push(0);
        Ok(())
    }

    fn generate_continue(&self, _: &BramaAstType, options: &mut BramaCompiler, _: usize) -> CompilerResult {       
        options.opcodes.push(VmOpCode::Jump as u8);
        options.loop_continues.push(options.opcodes.len());
        options.opcodes.push(0);
        options.opcodes.push(0);
        Ok(())
    }

    fn generate_return(&self, expression: &BramaAstType, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        self.generate_opcode(expression, upper_ast, options, storage_index)?;
        options.opcodes.push(VmOpCode::Return as u8);
        Ok(())
    }

    fn generate_whileloop(&self, control: &BramaAstType, body: &BramaAstType, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        /* Backup loop informations */
        let loop_breaks = options.loop_breaks.to_vec();
        let loop_continues = options.loop_continues.to_vec();
        
        let start_location = options.opcodes.len(); 
        self.generate_opcode(control, upper_ast, options, storage_index)?;
        options.opcodes.push(VmOpCode::Compare as u8);
        let compare_location = options.opcodes.len();

        options.opcodes.push(0_u8);
        options.opcodes.push(0_u8);

        self.generate_opcode(body, upper_ast, options, storage_index)?;

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

    fn generate_endlessloop(&self, expression: &BramaAstType, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        /* Backup loop informations */
        let loop_breaks = options.loop_breaks.to_vec();
        let loop_continues = options.loop_continues.to_vec();
        
        let start_location = options.opcodes.len(); 
        self.generate_opcode(expression, upper_ast, options, storage_index)?;

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

    fn generate_symbol(&self, variable: &String, _: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        let storage = &options.storages[storage_index];                
        let result = storage.get_function_constant(variable.to_string(), Vec::new(), "".to_string());
        match result {
            Some(index) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(index as u8);
                return Ok(())
            },
            _ => ()
        };

        let result = storage.get_class_constant(variable.to_string(), Vec::new(), "".to_string());
        match result {
            Some(index) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(index as u8);
                return Ok(())
            },
            _ => ()
        };

        match options.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
            /* Variable found */
            Some(location) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(location as u8);
                Ok(())
            },
            /* Variable not found, lets check for function */
            None => return Err("Value not found in storage")
        }
    }

    fn generate_control(&self, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        self.generate_opcode(left_ast, &BramaAstType::None, options, storage_index)?;
        self.generate_opcode(right_ast, &BramaAstType::None, options, storage_index)?;

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

    fn generate_assignment(&self, variable: &BramaAstType, operator: &BramaOperatorType, expression_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        match variable {
            BramaAstType::Symbol(symbol) => {
                let location = options.storages.get_mut(storage_index).unwrap().add_variable(&*symbol);
                let storage = &options.storages[storage_index];
                
                if let BramaAstType::Primative(primative) = expression_ast {
                    if mem::discriminant(&**primative) != mem::discriminant(&BramaPrimative::List(RefCell::new([].to_vec()))) && 
                    *operator == BramaOperatorType::Assign {
                        let result = storage.get_constant_location(primative.clone());
                        let primative_location = match result {
                            Some(index) => index as u8,
                            _ => return Err("Value not found in storage")
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

                    self.generate_opcode(expression_ast, &BramaAstType::None, options, storage_index)?;

                    let opcode = match operator {
                        BramaOperatorType::AssignAddition       => VmOpCode::Addition as u8,
                        BramaOperatorType::AssignDivision       => VmOpCode::Division as u8,
                        BramaOperatorType::AssignMultiplication => VmOpCode::Multiply as u8,
                        BramaOperatorType::AssignSubtraction    => VmOpCode::Subraction as u8,
                        _ => BramaOperatorType::None as u8
                    };

                    options.opcodes.push(opcode);
                } else {
                    self.generate_opcode(expression_ast, &BramaAstType::None, options, storage_index)?;
                }
                
                options.opcodes.push(VmOpCode::Store as u8);
                options.opcodes.push(location);
                Ok(())
            },

            BramaAstType::Indexer {body, indexer} => {
                self.generate_opcode(body, &BramaAstType::None, options, storage_index)?;
                self.generate_opcode(indexer, &BramaAstType::None, options, storage_index)?;

                self.generate_opcode(expression_ast, &BramaAstType::None, options, storage_index)?;
                
                options.opcodes.push(VmOpCode::SetItem as u8);
                Ok(())
            },
            _ => Ok(())
        }
    }

    fn generate_binary(&self, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult { 
        self.generate_opcode(left_ast, &BramaAstType::None, options, storage_index)?;
        self.generate_opcode(right_ast, &BramaAstType::None, options, storage_index)?;
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

    fn generate_prefix_unary(&self, operator: &BramaOperatorType, expression: &BramaAstType, _: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult { 
        
        if *operator == BramaOperatorType::Not { 
            return self.generate_not(expression, options, storage_index);
        }

        if let BramaAstType::Symbol(variable) = expression {
            let location = match options.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
                Some(location) => location,
                _ => return Err("Variable not found in storage")
            };

            /* Load data from memory */
            options.opcodes.push(VmOpCode::Load as u8);
            options.opcodes.push(location);
        
            let opcode = match operator {
                BramaOperatorType::Increment  => VmOpCode::Increment as u8,
                BramaOperatorType::Deccrement => VmOpCode::Decrement as u8,
                _ => return Err("Unary operator not found")
            };
    
            options.opcodes.push(opcode);
            options.opcodes.push(VmOpCode::CopyToStore as u8);
            options.opcodes.push(location);
            return Ok(());
        }

        Err("Unary expression not valid")
    }

    fn generate_not(&self, expression: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult { 
        self.generate_opcode(expression, &BramaAstType::None, options, storage_index)?;
        options.opcodes.push(VmOpCode::Not as u8);
        Ok(())
    }

    fn create_exit_jump(&self, options: &mut BramaCompiler, exit_locations: &mut Vec<usize>) {
        options.opcodes.push(VmOpCode::Jump as u8);
        exit_locations.push(options.opcodes.len());
        options.opcodes.push(0_u8);
        options.opcodes.push(0_u8);
    }

    fn create_compare(&self, options: &mut BramaCompiler) -> usize {
        options.opcodes.push(VmOpCode::Compare as u8);
        let compare_location = options.opcodes.len();

        options.opcodes.push(0_u8);
        options.opcodes.push(0_u8);

        compare_location
    }

    fn build_jump_location(&self, options: &mut BramaCompiler, jump_location: usize) {
        let current_location = options.opcodes.len();
        options.opcodes[jump_location]     = current_location as u8;
        options.opcodes[jump_location + 1] = (current_location >> 8) as u8;
    }

    fn build_compare_location(&self, options: &mut BramaCompiler, jump_location: usize) {
        let current_location = options.opcodes.len() - jump_location;
        options.opcodes[jump_location]     = current_location as u8;
        options.opcodes[jump_location + 1] = (current_location >> 8) as u8;
    }

    fn generate_if_condition(&self, condition: &BramaAstType, body: &BramaAstType, else_body: &Option<Box<BramaAstType>>, else_if: &Vec<Box<BramaIfStatementElseItem>>, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
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
        
        self.generate_opcode(condition, upper_ast, options, storage_index)?;
        let mut if_failed_location = self.create_compare(options);
        self.generate_opcode(body, upper_ast, options, storage_index)?;

        if !else_if.is_empty() || else_body.is_some() {
            /* After executing body, need to exit from 'if condition'.
               Jump to out of if condition */
            self.create_exit_jump(options, &mut exit_locations);
        }

        for else_if_item in else_if {
            /* Previous conditon should jump to this location */
            self.build_compare_location(options, if_failed_location);

            /* Build condition */
            self.generate_opcode(&else_if_item.condition, upper_ast, options, storage_index)?;

            if_failed_location = self.create_compare(options);

            self.generate_opcode(&else_if_item.body, upper_ast, options, storage_index)?;

            /* Jump to out of if condition */
            self.create_exit_jump(options, &mut exit_locations);
        }

        if let Some(_else_body) = else_body {
            self.build_compare_location(options, if_failed_location);
            self.generate_opcode(_else_body, upper_ast, options, storage_index)?;
        }
        else {
            self.build_compare_location(options, if_failed_location);
        }

        for exit_location in exit_locations {
            self.build_jump_location(options, exit_location);
        }

        Ok(())
    }

    fn generate_indexer(&self, body: &BramaAstType, indexer: &BramaAstType, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        self.generate_opcode(body, upper_ast, options, storage_index)?;
        self.generate_opcode(indexer, upper_ast, options, storage_index)?;
        options.opcodes.push(VmOpCode::GetItem as u8);

        Ok(())
    }

    fn generate_suffix_unary(&self, operator: &BramaOperatorType, expression: &BramaAstType, _: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult { 
        if let BramaAstType::Symbol(variable) = expression {
            let location = match options.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
                Some(location) => location,
                _ => return Err("Variable not found in storage")
            };

            options.opcodes.push(VmOpCode::Load as u8);
            options.opcodes.push(location);
            options.opcodes.push(VmOpCode::Dublicate as u8);

            let opcode = match operator {
                BramaOperatorType::Increment  => VmOpCode::Increment as u8,
                BramaOperatorType::Deccrement => VmOpCode::Decrement as u8,
                BramaOperatorType::Not        => VmOpCode::Not as u8,
                _ => return Err("Unary operator not found")
            };
    
            options.opcodes.push(opcode);
            options.opcodes.push(VmOpCode::Store as u8);
            options.opcodes.push(location);
            return Ok(());
        }

        Err("Unary expression not valid")
    }

    fn generate_block(&self, asts: &[BramaAstType], upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        for ast in asts {
            self.generate_opcode(&ast, upper_ast, options, storage_index)?;
        }
        Ok(())
    }
}